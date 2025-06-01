use crate::error::NjilError;
use serde_json::Value;
use crate::interpreter::Interpreter;
use super::StatementHandler;
use crate::statements::string::StringConcatHandler;

/// 打印语句处理器
pub struct PrintHandler;

// 静态实例
pub static PRINT_HANDLER: PrintHandler = PrintHandler;

impl StatementHandler for PrintHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        // 检查是否是数组，如果是，使用字符串连接处理器处理
        if let Value::Array(parts) = value {
            let text = StringConcatHandler::concat_strings(interpreter, parts)?;
            print!("{}", interpreter.value_to_string(&text));
        } else {
            // 处理普通值
            let text = interpreter.evaluate_value(value)?;
            print!("{}", interpreter.value_to_string(&text));
        }
        
        Ok(Value::Null)
    }
    
    fn name(&self) -> &'static str {
        "print"
    }
}

/// 打印换行语句处理器
pub struct PrintlnHandler;

// 静态实例
pub static PRINTLN_HANDLER: PrintlnHandler = PrintlnHandler;

impl StatementHandler for PrintlnHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        // 获取内容
        let content = if let Value::Object(obj) = value {
            if let Some(content_value) = obj.get("content") {
                content_value
            } else {
                return Err(NjilError::ExecutionError("println指令缺少content参数".to_string()));
            }
        } else {
            value
        };
        
        // 评估内容
        let text = interpreter.evaluate_value(content)?;
        println!("{}", interpreter.value_to_string(&text));
        
        Ok(Value::Null)
    }
    
    fn name(&self) -> &'static str {
        "println"
    }
} 