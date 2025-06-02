use crate::error::NjilError;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use serde_json::Value;
use std::path::Path;
use super::get_path_param;

/// 检查路径是否为目录处理器
pub struct FsIsDirHandler;

// 静态实例
pub static FS_IS_DIR_HANDLER: FsIsDirHandler = FsIsDirHandler;

impl StatementHandler for FsIsDirHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        // 获取路径参数
        let path = get_path_param(interpreter, value)?;
        
        // 检查是否为目录
        let is_dir = Path::new(&path).is_dir();
        
        Ok(Value::Bool(is_dir))
    }
    
    fn name(&self) -> &'static str {
        "system.fs.isDir"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["fs.isDir"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builtin::system::is_file::FS_IS_FILE_HANDLER;
    use std::fs::File;
    use tempfile::tempdir;
    
    #[test]
    fn test_fs_is_file_and_is_dir() {
        let mut interpreter = Interpreter::new();
        
        // 创建临时目录和文件
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let dir_path = temp_dir.path().join("testdir");
        
        File::create(&file_path).unwrap();
        std::fs::create_dir(&dir_path).unwrap();
        
        // 测试 isFile
        let result = FS_IS_FILE_HANDLER.handle(
            &mut interpreter, 
            &Value::String(file_path.to_str().unwrap().to_string())
        ).unwrap();
        assert_eq!(result, Value::Bool(true));
        
        let result = FS_IS_FILE_HANDLER.handle(
            &mut interpreter, 
            &Value::String(dir_path.to_str().unwrap().to_string())
        ).unwrap();
        assert_eq!(result, Value::Bool(false));
        
        // 测试 isDir
        let result = FS_IS_DIR_HANDLER.handle(
            &mut interpreter, 
            &Value::String(file_path.to_str().unwrap().to_string())
        ).unwrap();
        assert_eq!(result, Value::Bool(false));
        
        let result = FS_IS_DIR_HANDLER.handle(
            &mut interpreter, 
            &Value::String(dir_path.to_str().unwrap().to_string())
        ).unwrap();
        assert_eq!(result, Value::Bool(true));
    }
} 