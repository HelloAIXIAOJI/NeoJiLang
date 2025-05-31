use serde_json::Value;
use super::string_convert::to_string;
use super::number_convert::to_number;

/// 执行两个值之间的加法运算，根据类型自动转换
pub fn add(left: &Value, right: &Value) -> Value {
    match (left, right) {
        // 数字 + 数字 = 数字
        (Value::Number(l), Value::Number(r)) => {
            if let (Some(lf), Some(rf)) = (l.as_f64(), r.as_f64()) {
                if let Some(num) = serde_json::Number::from_f64(lf + rf) {
                    return Value::Number(num);
                }
            }
            Value::Null
        },
        
        // 字符串 + 任何类型 = 字符串连接
        (Value::String(l), _) => {
            let mut result = l.clone();
            result.push_str(&to_string(right));
            Value::String(result)
        },
        
        // 任何类型 + 字符串 = 字符串连接
        (_, Value::String(r)) => {
            let mut result = to_string(left);
            result.push_str(r);
            Value::String(result)
        },
        
        // 数组 + 数组 = 合并数组
        (Value::Array(l), Value::Array(r)) => {
            let mut result = l.clone();
            result.extend(r.clone());
            Value::Array(result)
        },
        
        // 数组 + 其他类型 = 添加到数组末尾
        (Value::Array(l), _) => {
            let mut result = l.clone();
            result.push(right.clone());
            Value::Array(result)
        },
        
        // 其他类型 + 数组 = 添加到数组开头
        (_, Value::Array(r)) => {
            let mut result = vec![left.clone()];
            result.extend(r.clone());
            Value::Array(result)
        },
        
        // 对象 + 对象 = 合并对象
        (Value::Object(l), Value::Object(r)) => {
            let mut result = l.clone();
            for (k, v) in r {
                result.insert(k.clone(), v.clone());
            }
            Value::Object(result)
        },
        
        // 布尔值 + 布尔值 = 逻辑或
        (Value::Bool(l), Value::Bool(r)) => Value::Bool(*l || *r),
        
        // 其他情况，尝试数值转换
        _ => {
            if let (Some(lf), Some(rf)) = (to_number(left), to_number(right)) {
                if let Some(num) = serde_json::Number::from_f64(lf + rf) {
                    return Value::Number(num);
                }
            }
            
            // 如果数值转换失败，转为字符串连接（修复：直接使用字符串连接，不添加前缀）
            Value::String(format!("{}{}", to_string(left), to_string(right)))
        }
    }
}

/// 执行两个值之间的减法运算，根据类型自动转换
pub fn subtract(left: &Value, right: &Value) -> Value {
    match (left, right) {
        // 数字 - 数字 = 数字
        (Value::Number(l), Value::Number(r)) => {
            if let (Some(lf), Some(rf)) = (l.as_f64(), r.as_f64()) {
                if let Some(num) = serde_json::Number::from_f64(lf - rf) {
                    return Value::Number(num);
                }
            }
            Value::Null
        },
        
        // 数组 - 值 = 移除包含该值的元素
        (Value::Array(l), _) => {
            let filtered: Vec<Value> = l.iter()
                .filter(|item| !super::comparison::is_equal(item, right))
                .cloned()
                .collect();
            Value::Array(filtered)
        },
        
        // 对象 - 字符串键 = 移除该键
        (Value::Object(l), Value::String(key)) => {
            let mut result = l.clone();
            result.remove(key);
            Value::Object(result)
        },
        
        // 字符串 - 字符串 = 移除所有匹配子串
        (Value::String(l), Value::String(r)) => {
            Value::String(l.replace(r, ""))
        },
        
        // 其他情况，尝试数值转换
        _ => {
            if let (Some(lf), Some(rf)) = (to_number(left), to_number(right)) {
                if let Some(num) = serde_json::Number::from_f64(lf - rf) {
                    return Value::Number(num);
                }
            }
            
            // 如果转换失败，返回null
            Value::Null
        }
    }
}

