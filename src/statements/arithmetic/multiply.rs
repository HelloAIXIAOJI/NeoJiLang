use crate::error::NjilError;
use serde_json::Value;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use crate::utils::type_convert;

/// 乘法运算处理器
pub struct MultiplyHandler;

// 静态实例
pub static MULTIPLY_HANDLER: MultiplyHandler = MultiplyHandler;

impl StatementHandler for MultiplyHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Array(operands) = value {
            if operands.is_empty() {
                return Err(NjilError::ExecutionError(
                    "乘法运算需要至少一个操作数".to_string()
                ));
            }
            
            // 计算所有操作数的积
            let mut result = Value::Number(serde_json::Number::from(1));
            
            for operand in operands {
                // 评估操作数
                let operand_value = interpreter.evaluate_value(operand)?;
                
                // 执行乘法运算
                result = type_convert::multiply(&result, &operand_value);
            }
            
            Ok(result)
        } else {
            Err(NjilError::ExecutionError(
                "乘法运算需要一个数组参数".to_string()
            ))
        }
    }
    
    fn name(&self) -> &'static str {
        "math.multiply"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["math.multiply", "multiply", "mul"]
    }
} 