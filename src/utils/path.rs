use serde_json::Value;
use crate::error::NjilError;
use std::collections::HashMap;

/// 变量路径部分
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PathPart {
    ObjectProperty(String),
    ArrayIndex(usize),
}

/// 解析变量路径
pub fn parse_path(path: &str) -> Result<Vec<PathPart>, NjilError> {
    let mut path_parts = Vec::new();
    let mut current_part = String::new();
    let mut in_brackets = false;
    
    // 解析路径
    for c in path.chars() {
        match c {
            '.' if !in_brackets => {
                if !current_part.is_empty() {
                    path_parts.push(PathPart::ObjectProperty(current_part));
                    current_part = String::new();
                }
            },
            '[' if !in_brackets => {
                if !current_part.is_empty() {
                    path_parts.push(PathPart::ObjectProperty(current_part));
                    current_part = String::new();
                }
                in_brackets = true;
            },
            ']' if in_brackets => {
                if let Ok(idx) = current_part.parse::<usize>() {
                    path_parts.push(PathPart::ArrayIndex(idx));
                } else {
                    return Err(NjilError::ExecutionError(
                        format!("无效的数组索引: {}", current_part)
                    ));
                }
                current_part = String::new();
                in_brackets = false;
            },
            _ => {
                current_part.push(c);
            }
        }
    }
    
    // 处理最后一部分
    if !current_part.is_empty() {
        path_parts.push(PathPart::ObjectProperty(current_part));
    }
    
    if path_parts.is_empty() {
        return Err(NjilError::ExecutionError("路径不能为空".to_string()));
    }
    
    Ok(path_parts)
}

/// 获取嵌套值
pub fn get_nested_value<'a>(base_value: &'a Value, path_parts: &[PathPart]) -> Result<&'a Value, NjilError> {
    let mut current_value = base_value;
    
    for (_i, part) in path_parts.iter().enumerate() {
        match part {
            PathPart::ArrayIndex(idx) => {
                // 数组索引访问
                if let Value::Array(arr) = current_value {
                    if *idx < arr.len() {
                        current_value = &arr[*idx];
                    } else {
                        // 索引越界，返回 null
                        static NULL_VALUE: Value = Value::Null;
                        return Ok(&NULL_VALUE);
                    }
                } else {
                    // 不是数组，返回 null
                    static NULL_VALUE: Value = Value::Null;
                    return Ok(&NULL_VALUE);
                }
            },
            PathPart::ObjectProperty(prop) => {
                // 对象属性访问
                if let Value::Object(obj) = current_value {
                    if let Some(prop_value) = obj.get(prop) {
                        current_value = prop_value;
                    } else {
                        // 属性不存在，返回 null
                        static NULL_VALUE: Value = Value::Null;
                        return Ok(&NULL_VALUE);
                    }
                } else {
                    // 不是对象，返回 null
                    static NULL_VALUE: Value = Value::Null;
                    return Ok(&NULL_VALUE);
                }
            }
        }
    }
    
    Ok(current_value)
}

/// 设置嵌套值（在 variables 哈希表中）
pub fn set_nested_value(variables: &mut HashMap<String, Value>, path: &str, value: Value) -> Result<(), NjilError> {
    // 解析变量路径
    let path_parts = parse_path(path)?;
    
    // 获取基础变量名
    let base_var_name = match &path_parts[0] {
        PathPart::ObjectProperty(name) => name,
        _ => return Err(NjilError::ExecutionError("变量名不能是数组索引".to_string())),
    };
    
    // 确保基础变量存在
    if !variables.contains_key(base_var_name) {
        // 如果基础变量不存在，创建一个空对象
        variables.insert(base_var_name.clone(), Value::Object(serde_json::Map::new()));
    }
    
    // 获取可变引用
    let mut current_value = variables.get_mut(base_var_name).unwrap();
    
    // 遍历路径，找到要设置的位置
    for i in 1..path_parts.len() - 1 {
        let part = &path_parts[i];
        
        // 处理数组索引访问
        if let PathPart::ArrayIndex(idx) = part {
            // 确保当前值是数组
            if !current_value.is_array() {
                *current_value = Value::Array(Vec::new());
            }
            
            // 确保数组足够长
            if let Value::Array(arr) = current_value {
                while arr.len() <= *idx {
                    arr.push(Value::Null);
                }
                
                // 如果数组元素是null，替换为对象
                if arr[*idx].is_null() && i < path_parts.len() - 2 {
                    arr[*idx] = Value::Object(serde_json::Map::new());
                }
                
                current_value = &mut arr[*idx];
            }
        } 
        // 处理对象属性访问
        else if let PathPart::ObjectProperty(prop) = part {
            // 确保当前值是对象
            if !current_value.is_object() {
                *current_value = Value::Object(serde_json::Map::new());
            }
            
            // 获取或创建属性
            if let Value::Object(obj) = current_value {
                if !obj.contains_key(prop) && i < path_parts.len() - 2 {
                    obj.insert(prop.clone(), Value::Object(serde_json::Map::new()));
                } else if !obj.contains_key(prop) {
                    obj.insert(prop.clone(), Value::Null);
                }
                
                current_value = obj.get_mut(prop).unwrap();
            }
        }
    }
    
    // 设置最终值
    let last_part = &path_parts[path_parts.len() - 1];
    match last_part {
        PathPart::ArrayIndex(idx) => {
            // 确保当前值是数组
            if !current_value.is_array() {
                *current_value = Value::Array(Vec::new());
            }
            
            // 设置数组元素
            if let Value::Array(arr) = current_value {
                while arr.len() <= *idx {
                    arr.push(Value::Null);
                }
                arr[*idx] = value;
            }
        },
        PathPart::ObjectProperty(prop) => {
            // 确保当前值是对象
            if !current_value.is_object() {
                *current_value = Value::Object(serde_json::Map::new());
            }
            
            // 设置对象属性
            if let Value::Object(obj) = current_value {
                obj.insert(prop.clone(), value);
            }
        }
    }
    
    Ok(())
}

