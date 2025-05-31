use serde_json::Value;

/// 将值转换为数字类型
pub fn to_number(value: &Value) -> Option<f64> {
    match value {
        Value::Number(n) => n.as_f64(),
        Value::String(s) => {
            // 尝试将字符串解析为数字
            if let Ok(n) = s.parse::<f64>() {
                Some(n)
            } else {
                None
            }
        },
        Value::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
        Value::Array(a) => {
            if a.is_empty() {
                Some(0.0)
            } else if a.len() == 1 {
                // 如果数组只有一个元素，尝试将其转换为数字
                to_number(&a[0])
            } else {
                // 如果数组有多个元素，返回数组长度
                Some(a.len() as f64)
            }
        },
        Value::Object(o) => Some(o.len() as f64), // 对象的键值对数量
        Value::Null => Some(0.0),
    }
} 