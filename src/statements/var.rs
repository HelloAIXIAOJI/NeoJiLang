use crate::error::NjilError;
use serde_json::Value;
use crate::interpreter::Interpreter;
use crate::errortip;
use super::StatementHandler;
use crate::utils::path::{self, PathPart};

/// 变量设置语句处理器
pub struct VarSetHandler;

// 静态实例
pub static VAR_SET_HANDLER: VarSetHandler = VarSetHandler;

impl StatementHandler for VarSetHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Object(var_map) = value {
            for (var_path, var_value) in var_map {
                // 先评估值
                let evaluated_value = match var_value {
                    Value::Object(obj) => {
                        // 如果是嵌套对象，需要递归处理
                        let mut result_obj = serde_json::Map::new();
                        
                        for (key, val) in obj {
                            // 递归评估值
                            let eval_val = interpreter.evaluate_value(val)?;
                            result_obj.insert(key.clone(), eval_val);
                        }
                        
                        Value::Object(result_obj)
                    },
                    _ => interpreter.evaluate_value(var_value)?
                };
                
                // 检查是否为嵌套路径
                if var_path.contains('.') || var_path.contains('[') {
                    path::set_nested_value(&mut interpreter.variables, var_path, evaluated_value)?;
                } else {
                    // 普通变量设置
                    interpreter.variables.insert(var_path.clone(), evaluated_value);
                }
            }
            Ok(Value::Null)
        } else {
            Err(NjilError::ExecutionError(errortip::var::var_set_requires_object().to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "var.set"
    }
}

/// 变量获取语句处理器
pub struct VarHandler;

// 静态实例
pub static VAR_HANDLER: VarHandler = VarHandler;

impl StatementHandler for VarHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::String(var_path) = value {
            // 解析变量路径
            if var_path.contains('.') || var_path.contains('[') {
                return get_nested_variable(interpreter, var_path);
            } else {
                // 普通变量访问
                if let Some(var_value) = interpreter.variables.get(var_path) {
                    return Ok(var_value.clone());
                } else {
                    return Err(NjilError::ExecutionError(errortip::var::undefined_variable(var_path)));
                }
            }
        } else {
            Err(NjilError::ExecutionError(errortip::var::var_requires_string().to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "var"
    }
}

/// 获取嵌套变量
fn get_nested_variable(interpreter: &Interpreter, var_path: &str) -> Result<Value, NjilError> {
    // 解析变量路径
    let path_parts = path::parse_path(var_path)?;
    
    // 获取基础变量名
    let base_var_name = match &path_parts[0] {
        PathPart::ObjectProperty(name) => name,
        _ => return Err(NjilError::ExecutionError("变量名不能是数组索引".to_string())),
    };
    
    // 获取基础变量
    if let Some(base_var) = interpreter.variables.get(base_var_name) {
        // 如果路径只有一个部分，直接返回基础变量
        if path_parts.len() == 1 {
            return Ok(base_var.clone());
        }
        
        // 获取嵌套值
        match path::get_nested_value(base_var, &path_parts[1..]) {
            Ok(nested_value) => return Ok(nested_value.clone()),
            Err(e) => return Err(e),
        }
    } else {
        return Err(NjilError::ExecutionError(errortip::var::undefined_variable(base_var_name)));
    }
} 