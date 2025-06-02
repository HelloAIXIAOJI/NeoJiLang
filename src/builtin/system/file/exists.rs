use crate::error::NjilError;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use serde_json::Value;
use std::path::Path;
use super::get_path_param;

/// 检查文件或目录是否存在处理器
pub struct FsExistsHandler;

// 静态实例
pub static FS_EXISTS_HANDLER: FsExistsHandler = FsExistsHandler;

impl StatementHandler for FsExistsHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        // 获取路径参数
        let path = get_path_param(interpreter, value)?;
        
        // 检查文件或目录是否存在
        let exists = Path::new(&path).exists();
        
        Ok(Value::Bool(exists))
    }
    
    fn name(&self) -> &'static str {
        "system.fs.exists"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["fs.exists"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;
    
    #[test]
    fn test_fs_exists() {
        let mut interpreter = Interpreter::new();
        
        // 创建临时目录和文件
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let dir_path = temp_dir.path().join("testdir");
        
        File::create(&file_path).unwrap();
        std::fs::create_dir(&dir_path).unwrap();
        
        // 测试文件存在
        let result = FS_EXISTS_HANDLER.handle(
            &mut interpreter, 
            &Value::String(file_path.to_str().unwrap().to_string())
        ).unwrap();
        assert_eq!(result, Value::Bool(true));
        
        // 测试目录存在
        let result = FS_EXISTS_HANDLER.handle(
            &mut interpreter, 
            &Value::String(dir_path.to_str().unwrap().to_string())
        ).unwrap();
        assert_eq!(result, Value::Bool(true));
        
        // 测试不存在的路径
        let non_existent = temp_dir.path().join("non_existent.txt");
        let result = FS_EXISTS_HANDLER.handle(
            &mut interpreter, 
            &Value::String(non_existent.to_str().unwrap().to_string())
        ).unwrap();
        assert_eq!(result, Value::Bool(false));
    }
} 