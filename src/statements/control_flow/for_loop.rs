use crate::error::NjilError;
use serde_json::Value;
use crate::interpreter::Interpreter;
use crate::errortip;
use crate::statements::StatementHandler;
use crate::statements::handle_statement;

/// for循环语句处理器
pub struct ForLoopHandler;

// 静态实例
pub static FOR_LOOP_HANDLER: ForLoopHandler = ForLoopHandler;

impl StatementHandler for ForLoopHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Object(for_obj) = value {
            // 检查必要的字段
            if !for_obj.contains_key("count") || !for_obj.contains_key("body") {
                return Err(NjilError::ExecutionError(
                    errortip::control_flow::for_missing_fields().to_string()
                ));
            }

            // 获取循环次数
            let count_value = interpreter.evaluate_value(for_obj.get("count").unwrap())?;
            let count = match count_value {
                Value::Number(n) => n.as_u64().unwrap_or(0) as usize,
                _ => return Err(NjilError::ExecutionError(errortip::control_flow::count_requires_number().to_string())),
            };
            
            // 获取循环变量名（可选）
            let var_name = match for_obj.get("var") {
                Some(Value::String(name)) => Some(name.clone()),
                None => None,
                _ => return Err(NjilError::ExecutionError(errortip::control_flow::var_name_requires_string().to_string())),
            };
            
            let body = for_obj.get("body").unwrap();
            
            // 执行循环
            let mut last_result = Value::Null;
            for i in 0..count {
                // 如果指定了循环变量，设置它
                if let Some(ref name) = var_name {
                    interpreter.variables.insert(name.clone(), Value::Number(i.into()));
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
            Err(NjilError::ExecutionError(errortip::control_flow::for_requires_object().to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "loop.for"
    }
} 