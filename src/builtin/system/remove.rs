use crate::error::NjilError;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use serde_json::Value;
use std::fs;
use std::path::Path;
use super::get_path_param;

/// 删除文件或目录处理器
pub struct FsRemoveHandler;

// 静态实例
pub static FS_REMOVE_HANDLER: FsRemoveHandler = FsRemoveHandler;

impl StatementHandler for FsRemoveHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        // 获取路径参数
        let path = get_path_param(interpreter, value)?;
        
        // 获取递归删除参数
        let recursive = if let Value::Object(obj) = value {
            if let Some(Value::Bool(rec)) = obj.get("recursive") {
                *rec
            } else {
                false // 默认不递归删除
            }
        } else {
            false
        };
        
        // 删除文件或目录
        let result = if Path::new(&path).is_dir() {
            if recursive {
                fs::remove_dir_all(&path)
            } else {
                fs::remove_dir(&path)
            }
        } else {
            fs::remove_file(&path)
        };
        
        match result {
            Ok(_) => Ok(Value::Bool(true)),
            Err(e) => Err(NjilError::ExecutionError(format!("删除文件或目录失败: {}", e))),
        }
    }
    
    fn name(&self) -> &'static str {
        "system.fs.remove"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["fs.remove"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builtin::system::mkdir::FS_MKDIR_HANDLER;
    use std::fs::File;
    use tempfile::tempdir;
    
    #[test]
    fn test_fs_mkdir_and_remove() {
        let mut interpreter = Interpreter::new();
        
        // 创建临时目录
        let temp_dir = tempdir().unwrap();
        let new_dir_path = temp_dir.path().join("newdir");
        let nested_dir_path = temp_dir.path().join("parent/child");
        
        // 测试创建简单目录
        let mut obj = serde_json::Map::new();
        obj.insert("path".to_string(), Value::String(new_dir_path.to_str().unwrap().to_string()));
        
        let result = FS_MKDIR_HANDLER.handle(
            &mut interpreter, 
            &Value::Object(obj)
        ).unwrap();
        assert_eq!(result, Value::Bool(true));
        assert!(new_dir_path.exists());
        
        // 测试创建嵌套目录（使用recursive参数）
        let mut obj = serde_json::Map::new();
        obj.insert("path".to_string(), Value::String(nested_dir_path.to_str().unwrap().to_string()));
        obj.insert("recursive".to_string(), Value::Bool(true));
        
        let result = FS_MKDIR_HANDLER.handle(
            &mut interpreter, 
            &Value::Object(obj)
        ).unwrap();
        assert_eq!(result, Value::Bool(true));
        assert!(nested_dir_path.exists());
        
        // 测试删除目录
        let result = FS_REMOVE_HANDLER.handle(
            &mut interpreter, 
            &Value::String(new_dir_path.to_str().unwrap().to_string())
        ).unwrap();
        assert_eq!(result, Value::Bool(true));
        assert!(!new_dir_path.exists());
    }
} 