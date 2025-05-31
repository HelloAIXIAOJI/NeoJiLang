use crate::error::NjilError;
use serde_json::Value;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use crate::utils::type_convert;

/// 取模运算处理器
pub struct ModuloHandler;

// 静态实例
pub static MODULO_HANDLER: ModuloHandler = ModuloHandler;

impl StatementHandler for ModuloHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Object(mod_obj) = value {
            // 检查必要的字段
            if !mod_obj.contains_key("dividend") || !mod_obj.contains_key("divisor") {
                return Err(NjilError::ExecutionError(
                    "取模运算需要dividend和divisor字段".to_string()
                ));
            }
            
            // 获取被除数
            let dividend_value = interpreter.evaluate_value(mod_obj.get("dividend").unwrap())?;
            let dividend = match type_convert::to_number(&dividend_value) {
                Some(n) => n,
                None => return Ok(Value::Null),
            };
            
            // 获取除数
            let divisor_value = interpreter.evaluate_value(mod_obj.get("divisor").unwrap())?;
            let divisor = match type_convert::to_number(&divisor_value) {
                Some(n) => n,
                None => return Ok(Value::Null),
            };
            
            // 检查除数是否为零
            if divisor == 0.0 {
                return Ok(Value::Null);
            }
            
            // 计算余数
            let remainder = dividend % divisor;
            
            if let Some(num) = serde_json::Number::from_f64(remainder) {
                Ok(Value::Number(num))
            } else {
                Ok(Value::Null)
            }
        } else {
            Err(NjilError::ExecutionError(
                "取模运算需要一个对象参数".to_string()
            ))
        }
    }
    
    fn name(&self) -> &'static str {
        "math.modulo"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["math.modulo", "modulo", "mod"]
    }
} 