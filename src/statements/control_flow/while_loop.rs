use crate::error::NjilError;
use serde_json::Value;
use crate::interpreter::Interpreter;
use crate::errortip;
use crate::statements::StatementHandler;
use crate::statements::handle_statement;

/// while循环语句处理器
pub struct WhileLoopHandler;

// 静态实例
pub static WHILE_LOOP_HANDLER: WhileLoopHandler = WhileLoopHandler;

impl StatementHandler for WhileLoopHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Object(while_obj) = value {
            // 检查必要的字段
            if !while_obj.contains_key("condition") || !while_obj.contains_key("body") {
                return Err(NjilError::ExecutionError(
                    errortip::control_flow::while_missing_fields().to_string()
                ));
            }

            let condition = while_obj.get("condition").unwrap();
            let body = while_obj.get("body").unwrap();
            
            // 执行循环
            let mut last_result = Value::Null;
            loop {
                // 检查条件
                let condition_result = interpreter.evaluate_value(condition)?;
                let is_true = match condition_result {
                    Value::Bool(b) => b,
                    Value::Number(n) => n.as_f64().unwrap_or(0.0) != 0.0,
                    Value::String(s) => !s.is_empty(),
                    Value::Array(a) => !a.is_empty(),
                    Value::Object(o) => !o.is_empty(),
                    Value::Null => false,
                };
                
                if !is_true {
                    break;
                }
                
                // 执行循环体
                match body {
                    Value::Array(statements) => {
                        for stmt in statements {
                            // 检查是否有返回值
                            if interpreter.is_returning() {
                                return Ok(Value::Null);
                            }
                            
                            // 执行语句
                            match handle_statement(interpreter, stmt) {
                                Ok(result) => {
                                    last_result = result;
                                },
                                Err(NjilError::LoopBreak) => {
                                    return Ok(Value::Null);
                                },
                                Err(NjilError::LoopContinue) => {
                                    break;
                                },
                                Err(e) => return Err(e),
                            }
                        }
                    },
                    _ => {
                        // 执行单个语句
                        match handle_statement(interpreter, body) {
                            Ok(result) => {
                                last_result = result;
                            },
                            Err(NjilError::LoopBreak) => {
                                return Ok(Value::Null);
                            },
                            Err(NjilError::LoopContinue) => {
                                continue;
                            },
                            Err(e) => return Err(e),
                        }
                    },
                }
            }
            
            Ok(last_result)
        } else {
            Err(NjilError::ExecutionError(errortip::control_flow::while_requires_object().to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "loop.while"
    }
} 