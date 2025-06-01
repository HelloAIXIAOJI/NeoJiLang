use std::collections::HashMap;
use std::sync::{OnceLock, Mutex};
use serde_json::Value;
use crate::error::NjilError;
use crate::interpreter::Interpreter;
use crate::errortip;

pub mod print;
pub mod string;
pub mod var;
pub mod return_stmt;
pub mod json;
pub mod control_flow;
pub mod logic;
pub mod type_convert;
pub mod arithmetic;
pub mod function_call;
pub mod sleep;
pub mod constant;

use print::PRINT_HANDLER;
use print::PRINTLN_HANDLER;
use string::STRING_CONCAT_HANDLER;
use var::{VAR_HANDLER, VAR_SET_HANDLER, VAR_SET_MULTI_HANDLER};
use return_stmt::RETURN_HANDLER;
use json::{JSON_NEW_HANDLER, JSON_GET_HANDLER, JSON_SET_HANDLER};
use control_flow::get_all_handlers as get_all_control_flow_handlers;
pub use control_flow::{IF_HANDLER, WHILE_LOOP_HANDLER, FOR_LOOP_HANDLER, FOREACH_LOOP_HANDLER, BREAK_HANDLER, CONTINUE_HANDLER};
use sleep::SLEEP_HANDLER;
use constant::{CONST_HANDLER, CONST_SET_HANDLER, CONST_SET_MULTI_HANDLER, CONST_HAS_HANDLER};

use logic::{
    LOGIC_AND_HANDLER,
    LOGIC_OR_HANDLER,
    LOGIC_NOT_HANDLER,
};
use type_convert::{TYPE_CONVERT_HANDLER, TO_BOOL_HANDLER, TO_NUMBER_HANDLER, TO_STRING_HANDLER, TO_ARRAY_HANDLER, TO_OBJECT_HANDLER, TYPE_OF_HANDLER};
use arithmetic::get_all_handlers as get_all_arithmetic_handlers;
use function_call::FUNCTION_CALL_HANDLER;

/// 语句处理器特性
pub trait StatementHandler: Send + Sync {
    /// 处理语句
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError>;
    
    /// 获取处理器名称
    fn name(&self) -> &'static str;
    
    /// 获取处理器别名（可选）
    fn aliases(&self) -> Vec<&'static str> {
        vec![]
    }
}

/// 语句注册表，用于管理语句处理器
pub struct StatementRegistry {
    handlers: HashMap<String, &'static dyn StatementHandler>,
}

impl StatementRegistry {
    /// 创建一个新的语句注册表
    pub fn new() -> Self {
        let mut registry = Self {
            handlers: HashMap::new(),
        };
        
        // 注册核心语句处理器
        registry.register_handler(&PRINT_HANDLER);
        registry.register_handler(&PRINTLN_HANDLER);
        registry.register_handler(&VAR_HANDLER);
        registry.register_handler(&VAR_SET_HANDLER);
        registry.register_handler(&VAR_SET_MULTI_HANDLER);
        registry.register_handler(&RETURN_HANDLER);
        registry.register_handler(&STRING_CONCAT_HANDLER);
        registry.register_handler(&SLEEP_HANDLER);
        
        // 注册常量语句处理器
        registry.register_handler(&CONST_HANDLER);
        registry.register_handler(&CONST_SET_HANDLER);
        registry.register_handler(&CONST_SET_MULTI_HANDLER);
        registry.register_handler(&CONST_HAS_HANDLER);
        
        // 注册JSON语句处理器
        registry.register_handler(&JSON_NEW_HANDLER);
        registry.register_handler(&JSON_GET_HANDLER);
        registry.register_handler(&JSON_SET_HANDLER);
        
        // 注册控制流语句处理器 - 使用统一的注册方法
        for handler in get_all_control_flow_handlers() {
            registry.register_handler(handler);
        }
        
        // 注册逻辑运算语句处理器
        registry.register_handler(&LOGIC_AND_HANDLER);
        registry.register_handler(&LOGIC_OR_HANDLER);
        registry.register_handler(&LOGIC_NOT_HANDLER);
        
        // 注册类型转换语句处理器
        registry.register_handler(&TYPE_CONVERT_HANDLER);
        registry.register_handler(&TO_BOOL_HANDLER);
        registry.register_handler(&TO_NUMBER_HANDLER);
        registry.register_handler(&TO_STRING_HANDLER);
        registry.register_handler(&TO_ARRAY_HANDLER);
        registry.register_handler(&TO_OBJECT_HANDLER);
        registry.register_handler(&TYPE_OF_HANDLER);
        
        // 注册算术运算语句处理器
        for handler in get_all_arithmetic_handlers() {
            registry.register_handler(handler);
        }
        
        // 注册函数调用处理器
        registry.register_handler(&FUNCTION_CALL_HANDLER);
        
        registry
    }
    
