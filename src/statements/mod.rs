use std::collections::HashMap;
use std::sync::OnceLock;
use serde_json::Value;
use crate::error::NjilError;
use crate::interpreter::Interpreter;
use crate::errortip;

pub mod print;
pub mod var;
pub mod return_stmt;
pub mod string;

/// 语句处理器特质，定义了语句处理器的接口
pub trait StatementHandler: Send + Sync {
    /// 处理语句
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError>;
    
    /// 获取处理器名称
    fn name(&self) -> &'static str;
    
    /// 获取处理器可以处理的所有语句名称
    fn aliases(&self) -> Vec<&'static str> {
        vec![self.name()]
    }
}

/// 语句处理器注册表
struct StatementRegistry {
    handlers: HashMap<&'static str, &'static dyn StatementHandler>,
}

impl StatementRegistry {
    fn new() -> Self {
        let mut registry = StatementRegistry {
            handlers: HashMap::new(),
        };
        
        // 注册内置处理器
        registry.register(&print::PRINT_HANDLER);
        registry.register(&var::VAR_SET_HANDLER);
        registry.register(&var::VAR_HANDLER);
        registry.register(&return_stmt::RETURN_HANDLER);
        registry.register(&string::STRING_CONCAT_HANDLER);
        
        registry
    }
    
    fn register(&mut self, handler: &'static dyn StatementHandler) {
        for alias in handler.aliases() {
            self.handlers.insert(alias, handler);
        }
    }
    
    fn get(&self, name: &str) -> Option<&'static dyn StatementHandler> {
        self.handlers.get(name).copied()
    }
}

// 全局静态注册表
static REGISTRY: OnceLock<StatementRegistry> = OnceLock::new();

fn get_registry() -> &'static StatementRegistry {
    REGISTRY.get_or_init(StatementRegistry::new)
}

/// 处理语句
pub fn handle_statement(interpreter: &mut Interpreter, statement: &Value) -> Result<Value, NjilError> {
    if let Value::Object(obj) = statement {
        if obj.len() == 1 {
            let (key, value) = obj.iter().next().unwrap();
            if let Some(handler) = get_registry().get(key) {
                return handler.handle(interpreter, value);
            }
            return Err(NjilError::ExecutionError(errortip::unknown_instruction(key)));
        }
    }
    
    Ok(statement.clone())
} 