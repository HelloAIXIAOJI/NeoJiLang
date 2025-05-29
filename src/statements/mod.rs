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

use print::PRINT_HANDLER;
use string::STRING_CONCAT_HANDLER;
use var::{VAR_HANDLER, VAR_SET_HANDLER};
use return_stmt::RETURN_HANDLER;
use json::{JSON_NEW_HANDLER, JSON_GET_HANDLER, JSON_SET_HANDLER};

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
        
        // 注册内置语句处理器
        registry.register_handler(&PRINT_HANDLER);
        registry.register_handler(&STRING_CONCAT_HANDLER);
        registry.register_handler(&VAR_HANDLER);
        registry.register_handler(&VAR_SET_HANDLER);
        registry.register_handler(&RETURN_HANDLER);
        
        // 注册JSON相关语句处理器
        registry.register_handler(&JSON_NEW_HANDLER);
        registry.register_handler(&JSON_GET_HANDLER);
        registry.register_handler(&JSON_SET_HANDLER);
        
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
            
            return Err(NjilError::ExecutionError(errortip::unknown_instruction(key)));
        }
    }
    
    Ok(statement.clone())
} 