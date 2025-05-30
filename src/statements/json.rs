use crate::error::NjilError;
use serde_json::{Value, Map};
use crate::interpreter::Interpreter;
use super::StatementHandler;
use crate::utils::path;

/// JSON新建语句处理器
pub struct JsonNewHandler;

// 静态实例
pub static JSON_NEW_HANDLER: JsonNewHandler = JsonNewHandler;

impl StatementHandler for JsonNewHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        match value {
            // 如果传入null，创建一个空的JSON对象
            Value::Null => Ok(Value::Object(Map::new())),
            
            // 如果传入一个对象，需要递归处理对象中可能的语句
            Value::Object(obj) => {
                let mut result_obj = Map::new();
                
                // 遍历对象的每个键值对
                for (key, val) in obj {
                    // 递归处理值，支持嵌套语句
                    let processed_val = interpreter.evaluate_value(val)?;
                    result_obj.insert(key.clone(), processed_val);
                }
                
                Ok(Value::Object(result_obj))
            },
            
            // 如果传入一个数组，需要递归处理数组中可能的语句
            Value::Array(arr) => {
                let mut result_arr = Vec::with_capacity(arr.len());
                
                // 遍历数组中的每个元素
                for item in arr {
                    // 递归处理值，支持嵌套语句
                    let processed_item = interpreter.evaluate_value(item)?;
                    result_arr.push(processed_item);
                }
                
                Ok(Value::Array(result_arr))
            },
            
            // 其他类型，返回错误
            _ => Err(NjilError::ExecutionError(format!(
                "json.new 需要null、对象或数组类型参数，但收到了: {}", 
                value.to_string()
            )))
        }
    }
    
    fn name(&self) -> &'static str {
        "json.new"
    }
}

/// JSON获取属性语句处理器
pub struct JsonGetHandler;

// 静态实例
pub static JSON_GET_HANDLER: JsonGetHandler = JsonGetHandler;

impl StatementHandler for JsonGetHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Object(obj) = value {
            // 获取JSON对象
            let json_obj = if let Some(obj_value) = obj.get("object") {
                interpreter.evaluate_value(obj_value)?
            } else {
                return Err(NjilError::ExecutionError(
                    "json.get 缺少 'object' 参数".to_string()
                ));
            };
            
            // 获取键或索引
            let key = if let Some(key_value) = obj.get("key") {
                interpreter.evaluate_value(key_value)?
            } else {
                return Err(NjilError::ExecutionError(
                    "json.get 缺少 'key' 参数".to_string()
                ));
            };
            
            // 处理嵌套路径
            if let Value::String(path_str) = &key {
                if path_str.contains('.') || path_str.contains('[') {
                    // 解析路径
                    let path_parts = path::parse_path(path_str)?;
                    
                    // 获取基础属性
                    let base_prop = match &path_parts[0] {
                        path::PathPart::ObjectProperty(name) => name,
                        _ => return Err(NjilError::ExecutionError("路径不能以数组索引开始".to_string())),
                    };
                    
                    // 检查基础属性是否存在
                    if let Value::Object(map) = &json_obj {
                        if !map.contains_key(base_prop) {
                            return Ok(Value::Null);
                        }
                        
                        // 从基础属性继续访问嵌套路径
                        if path_parts.len() > 1 {
                            // 获取基础属性值
                            let base_value = map.get(base_prop).unwrap();
                            
                            // 使用通用路径访问方法获取嵌套值
                            match path::get_nested_value(base_value, &path_parts[1..]) {
                                Ok(nested_value) => return Ok(nested_value.clone()),
                                Err(_) => return Ok(Value::Null), // 如果路径不存在，返回null
                            }
                        } else {
                            // 如果只有基础属性，直接返回其值
                            return Ok(map.get(base_prop).unwrap().clone());
                        }
                    } else if let Value::Array(_arr) = &json_obj {
                        // 不支持对数组使用嵌套路径
                        return Err(NjilError::ExecutionError(
                            "不支持对数组使用嵌套路径".to_string()
                        ));
                    } else {
                        return Ok(Value::Null);
                    }
                }
            }
            
