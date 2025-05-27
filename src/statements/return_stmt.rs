use crate::error::NjilError;
use serde_json::Value;
use crate::interpreter::Interpreter;
use super::StatementHandler;

/// 返回语句处理器
pub struct ReturnHandler;

// 静态实例
pub static RETURN_HANDLER: ReturnHandler = ReturnHandler;

impl StatementHandler for ReturnHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        let result = interpreter.evaluate_value(value)?;
        Err(NjilError::ReturnValue(result))
    }
    
    fn name(&self) -> &'static str {
        "return"
    }
} 