use crate::error::NjilError;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use serde_json::Value;
use std::fs;
use std::path::Path;
use super::copy_dir_recursive;

/// 复制文件或目录处理器
pub struct FsCopyHandler;

// 静态实例
pub static FS_COPY_HANDLER: FsCopyHandler = FsCopyHandler;

impl StatementHandler for FsCopyHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Object(obj) = value {
            // 获取源路径
            let from = if let Some(from_value) = obj.get("from") {
                let evaluated = interpreter.evaluate_value(from_value)?;
                if let Value::String(path) = evaluated {
                    path
                } else {
                    return Err(NjilError::ExecutionError("from参数必须是字符串".to_string()));
                }
            } else {
                return Err(NjilError::ExecutionError("缺少from参数".to_string()));
            };
            
            // 获取目标路径
            let to = if let Some(to_value) = obj.get("to") {
                let evaluated = interpreter.evaluate_value(to_value)?;
                if let Value::String(path) = evaluated {
                    path
                } else {
                    return Err(NjilError::ExecutionError("to参数必须是字符串".to_string()));
                }
            } else {
                return Err(NjilError::ExecutionError("缺少to参数".to_string()));
            };
            
            // 获取递归复制参数
            let recursive = if let Some(Value::Bool(rec)) = obj.get("recursive") {
                *rec
            } else {
                false // 默认不递归复制
            };
            
            // 复制文件或目录
            if Path::new(&from).is_dir() {
                if recursive {
                    // 递归复制目录
                    copy_dir_recursive(&from, &to)?;
                } else {
                    return Err(NjilError::ExecutionError("不能复制目录，除非设置recursive=true".to_string()));
                }
            } else {
                // 复制文件
                fs::copy(&from, &to)
                    .map_err(|e| NjilError::ExecutionError(format!("复制文件失败: {}", e)))?;
            }
            
            Ok(Value::Bool(true))
        } else {
            Err(NjilError::ExecutionError("system.fs.copy需要一个对象参数".to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "system.fs.copy"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["fs.copy"]
    }
} 