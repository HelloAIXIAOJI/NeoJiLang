use crate::error::NjilError;
use serde_json::Value;
use crate::interpreter::Interpreter;
use super::StatementHandler;

/// 打印语句处理器
pub struct PrintHandler;

// 静态实例
pub static PRINT_HANDLER: PrintHandler = PrintHandler;

impl StatementHandler for PrintHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        let text = interpreter.evaluate_value(value)?;
        println!("{}", interpreter.value_to_string(&text));
        Ok(Value::Null)
    }
    
    fn name(&self) -> &'static str {
        "print"
    }
} 