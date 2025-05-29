use crate::error::NjilError;
use serde_json::Value;
use crate::interpreter::Interpreter;
use crate::errortip;
use super::StatementHandler;

/// 变量设置语句处理器
pub struct VarSetHandler;

// 静态实例
pub static VAR_SET_HANDLER: VarSetHandler = VarSetHandler;

impl StatementHandler for VarSetHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Object(var_map) = value {
            for (var_name, var_value) in var_map {
                let evaluated_value = interpreter.evaluate_value(var_value)?;
                interpreter.variables.insert(var_name.clone(), evaluated_value);
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
            // 检查是否包含点号，表示访问对象属性
            if var_path.contains('.') {
                let parts: Vec<&str> = var_path.split('.').collect();
                let base_var_name = parts[0];
                
                // 获取基础变量
                if let Some(base_var) = interpreter.variables.get(base_var_name) {
                    // 逐层访问属性
                    let mut current_value = base_var;
                    for &prop in parts.iter().skip(1) {
                        match current_value {
                            Value::Object(obj) => {
                                if let Some(prop_value) = obj.get(prop) {
                                    current_value = prop_value;
                                } else {
                                    return Err(NjilError::ExecutionError(
                                        format!("对象 {} 没有属性: {}", base_var_name, prop)
                                    ));
                                }
                            },
                            _ => {
                                return Err(NjilError::ExecutionError(
                                    format!("变量 {} 不是对象，无法访问属性", base_var_name)
                                ));
                            }
                        }
                    }
                    return Ok(current_value.clone());
                } else {
                    return Err(NjilError::ExecutionError(errortip::var::undefined_variable(base_var_name)));
                }
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