use crate::error::NjilError;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use serde_json::Value;
use std::fs;
use super::get_path_param;

/// 列出目录内容处理器
pub struct FsListHandler;

// 静态实例
pub static FS_LIST_HANDLER: FsListHandler = FsListHandler;

impl StatementHandler for FsListHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        // 获取路径参数
        let path = get_path_param(interpreter, value)?;
        
        // 列出目录内容
        let entries = fs::read_dir(&path)
            .map_err(|e| NjilError::ExecutionError(format!("读取目录失败: {}", e)))?;
        
        // 创建结果数组
        let mut result = Vec::new();
        
        // 遍历目录条目
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().to_string_lossy().to_string();
                let file_type = entry.file_type().map_err(|e| NjilError::ExecutionError(format!("获取文件类型失败: {}", e)))?;
                
                // 创建条目对象
                let mut entry_obj = serde_json::Map::new();
                entry_obj.insert("name".to_string(), Value::String(file_name));
                entry_obj.insert("isDir".to_string(), Value::Bool(file_type.is_dir()));
                entry_obj.insert("isFile".to_string(), Value::Bool(file_type.is_file()));
                
                result.push(Value::Object(entry_obj));
            }
        }
        
        Ok(Value::Array(result))
    }
    
    fn name(&self) -> &'static str {
        "system.fs.list"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["fs.list"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;
    
    #[test]
    fn test_fs_list() {
        let mut interpreter = Interpreter::new();
        
        // 创建临时目录和文件
        let temp_dir = tempdir().unwrap();
        let file1_path = temp_dir.path().join("file1.txt");
        let file2_path = temp_dir.path().join("file2.txt");
        let dir_path = temp_dir.path().join("subdir");
        
        File::create(&file1_path).unwrap();
        File::create(&file2_path).unwrap();
        std::fs::create_dir(&dir_path).unwrap();
        
        // 测试列出目录内容
        let result = FS_LIST_HANDLER.handle(
            &mut interpreter, 
            &Value::String(temp_dir.path().to_str().unwrap().to_string())
        ).unwrap();
        
        if let Value::Array(entries) = result {
            assert_eq!(entries.len(), 3); // 两个文件和一个目录
            
            // 检查是否包含我们创建的文件和目录
            let mut found_file1 = false;
            let mut found_file2 = false;
            let mut found_dir = false;
            
            for entry in entries {
                if let Value::Object(obj) = entry {
                    if let Some(Value::String(name)) = obj.get("name") {
                        if name == "file1.txt" {
                            found_file1 = true;
                            assert_eq!(obj.get("isFile"), Some(&Value::Bool(true)));
                            assert_eq!(obj.get("isDir"), Some(&Value::Bool(false)));
                        } else if name == "file2.txt" {
                            found_file2 = true;
                            assert_eq!(obj.get("isFile"), Some(&Value::Bool(true)));
                            assert_eq!(obj.get("isDir"), Some(&Value::Bool(false)));
                        } else if name == "subdir" {
                            found_dir = true;
                            assert_eq!(obj.get("isFile"), Some(&Value::Bool(false)));
                            assert_eq!(obj.get("isDir"), Some(&Value::Bool(true)));
                        }
                    }
                }
            }
            
            assert!(found_file1, "file1.txt not found in directory listing");
            assert!(found_file2, "file2.txt not found in directory listing");
            assert!(found_dir, "subdir not found in directory listing");
        } else {
            panic!("Expected array result from fs.list");
        }
    }
} 