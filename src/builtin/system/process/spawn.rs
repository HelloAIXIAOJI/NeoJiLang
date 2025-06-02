use crate::error::NjilError;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use serde_json::Value;
use std::process::{Command, Stdio};
use std::collections::HashMap;

/// 启动命令但不等待处理器
pub struct ProcessSpawnHandler;

// 静态实例
pub static PROCESS_SPAWN_HANDLER: ProcessSpawnHandler = ProcessSpawnHandler;

impl StatementHandler for ProcessSpawnHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        let (command, args, env, shell) = parse_command_args(interpreter, value)?;
        
        // 创建命令
        let child = if shell {
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
            
            // 设置标准输入/输出/错误为空
            cmd.stdin(Stdio::null())
               .stdout(Stdio::null())
               .stderr(Stdio::null());
            
            // 启动命令但不等待
            cmd.spawn()
                .map_err(|e| NjilError::ExecutionError(format!("启动命令失败: {}", e)))?
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
            
            // 设置标准输入/输出/错误为空
            cmd.stdin(Stdio::null())
               .stdout(Stdio::null())
               .stderr(Stdio::null());
            
            // 启动命令但不等待
            cmd.spawn()
                .map_err(|e| NjilError::ExecutionError(format!("启动命令失败: {}", e)))?
        };
        
        // 返回进程ID
        let pid = child.id();
        
        // 创建结果对象
        let mut result = serde_json::Map::new();
        result.insert("pid".to_string(), Value::Number(serde_json::Number::from(pid)));
        
        Ok(Value::Object(result))
    }
    
    fn name(&self) -> &'static str {
        "system.process.spawn"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["process.spawn"]
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
    fn test_process_spawn() {
        let mut interpreter = Interpreter::new();
        
        // 测试简单命令启动
        #[cfg(target_os = "windows")]
        let cmd = "ping 127.0.0.1 -n 1";
        #[cfg(not(target_os = "windows"))]
        let cmd = "sleep 0.1";
        
        let result = PROCESS_SPAWN_HANDLER.handle(
            &mut interpreter,
            &Value::String(cmd.to_string())
        ).unwrap();
        
        if let Value::Object(obj) = result {
            if let Some(Value::Number(pid)) = obj.get("pid") {
                assert!(pid.as_u64().unwrap() > 0);
            } else {
                panic!("Expected numeric pid");
            }
        } else {
            panic!("Expected object result");
        }
    }
} 