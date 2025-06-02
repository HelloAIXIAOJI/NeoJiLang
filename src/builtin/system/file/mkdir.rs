use crate::error::NjilError;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use serde_json::Value;
use std::fs;
use super::get_path_param;

/// 创建目录处理器
pub struct FsMkdirHandler;

// 静态实例
pub static FS_MKDIR_HANDLER: FsMkdirHandler = FsMkdirHandler;

impl StatementHandler for FsMkdirHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        // 获取路径参数
        let path = get_path_param(interpreter, value)?;
        
        // 获取递归创建参数
        let recursive = if let Value::Object(obj) = value {
            if let Some(Value::Bool(rec)) = obj.get("recursive") {
                *rec
            } else {
                false // 默认不递归创建
            }
        } else {
            false
        };
        
        // 创建目录
        let result = if recursive {
            fs::create_dir_all(&path)
        } else {
            fs::create_dir(&path)
        };
        
        match result {
            Ok(_) => Ok(Value::Bool(true)),
            Err(e) => Err(NjilError::ExecutionError(format!("创建目录失败: {}", e))),
        }
    }
    
    fn name(&self) -> &'static str {
        "system.fs.mkdir"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["fs.mkdir"]
    }
} 