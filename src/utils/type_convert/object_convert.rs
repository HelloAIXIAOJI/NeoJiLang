use serde_json::Value;

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