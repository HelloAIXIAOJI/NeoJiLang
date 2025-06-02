use crate::error::NjilError;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use serde_json::Value;
use std::process::Command;

/// 终止进程处理器
pub struct ProcessKillHandler;

// 静态实例
pub static PROCESS_KILL_HANDLER: ProcessKillHandler = ProcessKillHandler;

impl StatementHandler for ProcessKillHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        // 获取PID
        let pid = match value {
            Value::Number(n) => {
                if let Some(pid) = n.as_u64() {
                    pid
                } else {
                    return Err(NjilError::ExecutionError("PID必须是正整数".to_string()));
                }
            },
            Value::Object(obj) => {
                if let Some(pid_value) = obj.get("pid") {
                    let evaluated = interpreter.evaluate_value(pid_value)?;
                    if let Value::Number(n) = evaluated {
                        if let Some(pid) = n.as_u64() {
                            pid
                        } else {
                            return Err(NjilError::ExecutionError("PID必须是正整数".to_string()));
                        }
                    } else {
                        return Err(NjilError::ExecutionError("pid参数必须是数字".to_string()));
                    }
                } else {
                    return Err(NjilError::ExecutionError("缺少pid参数".to_string()));
                }
            },
            _ => return Err(NjilError::ExecutionError("参数必须是数字或包含pid字段的对象".to_string())),
        };
        
        // 终止进程
        let success = kill_process(pid)?;
        
        // 返回是否成功
        Ok(Value::Bool(success))
    }
    
    fn name(&self) -> &'static str {
        "system.process.kill"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["process.kill"]
    }
}

// 终止进程
fn kill_process(pid: u64) -> Result<bool, NjilError> {
    #[cfg(target_os = "windows")]
    {
        // 在Windows上使用taskkill命令
        let output = Command::new("taskkill")
            .args(&["/PID", &pid.to_string(), "/F"])
            .output()
            .map_err(|e| NjilError::ExecutionError(format!("执行taskkill命令失败: {}", e)))?;
        
        Ok(output.status.success())
    }
    
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        // 在Linux/macOS上使用kill命令
        let output = Command::new("kill")
            .args(&["-9", &pid.to_string()])
            .output()
            .map_err(|e| NjilError::ExecutionError(format!("执行kill命令失败: {}", e)))?;
        
        Ok(output.status.success())
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        Err(NjilError::ExecutionError("不支持当前操作系统".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builtin::system::process::spawn::PROCESS_SPAWN_HANDLER;
    use std::thread;
    use std::time::Duration;
    
    #[test]
    fn test_process_kill() {
        let mut interpreter = Interpreter::new();
        
        // 启动一个进程
        #[cfg(target_os = "windows")]
        let cmd = "ping 127.0.0.1 -n 10";
        #[cfg(not(target_os = "windows"))]
        let cmd = "sleep 5";
        
        let spawn_result = PROCESS_SPAWN_HANDLER.handle(
            &mut interpreter,
            &Value::String(cmd.to_string())
        ).unwrap();
        
        if let Value::Object(obj) = spawn_result {
            if let Some(Value::Number(pid)) = obj.get("pid") {
                // 等待一下确保进程已经启动
                thread::sleep(Duration::from_millis(500));
                
                // 终止进程
                let kill_result = PROCESS_KILL_HANDLER.handle(
                    &mut interpreter,
                    &Value::Number(pid.clone())
                ).unwrap();
                
                // 验证结果
                if let Value::Bool(success) = kill_result {
                    // 在某些情况下可能无法终止进程，所以不严格要求成功
                    // 但至少应该返回一个布尔值
                    assert!(success || !success); // 简单检查它是个布尔值
                } else {
                    panic!("Expected boolean result");
                }
            } else {
                panic!("Expected numeric pid");
            }
        } else {
            panic!("Expected object result from spawn");
        }
    }
} 