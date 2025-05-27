use crate::error::NjilError;
use serde_json::Value;
use crate::interpreter::Interpreter;
use crate::errortip;
use super::StatementHandler;

/// 字符串连接处理器，同时处理string.concat和txtlink
pub struct StringConcatHandler;

// 静态实例
pub static STRING_CONCAT_HANDLER: StringConcatHandler = StringConcatHandler;

impl StringConcatHandler {
    pub fn concat_strings(interpreter: &mut Interpreter, parts: &[Value]) -> Result<Value, NjilError> {
        let mut result = String::new();
        for part in parts {
            let evaluated = interpreter.evaluate_value(part)?;
            result.push_str(&interpreter.value_to_string(&evaluated));
        }
        Ok(Value::String(result))
    }
}

impl StatementHandler for StringConcatHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Array(parts) = value {
            Self::concat_strings(interpreter, parts)
        } else {
            Err(NjilError::ExecutionError(errortip::string::concat_requires_array().to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "string.concat"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["string.concat", "txtlink"]
    }
} 