            match (&json_obj, &key) {
                // 处理对象属性访问
                (Value::Object(map), Value::String(key_str)) => {
                    if let Some(value) = map.get(key_str) {
                        return Ok(value.clone());
                    }
                    return Ok(Value::Null);
                },
                
                // 处理数组索引访问
                (Value::Array(arr), Value::Number(idx)) => {
                    if let Some(idx_u64) = idx.as_u64() {
                        let idx_usize = idx_u64 as usize;
                        if idx_usize < arr.len() {
                            return Ok(arr[idx_usize].clone());
                        }
                    }
                    return Ok(Value::Null);
                },
                
                // 不支持的类型组合
                _ => return Err(NjilError::ExecutionError(format!(
                    "json.get 不支持的类型组合: 对象类型 {} 和键类型 {}", 
                    json_obj.to_string(), key.to_string()
                ))),
            }
        }
        
        Err(NjilError::ExecutionError(
            "json.get 需要一个包含 'object' 和 'key' 的对象".to_string()
        ))
    }
    
    fn name(&self) -> &'static str {
        "json.get"
    }
}

/// JSON设置属性语句处理器
pub struct JsonSetHandler;

// 静态实例
pub static JSON_SET_HANDLER: JsonSetHandler = JsonSetHandler;

impl StatementHandler for JsonSetHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Object(obj) = value {
            // 获取JSON对象
            let json_obj = if let Some(obj_value) = obj.get("object") {
                interpreter.evaluate_value(obj_value)?
            } else {
                return Err(NjilError::ExecutionError(
                    "json.set 缺少 'object' 参数".to_string()
                ));
            };
            
            // 创建一个新的JSON对象，避免修改原对象
            let mut result_obj = json_obj.clone();
            
            // 获取键或索引
            let key = if let Some(key_value) = obj.get("key") {
                interpreter.evaluate_value(key_value)?
            } else {
                return Err(NjilError::ExecutionError(
                    "json.set 缺少 'key' 参数".to_string()
                ));
            };
            
            // 获取要设置的值
            let set_value = if let Some(val) = obj.get("value") {
                interpreter.evaluate_value(val)?
            } else {
                return Err(NjilError::ExecutionError(
                    "json.set 缺少 'value' 参数".to_string()
                ));
            };
            
            // 处理嵌套路径
            if let Value::String(path_str) = &key {
                if path_str.contains('.') || path_str.contains('[') {
                    // 解析路径
                    let path_parts = path::parse_path(path_str)?;
                    
                    // 设置嵌套值
                    match path::set_json_nested_value(&mut result_obj, &path_parts, set_value.clone()) {
                        Ok(_) => return Ok(result_obj),
                        Err(e) => return Err(NjilError::ExecutionError(
                            format!("设置嵌套路径 {} 失败: {}", path_str, e)
                        )),
                    }
                }
            }
            
            match (&mut result_obj, &key) {
                // 处理对象属性设置
                (Value::Object(map), Value::String(key_str)) => {
                    map.insert(key_str.clone(), set_value);
                    return Ok(result_obj);
                },
                
                // 处理数组索引设置
                (Value::Array(arr), Value::Number(idx)) => {
                    if let Some(idx_u64) = idx.as_u64() {
                        let idx_usize = idx_u64 as usize;
                        // 确保索引在范围内
                        if idx_usize < arr.len() {
                            arr[idx_usize] = set_value;
                            return Ok(result_obj);
                        } else {
                            return Err(NjilError::ExecutionError(format!(
                                "json.set 数组索引越界: 索引 {} 超出数组长度 {}", 
                                idx_usize, arr.len()
                            )));
                        }
                    }
                    return Err(NjilError::ExecutionError(
                        "json.set 数组索引必须是非负整数".to_string()
                    ));
                },
                
                // 不支持的类型组合
                _ => return Err(NjilError::ExecutionError(format!(
                    "json.set 不支持的类型组合: 对象类型 {} 和键类型 {}", 
                    result_obj.to_string(), key.to_string()
                ))),
            }
        }
        
        Err(NjilError::ExecutionError(
            "json.set 需要一个包含 'object', 'key' 和 'value' 的对象".to_string()
        ))
    }
    
    fn name(&self) -> &'static str {
        "json.set"
    }
} 