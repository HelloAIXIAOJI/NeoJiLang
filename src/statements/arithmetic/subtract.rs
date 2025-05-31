use crate::error::NjilError;
use serde_json::Value;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use crate::utils::type_convert;

/// 减法运算处理器
pub struct SubtractHandler;

// 静态实例
pub static SUBTRACT_HANDLER: SubtractHandler = SubtractHandler;

impl StatementHandler for SubtractHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        // 处理数组格式参数
        if let Value::Array(operands) = value {
            if operands.is_empty() {
                return Err(NjilError::ExecutionError(
                    "减法运算需要至少一个操作数".to_string()
                ));
            }
            
            // 评估第一个操作数
            let first_operand = interpreter.evaluate_value(&operands[0])?;
            
            // 如果只有一个操作数，返回其负值
            if operands.len() == 1 {
                if let Some(n) = type_convert::to_number(&first_operand) {
                    if let Some(num) = serde_json::Number::from_f64(-n) {
                        return Ok(Value::Number(num));
                    }
                }
                return Ok(Value::Null);
            }
            
            // 计算剩余操作数的差
            let mut result = first_operand;
            
            for operand in &operands[1..] {
                // 评估操作数
                let operand_value = interpreter.evaluate_value(operand)?;
                
                // 执行减法运算
                result = type_convert::subtract(&result, &operand_value);
            }
            
            return Ok(result);
        }
        
        // 处理对象格式参数
        if let Value::Object(sub_obj) = value {
            // 检查必要的字段
            if !sub_obj.contains_key("minuend") {
                return Err(NjilError::ExecutionError(
                    "减法运算需要minuend字段".to_string()
                ));
            }
            
            // 获取被减数
            let minuend_value = interpreter.evaluate_value(sub_obj.get("minuend").unwrap())?;
            
            // 检查是否有减数列表
            if sub_obj.contains_key("subtrahends") {
                let subtrahends_value = interpreter.evaluate_value(sub_obj.get("subtrahends").unwrap())?;
                
                if let Value::Array(subtrahends) = subtrahends_value {
                    if subtrahends.is_empty() {
                        return Ok(minuend_value); // 如果没有减数，返回被减数
                    }
                    
                    let mut result = minuend_value;
                    
                    for subtrahend in &subtrahends {
                        // 评估减数
                        let subtrahend_value = interpreter.evaluate_value(subtrahend)?;
                        
                        // 执行减法运算
                        result = type_convert::subtract(&result, &subtrahend_value);
                    }
                    
                    return Ok(result);
                } else {
                    return Err(NjilError::ExecutionError(
                        "subtrahends字段必须是数组".to_string()
                    ));
                }
            }
            
            // 检查是否有单个减数
            if sub_obj.contains_key("subtrahend") {
                let subtrahend_value = interpreter.evaluate_value(sub_obj.get("subtrahend").unwrap())?;
                
                // 执行减法运算
                return Ok(type_convert::subtract(&minuend_value, &subtrahend_value));
            }
            
            return Err(NjilError::ExecutionError(
                "减法运算需要subtrahends或subtrahend字段".to_string()
            ));
        }
        
        Err(NjilError::ExecutionError(
            "减法运算需要一个数组或对象参数".to_string()
        ))
    }
    
    fn name(&self) -> &'static str {
        "math.subtract"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["math.subtract", "subtract", "sub"]
    }
}