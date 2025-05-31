use crate::error::NjilError;
use serde_json::Value;
use crate::interpreter::Interpreter;
use crate::debug_println;
use super::StatementHandler;

/// 函数调用处理器
pub struct FunctionCallHandler;

// 静态实例
pub static FUNCTION_CALL_HANDLER: FunctionCallHandler = FunctionCallHandler;

impl StatementHandler for FunctionCallHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        debug_println!("函数调用处理器被调用，参数: {}", serde_json::to_string_pretty(value).unwrap());
        
        match value {
            Value::String(function_name) => {
                debug_println!("简单函数调用: {}", function_name);
                // 简单形式：直接传递函数名，无参数
                let result = interpreter.call_function(function_name, &[]);
                debug_println!("函数 {} 调用结果: {:?}", function_name, result);
                result
            },
            Value::Object(obj) => {
                // 高级形式：提供函数名和参数
                // 检查必要的字段
                if !obj.contains_key("name") {
                    return Err(NjilError::ExecutionError("函数调用需要name字段".to_string()));
                }
                
                // 获取函数名
                let function_name = match obj.get("name") {
                    Some(Value::String(name)) => name,
                    _ => return Err(NjilError::ExecutionError("函数名必须是字符串".to_string())),
                };
                
                debug_println!("高级函数调用: {}", function_name);
                
                // 获取参数（如果有）
                let args = match obj.get("args") {
                    Some(Value::Array(args)) => {
                        // 对每个参数进行评估，这样嵌套的函数调用也能工作
                        let mut evaluated_args = Vec::new();
                        for (i, arg) in args.iter().enumerate() {
                            debug_println!("  评估参数 #{}: {}", i, serde_json::to_string_pretty(arg).unwrap());
                            let evaluated_arg = interpreter.evaluate_value(arg)?;
                            debug_println!("  参数 #{} 评估结果: {}", i, serde_json::to_string_pretty(&evaluated_arg).unwrap());
                            evaluated_args.push(evaluated_arg);
                        }
                        evaluated_args
                    },
                    Some(_) => return Err(NjilError::ExecutionError("函数参数必须是数组".to_string())),
                    None => Vec::new(),
                };
                
                // 调用函数
                debug_println!("调用函数 {} 参数: {}", function_name, serde_json::to_string_pretty(&args).unwrap());
                let result = interpreter.call_function(function_name, &args);
                debug_println!("函数 {} 调用结果: {:?}", function_name, result);
                result
            },
            _ => Err(NjilError::ExecutionError("函数调用需要字符串或对象参数".to_string())),
        }
    }
    
    fn name(&self) -> &'static str {
        "function.call"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["function.call", "call", "func.call"]
    }
} 