    /// 注册一个语句处理器
    pub fn register_handler(&mut self, handler: &'static dyn StatementHandler) {
        // 注册主名称
        self.handlers.insert(handler.name().to_string(), handler);
        
        // 注册别名
        for alias in handler.aliases() {
            self.handlers.insert(alias.to_string(), handler);
        }
    }
    
    /// 获取指定名称的处理器
    pub fn get(&self, name: &str) -> Option<&'static dyn StatementHandler> {
        self.handlers.get(name).copied()
    }
}

// 使用Mutex包装StatementRegistry，使其可以安全地修改
static REGISTRY: OnceLock<Mutex<StatementRegistry>> = OnceLock::new();

/// 获取全局语句注册表
fn get_registry() -> &'static Mutex<StatementRegistry> {
    REGISTRY.get_or_init(|| Mutex::new(StatementRegistry::new()))
}

/// 注册一个语句处理器到全局注册表
pub fn register_handler(handler: &'static dyn StatementHandler) {
    // 使用Mutex获取可变引用
    if let Ok(mut registry) = get_registry().lock() {
        registry.register_handler(handler);
    }
}

/// 处理语句
pub fn handle_statement(interpreter: &mut Interpreter, statement: &Value) -> Result<Value, NjilError> {
    if let Value::Object(obj) = statement {
        if obj.len() == 1 {
            let (key, value) = obj.iter().next().unwrap();
            
            // 检查是否为变量访问
            if key == "var" && value.is_string() {
                if let Value::String(var_path) = value {
                    // 特殊处理嵌套变量路径
                    if var_path.contains('.') || var_path.contains('[') {
                        // 使用 VarHandler 处理嵌套变量路径
                        return VAR_HANDLER.handle(interpreter, value);
                    }
                }
            }
            
            // 检查是否为常量访问
            if key == "const" && value.is_string() {
                if let Value::String(const_path) = value {
                    // 特殊处理嵌套常量路径
                    if const_path.contains('.') || const_path.contains('[') {
                        // 使用 ConstHandler 处理嵌套常量路径
                        return CONST_HANDLER.handle(interpreter, value);
                    }
                }
            }
            
            // 获取语句处理器
            let handler: Option<&'static dyn StatementHandler> = {
                if let Ok(registry) = get_registry().lock() {
                    registry.get(key)
                } else {
                    None
                }
            };
            
            if let Some(handler) = handler {
                return handler.handle(interpreter, value);
            }
            
            // 如果找不到处理器，检查是否是嵌套变量路径的一部分
            if key.contains('.') || key.contains('[') {
                // 尝试解析为变量路径
                let parts: Vec<&str> = key.split('.').collect();
                if parts.len() > 1 {
                    // 检查第一部分是否是变量
                    let base_var = parts[0];
                    
                    if interpreter.variables.contains_key(base_var) {
                        // 这可能是一个嵌套变量访问，而不是未知指令
                        // 将其转换为正确的变量访问形式
                        let mut var_obj = serde_json::Map::new();
                        var_obj.insert("var".to_string(), Value::String(key.to_string()));
                        let var_statement = Value::Object(var_obj);
                        return interpreter.evaluate_value(&var_statement);
                    }
                    
                    // 检查第一部分是否是常量
                    if interpreter.has_constant(base_var) {
                        // 这可能是一个嵌套常量访问
                        let mut const_obj = serde_json::Map::new();
                        const_obj.insert("const".to_string(), Value::String(key.to_string()));
                        let const_statement = Value::Object(const_obj);
                        return interpreter.evaluate_value(&const_statement);
                    }
                }
            }
            
            return Err(NjilError::ExecutionError(errortip::unknown_instruction(key)));
        }
    }
    
    Ok(statement.clone())
} 