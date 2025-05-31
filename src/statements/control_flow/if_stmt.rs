use crate::error::NjilError;
use serde_json::Value;
use crate::interpreter::Interpreter;
use crate::errortip;
use crate::statements::StatementHandler;
use crate::statements::handle_statement;

/// 条件判断语句处理器
pub struct IfHandler;

// 静态实例
pub static IF_HANDLER: IfHandler = IfHandler;

impl StatementHandler for IfHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Object(if_obj) = value {
            // 检查必要的字段
            if !if_obj.contains_key("condition") || !if_obj.contains_key("then") {
                return Err(NjilError::ExecutionError(
                    errortip::control_flow::if_missing_fields().to_string()
                ));
            }

            // 获取条件
            let condition = if_obj.get("condition").unwrap();
            
            // 修改：安全地评估条件，如果是变量不存在的错误，则视为false
            let condition_result = match interpreter.evaluate_value(condition) {
                Ok(value) => value,
                Err(NjilError::ExecutionError(msg)) => {
                    // 检查是否是变量未定义错误
                    if msg.starts_with("未定义的变量:") {
                        Value::Null // 变量不存在时，视为null（假）
                    } else {
                        return Err(NjilError::ExecutionError(msg));
                    }
                },
                Err(e) => return Err(e),
            };
            
            // 检查条件是否为真
            let is_true = match condition_result {
                Value::Bool(b) => b,
                Value::Number(n) => n.as_f64().unwrap_or(0.0) != 0.0,
                Value::String(s) => !s.is_empty(),
                Value::Array(a) => !a.is_empty(),
                Value::Object(o) => !o.is_empty(),
                Value::Null => false,
            };
            
            // 根据条件执行相应的分支
            if is_true {
                let then_branch = if_obj.get("then").unwrap();
                
                // 处理then分支（可以是单个语句或语句数组）
                match then_branch {
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
                    _ => handle_statement(interpreter, then_branch),
                }
            } else if let Some(else_branch) = if_obj.get("else") {
                // 处理else分支（可以是单个语句或语句数组）
                match else_branch {
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
                    _ => handle_statement(interpreter, else_branch),
                }
            } else {
                // 没有else分支，返回null
                Ok(Value::Null)
            }
        } else {
            Err(NjilError::ExecutionError(errortip::control_flow::if_requires_object().to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "if"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["if.else"]
    }
} 