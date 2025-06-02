use crate::error::NjilError;
use crate::interpreter::Interpreter;
use crate::errortip;
use crate::statements::StatementHandler;
use serde_json::Value;

/// Throw语句处理器，用于抛出异常
pub struct ThrowHandler;

// 静态实例
pub static THROW_HANDLER: ThrowHandler = ThrowHandler;

impl StatementHandler for ThrowHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        // 首先评估异常值
        let exception_value = match interpreter.evaluate_value(value) {
            Ok(val) => val,
            Err(e) => return Err(e),
        };
        
        // 抛出异常
        Err(NjilError::ThrowException(exception_value))
    }
    
    fn name(&self) -> &'static str {
        "throw"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_throw_handler() {
        let mut interpreter = Interpreter::new();
        let handler = ThrowHandler;
        
        // 测试抛出字符串异常
        let value = json!("测试异常");
        let result = handler.handle(&mut interpreter, &value);
        assert!(result.is_err());
        if let Err(NjilError::ThrowException(error_value)) = result {
            assert_eq!(error_value, json!("测试异常"));
        } else {
            panic!("应该抛出ThrowException类型的异常");
        }
        
        // 测试抛出对象异常
        let value = json!({
            "code": 500,
            "message": "服务器错误"
        });
        let result = handler.handle(&mut interpreter, &value);
        assert!(result.is_err());
        if let Err(NjilError::ThrowException(error_value)) = result {
            assert_eq!(error_value, json!({
                "code": 500,
                "message": "服务器错误"
            }));
        } else {
            panic!("应该抛出ThrowException类型的异常");
        }
    }
} 