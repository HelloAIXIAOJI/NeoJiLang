use crate::error::NjilError;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use serde_json::Value;
use std::fs;
use std::path::Path;

/// 处理文件系统列出目录内容的操作
pub struct FsListHandler;

impl StatementHandler for FsListHandler {
    fn name(&self) -> &'static str {
        "system.fs.list"
    }

    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        let path = super::get_path_param(interpreter, value)?;
        
        // 检查路径是否存在且是目录
        let path_obj = Path::new(&path);
        if !path_obj.exists() {
            return Err(NjilError::ExecutionError(format!("路径不存在: {}", path)));
        }
        
        if !path_obj.is_dir() {
            return Err(NjilError::ExecutionError(format!("路径不是目录: {}", path)));
        }
        
        // 读取目录内容
        let entries = fs::read_dir(path)
            .map_err(|e| NjilError::ExecutionError(format!("读取目录失败: {}", e)))?;
        
        // 将目录内容转换为JSON数组
        let mut result = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| NjilError::ExecutionError(format!("读取目录条目失败: {}", e)))?;
            let file_name = entry.file_name().to_string_lossy().to_string();
            result.push(Value::String(file_name));
        }
        
        Ok(Value::Array(result))
    }
}

/// 静态实例
pub static FS_LIST_HANDLER: FsListHandler = FsListHandler;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;
    
    #[test]
    fn test_fs_list() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path().to_str().unwrap().to_string();
        
        // 创建测试文件
        File::create(temp_dir.path().join("file1.txt")).unwrap();
        File::create(temp_dir.path().join("file2.txt")).unwrap();
        fs::create_dir(temp_dir.path().join("subdir")).unwrap();
        
        let mut interpreter = Interpreter::new();
        let handler = FsListHandler;
        
        // 测试列出目录内容
        let value = Value::String(dir_path.clone());
        let result = handler.handle(&mut interpreter, &value).unwrap();
        
        if let Value::Array(files) = result {
            assert_eq!(files.len(), 3);
            // 检查是否包含所有文件
            let file_names: Vec<String> = files.iter()
                .map(|v| v.as_str().unwrap().to_string())
                .collect();
            assert!(file_names.contains(&"file1.txt".to_string()));
            assert!(file_names.contains(&"file2.txt".to_string()));
            assert!(file_names.contains(&"subdir".to_string()));
        } else {
            panic!("Expected array result");
        }
        
        // 测试非目录路径
        let non_dir_path = temp_dir.path().join("file1.txt").to_str().unwrap().to_string();
        let value = Value::String(non_dir_path);
        let result = handler.handle(&mut interpreter, &value);
        assert!(result.is_err());
        
        // 测试不存在的路径
        let non_existent_path = temp_dir.path().join("non_existent").to_str().unwrap().to_string();
        let value = Value::String(non_existent_path);
        let result = handler.handle(&mut interpreter, &value);
        assert!(result.is_err());
    }
} 