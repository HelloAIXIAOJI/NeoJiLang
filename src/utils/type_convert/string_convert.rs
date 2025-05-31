use serde_json::Value;

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