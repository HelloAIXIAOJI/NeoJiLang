use crate::error::NjilError;
use serde_json::Value;
use crate::interpreter::Interpreter;
use crate::errortip;
use crate::statements::StatementHandler;
use crate::statements::handle_statement;

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