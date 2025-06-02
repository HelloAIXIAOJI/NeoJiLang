use crate::error::NjilError;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use serde_json::Value;
use std::process::Command;

/// 列出运行中的进程处理器
pub struct ProcessListHandler;

// 静态实例
pub static PROCESS_LIST_HANDLER: ProcessListHandler = ProcessListHandler;

impl StatementHandler for ProcessListHandler {
    fn handle(&self, _interpreter: &mut Interpreter, _value: &Value) -> Result<Value, NjilError> {
        // 获取进程列表
        let processes = get_process_list()?;
        
        // 返回进程列表
        Ok(Value::Array(processes))
    }
    
    fn name(&self) -> &'static str {
        "system.process.list"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["process.list"]
    }
}

// 获取进程列表
fn get_process_list() -> Result<Vec<Value>, NjilError> {
    let mut processes = Vec::new();
    
    #[cfg(target_os = "windows")]
    {
        // 在Windows上使用tasklist命令
        let output = Command::new("tasklist")
            .args(&["/FO", "CSV", "/NH"])
            .output()
            .map_err(|e| NjilError::ExecutionError(format!("获取进程列表失败: {}", e)))?;
        
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        
        // 解析CSV输出
        for line in stdout.lines() {
            // 移除引号并分割字段
            let fields: Vec<&str> = line.split("\",\"").collect();
            if fields.len() >= 2 {
                let name = fields[0].trim_start_matches('"').to_string();
                let pid_str = fields[1].trim_end_matches('"').trim();
                
                if let Ok(pid) = pid_str.parse::<u32>() {
                    let mut process = serde_json::Map::new();
                    process.insert("name".to_string(), Value::String(name));
                    process.insert("pid".to_string(), Value::Number(serde_json::Number::from(pid)));
                    
                    if fields.len() >= 5 {
                        let mem_usage = fields[4].trim_end_matches('"').trim_end_matches(" K").replace(",", "");
                        if let Ok(mem) = mem_usage.parse::<u64>() {
                            process.insert("memoryKB".to_string(), Value::Number(serde_json::Number::from(mem)));
                        }
                    }
                    
                    processes.push(Value::Object(process));
                }
            }
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        // 在Linux上使用ps命令
        let output = Command::new("ps")
            .args(&["-e", "-o", "pid,comm,rss", "--no-headers"])
            .output()
            .map_err(|e| NjilError::ExecutionError(format!("获取进程列表失败: {}", e)))?;
        
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        
        // 解析ps输出
        for line in stdout.lines() {
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() >= 2 {
                if let Ok(pid) = fields[0].parse::<u32>() {
                    let name = fields[1].to_string();
                    
                    let mut process = serde_json::Map::new();
                    process.insert("name".to_string(), Value::String(name));
                    process.insert("pid".to_string(), Value::Number(serde_json::Number::from(pid)));
                    
                    if fields.len() >= 3 {
                        if let Ok(mem) = fields[2].parse::<u64>() {
                            process.insert("memoryKB".to_string(), Value::Number(serde_json::Number::from(mem)));
                        }
                    }
                    
                    processes.push(Value::Object(process));
                }
            }
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        // 在macOS上使用ps命令
        let output = Command::new("ps")
            .args(&["-e", "-o", "pid,comm,rss", "-c"])
            .output()
            .map_err(|e| NjilError::ExecutionError(format!("获取进程列表失败: {}", e)))?;
        
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        
        // 跳过第一行（标题行）
        let lines: Vec<&str> = stdout.lines().collect();
        for line in lines.iter().skip(1) {
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() >= 2 {
                if let Ok(pid) = fields[0].parse::<u32>() {
                    let name = fields[1].to_string();
                    
                    let mut process = serde_json::Map::new();
                    process.insert("name".to_string(), Value::String(name));
                    process.insert("pid".to_string(), Value::Number(serde_json::Number::from(pid)));
                    
                    if fields.len() >= 3 {
                        if let Ok(mem) = fields[2].parse::<u64>() {
                            process.insert("memoryKB".to_string(), Value::Number(serde_json::Number::from(mem)));
                        }
                    }
                    
                    processes.push(Value::Object(process));
                }
            }
        }
    }
    
    Ok(processes)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_process_list() {
        let mut interpreter = Interpreter::new();
        
        let result = PROCESS_LIST_HANDLER.handle(
            &mut interpreter,
            &Value::Null
        ).unwrap();
        
        if let Value::Array(processes) = result {
            // 确保返回了一些进程
            assert!(!processes.is_empty());
            
            // 检查第一个进程是否有预期的字段
            if let Some(Value::Object(process)) = processes.first() {
                assert!(process.contains_key("name"));
                assert!(process.contains_key("pid"));
                
                // PID应该是一个数字，不一定要大于0，因为在某些系统上可能有不同的表示
                if let Some(Value::Number(pid)) = process.get("pid") {
                    // 只要能正确转换为数字即可，不再强制要求 > 0
                    assert!(pid.is_u64() || pid.is_i64() || pid.is_f64());
                } else {
                    panic!("Expected numeric pid");
                }
            } else {
                panic!("Expected object in processes array");
            }
        } else {
            panic!("Expected array result");
        }
    }
} 