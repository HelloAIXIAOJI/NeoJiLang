use crate::error::NjilError;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use serde_json::Value;
use std::process::Command;
use std::collections::HashMap;

/// 执行命令并等待完成处理器
pub struct ProcessExecHandler;

// 静态实例
pub static PROCESS_EXEC_HANDLER: ProcessExecHandler = ProcessExecHandler;

impl StatementHandler for ProcessExecHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        let (command, args, env, shell) = parse_command_args(interpreter, value)?;
        
        // 创建命令
        let output = if shell {
            // 使用系统shell执行命令
            #[cfg(target_os = "windows")]
            let mut cmd = Command::new("cmd");
            #[cfg(target_os = "windows")]
            cmd.args(&["/C", &command]);
            
            #[cfg(not(target_os = "windows"))]
            let mut cmd = Command::new("sh");
            #[cfg(not(target_os = "windows"))]
            cmd.args(&["-c", &command]);
            
            // 添加参数和环境变量
            if !args.is_empty() {
                cmd.args(args);
            }
            if !env.is_empty() {
                cmd.envs(env);
            }
            
            // 执行命令并获取输出
            cmd.output()
                .map_err(|e| NjilError::ExecutionError(format!("执行命令失败: {}", e)))?
        } else {
            // 不使用shell，直接执行命令
            let mut cmd = Command::new(&command);
            
            // 添加参数和环境变量
            if !args.is_empty() {
                cmd.args(args);
            }
            if !env.is_empty() {
                cmd.envs(env);
            }
            
            // 执行命令并获取输出
            cmd.output()
                .map_err(|e| NjilError::ExecutionError(format!("执行命令失败: {}", e)))?
        };
        
        // 创建结果对象
        let mut result = serde_json::Map::new();
        
        // 添加状态码
        if let Some(code) = output.status.code() {
            result.insert("exitCode".to_string(), Value::Number(serde_json::Number::from(code)));
        } else {
            result.insert("exitCode".to_string(), Value::Null);
        }
        
        // 添加成功标志
        result.insert("success".to_string(), Value::Bool(output.status.success()));
        
        // 添加标准输出
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        result.insert("stdout".to_string(), Value::String(stdout));
        
        // 添加标准错误
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        result.insert("stderr".to_string(), Value::String(stderr));
        
        Ok(Value::Object(result))
    }
    
    fn name(&self) -> &'static str {
        "system.process.exec"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["process.exec"]
    }
}

// 辅助函数：解析命令和参数
fn parse_command_args(interpreter: &mut Interpreter, value: &Value) -> Result<(String, Vec<String>, HashMap<String, String>, bool), NjilError> {
    match value {
        Value::String(cmd) => {
            // 如果只提供了字符串，那就直接作为命令执行
            Ok((cmd.clone(), Vec::new(), HashMap::new(), true))
        },
        Value::Object(obj) => {
            // 获取命令
            let command = if let Some(cmd_value) = obj.get("command") {
                let evaluated = interpreter.evaluate_value(cmd_value)?;
                if let Value::String(cmd) = evaluated {
                    cmd
                } else {
                    return Err(NjilError::ExecutionError("command参数必须是字符串".to_string()));
                }
            } else {
                return Err(NjilError::ExecutionError("缺少command参数".to_string()));
            };
            
            // 获取参数
            let args = if let Some(args_value) = obj.get("args") {
                let evaluated = interpreter.evaluate_value(args_value)?;
                if let Value::Array(arr) = evaluated {
                    let mut args_vec = Vec::new();
                    for arg in arr {
                        if let Value::String(s) = arg {
                            args_vec.push(s);
                        } else {
                            return Err(NjilError::ExecutionError("args数组的所有元素必须是字符串".to_string()));
                        }
                    }
                    args_vec
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            };
            
            // 获取环境变量
            let env = if let Some(env_value) = obj.get("env") {
                let evaluated = interpreter.evaluate_value(env_value)?;
                if let Value::Object(env_obj) = evaluated {
                    let mut env_map = HashMap::new();
                    for (key, val) in env_obj {
                        if let Value::String(s) = val {
                            env_map.insert(key.clone(), s);
                        } else {
                            return Err(NjilError::ExecutionError("env对象的所有值必须是字符串".to_string()));
                        }
                    }
                    env_map
                } else {
                    HashMap::new()
                }
            } else {
                HashMap::new()
            };
            
            // 获取是否使用shell
            let use_shell = if let Some(Value::Bool(shell)) = obj.get("shell") {
                *shell
            } else {
                true // 默认使用shell
            };
            
            Ok((command, args, env, use_shell))
        },
        _ => Err(NjilError::ExecutionError("参数必须是字符串或对象".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_process_exec() {
        let mut interpreter = Interpreter::new();
        
        // 测试简单命令执行
        #[cfg(target_os = "windows")]
        let cmd = "echo Hello, World!";
        #[cfg(not(target_os = "windows"))]
        let cmd = "echo 'Hello, World!'";
        
        let result = PROCESS_EXEC_HANDLER.handle(
            &mut interpreter,
            &Value::String(cmd.to_string())
        ).unwrap();
        
        if let Value::Object(obj) = result {
            assert_eq!(obj.get("success"), Some(&Value::Bool(true)));
            
            if let Some(Value::String(stdout)) = obj.get("stdout") {
                #[cfg(target_os = "windows")]
                assert_eq!(stdout.trim(), "Hello, World!");
                #[cfg(not(target_os = "windows"))]
                assert_eq!(stdout.trim(), "Hello, World!");
            } else {
                panic!("Expected string stdout");
            }
        } else {
            panic!("Expected object result");
        }
        
        // 测试带参数的命令
        let mut obj = serde_json::Map::new();
        #[cfg(target_os = "windows")]
        obj.insert("command".to_string(), Value::String("echo".to_string()));
        #[cfg(not(target_os = "windows"))]
        obj.insert("command".to_string(), Value::String("echo".to_string()));
        
        let args = vec![Value::String("Hello".to_string()), Value::String("World".to_string())];
        obj.insert("args".to_string(), Value::Array(args));
        
        let result = PROCESS_EXEC_HANDLER.handle(
            &mut interpreter,
            &Value::Object(obj)
        ).unwrap();
        
        if let Value::Object(obj) = result {
            assert_eq!(obj.get("success"), Some(&Value::Bool(true)));
            
            if let Some(Value::String(stdout)) = obj.get("stdout") {
                assert!(stdout.contains("Hello") && stdout.contains("World"));
            } else {
                panic!("Expected string stdout");
            }
        } else {
            panic!("Expected object result");
        }
    }
} 