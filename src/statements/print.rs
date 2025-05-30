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
            println!("{}", interpreter.value_to_string(&text));
        } else {
            // 处理普通值
            let text = interpreter.evaluate_value(value)?;
            println!("{}", interpreter.value_to_string(&text));
        }
        
        Ok(Value::Null)
    }
    
    fn name(&self) -> &'static str {
        "print"
    }
} 