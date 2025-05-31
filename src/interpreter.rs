use crate::error::NjilError;
use crate::types::{Function, NjilProgram};
use crate::statements;
use crate::statements::StatementHandler;
use crate::builtin::BuiltinModuleRegistry;
use crate::debug_println;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use regex;

/// 解释器，负责执行NeoJiLang代码
pub struct Interpreter {
    pub(crate) variables: HashMap<String, Value>,
    pub(crate) statement_handlers: HashMap<String, Box<dyn StatementHandler>>,
    pub(crate) builtin_modules: BuiltinModuleRegistry,
    pub(crate) returning: Option<Value>,
    loaded_modules: HashSet<String>,
    current_dir: Option<PathBuf>,
    current_program: Option<NjilProgram>,
}

impl Interpreter {
    /// 创建一个新的解释器实例
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            statement_handlers: HashMap::new(),
            builtin_modules: BuiltinModuleRegistry::new(),
            returning: None,
            loaded_modules: HashSet::new(),
            current_dir: None,
            current_program: None,
        }
    }

    /// 创建一个新的干净的解释器实例，但保留变量
    pub fn create_clean_instance(&self) -> Self {
        let mut new_instance = Self {
            variables: HashMap::new(),
            statement_handlers: HashMap::new(),
            builtin_modules: self.builtin_modules.clone_empty(),
            returning: None,
            loaded_modules: self.loaded_modules.clone(),
            current_dir: self.current_dir.clone(),
            current_program: self.current_program.clone(),
        };
        
        // 复制变量
        for (key, value) in &self.variables {
            new_instance.variables.insert(key.clone(), value.clone());
        }
        
        new_instance
    }

    pub fn load_file<P: AsRef<Path>>(&mut self, path: P) -> Result<NjilProgram, NjilError> {
        let path = path.as_ref();
        
        // 设置当前目录为文件所在目录，用于相对路径导入
        if let Some(parent) = path.parent() {
            self.current_dir = Some(parent.to_path_buf());
        }
        
        let content = fs::read_to_string(path)?;
        let program: NjilProgram = serde_json::from_str(&content)?;
        
        // 保存当前程序
        self.current_program = Some(program.clone());
        
        Ok(program)
    }

    pub fn execute(&mut self, program: &NjilProgram) -> Result<Value, NjilError> {
        // 保存当前程序
        self.current_program = Some(program.clone());
        
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
            let module = self.builtin_modules.get_module(&module_name_owned)
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
        if let Some(module) = self.builtin_modules.get_module(module_name) {
            let init_fn = module.initialize();
            init_fn(self)?;
        }
        
        Ok(())
    }
    
    /// 导入所有内置模块（用于NJIS）
    pub fn import_all_builtin_modules(&mut self) -> Result<(), NjilError> {
        // 获取所有模块名称的副本，避免借用冲突
        let module_names: Vec<String> = self.builtin_modules.get_module_names()
            .iter().map(|&s| s.clone()).collect();
        
        // 逐个导入模块
        for module_name in module_names {
            self.import_builtin_module(&module_name)?;
        }
        
        Ok(())
    }

    fn execute_function(&mut self, function: &Function) -> Result<Value, NjilError> {
        debug_println!("[Interpreter::execute_function] 开始执行函数, 语句数量: {}", function.body.len());
        
        for (i, statement) in function.body.iter().enumerate() {
            debug_println!("[Interpreter::execute_function] 执行语句 #{}: {}", i, serde_json::to_string_pretty(statement).unwrap());
            
            match self.execute_statement(statement) {
                Ok(result) => {
                    debug_println!("[Interpreter::execute_function] 语句 #{} 执行成功, 结果: {}", i, serde_json::to_string_pretty(&result).unwrap());
                },
                Err(NjilError::ReturnValue(value)) => {
                    debug_println!("[Interpreter::execute_function] 遇到return语句, 返回值: {}", serde_json::to_string_pretty(&value).unwrap());
                    let value_clone = value.clone();
                    self.returning = Some(value);
                    return Ok(value_clone);
                }
                Err(e) => {
                    debug_println!("[Interpreter::execute_function] 语句 #{} 执行失败: {:?}", i, e);
                    return Err(e);
                },
            }
        }
        
        debug_println!("[Interpreter::execute_function] 函数执行完毕，但没有返回值");
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
                    
                    // 处理函数调用
                    if key == "function.call" || key == "call" || key == "func.call" {
                        return statements::function_call::FUNCTION_CALL_HANDLER.handle(self, value);
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

    pub fn value_to_string(&mut self, value: &Value) -> String {
        match value {
            Value::String(s) => {
                // 处理变量引用，格式为 ${var:name}
                let mut result = s.clone();
                let var_pattern = regex::Regex::new(r"\$\{var:([^}]+)\}").unwrap();
                
                // 收集所有需要替换的变量
                let mut replacements = Vec::new();
                for cap in var_pattern.captures_iter(&result) {
                    let var_name = cap[1].to_string();
                    let full_match = cap[0].to_string();
                    
                    // 获取变量值
                    let var_value = if let Some(val) = self.variables.get(&var_name) {
                        val.clone()
                    } else {
                        Value::String(format!("undefined:{}", var_name))
                    };
                    
                    replacements.push((full_match, var_value));
                }
                
                // 执行替换
                for (pattern, var_value) in replacements {
                    let replacement = self.value_to_string(&var_value);
                    result = result.replace(&pattern, &replacement);
                }
                
                result
            },
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => "null".to_string(),
            Value::Array(arr) => {
                let items: Vec<String> = arr
                    .iter()
                    .map(|v| self.value_to_string(v))
                    .collect();
                format!("[{}]", items.join(", "))
            }
            Value::Object(obj) => {
                // 尝试执行算术运算或比较运算
                match self.execute_operation(value) {
                    Ok(result) if result != *value => {
                        // 如果执行成功且结果不同于原值，返回结果的字符串表示
                        return self.value_to_string(&result);
                    }
                    _ => {
                        // 尝试将对象序列化为JSON字符串
                        match serde_json::to_string(obj) {
                            Ok(json) => json,
                            Err(_) => {
                                // 如果序列化失败，使用简单的键值对格式
                                let pairs: Vec<String> = obj
                                    .iter()
                                    .map(|(k, v)| format!("{}: {}", k, self.value_to_string(v)))
                                    .collect();
                                format!("{{{}}}", pairs.join(", "))
                            }
                        }
                    }
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
        self.returning.is_some()
    }
    
    /// 设置返回状态
    pub fn set_returning(&mut self, returning: bool) {
        self.returning = if returning { Some(Value::Null) } else { None };
    }

    /// 执行算术运算或比较运算，直接获取结果
    pub fn execute_operation(&mut self, value: &Value) -> Result<Value, NjilError> {
        if let Value::Object(obj) = value {
            if obj.len() == 1 {
                let (key, _val) = obj.iter().next().unwrap();
                
                // 检查是否是算术运算或比较运算
                if key.starts_with("math.") || 
                   key == "add" || key == "subtract" || key == "sub" || 
                   key == "multiply" || key == "mul" || key == "divide" || 
                   key == "div" || key == "modulo" || key == "mod" || 
                   key == "compare" || key == "cmp" {
                    
                    // 尝试执行操作
                    return self.execute_statement(value);
                }
            }
        }
        
        // 如果不是操作或执行失败，返回原值
        Ok(value.clone())
    }

    /// 调用函数
    pub fn call_function(&mut self, function_name: &str, args: &[Value]) -> Result<Value, NjilError> {
        debug_println!("[Interpreter::call_function] 开始调用函数: {}", function_name);
        
        // 检查函数是否存在
        let function = match self.get_function(function_name) {
            Some(func) => {
                debug_println!("[Interpreter::call_function] 找到函数: {}", function_name);
                func.clone()
            },
            None => {
                debug_println!("[Interpreter::call_function] 找不到函数: {}", function_name);
                return Err(NjilError::ExecutionError(format!("找不到函数: {}", function_name)));
            },
        };
        
        // 创建一个新的变量作用域（新的解释器实例，但共享模块加载状态）
        let mut function_interpreter = self.create_clean_instance();
        debug_println!("[Interpreter::call_function] 创建了新的解释器实例");
        
        // 如果有参数，则设置参数变量
        if !args.is_empty() {
            debug_println!("[Interpreter::call_function] 设置参数变量, 参数数量: {}", args.len());
            
            // 参数通过 $args 数组传递
            function_interpreter.set_variable("$args".to_string(), Value::Array(args.to_vec()));
            
            // 如果需要，可以添加命名参数支持
            // 例如，第一个参数可以通过 $1 访问，第二个通过 $2，以此类推
            for (i, arg) in args.iter().enumerate() {
                let param_name = format!("${}", i + 1);
                debug_println!("[Interpreter::call_function] 设置参数 {}: {}", param_name, serde_json::to_string_pretty(arg).unwrap());
                function_interpreter.set_variable(param_name, arg.clone());
            }
        }
        
        // 执行函数
        debug_println!("[Interpreter::call_function] 开始执行函数: {}", function_name);
        let result = function_interpreter.execute_function(&function);
        debug_println!("[Interpreter::call_function] 函数执行结果: {:?}", result);
        result
    }
    
    /// 获取函数定义
    pub fn get_function(&self, function_name: &str) -> Option<&Function> {
        // 首先在当前加载的程序中查找
        if let Some(program) = &self.current_program {
            if let Some(func) = program.program.functions.get(function_name) {
                return Some(func);
            }
        }
        
        // 如果没有找到，可以在未来实现在导入的模块中查找
        None
    }
}

/// 从文件加载并执行NJIL程序
pub fn run_file<P: AsRef<Path>>(file_path: P) -> Result<Value, NjilError> {
    let mut interpreter = Interpreter::new();
    let program = interpreter.load_file(file_path)?;
    interpreter.execute(&program)
} 