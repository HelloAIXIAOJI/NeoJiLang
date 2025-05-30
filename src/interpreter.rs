use crate::error::NjilError;
use crate::types::{Function, NjilProgram};
use crate::statements;
use crate::statements::StatementHandler;
use crate::builtin::BuiltinModuleRegistry;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

pub struct Interpreter {
    pub(crate) variables: HashMap<String, Value>,
    returning: bool,
    module_registry: BuiltinModuleRegistry,
    loaded_modules: HashSet<String>,
    current_dir: Option<PathBuf>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            returning: false,
            module_registry: BuiltinModuleRegistry::new(),
            loaded_modules: HashSet::new(),
            current_dir: None,
        }
    }

    pub fn load_file<P: AsRef<Path>>(&mut self, path: P) -> Result<NjilProgram, NjilError> {
        let path = path.as_ref();
        
        // 设置当前目录为文件所在目录，用于相对路径导入
        if let Some(parent) = path.parent() {
            self.current_dir = Some(parent.to_path_buf());
        }
        
        let content = fs::read_to_string(path)?;
        let program: NjilProgram = serde_json::from_str(&content)?;
        Ok(program)
    }

    pub fn execute(&mut self, program: &NjilProgram) -> Result<Value, NjilError> {
        // 处理导入
        if let Some(imports) = &program.import {
            self.process_imports(imports)?;
        }
        
        // 执行主函数
        if let Some(main_fn) = program.program.functions.get("main") {
            self.execute_function(main_fn)
        } else {
            Err(NjilError::ExecutionError("找不到main函数".to_string()))
        }
    }
    
    /// 处理导入语句
    fn process_imports(&mut self, imports: &Vec<Value>) -> Result<(), NjilError> {
        for import in imports {
            if let Value::String(import_path) = import {
                // 检查是否为内置模块（以!开头）
                if import_path.starts_with('!') {
                    let module_name = &import_path[1..]; // 去掉!前缀
                    self.import_builtin_module(module_name)?;
                } else {
                    // 导入外部文件（暂未实现）
                    return Err(NjilError::ExecutionError(format!("不支持导入外部文件: {}", import_path)));
                }
            } else {
                return Err(NjilError::ExecutionError("导入路径必须是字符串".to_string()));
            }
        }
        
        Ok(())
    }
    
    /// 导入内置模块
    fn import_builtin_module(&mut self, module_name: &str) -> Result<(), NjilError> {
        // 检查模块是否已加载
        if self.loaded_modules.contains(module_name) {
            return Ok(());
        }
        
        // 克隆模块名称以避免借用冲突
        let module_name_owned = module_name.to_string();
        
        // 获取并存储模块中的所有处理器
        let handlers = {
            let module = self.module_registry.get_module(&module_name_owned)
                .ok_or_else(|| NjilError::ExecutionError(format!("找不到内置模块: {}", module_name_owned)))?;
            
            // 获取模块中的所有处理器
            module.get_handlers()
        };
        
        // 将处理器注册到语句处理系统
        for handler in handlers {
            statements::register_handler(handler);
        }
        
        // 标记模块为已加载
        self.loaded_modules.insert(module_name_owned.clone());
        
        // 单独处理模块初始化
        self.initialize_module(&module_name_owned)?;
        
        Ok(())
    }
    
    /// 初始化模块（分离出来以避免借用冲突）
    fn initialize_module(&mut self, module_name: &str) -> Result<(), NjilError> {
        // 获取初始化函数并执行
        if let Some(module) = self.module_registry.get_module(module_name) {
            let init_fn = module.initialize();
            init_fn(self)?;
        }
        
        Ok(())
    }
    
    /// 导入所有内置模块（用于NJIS）
    pub fn import_all_builtin_modules(&mut self) -> Result<(), NjilError> {
        // 获取所有模块名称的副本，避免借用冲突
        let module_names: Vec<String> = self.module_registry.get_module_names()
            .iter().map(|&s| s.clone()).collect();
        
        // 逐个导入模块
        for module_name in module_names {
            self.import_builtin_module(&module_name)?;
        }
        
        Ok(())
    }

    fn execute_function(&mut self, function: &Function) -> Result<Value, NjilError> {
        for statement in &function.body {
            match self.execute_statement(statement) {
                Ok(_) => {},
                Err(NjilError::ReturnValue(value)) => {
                    self.returning = true;
                    return Ok(value);
                }
                Err(e) => return Err(e),
            }
        }
        
        Err(NjilError::ExecutionError("函数没有返回值".to_string()))
    }

    fn execute_statement(&mut self, statement: &Value) -> Result<Value, NjilError> {
        // 使用语句处理函数处理语句
        statements::handle_statement(self, statement)
    }

    pub fn evaluate_value(&mut self, value: &Value) -> Result<Value, NjilError> {
        match value {
            Value::String(_) => Ok(value.clone()),
            Value::Object(obj) => {
                // 如果是对象，尝试执行它
                if obj.len() == 1 {
                    let (key, value) = obj.iter().next().unwrap();
                    
                    // 特殊处理嵌套变量路径的情况
                    if key == "var" && value.is_string() {
                        if let Value::String(var_path) = value {
                            if var_path.contains('.') || var_path.contains('[') {
                                return statements::var::VAR_HANDLER.handle(self, value);
                            }
                        }
                    }
                }
                
                self.execute_statement(value)
            }
            Value::Array(_) => Ok(value.clone()),
            Value::Null => Ok(value.clone()),
            Value::Bool(_) => Ok(value.clone()),
            Value::Number(_) => Ok(value.clone()),
        }
    }

    /// 解析内容值，支持变量引用和表达式执行
    /// 这个方法可以被各个模块使用，用于统一处理内容值
    pub fn parse_content(&mut self, content: &Value) -> Result<String, NjilError> {
        // 先评估内容值（执行变量引用等）
        let evaluated = self.evaluate_value(content)?;
        
        // 将评估后的值转换为字符串
        let result = match &evaluated {
            Value::String(s) => s.clone(),
            Value::Object(_) | Value::Array(_) => {
                // 对于对象和数组，使用JSON序列化
                serde_json::to_string_pretty(&evaluated)
                    .map_err(|e| NjilError::ExecutionError(format!("序列化内容失败: {}", e)))?
            },
            _ => self.value_to_string(&evaluated)
        };
        
        Ok(result)
    }

    pub fn value_to_string(&self, value: &Value) -> String {
        match value {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => "null".to_string(),
            Value::Array(arr) => {
                let mut result = String::new();
                for (i, item) in arr.iter().enumerate() {
                    if i > 0 {
                        result.push(' ');
                    }
                    result.push_str(&self.value_to_string(item));
                }
                result
            }
            Value::Object(obj) => {
                // 检查是否是json.new创建的特殊格式
                if obj.contains_key("type") && obj.contains_key("value") && obj.len() == 2 {
                    // 这是json.new创建的值，直接返回value字段的内容
                    if let Some(val_type) = obj.get("type") {
                        if let Some(val) = obj.get("value") {
                            if let Value::String(type_str) = val_type {
                                match type_str.as_str() {
                                    "number" => {
                                        if let Value::Number(n) = val {
                                            return n.to_string();
                                        }
                                    },
                                    "string" => {
                                        if let Value::String(s) = val {
                                            return s.clone();
                                        }
                                    },
                                    "boolean" => {
                                        if let Value::Bool(b) = val {
                                            return b.to_string();
                                        }
                                    },
                                    _ => {}
                                }
                            }
                            // 如果类型不匹配或无法处理，尝试直接返回值
                            return self.value_to_string(val);
                        }
                    }
                }
                
                // 对于普通对象类型，序列化为JSON字符串
                if let Ok(json_str) = serde_json::to_string(obj) {
                    json_str
                } else {
                    // 如果序列化失败，使用简单的键值对表示
                    let mut result = String::from("{");
                    for (i, (key, val)) in obj.iter().enumerate() {
                        if i > 0 {
                            result.push_str(", ");
                        }
                        result.push_str(&format!("\"{}\": {}", key, self.value_to_string(val)));
                    }
                    result.push('}');
                    result
                }
            }
        }
    }
    
    #[allow(dead_code)]
    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
    
    #[allow(dead_code)]
    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
    
    /// 检查是否正在返回
    pub fn is_returning(&self) -> bool {
        self.returning
    }
    
    /// 设置返回状态
    pub fn set_returning(&mut self, returning: bool) {
        self.returning = returning;
    }
}

/// 从文件加载并执行NJIL程序
pub fn run_file<P: AsRef<Path>>(file_path: P) -> Result<Value, NjilError> {
    let mut interpreter = Interpreter::new();
    let program = interpreter.load_file(file_path)?;
    interpreter.execute(&program)
} 