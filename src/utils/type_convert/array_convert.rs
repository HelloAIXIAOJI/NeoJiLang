use serde_json::Value;

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