/// 设置嵌套值（在 JSON 对象中）
pub fn set_json_nested_value(json_obj: &mut Value, path_parts: &[PathPart], value: Value) -> Result<(), NjilError> {
    if path_parts.is_empty() {
        return Err(NjilError::ExecutionError("路径不能为空".to_string()));
    }
    
    let mut current_value = json_obj;
    
    // 遍历路径，找到要设置的位置
    for i in 0..path_parts.len() - 1 {
        let part = &path_parts[i];
        
        // 处理数组索引访问
        if let PathPart::ArrayIndex(idx) = part {
            // 确保当前值是数组
            if !current_value.is_array() {
                *current_value = Value::Array(Vec::new());
            }
            
            // 确保数组足够长
            if let Value::Array(arr) = current_value {
                while arr.len() <= *idx {
                    arr.push(Value::Null);
                }
                
                // 如果数组元素是null，替换为对象或数组，取决于下一个路径部分
                if arr[*idx].is_null() {
                    if i < path_parts.len() - 2 {
                        match &path_parts[i + 1] {
                            PathPart::ObjectProperty(_) => {
                                arr[*idx] = Value::Object(serde_json::Map::new());
                            },
                            PathPart::ArrayIndex(_) => {
                                arr[*idx] = Value::Array(Vec::new());
                            }
                        }
                    }
                }
                
                current_value = &mut arr[*idx];
            }
        } 
        // 处理对象属性访问
        else if let PathPart::ObjectProperty(prop) = part {
            // 确保当前值是对象
            if !current_value.is_object() {
                *current_value = Value::Object(serde_json::Map::new());
            }
            
            // 获取或创建属性
            if let Value::Object(obj) = current_value {
                if !obj.contains_key(prop) {
                    // 如果属性不存在，根据下一个路径部分创建适当的值
                    if i < path_parts.len() - 2 {
                        match &path_parts[i + 1] {
                            PathPart::ObjectProperty(_) => {
                                obj.insert(prop.clone(), Value::Object(serde_json::Map::new()));
                            },
                            PathPart::ArrayIndex(_) => {
                                obj.insert(prop.clone(), Value::Array(Vec::new()));
                            }
                        }
                    } else {
                        obj.insert(prop.clone(), Value::Null);
                    }
                }
                
                current_value = obj.get_mut(prop).unwrap();
            }
        }
    }
    
    // 设置最终值
    let last_part = &path_parts[path_parts.len() - 1];
    match last_part {
        PathPart::ArrayIndex(idx) => {
            // 确保当前值是数组
            if !current_value.is_array() {
                *current_value = Value::Array(Vec::new());
            }
            
            // 设置数组元素
            if let Value::Array(arr) = current_value {
                while arr.len() <= *idx {
                    arr.push(Value::Null);
                }
                arr[*idx] = value;
            }
        },
        PathPart::ObjectProperty(prop) => {
            // 确保当前值是对象
            if !current_value.is_object() {
                *current_value = Value::Object(serde_json::Map::new());
            }
            
            // 设置对象属性
            if let Value::Object(obj) = current_value {
                obj.insert(prop.clone(), value);
            }
        }
    }
    
    Ok(())
}

/// 获取嵌套值的路径字符串表示
pub fn path_to_string(path_parts: &[PathPart]) -> String {
    let mut result = String::new();
    
    for (i, part) in path_parts.iter().enumerate() {
        match part {
            PathPart::ObjectProperty(prop) => {
                if i > 0 {
                    result.push('.');
                }
                result.push_str(prop);
            },
            PathPart::ArrayIndex(idx) => {
                result.push_str(&format!("[{}]", idx));
            }
        }
    }
    
    result
} 