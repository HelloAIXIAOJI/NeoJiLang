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
        if let Value::String(var_name) = value {
            if let Some(var_value) = interpreter.variables.get(var_name) {
                Ok(var_value.clone())
            } else {
                Err(NjilError::ExecutionError(errortip::var::undefined_variable(var_name)))
            }
        } else {
            Err(NjilError::ExecutionError(errortip::var::var_requires_string().to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "var"
    }
} 