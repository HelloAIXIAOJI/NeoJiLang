use crate::error::NjilError;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use serde_json::Value;
use std::fs;
use std::path::Path;
use super::BuiltinModule;

/// System模块，提供系统和环境相关功能
pub struct SystemModule;

impl SystemModule {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuiltinModule for SystemModule {
    fn name(&self) -> &'static str {
        "system"
    }
    
    fn get_handlers(&self) -> Vec<&'static dyn StatementHandler> {
        vec![
            // 文件系统相关处理器
            &FS_EXISTS_HANDLER,
            &FS_IS_FILE_HANDLER,
            &FS_IS_DIR_HANDLER,
            &FS_MKDIR_HANDLER,
            &FS_REMOVE_HANDLER,
            &FS_COPY_HANDLER,
            &FS_MOVE_HANDLER,
            &FS_LIST_HANDLER,
        ]
    }
}

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

/// 检查路径是否为文件处理器
pub struct FsIsFileHandler;

// 静态实例
pub static FS_IS_FILE_HANDLER: FsIsFileHandler = FsIsFileHandler;

impl StatementHandler for FsIsFileHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        // 获取路径参数
        let path = get_path_param(interpreter, value)?;
        
        // 检查是否为文件
        let is_file = Path::new(&path).is_file();
        
        Ok(Value::Bool(is_file))
    }
    
    fn name(&self) -> &'static str {
        "system.fs.isFile"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["fs.isFile"]
    }
}

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

/// 移动文件或目录处理器
pub struct FsMoveHandler;

// 静态实例
pub static FS_MOVE_HANDLER: FsMoveHandler = FsMoveHandler;

impl StatementHandler for FsMoveHandler {
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
            
            // 移动文件或目录
            fs::rename(&from, &to)
                .map_err(|e| NjilError::ExecutionError(format!("移动文件或目录失败: {}", e)))?;
            
            Ok(Value::Bool(true))
        } else {
            Err(NjilError::ExecutionError("system.fs.move需要一个对象参数".to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "system.fs.move"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["fs.move"]
    }
}

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

// 辅助函数: 从参数中获取路径
fn get_path_param(interpreter: &mut Interpreter, value: &Value) -> Result<String, NjilError> {
    match value {
        Value::String(path) => Ok(path.clone()),
        Value::Object(obj) => {
            if let Some(path_value) = obj.get("path") {
                let evaluated = interpreter.evaluate_value(path_value)?;
                if let Value::String(path) = evaluated {
                    Ok(path)
                } else {
                    Err(NjilError::ExecutionError("path参数必须是字符串".to_string()))
                }
            } else {
                Err(NjilError::ExecutionError("缺少path参数".to_string()))
            }
        },
        _ => Err(NjilError::ExecutionError("参数必须是字符串或包含path字段的对象".to_string())),
    }
}

// 辅助函数: 递归复制目录
fn copy_dir_recursive(from: &str, to: &str) -> Result<(), NjilError> {
    // 创建目标目录
    fs::create_dir_all(to)
        .map_err(|e| NjilError::ExecutionError(format!("创建目标目录失败: {}", e)))?;
    
    // 读取源目录
    let entries = fs::read_dir(from)
        .map_err(|e| NjilError::ExecutionError(format!("读取源目录失败: {}", e)))?;
    
    // 遍历源目录中的条目
    for entry in entries {
        let entry = entry.map_err(|e| NjilError::ExecutionError(format!("读取目录条目失败: {}", e)))?;
        let file_type = entry.file_type().map_err(|e| NjilError::ExecutionError(format!("获取文件类型失败: {}", e)))?;
        
        let src_path = entry.path();
        let file_name = entry.file_name().to_string_lossy().to_string();
        let dst_path = Path::new(to).join(file_name);
        
        if file_type.is_dir() {
            // 递归复制子目录
            copy_dir_recursive(
                src_path.to_str().unwrap(),
                dst_path.to_str().unwrap()
            )?;
        } else {
            // 复制文件
            fs::copy(&src_path, &dst_path)
                .map_err(|e| NjilError::ExecutionError(format!("复制文件失败: {}", e)))?;
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;
    
    #[test]
    fn test_fs_exists() {
        let mut interpreter = Interpreter::new();
        
        // 创建临时目录和文件
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let dir_path = temp_dir.path().join("testdir");
        
        File::create(&file_path).unwrap();
        fs::create_dir(&dir_path).unwrap();
        
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
    
    #[test]
    fn test_fs_is_file_and_is_dir() {
        let mut interpreter = Interpreter::new();
        
        // 创建临时目录和文件
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let dir_path = temp_dir.path().join("testdir");
        
        File::create(&file_path).unwrap();
        fs::create_dir(&dir_path).unwrap();
        
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
    
    #[test]
    fn test_fs_copy_and_move() {
        let mut interpreter = Interpreter::new();
        
        // 创建临时目录和文件
        let temp_dir = tempdir().unwrap();
        let src_file_path = temp_dir.path().join("source.txt");
        let dst_file_path = temp_dir.path().join("dest.txt");
        let move_file_path = temp_dir.path().join("moved.txt");
        
        // 创建源文件
        let mut file = File::create(&src_file_path).unwrap();
        writeln!(file, "测试内容").unwrap();
        
        // 测试复制文件
        let mut obj = serde_json::Map::new();
        obj.insert("from".to_string(), Value::String(src_file_path.to_str().unwrap().to_string()));
        obj.insert("to".to_string(), Value::String(dst_file_path.to_str().unwrap().to_string()));
        
        let result = FS_COPY_HANDLER.handle(
            &mut interpreter, 
            &Value::Object(obj)
        ).unwrap();
        assert_eq!(result, Value::Bool(true));
        assert!(dst_file_path.exists());
        
        // 测试移动文件
        let mut obj = serde_json::Map::new();
        obj.insert("from".to_string(), Value::String(dst_file_path.to_str().unwrap().to_string()));
        obj.insert("to".to_string(), Value::String(move_file_path.to_str().unwrap().to_string()));
        
        let result = FS_MOVE_HANDLER.handle(
            &mut interpreter, 
            &Value::Object(obj)
        ).unwrap();
        assert_eq!(result, Value::Bool(true));
        assert!(!dst_file_path.exists());
        assert!(move_file_path.exists());
    }
    
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
        fs::create_dir(&dir_path).unwrap();
        
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