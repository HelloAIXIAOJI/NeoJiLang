use crate::error::NjilError;
use serde_json::Value;
use crate::interpreter::Interpreter;
use crate::errortip;
use crate::debug_println;
use super::StatementHandler;
use crate::utils::path::{self, PathPart};

/// 常量定义语句处理器
pub struct ConstSetHandler;

// 静态实例
pub static CONST_SET_HANDLER: ConstSetHandler = ConstSetHandler;

impl StatementHandler for ConstSetHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Object(const_obj) = value {
            // 检查是否有name和value字段
            if !const_obj.contains_key("name") || !const_obj.contains_key("value") {
                return Err(NjilError::ExecutionError(
                    "const.set需要name和value字段".to_string()
                ));
            }
            
            // 获取常量名
            let name_value = interpreter.evaluate_value(const_obj.get("name").unwrap())?;
            let const_name = match name_value {
                Value::String(s) => s,
                _ => return Err(NjilError::ExecutionError("常量名必须是字符串".to_string())),
            };
            
            debug_println!("设置常量 {} 的值", const_name);
            
            // 检查常量是否已存在
            if interpreter.has_constant(&const_name) {
                return Err(NjilError::ExecutionError(
                    format!("常量'{}'已经定义，常量不可重新定义", const_name)
                ));
            }
            
            // 获取常量值
            let const_value = interpreter.evaluate_value(const_obj.get("value").unwrap())?;
            debug_println!("常量 {} 的值为: {}", const_name, serde_json::to_string_pretty(&const_value).unwrap());
                
            // 检查是否为嵌套路径
            if const_name.contains('.') || const_name.contains('[') {
                path::set_nested_value(&mut interpreter.constants, &const_name, const_value)?;
            } else {
                // 普通常量设置
                interpreter.constants.insert(const_name, const_value);
            }
            
            Ok(Value::Null)
        } else {
            Err(NjilError::ExecutionError("const.set需要一个对象作为参数".to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "const.set"
    }
}

/// 多常量定义语句处理器 - 使用简洁的键值对格式
pub struct ConstSetMultiHandler;

// 静态实例
pub static CONST_SET_MULTI_HANDLER: ConstSetMultiHandler = ConstSetMultiHandler;

impl StatementHandler for ConstSetMultiHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Object(const_obj) = value {
            debug_println!("使用键值对格式设置多个常量，数量: {}", const_obj.len());
            
            // 遍历对象中的所有键值对，每一个都作为常量
            for (const_name, const_value_raw) in const_obj {
                // 检查常量是否已存在
                if interpreter.has_constant(const_name) {
                    return Err(NjilError::ExecutionError(
                        format!("常量'{}'已经定义，常量不可重新定义", const_name)
                    ));
                }
                
                // 计算常量值
                let const_value = interpreter.evaluate_value(const_value_raw)?;
                debug_println!("设置常量 {} 的值: {}", const_name, serde_json::to_string_pretty(&const_value).unwrap());
                
                // 检查是否为嵌套路径
                if const_name.contains('.') || const_name.contains('[') {
                    path::set_nested_value(&mut interpreter.constants, const_name, const_value)?;
                } else {
                    // 普通常量设置
                    interpreter.constants.insert(const_name.clone(), const_value);
                }
            }
            
            Ok(Value::Null)
        } else {
            Err(NjilError::ExecutionError("const.set.m需要一个对象作为参数".to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "const.set.m"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["const.m"]
    }
}

/// 常量获取语句处理器
pub struct ConstHandler;

// 静态实例
pub static CONST_HANDLER: ConstHandler = ConstHandler;

impl StatementHandler for ConstHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        // 获取常量名
        let const_name = match value {
            Value::String(name) => name,
            _ => return Err(NjilError::ExecutionError("常量名必须是字符串".to_string())),
        };
        
        debug_println!("获取常量 {}", const_name);
        
        // 检查是否为嵌套路径
        if const_name.contains('.') || const_name.contains('[') {
            // 处理嵌套路径的常量访问
            // 1. 先解析路径
            let path_parts = match path::parse_path(const_name) {
                Ok(parts) => parts,
                Err(e) => return Err(NjilError::ExecutionError(format!("无效的常量路径: {}", e))),
            };
            
            // 2. 获取基础变量名
            let base_const_name = match &path_parts[0] {
                path::PathPart::ObjectProperty(name) => name.clone(),
                _ => return Err(NjilError::ExecutionError("常量名不能以数组索引开始".to_string())),
            };
            
            // 3. 检查基础常量是否存在
            if !interpreter.constants.contains_key(&base_const_name) {
                return Err(NjilError::ExecutionError(format!("未定义的常量: {}", base_const_name)));
            }
            
            // 4. 获取基础常量值
            let base_value = interpreter.constants.get(&base_const_name).unwrap();
            
            // 5. 使用剩余路径部分获取嵌套值
            let remaining_path = &path_parts[1..];
            match path::get_nested_value(base_value, remaining_path) {
                Ok(value) => Ok(value.clone()),
                Err(_) => Err(NjilError::ExecutionError(format!("在常量'{}'中找不到路径'{}'", base_const_name, const_name))),
            }
        } else {
            // 普通常量获取
            match interpreter.constants.get(const_name) {
                Some(value) => Ok(value.clone()),
                None => Err(NjilError::ExecutionError(format!("未定义的常量: {}", const_name))),
            }
        }
    }
    
    fn name(&self) -> &'static str {
        "const"
    }
}

/// 常量检查语句处理器
pub struct ConstHasHandler;

// 静态实例
pub static CONST_HAS_HANDLER: ConstHasHandler = ConstHasHandler;

impl StatementHandler for ConstHasHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        // 先计算参数值
        let const_name_value = interpreter.evaluate_value(value)?;
        
        // 获取常量名
        let const_name = match const_name_value {
            Value::String(name) => name,
            _ => return Err(NjilError::ExecutionError("常量名必须是字符串".to_string())),
        };
        
        debug_println!("检查常量是否存在: {}", const_name);
        
        // 检查是否为嵌套路径
        if const_name.contains('.') || const_name.contains('[') {
            // 处理嵌套路径的常量访问
            // 1. 先解析路径
            let path_parts = match path::parse_path(&const_name) {
                Ok(parts) => parts,
                Err(_) => return Ok(Value::Bool(false)),
            };
            
            // 2. 获取基础变量名
            let base_const_name = match &path_parts[0] {
                path::PathPart::ObjectProperty(name) => name.clone(),
                _ => return Ok(Value::Bool(false)),
            };
            
            // 3. 检查基础常量是否存在
            if !interpreter.constants.contains_key(&base_const_name) {
                return Ok(Value::Bool(false));
            }
            
            // 4. 获取基础常量值
            let base_value = interpreter.constants.get(&base_const_name).unwrap();
            
            // 5. 使用剩余路径部分获取嵌套值
            let remaining_path = &path_parts[1..];
            match path::get_nested_value(base_value, remaining_path) {
                Ok(_) => Ok(Value::Bool(true)),
                Err(_) => Ok(Value::Bool(false)),
            }
        } else {
            // 普通常量检查
            Ok(Value::Bool(interpreter.has_constant(&const_name)))
        }
    }
    
    fn name(&self) -> &'static str {
        "has_constant"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["const.has", "const.exists"]
    }
} 