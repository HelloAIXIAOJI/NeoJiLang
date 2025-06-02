use crate::error::NjilError;
use crate::interpreter::Interpreter;
use crate::errortip;
use crate::statements::StatementHandler;
use crate::statements::handle_statement;
use serde_json::Value;

/// Try/Catch语句处理器，用于捕获异常
pub struct TryCatchHandler;

// 静态实例
pub static TRY_CATCH_HANDLER: TryCatchHandler = TryCatchHandler;

impl StatementHandler for TryCatchHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Object(try_catch_obj) = value {
            // 检查必要的字段
            if !try_catch_obj.contains_key("try") {
                return Err(NjilError::ExecutionError(
                    errortip::exception::try_catch_missing_fields().to_string()
                ));
            }

            // 获取try块
            let try_block = try_catch_obj.get("try").unwrap();
            
            // 执行try块（可以是单个语句或语句数组）
            let try_result = match try_block {
                Value::Array(statements) => {
                    let mut last_result = Value::Null;
                    for stmt in statements {
                        match handle_statement(interpreter, stmt) {
                            Ok(result) => {
                                // 检查是否有返回值或中断
                                if interpreter.is_returning() {
                                    return Ok(Value::Null);
                                }
                                last_result = result;
                            },
                            Err(e) => {
                                // 如果是异常，检查是否有catch块
                                if let Some(catch_block) = try_catch_obj.get("catch") {
                                    return handle_catch_block(interpreter, catch_block, e);
                                } else {
                                    // 没有catch块，异常继续向上抛出
                                    return Err(e);
                                }
                            }
                        }
                    }
                    Ok(last_result)
                },
                _ => {
                    match handle_statement(interpreter, try_block) {
                        Ok(result) => Ok(result),
                        Err(e) => {
                            // 如果是异常，检查是否有catch块
                            if let Some(catch_block) = try_catch_obj.get("catch") {
                                handle_catch_block(interpreter, catch_block, e)
                            } else {
                                // 没有catch块，异常继续向上抛出
                                Err(e)
                            }
                        }
                    }
                }
            };
            
            try_result
        } else {
            Err(NjilError::ExecutionError(errortip::exception::try_catch_requires_object().to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "try"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["try.catch"]
    }
}

/// 处理catch块
fn handle_catch_block(interpreter: &mut Interpreter, catch_block: &Value, error: NjilError) -> Result<Value, NjilError> {
    // 检查是否是用户抛出的异常
    let error_value = match error {
        NjilError::ThrowException(value) => value,
        // 对于其他类型的错误，转换为字符串异常值
        other => Value::String(other.to_string()),
    };
    
    // 检查catch块类型
    match catch_block {
        Value::Object(catch_obj) => {
            // 检查是否有var字段，用于存储异常值
            if let Some(var_value) = catch_obj.get("var") {
                if let Value::String(var_name) = var_value {
                    // 将异常值设置到变量中 - 需要克隆值
                    interpreter.set_variable(var_name.clone(), error_value.clone());
                } else {
                    return Err(NjilError::ExecutionError(errortip::exception::catch_var_requires_string().to_string()));
                }
            }
            
            // 执行catch块的body
            if let Some(body) = catch_obj.get("body") {
                // 执行catch块的主体
                match body {
                    Value::Array(statements) => {
                        let mut last_result = Value::Null;
                        for stmt in statements {
                            // 检查是否有返回值或中断
                            if interpreter.is_returning() {
                                return Ok(Value::Null);
                            }
                            
                            last_result = handle_statement(interpreter, stmt)?;
                        }
                        Ok(last_result)
                    },
                    _ => handle_statement(interpreter, body)
                }
            } else {
                // 如果没有body，返回异常值本身
                Ok(error_value)
            }
        },
        Value::Array(statements) => {
            // catch块是语句数组
            let mut last_result = Value::Null;
            for stmt in statements {
                // 检查是否有返回值或中断
                if interpreter.is_returning() {
                    return Ok(Value::Null);
                }
                
                last_result = handle_statement(interpreter, stmt)?;
            }
            Ok(last_result)
        },
        _ => {
            // catch块是单条语句
            handle_statement(interpreter, catch_block)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_try_catch_handler() {
        let mut interpreter = Interpreter::new();
        
        // 测试正常执行无异常的情况
        let try_catch_value = json!({
            "try": [
                {"var.set": {"name": "result", "value": "success"}}
            ]
        });
        
        let result = TRY_CATCH_HANDLER.handle(&mut interpreter, &try_catch_value);
        assert!(result.is_ok());
        assert_eq!(interpreter.get_variable("result").unwrap(), &json!("success"));
        
        // 测试捕获异常的情况
        let try_catch_value = json!({
            "try": [
                {"throw": "测试异常"}
            ],
            "catch": {
                "var": "error",
                "body": [
                    {"var.set": {"name": "result", "value": "caught"}}
                ]
            }
        });
        
        let result = TRY_CATCH_HANDLER.handle(&mut interpreter, &try_catch_value);
        assert!(result.is_ok());
        assert_eq!(interpreter.get_variable("error").unwrap(), &json!("测试异常"));
        assert_eq!(interpreter.get_variable("result").unwrap(), &json!("caught"));
    }
} 