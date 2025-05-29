use crate::error::NjilError;
use serde_json::Value;
use crate::interpreter::Interpreter;
use crate::errortip;
use super::StatementHandler;
use super::handle_statement;

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

/// foreach循环语句处理器
pub struct ForeachLoopHandler;

// 静态实例
pub static FOREACH_LOOP_HANDLER: ForeachLoopHandler = ForeachLoopHandler;

impl StatementHandler for ForeachLoopHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Object(foreach_obj) = value {
            // 检查必要的字段
            if !foreach_obj.contains_key("collection") || !foreach_obj.contains_key("body") || !foreach_obj.contains_key("var") {
                return Err(NjilError::ExecutionError(
                    errortip::control_flow::foreach_missing_fields().to_string()
                ));
            }

            // 获取集合
            let collection_value = interpreter.evaluate_value(foreach_obj.get("collection").unwrap())?;
            
            // 获取变量名
            let var_name = match foreach_obj.get("var") {
                Some(Value::String(name)) => name.clone(),
                _ => return Err(NjilError::ExecutionError(errortip::control_flow::var_name_requires_string().to_string())),
            };
            
            // 获取索引变量名（可选）
            let index_var = match foreach_obj.get("index") {
                Some(Value::String(name)) => Some(name.clone()),
                None => None,
                _ => return Err(NjilError::ExecutionError(errortip::control_flow::var_name_requires_string().to_string())),
            };
            
            let body = foreach_obj.get("body").unwrap();
            
            // 执行循环
            let mut last_result = Value::Null;
            
            match collection_value {
                Value::Array(arr) => {
                    // 遍历数组
                    for (i, item) in arr.iter().enumerate() {
                        // 设置循环变量
                        interpreter.variables.insert(var_name.clone(), item.clone());
                        
                        // 设置索引变量（如果有）
                        if let Some(ref idx_var) = index_var {
                            interpreter.variables.insert(idx_var.clone(), Value::Number(i.into()));
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
                },
                Value::Object(obj) => {
                    // 遍历对象
                    for (i, (key, value)) in obj.iter().enumerate() {
                        // 创建包含键和值的对象
                        let mut entry = serde_json::Map::new();
                        entry.insert("key".to_string(), Value::String(key.clone()));
                        entry.insert("value".to_string(), value.clone());
                        
                        // 设置循环变量
                        interpreter.variables.insert(var_name.clone(), Value::Object(entry));
                        
                        // 设置索引变量（如果有）
                        if let Some(ref idx_var) = index_var {
                            interpreter.variables.insert(idx_var.clone(), Value::Number(i.into()));
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
                },
                _ => return Err(NjilError::ExecutionError(errortip::control_flow::collection_requires_array_or_object().to_string())),
            }
            
            Ok(last_result)
        } else {
            Err(NjilError::ExecutionError(errortip::control_flow::foreach_requires_object().to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "loop.foreach"
    }
}

/// break语句处理器
pub struct BreakHandler;

// 静态实例
pub static BREAK_HANDLER: BreakHandler = BreakHandler;

impl StatementHandler for BreakHandler {
    fn handle(&self, _interpreter: &mut Interpreter, _value: &Value) -> Result<Value, NjilError> {
        // 直接返回LoopBreak错误，由循环处理器捕获
        Err(NjilError::LoopBreak)
    }
    
    fn name(&self) -> &'static str {
        "loop.break"
    }
}

/// continue语句处理器
pub struct ContinueHandler;

// 静态实例
pub static CONTINUE_HANDLER: ContinueHandler = ContinueHandler;

impl StatementHandler for ContinueHandler {
    fn handle(&self, _interpreter: &mut Interpreter, _value: &Value) -> Result<Value, NjilError> {
        // 直接返回LoopContinue错误，由循环处理器捕获
        Err(NjilError::LoopContinue)
    }
    
    fn name(&self) -> &'static str {
        "loop.continue"
    }
} 