/// 执行两个值之间的乘法运算，根据类型自动转换
pub fn multiply(left: &Value, right: &Value) -> Value {
    match (left, right) {
        // 数字 * 数字 = 数字
        (Value::Number(l), Value::Number(r)) => {
            if let (Some(lf), Some(rf)) = (l.as_f64(), r.as_f64()) {
                if let Some(num) = serde_json::Number::from_f64(lf * rf) {
                    return Value::Number(num);
                }
            }
            Value::Null
        },
        
        // 字符串 * 数字 = 重复字符串
        (Value::String(s), Value::Number(n)) => {
            if let Some(count) = n.as_u64() {
                let count = count as usize;
                if count > 0 {
                    return Value::String(s.repeat(count));
                }
            }
            Value::String(String::new())
        },
        
        // 数字 * 字符串 = 重复字符串
        (Value::Number(n), Value::String(s)) => {
            if let Some(count) = n.as_u64() {
                let count = count as usize;
                if count > 0 {
                    return Value::String(s.repeat(count));
                }
            }
            Value::String(String::new())
        },
        
        // 数组 * 数字 = 重复数组
        (Value::Array(a), Value::Number(n)) => {
            if let Some(count) = n.as_u64() {
                let count = count as usize;
                if count > 0 {
                    let mut result = Vec::new();
                    for _ in 0..count {
                        result.extend(a.clone());
                    }
                    return Value::Array(result);
                }
            }
            Value::Array(Vec::new())
        },
        
        // 数字 * 数组 = 重复数组
        (Value::Number(n), Value::Array(a)) => {
            if let Some(count) = n.as_u64() {
                let count = count as usize;
                if count > 0 {
                    let mut result = Vec::new();
                    for _ in 0..count {
                        result.extend(a.clone());
                    }
                    return Value::Array(result);
                }
            }
            Value::Array(Vec::new())
        },
        
        // 其他情况，尝试数值转换
        _ => {
            if let (Some(lf), Some(rf)) = (to_number(left), to_number(right)) {
                if let Some(num) = serde_json::Number::from_f64(lf * rf) {
                    return Value::Number(num);
                }
            }
            
            // 如果转换失败，返回null
            Value::Null
        }
    }
}

/// 执行两个值之间的除法运算，根据类型自动转换
pub fn divide(left: &Value, right: &Value) -> Value {
    match (left, right) {
        // 数字 / 数字 = 数字
        (Value::Number(l), Value::Number(r)) => {
            if let (Some(lf), Some(rf)) = (l.as_f64(), r.as_f64()) {
                if rf == 0.0 {
                    return Value::Null; // 除以零返回null
                }
                if let Some(num) = serde_json::Number::from_f64(lf / rf) {
                    return Value::Number(num);
                }
            }
            Value::Null
        },
        
        // 字符串 / 字符串 = 分割为数组
        (Value::String(s), Value::String(delimiter)) => {
            let parts: Vec<Value> = s.split(delimiter)
                .map(|part| Value::String(part.to_string()))
                .collect();
            Value::Array(parts)
        },
        
        // 数组 / 数字 = 分割数组
        (Value::Array(a), Value::Number(n)) => {
            if let Some(chunk_size) = n.as_u64() {
                if chunk_size == 0 {
                    return Value::Null;
                }
                
                let chunk_size = chunk_size as usize;
                let mut result = Vec::new();
                let mut current_chunk = Vec::new();
                
                for (i, item) in a.iter().enumerate() {
                    if i > 0 && i % chunk_size == 0 {
                        result.push(Value::Array(current_chunk));
                        current_chunk = Vec::new();
                    }
                    current_chunk.push(item.clone());
                }
                
                if !current_chunk.is_empty() {
                    result.push(Value::Array(current_chunk));
                }
                
                return Value::Array(result);
            }
            Value::Array(Vec::new())
        },
        
        // 其他情况，尝试数值转换
        _ => {
            if let (Some(lf), Some(rf)) = (to_number(left), to_number(right)) {
                if rf == 0.0 {
                    return Value::Null; // 除以零返回null
                }
                if let Some(num) = serde_json::Number::from_f64(lf / rf) {
                    return Value::Number(num);
                }
            }
            
            // 如果转换失败，返回null
            Value::Null
        }
    }
} 