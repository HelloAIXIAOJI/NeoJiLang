// 类型转换工具模块 - 导出所有类型转换功能

// 子模块
pub mod bool_convert;
pub mod number_convert;
pub mod string_convert;
pub mod array_convert;
pub mod object_convert;
pub mod arithmetic;
pub mod comparison;

// 重新导出所有功能，方便其他模块使用
pub use self::bool_convert::to_bool;
pub use self::number_convert::to_number;
pub use self::string_convert::to_string;
pub use self::array_convert::to_array;
pub use self::object_convert::to_object;
pub use self::arithmetic::{add, subtract, multiply, divide};
pub use self::comparison::{is_equal, compare};

/// 将值转换为指定类型
pub fn convert_to_type(value: &serde_json::Value, target_type: &str) -> serde_json::Value {
    match target_type.to_lowercase().as_str() {
        "boolean" | "bool" => serde_json::Value::Bool(to_bool(value)),
        "number" | "int" | "float" => {
            if let Some(n) = to_number(value) {
                serde_json::Value::Number(serde_json::Number::from_f64(n).unwrap_or_else(|| serde_json::Number::from(0)))
            } else {
                serde_json::Value::Null
            }
        },
        "string" | "str" => serde_json::Value::String(to_string(value)),
        "array" | "list" => serde_json::Value::Array(to_array(value)),
        "object" | "map" => serde_json::Value::Object(to_object(value)),
        _ => value.clone(), // 未知类型，保持原样
    }
} 