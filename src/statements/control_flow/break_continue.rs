use crate::error::NjilError;
use serde_json::Value;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;

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