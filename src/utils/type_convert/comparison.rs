use serde_json::Value;
use super::bool_convert::to_bool;
use super::number_convert::to_number;
use super::string_convert::to_string;

/// 比较两个值是否相等
pub fn is_equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        // 相同类型直接比较
        (Value::Null, Value::Null) => true,
        (Value::Bool(l), Value::Bool(r)) => l == r,
        (Value::Number(l), Value::Number(r)) => {
            if let (Some(lf), Some(rf)) = (l.as_f64(), r.as_f64()) {
                (lf - rf).abs() < f64::EPSILON
            } else {
                false
            }
        },
        (Value::String(l), Value::String(r)) => l == r,
        
        // 数字和字符串比较
        (Value::Number(n), Value::String(s)) | (Value::String(s), Value::Number(n)) => {
            if let Ok(parsed) = s.parse::<f64>() {
                if let Some(num) = n.as_f64() {
                    return (parsed - num).abs() < f64::EPSILON;
                }
            }
            false
        },
        
        // 布尔值和其他类型比较
        (Value::Bool(b), _) => *b == to_bool(right),
        (_, Value::Bool(b)) => *b == to_bool(left),
        
        // 数组比较
        (Value::Array(l), Value::Array(r)) => {
            if l.len() != r.len() {
                return false;
            }
            l.iter().zip(r.iter()).all(|(lv, rv)| is_equal(lv, rv))
        },
        
        // 对象比较
        (Value::Object(l), Value::Object(r)) => {
            if l.len() != r.len() {
                return false;
            }
            l.iter().all(|(k, lv)| {
                if let Some(rv) = r.get(k) {
                    is_equal(lv, rv)
                } else {
                    false
                }
            })
        },
        
        // 其他类型比较，尝试转换为字符串
        _ => to_string(left) == to_string(right),
    }
}

/// 比较两个值的大小关系
pub fn compare(left: &Value, right: &Value) -> Option<std::cmp::Ordering> {
    match (left, right) {
        // 数字比较
        (Value::Number(l), Value::Number(r)) => {
            if let (Some(lf), Some(rf)) = (l.as_f64(), r.as_f64()) {
                return lf.partial_cmp(&rf);
            }
        },
        
        // 字符串比较
        (Value::String(l), Value::String(r)) => {
            return Some(l.cmp(r));
        },
        
        // 数字和字符串比较
        (Value::Number(n), Value::String(s)) => {
            if let (Some(num), Ok(parsed)) = (n.as_f64(), s.parse::<f64>()) {
                return num.partial_cmp(&parsed);
            }
        },
        (Value::String(s), Value::Number(n)) => {
            if let (Ok(parsed), Some(num)) = (s.parse::<f64>(), n.as_f64()) {
                return parsed.partial_cmp(&num);
            }
        },
        
        // 布尔值比较
        (Value::Bool(l), Value::Bool(r)) => {
            return Some(l.cmp(r));
        },
        
        // 数组长度比较
        (Value::Array(l), Value::Array(r)) => {
            return Some(l.len().cmp(&r.len()));
        },
        
        // 对象大小比较
        (Value::Object(l), Value::Object(r)) => {
            return Some(l.len().cmp(&r.len()));
        },
        
        // 其他类型比较，尝试转换为数字
        _ => {
            if let (Some(lf), Some(rf)) = (to_number(left), to_number(right)) {
                return lf.partial_cmp(&rf);
            }
        }
    }
    
    // 无法比较时返回None
    None
} 