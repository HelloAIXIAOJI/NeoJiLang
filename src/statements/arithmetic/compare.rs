use crate::error::NjilError;
use serde_json::Value;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use crate::utils::type_convert;

/// 比较运算处理器
pub struct CompareHandler;

// 静态实例
pub static COMPARE_HANDLER: CompareHandler = CompareHandler;

impl StatementHandler for CompareHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Object(compare_obj) = value {
            // 检查必要的字段
            if !compare_obj.contains_key("left") || !compare_obj.contains_key("right") || !compare_obj.contains_key("op") {
                return Err(NjilError::ExecutionError(
                    "比较运算需要left、right和op字段".to_string()
                ));
            }
            
            // 获取左操作数
            let left_value = interpreter.evaluate_value(compare_obj.get("left").unwrap())?;
            
            // 获取右操作数
            let right_value = interpreter.evaluate_value(compare_obj.get("right").unwrap())?;
            
            // 获取操作符
            let op_value = interpreter.evaluate_value(compare_obj.get("op").unwrap())?;
            let op = if let Value::String(s) = &op_value {
                s.as_str()
            } else {
                return Err(NjilError::ExecutionError(
                    "比较运算的op字段必须是字符串".to_string()
                ));
            };
            
            // 执行比较
            let result = match op {
                "==" => self.equals(&left_value, &right_value),
                "!=" => !self.equals(&left_value, &right_value),
                ">" => self.greater_than(&left_value, &right_value),
                ">=" => self.greater_than_or_equal(&left_value, &right_value),
                "<" => self.less_than(&left_value, &right_value),
                "<=" => self.less_than_or_equal(&left_value, &right_value),
                _ => return Err(NjilError::ExecutionError(
                    format!("不支持的比较操作符: {}", op)
                )),
            };
            
            Ok(Value::Bool(result))
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

impl CompareHandler {
    /// 判断两个值是否相等
    fn equals(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => {
                // 数字比较
                if let (Some(l_f64), Some(r_f64)) = (l.as_f64(), r.as_f64()) {
                    (l_f64 - r_f64).abs() < f64::EPSILON
                } else {
                    false
                }
            },
            (Value::String(l), Value::String(r)) => l == r,
            (Value::Bool(l), Value::Bool(r)) => l == r,
            (Value::Null, Value::Null) => true,
            _ => false, // 不同类型视为不相等
        }
    }
    
    /// 判断左值是否大于右值
    fn greater_than(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => {
                if let (Some(l_f64), Some(r_f64)) = (l.as_f64(), r.as_f64()) {
                    l_f64 > r_f64
                } else {
                    false
                }
            },
            (Value::String(l), Value::String(r)) => l > r,
            _ => false, // 其他类型不支持大于比较
        }
    }
    
    /// 判断左值是否大于等于右值
    fn greater_than_or_equal(&self, left: &Value, right: &Value) -> bool {
        self.greater_than(left, right) || self.equals(left, right)
    }
    
    /// 判断左值是否小于右值
    fn less_than(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => {
                if let (Some(l_f64), Some(r_f64)) = (l.as_f64(), r.as_f64()) {
                    l_f64 < r_f64
                } else {
                    false
                }
            },
            (Value::String(l), Value::String(r)) => l < r,
            _ => false, // 其他类型不支持小于比较
        }
    }
    
    /// 判断左值是否小于等于右值
    fn less_than_or_equal(&self, left: &Value, right: &Value) -> bool {
        self.less_than(left, right) || self.equals(left, right)
    }
} 