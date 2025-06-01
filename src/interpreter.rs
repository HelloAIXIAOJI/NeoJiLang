use crate::error::NjilError;
use crate::types::{Function, NjilProgram};
use crate::statements;
use crate::statements::StatementHandler;
use crate::builtin::BuiltinModuleRegistry;
use crate::debug_println;
use crate::preprocessor::Preprocessor;
use crate::utils::path;
use crate::utils::path::PathPart;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use regex;

/// 解释器，负责执行NeoJiLang代码
pub struct Interpreter {
    pub(crate) variables: HashMap<String, Value>,
    pub(crate) constants: HashMap<String, Value>,
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
            constants: HashMap::new(),
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
            constants: self.constants.clone(),
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
        
        // 使用预处理器处理文件内容，移除注释
        let processed_content = Preprocessor::preprocess_file(path)?;
        debug_println!("文件预处理完成，准备解析JSON");
        
        // 解析处理后的内容
        let program: NjilProgram = match serde_json::from_str(&processed_content) {
            Ok(p) => p,
            Err(e) => {
                return Err(NjilError::ExecutionError(
                    format!("解析JSON失败: {}，请检查文件格式是否正确", e)
                ));
            }
        };
        
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
                    // 检查是否为NJIM模块
                    if import_path.ends_with(".njim") || self.is_njim_module(import_path) {
                        self.import_njim_module(import_path)?;
                    } else {
                        // 其他导入方式（暂不支持）
                        return Err(NjilError::ExecutionError(format!("不支持导入外部文件: {}", import_path)));
                    }
                }
            } else {
                return Err(NjilError::ExecutionError("导入路径必须是字符串".to_string()));
            }
        }
        
        Ok(())
    }
    
    /// 检查是否为NJIM模块
    fn is_njim_module(&self, import_path: &str) -> bool {
        // 如果扩展名已经是.njim，则返回true
        if import_path.ends_with(".njim") {
            return true;
        }
        
        // 尝试解析模块路径，如果成功则认为是NJIM模块
        if let Some(dir) = &self.current_dir {
            let module_path = dir.join(import_path);
            if module_path.exists() && module_path.is_file() {
                return true;
            }
            
            let with_ext = module_path.with_extension("njim");
            if with_ext.exists() && with_ext.is_file() {
                return true;
            }
        }
        
        false
    }
    
    /// 导入NJIM模块
    fn import_njim_module(&mut self, module_path: &str) -> Result<(), NjilError> {
        debug_println!("导入NJIM模块: {}", module_path);
        
        // 解析模块路径
        let full_path = if let Some(dir) = &self.current_dir {
            if module_path.starts_with('/') {
                // 绝对路径（相对于项目根目录）
                PathBuf::from(module_path.trim_start_matches('/'))
            } else {
                // 相对路径（相对于当前目录）
                dir.join(module_path)
            }
        } else {
            PathBuf::from(module_path)
        };
        
        debug_println!("解析的模块路径: {}", full_path.display());
        
        // 加载模块
        let module_result = crate::module::load_module(&full_path);
        
        match module_result {
            Ok(module) => {
                let module_name = module.module.clone();
                let namespace = module.namespace.clone().unwrap_or_else(|| module_name.clone());
                
                debug_println!("模块加载成功: {}, 命名空间: {}", module_name, namespace);
                
                // 处理模块导出的常量
                if let Some(constants) = &module.exports.constants {
                    for (const_name, const_value) in constants {
                        let full_const_name = format!("{}.{}", namespace, const_name);
                        debug_println!("导入常量: {}", full_const_name);
                        self.constants.insert(full_const_name, const_value.clone());
                    }
                }
                
                // 处理模块导出的函数
                if let Some(functions) = &module.exports.functions {
                    debug_println!("导入模块函数, 数量: {}", functions.len());
                    for (func_name, func_def) in functions {
                        let full_func_name = format!("{}.{}", namespace, func_name);
                        debug_println!("导入函数: {}", full_func_name);
                        
                        // 将函数添加到当前解释器的函数表中
                        if let Some(current_program) = &mut self.current_program {
                            current_program.program.functions.insert(full_func_name, func_def.clone());
                        }
                    }
                }
                
                // 记录已加载的模块，用于函数调用时查找
                self.loaded_modules.insert(namespace);
                
                Ok(())
            },
            Err(e) => {
                Err(NjilError::ExecutionError(format!("导入模块失败: {}", e)))
            }
        }
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
                    
                    // 特殊处理嵌套常量路径的情况
                    if key == "const" && value.is_string() {
                        if let Value::String(const_path) = value {
                            if const_path.contains('.') || const_path.contains('[') {
                                return statements::constant::CONST_HANDLER.handle(self, value);
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
                // 先替换变量引用
                let with_vars_replaced = self.replace_var_in_string(s);
                
                // 再替换常量引用
                self.replace_constant_in_string(&with_vars_replaced)
            },
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    i.to_string()
                } else if let Some(f) = n.as_f64() {
                    f.to_string()
                } else {
                    n.to_string()
                }
            },
            Value::Bool(b) => b.to_string(),
            Value::Null => "null".to_string(),
            Value::Array(arr) => {
                let mut parts = Vec::new();
                for item in arr {
                    parts.push(self.value_to_string(item));
                }
                parts.join(", ")
            },
            Value::Object(obj) => {
                let mut parts = Vec::new();
                for (key, val) in obj {
                    parts.push(format!("{}: {}", key, self.value_to_string(val)));
                }
                format!("{{{}}}", parts.join(", "))
            },
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

    /// 检查常量是否存在
    pub fn has_constant(&self, name: &str) -> bool {
        self.constants.contains_key(name)
    }

    /// 获取常量值
    pub fn get_constant(&self, name: &str) -> Option<&Value> {
        self.constants.get(name)
    }

    /// 将变量字符串模式替换为实际值
    fn replace_var_in_string(&mut self, input: &str) -> String {
        let var_pattern = regex::Regex::new(r"\$\{var:([^}]+)\}").unwrap();
        let mut result = input.to_string();
        
        // 收集所有需要替换的变量
        let mut replacements = Vec::new();
        for cap in var_pattern.captures_iter(&result) {
            let var_name = cap[1].to_string();
            let full_match = cap[0].to_string();
            
            debug_println!("[replace_var_in_string] 尝试替换变量: {}", var_name);
            
            // 获取变量值
            let var_value = if let Some(val) = self.variables.get(&var_name) {
                debug_println!("[replace_var_in_string] 找到变量 {} 的值: {}", var_name, serde_json::to_string(val).unwrap());
                val.clone()
            } else if var_name.contains('.') || var_name.contains('[') {
                // 尝试处理嵌套变量路径
                debug_println!("[replace_var_in_string] 尝试解析嵌套变量路径: {}", var_name);
                match statements::var::get_nested_variable(self, &var_name) {
                    Ok(nested_val) => {
                        debug_println!("[replace_var_in_string] 找到嵌套变量值: {}", serde_json::to_string(&nested_val).unwrap());
                        nested_val
                    },
                    Err(e) => {
                        debug_println!("[replace_var_in_string] 获取嵌套变量失败: {:?}", e);
                        Value::String(format!("undefined:{}", var_name))
                    }
                }
            } else {
                debug_println!("[replace_var_in_string] 变量 {} 未定义", var_name);
                Value::String(format!("undefined:{}", var_name))
            };
            
            replacements.push((full_match, var_value));
        }
        
        // 执行替换
        for (pattern, var_value) in replacements {
            let replacement = self.value_to_string(&var_value);
            debug_println!("[replace_var_in_string] 替换 {} 为 {}", pattern, replacement);
            result = result.replace(&pattern, &replacement);
        }
        
        result
    }

    /// 将常量字符串模式替换为实际值
    fn replace_constant_in_string(&mut self, input: &str) -> String {
        let const_pattern = regex::Regex::new(r"\$\{const:([^}]+)\}").unwrap();
        let mut result = input.to_string();
        
        // 收集所有需要替换的常量
        let mut replacements = Vec::new();
        for cap in const_pattern.captures_iter(&result) {
            let const_path = cap[1].to_string();
            let full_match = cap[0].to_string();
            
            debug_println!("[replace_constant_in_string] 尝试替换常量: {}", const_path);
            
            // 处理嵌套路径的常量访问
            if const_path.contains('.') {
                let parts: Vec<&str> = const_path.split('.').collect();
                if parts.len() >= 2 {
                    let module_name = parts[0];
                    let const_name = parts[1];
                    let full_const_name = format!("{}.{}", module_name, const_name);
                    
                    debug_println!("[replace_constant_in_string] 查找常量: {}", full_const_name);
                    
                    if let Some(value) = self.constants.get(&full_const_name) {
                        let const_value = value.clone();
                        let replacement = self.value_to_string(&const_value);
                        debug_println!("[replace_constant_in_string] 替换 {} 为 {}", full_match, replacement);
                        replacements.push((full_match, replacement));
                    } else {
                        debug_println!("[replace_constant_in_string] 未找到常量: {}", full_const_name);
                        replacements.push((full_match.clone(), full_match));
                    }
                } else {
                    replacements.push((full_match.clone(), full_match));
                }
            } else {
                // 普通常量
                if let Some(value) = self.constants.get(&const_path) {
                    let const_value = value.clone();
                    let replacement = self.value_to_string(&const_value);
                    debug_println!("[replace_constant_in_string] 替换 {} 为 {}", full_match, replacement);
                    replacements.push((full_match, replacement));
                } else {
                    debug_println!("[replace_constant_in_string] 未找到常量: {}", const_path);
                    replacements.push((full_match.clone(), full_match));
                }
            }
        }
        
        // 执行替换
        for (pattern, replacement) in replacements {
            result = result.replace(&pattern, &replacement);
        }
        
        result
    }
}

/// 从文件加载并执行NJIL程序
pub fn run_file<P: AsRef<Path>>(file_path: P) -> Result<Value, NjilError> {
    let mut interpreter = Interpreter::new();
    let program = interpreter.load_file(file_path)?;
    interpreter.execute(&program)
} 