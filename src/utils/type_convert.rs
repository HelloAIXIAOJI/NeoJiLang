use serde_json::Value;

/// 类型转换工具模块
/// 提供各种类型之间的转换功能，增强NJIL的弱类型系统

/// 将值转换为布尔类型
pub fn to_bool(value: &Value) -> bool {
    match value {
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_f64().unwrap_or(0.0) != 0.0,
        Value::String(s) => !s.is_empty() && s != "0" && s.to_lowercase() != "false",
        Value::Array(a) => !a.is_empty(),
        Value::Object(o) => !o.is_empty(),
        Value::Null => false,
    }
}

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

/// 将值转换为字符串类型
pub fn to_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        Value::Array(arr) => {
            let mut result = String::from("[");
            for (i, item) in arr.iter().enumerate() {
                if i > 0 {
                    result.push_str(", ");
                }
                result.push_str(&to_string(item));
            }
            result.push(']');
            result
        }
        Value::Object(obj) => {
            // 检查是否是json.new创建的特殊格式
            if obj.contains_key("type") && obj.contains_key("value") && obj.len() == 2 {
                // 这是json.new创建的值，直接返回value字段的内容
                if let Some(val) = obj.get("value") {
                    return to_string(val);
                }
            }
            
            // 对于普通对象类型，序列化为JSON字符串
            if let Ok(json_str) = serde_json::to_string(obj) {
                json_str
            } else {
                // 如果序列化失败，使用简单的键值对表示
                let mut result = String::from("{");
                for (i, (key, val)) in obj.iter().enumerate() {
                    if i > 0 {
                        result.push_str(", ");
                    }
                    result.push_str(&format!("\"{}\": {}", key, to_string(val)));
                }
                result.push('}');
                result
            }
        }
    }
}

/// 将值转换为数组类型
pub fn to_array(value: &Value) -> Vec<Value> {
    match value {
        Value::Array(a) => a.clone(),
        Value::String(s) => {
            // 将字符串拆分为字符数组
            s.chars().map(|c| Value::String(c.to_string())).collect()
        },
        Value::Object(o) => {
            // 将对象转换为键值对数组
            o.iter()
                .map(|(k, v)| {
                    let mut entry = serde_json::Map::new();
                    entry.insert("key".to_string(), Value::String(k.clone()));
                    entry.insert("value".to_string(), v.clone());
                    Value::Object(entry)
                })
                .collect()
        },
        // 其他类型，包装成单元素数组
        _ => vec![value.clone()],
    }
}

/// 将值转换为对象类型
pub fn to_object(value: &Value) -> serde_json::Map<String, Value> {
    match value {
        Value::Object(o) => o.clone(),
        Value::Array(a) => {
            // 将数组转换为索引-值对象
            let mut result = serde_json::Map::new();
            for (i, item) in a.iter().enumerate() {
                result.insert(i.to_string(), item.clone());
            }
            result
        },
        Value::String(s) => {
            // 将字符串转换为字符索引对象
            let mut result = serde_json::Map::new();
            for (i, c) in s.chars().enumerate() {
                result.insert(i.to_string(), Value::String(c.to_string()));
            }
            result.insert("length".to_string(), Value::Number(s.len().into()));
            result
        },
        // 其他类型，创建包含值的对象
        _ => {
            let mut result = serde_json::Map::new();
            result.insert("value".to_string(), value.clone());
            result
        },
    }
}

/// 将值转换为指定类型
pub fn convert_to_type(value: &Value, target_type: &str) -> Value {
    match target_type.to_lowercase().as_str() {
        "boolean" | "bool" => Value::Bool(to_bool(value)),
        "number" | "int" | "float" => {
            if let Some(n) = to_number(value) {
                Value::Number(serde_json::Number::from_f64(n).unwrap_or_else(|| serde_json::Number::from(0)))
            } else {
                Value::Null
            }
        },
        "string" | "str" => Value::String(to_string(value)),
        "array" | "list" => Value::Array(to_array(value)),
        "object" | "map" => Value::Object(to_object(value)),
        _ => value.clone(), // 未知类型，保持原样
    }
}

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
            
            // 如果数值转换失败，转为字符串连接
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
                .filter(|item| !is_equal(item, right))
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