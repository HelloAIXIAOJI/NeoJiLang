use crate::error::NjilError;
use serde_json::Value;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use crate::utils::type_convert;

/// 加法运算处理器
pub struct AddHandler;

// 静态实例
pub static ADD_HANDLER: AddHandler = AddHandler;

impl StatementHandler for AddHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Array(operands) = value {
            if operands.is_empty() {
                return Err(NjilError::ExecutionError(
                    "加法运算需要至少一个操作数".to_string()
                ));
            }
            
            // 评估第一个操作数
            let first_operand = interpreter.evaluate_value(&operands[0])?;
            
            // 如果只有一个操作数，直接返回
            if operands.len() == 1 {
                return Ok(first_operand);
            }
            
            // 根据第一个操作数类型选择初始值
            let mut result = match &first_operand {
                Value::String(_) | Value::Array(_) => first_operand.clone(),
                _ => Value::Number(serde_json::Number::from(0)),
            };
            
            // 如果是字符串或数组，从第二个操作数开始计算
            // 否则从第一个操作数开始计算
            let start_index = if matches!(first_operand, Value::String(_) | Value::Array(_)) { 1 } else { 0 };
            
            for operand in &operands[start_index..] {
                // 评估操作数
                let operand_value = interpreter.evaluate_value(operand)?;
                
                // 执行加法运算
                result = type_convert::add(&result, &operand_value);
            }
            
            Ok(result)
        } else {
            Err(NjilError::ExecutionError(
                "加法运算需要一个数组参数".to_string()
            ))
        }
    }
    
    fn name(&self) -> &'static str {
        "math.add"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["math.add", "add"]
    }
} 