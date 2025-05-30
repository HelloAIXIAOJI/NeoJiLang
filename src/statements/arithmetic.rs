use crate::error::NjilError;
use serde_json::Value;
use crate::interpreter::Interpreter;
use super::StatementHandler;
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
            
            // 计算所有操作数的和
            let mut result = Value::Number(serde_json::Number::from(0));
            
            for operand in operands {
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

/// 减法运算处理器
pub struct SubtractHandler;

// 静态实例
pub static SUBTRACT_HANDLER: SubtractHandler = SubtractHandler;

impl StatementHandler for SubtractHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
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
            
            Ok(result)
        } else {
            Err(NjilError::ExecutionError(
                "减法运算需要一个数组参数".to_string()
            ))
        }
    }
    
    fn name(&self) -> &'static str {
        "math.subtract"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["math.subtract", "subtract", "sub"]
    }
}

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

/// 除法运算处理器
pub struct DivideHandler;

// 静态实例
pub static DIVIDE_HANDLER: DivideHandler = DivideHandler;

impl StatementHandler for DivideHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
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
            
            Ok(result)
        } else {
            Err(NjilError::ExecutionError(
                "除法运算需要一个数组参数".to_string()
            ))
        }
    }
    
    fn name(&self) -> &'static str {
        "math.divide"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["math.divide", "divide", "div"]
    }
}

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

/// 比较运算处理器
pub struct CompareHandler;

// 静态实例
pub static COMPARE_HANDLER: CompareHandler = CompareHandler;

impl StatementHandler for CompareHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Object(cmp_obj) = value {
            // 检查必要的字段
            if !cmp_obj.contains_key("left") || !cmp_obj.contains_key("right") || !cmp_obj.contains_key("operator") {
                return Err(NjilError::ExecutionError(
                    "比较运算需要left、right和operator字段".to_string()
                ));
            }
            
            // 获取左操作数
            let left_value = interpreter.evaluate_value(cmp_obj.get("left").unwrap())?;
            
            // 获取右操作数
            let right_value = interpreter.evaluate_value(cmp_obj.get("right").unwrap())?;
            
            // 获取比较运算符
            let operator = match cmp_obj.get("operator") {
                Some(Value::String(op)) => op,
                _ => return Err(NjilError::ExecutionError("operator字段必须是字符串".to_string())),
            };
            
            // 执行比较运算
            match operator.as_str() {
                "==" | "eq" => Ok(Value::Bool(type_convert::is_equal(&left_value, &right_value))),
                "!=" | "ne" => Ok(Value::Bool(!type_convert::is_equal(&left_value, &right_value))),
                ">" | "gt" => {
                    match type_convert::compare(&left_value, &right_value) {
                        Some(std::cmp::Ordering::Greater) => Ok(Value::Bool(true)),
                        _ => Ok(Value::Bool(false)),
                    }
                },
                ">=" | "ge" => {
                    match type_convert::compare(&left_value, &right_value) {
                        Some(std::cmp::Ordering::Greater) | Some(std::cmp::Ordering::Equal) => Ok(Value::Bool(true)),
                        _ => Ok(Value::Bool(false)),
                    }
                },
                "<" | "lt" => {
                    match type_convert::compare(&left_value, &right_value) {
                        Some(std::cmp::Ordering::Less) => Ok(Value::Bool(true)),
                        _ => Ok(Value::Bool(false)),
                    }
                },
                "<=" | "le" => {
                    match type_convert::compare(&left_value, &right_value) {
                        Some(std::cmp::Ordering::Less) | Some(std::cmp::Ordering::Equal) => Ok(Value::Bool(true)),
                        _ => Ok(Value::Bool(false)),
                    }
                },
                _ => Err(NjilError::ExecutionError(format!("未知的比较运算符: {}", operator))),
            }
        } else {
            Err(NjilError::ExecutionError(
                "比较运算需要一个对象参数".to_string()
            ))
        }
    }
    
    fn name(&self) -> &'static str {
        "math.compare"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["math.compare", "compare", "cmp"]
    }
} 