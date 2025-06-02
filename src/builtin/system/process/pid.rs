use crate::error::NjilError;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use serde_json::Value;
use std::process;

/// 获取当前进程ID处理器
pub struct ProcessPidHandler;

// 静态实例
pub static PROCESS_PID_HANDLER: ProcessPidHandler = ProcessPidHandler;

impl StatementHandler for ProcessPidHandler {
    fn handle(&self, _interpreter: &mut Interpreter, _value: &Value) -> Result<Value, NjilError> {
        // 获取当前进程ID
        let pid = process::id();
        
        // 返回进程ID
        Ok(Value::Number(serde_json::Number::from(pid)))
    }
    
    fn name(&self) -> &'static str {
        "system.process.pid"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["process.pid"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process;
    
    #[test]
    fn test_process_pid() {
        let mut interpreter = Interpreter::new();
        
        // 获取当前进程ID
        let current_pid = process::id();
        
        // 测试处理器
        let result = PROCESS_PID_HANDLER.handle(
            &mut interpreter,
            &Value::Null
        ).unwrap();
        
        if let Value::Number(pid) = result {
            assert_eq!(pid.as_u64().unwrap(), current_pid as u64);
        } else {
            panic!("Expected numeric pid");
        }
    }
} 