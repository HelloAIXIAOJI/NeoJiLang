use crate::error::NjilError;
use serde_json::Value;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use crate::utils::type_convert;

/// 除法运算处理器
pub struct DivideHandler;

// 静态实例
pub static DIVIDE_HANDLER: DivideHandler = DivideHandler;

impl StatementHandler for DivideHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        // 处理数组格式参数
        if let Value::Array(operands) = value {
            if operands.is_empty() {
                return Err(NjilError::ExecutionError(
                    "除法运算需要至少一个操作数".to_string()
                ));
            }
            
            // 评估第一个操作数
            let first_operand = interpreter.evaluate_value(&operands[0])?;
            
            // 如果只有一个操作数，返回其倒数
            if operands.len() == 1 {
                if let Some(n) = type_convert::to_number(&first_operand) {
                    if n == 0.0 {
                        return Ok(Value::Null); // 除以零返回null
                    }
                    if let Some(num) = serde_json::Number::from_f64(1.0 / n) {
                        return Ok(Value::Number(num));
                    }
                }
                return Ok(Value::Null);
            }
            
            // 计算剩余操作数的商
            let mut result = first_operand;
            
            for operand in &operands[1..] {
                // 评估操作数
                let operand_value = interpreter.evaluate_value(operand)?;
                
                // 执行除法运算
                result = type_convert::divide(&result, &operand_value);
            }
            
            return Ok(result);
        }
        
        // 处理对象格式参数
        if let Value::Object(div_obj) = value {
            // 检查必要的字段
            if !div_obj.contains_key("dividend") {
                return Err(NjilError::ExecutionError(
                    "除法运算需要dividend字段".to_string()
                ));
            }
            
            // 获取被除数
            let dividend_value = interpreter.evaluate_value(div_obj.get("dividend").unwrap())?;
            
            // 检查是否有除数列表
            if div_obj.contains_key("divisors") {
                let divisors_value = interpreter.evaluate_value(div_obj.get("divisors").unwrap())?;
                
                if let Value::Array(divisors) = divisors_value {
                    if divisors.is_empty() {
                        return Ok(dividend_value); // 如果没有除数，返回被除数
                    }
                    
                    let mut result = dividend_value;
                    
                    for divisor in &divisors {
                        // 评估除数
                        let divisor_value = interpreter.evaluate_value(divisor)?;
                        
                        // 执行除法运算
                        result = type_convert::divide(&result, &divisor_value);
                    }
                    
                    return Ok(result);
                } else {
                    return Err(NjilError::ExecutionError(
                        "divisors字段必须是数组".to_string()
                    ));
                }
            }
            
            // 检查是否有单个除数
            if div_obj.contains_key("divisor") {
                let divisor_value = interpreter.evaluate_value(div_obj.get("divisor").unwrap())?;
                
                // 执行除法运算
                return Ok(type_convert::divide(&dividend_value, &divisor_value));
            }
            
            return Err(NjilError::ExecutionError(
                "除法运算需要divisors或divisor字段".to_string()
            ));
        }
        
        Err(NjilError::ExecutionError(
            "除法运算需要一个数组或对象参数".to_string()
        ))
    }
    
    fn name(&self) -> &'static str {
        "math.divide"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["math.divide", "divide", "div"]
    }
} 