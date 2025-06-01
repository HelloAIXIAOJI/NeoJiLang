use std::fs;
use std::path::Path;
use serde_json::{Value, Map};
use crate::error::NjilError;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use super::BuiltinModule;

/// 系统模块，提供系统级功能
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

/// 检查文件或目录是否存在
pub struct FsExistsHandler;

// 静态实例
pub static FS_EXISTS_HANDLER: FsExistsHandler = FsExistsHandler;

impl StatementHandler for FsExistsHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        let path = match value {
            Value::String(p) => p.clone(),
            Value::Object(obj) => {
                let path_value = obj.get("path").ok_or_else(|| 
                    NjilError::ExecutionError("system.fs.exists需要一个path参数".to_string()))?;
                
                match interpreter.evaluate_value(path_value)? {
                    Value::String(p) => p,
                    _ => return Err(NjilError::ExecutionError("system.fs.exists的path参数必须是字符串".to_string())),
                }
            },
            _ => return Err(NjilError::ExecutionError("system.fs.exists需要一个字符串参数或包含path的对象".to_string())),
        };
        
        // 检查文件或目录是否存在
        let exists = Path::new(&path).exists();
        
        Ok(Value::Bool(exists))
    }
    
    fn name(&self) -> &'static str {
        "system.fs.exists"
    }
}

/// 检查路径是否为文件
pub struct FsIsFileHandler;

// 静态实例
pub static FS_IS_FILE_HANDLER: FsIsFileHandler = FsIsFileHandler;

impl StatementHandler for FsIsFileHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        let path = match value {
            Value::String(p) => p.clone(),
            Value::Object(obj) => {
                let path_value = obj.get("path").ok_or_else(|| 
                    NjilError::ExecutionError("system.fs.isFile需要一个path参数".to_string()))?;
                
                match interpreter.evaluate_value(path_value)? {
                    Value::String(p) => p,
                    _ => return Err(NjilError::ExecutionError("system.fs.isFile的path参数必须是字符串".to_string())),
                }
            },
            _ => return Err(NjilError::ExecutionError("system.fs.isFile需要一个字符串参数或包含path的对象".to_string())),
        };
        
        // 检查是否为文件
        let is_file = Path::new(&path).is_file();
        
        Ok(Value::Bool(is_file))
    }
    
    fn name(&self) -> &'static str {
        "system.fs.isFile"
    }
}

/// 检查路径是否为目录
pub struct FsIsDirHandler;

// 静态实例
pub static FS_IS_DIR_HANDLER: FsIsDirHandler = FsIsDirHandler;

impl StatementHandler for FsIsDirHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        let path = match value {
            Value::String(p) => p.clone(),
            Value::Object(obj) => {
                let path_value = obj.get("path").ok_or_else(|| 
                    NjilError::ExecutionError("system.fs.isDir需要一个path参数".to_string()))?;
                
                match interpreter.evaluate_value(path_value)? {
                    Value::String(p) => p,
                    _ => return Err(NjilError::ExecutionError("system.fs.isDir的path参数必须是字符串".to_string())),
                }
            },
            _ => return Err(NjilError::ExecutionError("system.fs.isDir需要一个字符串参数或包含path的对象".to_string())),
        };
        
        // 检查是否为目录
        let is_dir = Path::new(&path).is_dir();
        
        Ok(Value::Bool(is_dir))
    }
    
    fn name(&self) -> &'static str {
        "system.fs.isDir"
    }
}

/// 创建目录
pub struct FsMkdirHandler;

// 静态实例
pub static FS_MKDIR_HANDLER: FsMkdirHandler = FsMkdirHandler;

impl StatementHandler for FsMkdirHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        let (path, recursive) = match value {
            Value::String(p) => (p.clone(), false),
            Value::Object(obj) => {
                let path_value = obj.get("path").ok_or_else(|| 
                    NjilError::ExecutionError("system.fs.mkdir需要一个path参数".to_string()))?;
                
                let path = match interpreter.evaluate_value(path_value)? {
                    Value::String(p) => p,
                    _ => return Err(NjilError::ExecutionError("system.fs.mkdir的path参数必须是字符串".to_string())),
                };
                
                // 是否递归创建
                let recursive = if let Some(rec_value) = obj.get("recursive") {
                    match interpreter.evaluate_value(rec_value)? {
                        Value::Bool(b) => b,
                        _ => false,
                    }
                } else {
                    false
                };
                
                (path, recursive)
            },
            _ => return Err(NjilError::ExecutionError("system.fs.mkdir需要一个字符串参数或包含path的对象".to_string())),
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
}

/// 删除文件或目录
pub struct FsRemoveHandler;

// 静态实例
pub static FS_REMOVE_HANDLER: FsRemoveHandler = FsRemoveHandler;

impl StatementHandler for FsRemoveHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        let (path, recursive) = match value {
            Value::String(p) => (p.clone(), false),
            Value::Object(obj) => {
                let path_value = obj.get("path").ok_or_else(|| 
                    NjilError::ExecutionError("system.fs.remove需要一个path参数".to_string()))?;
                
                let path = match interpreter.evaluate_value(path_value)? {
                    Value::String(p) => p,
                    _ => return Err(NjilError::ExecutionError("system.fs.remove的path参数必须是字符串".to_string())),
                };
                
                // 是否递归删除
                let recursive = if let Some(rec_value) = obj.get("recursive") {
                    match interpreter.evaluate_value(rec_value)? {
                        Value::Bool(b) => b,
                        _ => false,
                    }
                } else {
                    false
                };
                
                (path, recursive)
            },
            _ => return Err(NjilError::ExecutionError("system.fs.remove需要一个字符串参数或包含path的对象".to_string())),
        };
        
        // 检查路径是否存在
        let path_obj = Path::new(&path);
        if !path_obj.exists() {
            return Ok(Value::Bool(false));
        }
        
        // 删除文件或目录
        let result = if path_obj.is_dir() {
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
}

/// 复制文件或目录
pub struct FsCopyHandler;

// 静态实例
pub static FS_COPY_HANDLER: FsCopyHandler = FsCopyHandler;

impl StatementHandler for FsCopyHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Object(obj) = value {
            // 获取源路径
            let src_value = obj.get("from").ok_or_else(|| 
                NjilError::ExecutionError("system.fs.copy需要一个from参数".to_string()))?;
            
            let src = match interpreter.evaluate_value(src_value)? {
                Value::String(p) => p,
                _ => return Err(NjilError::ExecutionError("system.fs.copy的from参数必须是字符串".to_string())),
            };
            
            // 获取目标路径
            let dest_value = obj.get("to").ok_or_else(|| 
                NjilError::ExecutionError("system.fs.copy需要一个to参数".to_string()))?;
            
            let dest = match interpreter.evaluate_value(dest_value)? {
                Value::String(p) => p,
                _ => return Err(NjilError::ExecutionError("system.fs.copy的to参数必须是字符串".to_string())),
            };
            
            // 是否递归复制目录
            let recursive = if let Some(rec_value) = obj.get("recursive") {
                match interpreter.evaluate_value(rec_value)? {
                    Value::Bool(b) => b,
                    _ => false,
                }
            } else {
                false
            };
            
            // 检查源路径是否存在
            let src_path = Path::new(&src);
            if !src_path.exists() {
                return Err(NjilError::ExecutionError(format!("源路径不存在: {}", src)));
            }
            
            // 复制文件或目录
            let result = if src_path.is_dir() {
                if recursive {
                    // 递归复制目录
                    copy_dir_recursive(src_path, Path::new(&dest))
                } else {
                    // 非递归模式下不复制目录
                    Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "需要设置recursive=true来复制目录"))
                }
            } else {
                // 复制文件
                fs::copy(&src, &dest).map(|_| ())
            };
            
            match result {
                Ok(_) => Ok(Value::Bool(true)),
                Err(e) => Err(NjilError::ExecutionError(format!("复制文件或目录失败: {}", e))),
            }
        } else {
            Err(NjilError::ExecutionError("system.fs.copy需要一个包含from和to的对象".to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "system.fs.copy"
    }
}

/// 递归复制目录的辅助函数
fn copy_dir_recursive(src: &Path, dest: &Path) -> std::io::Result<()> {
    // 如果目标目录不存在，创建它
    if !dest.exists() {
        fs::create_dir_all(dest)?;
    }
    
    // 遍历源目录中的所有条目
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let file_name = entry_path.file_name().unwrap();
        let dest_path = dest.join(file_name);
        
        if entry_path.is_dir() {
            // 递归复制子目录
            copy_dir_recursive(&entry_path, &dest_path)?;
        } else {
            // 复制文件
            fs::copy(&entry_path, &dest_path)?;
        }
    }
    
    Ok(())
}

/// 移动或重命名文件或目录
pub struct FsMoveHandler;

// 静态实例
pub static FS_MOVE_HANDLER: FsMoveHandler = FsMoveHandler;

impl StatementHandler for FsMoveHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Object(obj) = value {
            // 获取源路径
            let src_value = obj.get("from").ok_or_else(|| 
                NjilError::ExecutionError("system.fs.move需要一个from参数".to_string()))?;
            
            let src = match interpreter.evaluate_value(src_value)? {
                Value::String(p) => p,
                _ => return Err(NjilError::ExecutionError("system.fs.move的from参数必须是字符串".to_string())),
            };
            
            // 获取目标路径
            let dest_value = obj.get("to").ok_or_else(|| 
                NjilError::ExecutionError("system.fs.move需要一个to参数".to_string()))?;
            
            let dest = match interpreter.evaluate_value(dest_value)? {
                Value::String(p) => p,
                _ => return Err(NjilError::ExecutionError("system.fs.move的to参数必须是字符串".to_string())),
            };
            
            // 检查源路径是否存在
            if !Path::new(&src).exists() {
                return Err(NjilError::ExecutionError(format!("源路径不存在: {}", src)));
            }
            
            // 移动文件或目录
            match fs::rename(&src, &dest) {
                Ok(_) => Ok(Value::Bool(true)),
                Err(e) => Err(NjilError::ExecutionError(format!("移动文件或目录失败: {}", e))),
            }
        } else {
            Err(NjilError::ExecutionError("system.fs.move需要一个包含from和to的对象".to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "system.fs.move"
    }
}

/// 列出目录内容
pub struct FsListHandler;

// 静态实例
pub static FS_LIST_HANDLER: FsListHandler = FsListHandler;

impl StatementHandler for FsListHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        let (path, include_hidden, include_info) = match value {
            Value::String(p) => (p.clone(), false, false),
            Value::Object(obj) => {
                let path_value = obj.get("path").ok_or_else(|| 
                    NjilError::ExecutionError("system.fs.list需要一个path参数".to_string()))?;
                
                let path = match interpreter.evaluate_value(path_value)? {
                    Value::String(p) => p,
                    _ => return Err(NjilError::ExecutionError("system.fs.list的path参数必须是字符串".to_string())),
                };
                
                // 是否包含隐藏文件
                let include_hidden = if let Some(hidden_value) = obj.get("includeHidden") {
                    match interpreter.evaluate_value(hidden_value)? {
                        Value::Bool(b) => b,
                        _ => false,
                    }
                } else {
                    false
                };
                
                // 是否包含文件详细信息
                let include_info = if let Some(info_value) = obj.get("includeInfo") {
                    match interpreter.evaluate_value(info_value)? {
                        Value::Bool(b) => b,
                        _ => false,
                    }
                } else {
                    false
                };
                
                (path, include_hidden, include_info)
            },
            _ => return Err(NjilError::ExecutionError("system.fs.list需要一个字符串参数或包含path的对象".to_string())),
        };
        
        // 检查目录是否存在
        let dir_path = Path::new(&path);
        if !dir_path.exists() {
            return Err(NjilError::ExecutionError(format!("目录不存在: {}", path)));
        }
        
        if !dir_path.is_dir() {
            return Err(NjilError::ExecutionError(format!("路径不是目录: {}", path)));
        }
        
        // 列出目录内容
        let mut entries = Vec::new();
        
        match fs::read_dir(dir_path) {
            Ok(dir_iter) => {
                for entry_result in dir_iter {
                    match entry_result {
                        Ok(entry) => {
                            let file_name = entry.file_name();
                            let file_name_str = file_name.to_string_lossy().to_string();
                            
                            // 处理隐藏文件
                            if !include_hidden && file_name_str.starts_with('.') {
                                continue;
                            }
                            
                            if include_info {
                                // 包含详细信息
                                let mut entry_info = Map::new();
                                entry_info.insert("name".to_string(), Value::String(file_name_str));
                                
                                // 获取文件类型
                                let file_type = match entry.file_type() {
                                    Ok(ft) => {
                                        if ft.is_dir() {
                                            "directory"
                                        } else if ft.is_file() {
                                            "file"
                                        } else if ft.is_symlink() {
                                            "symlink"
                                        } else {
                                            "unknown"
                                        }
                                    },
                                    Err(_) => "unknown",
                                };
                                entry_info.insert("type".to_string(), Value::String(file_type.to_string()));
                                
                                // 获取文件大小
                                if let Ok(metadata) = entry.metadata() {
                                    if metadata.is_file() {
                                        entry_info.insert("size".to_string(), Value::Number(metadata.len().into()));
                                    }
                                    
                                    // 获取修改时间
                                    if let Ok(modified) = metadata.modified() {
                                        if let Ok(modified_secs) = modified.duration_since(std::time::UNIX_EPOCH) {
                                            entry_info.insert("modified".to_string(), Value::Number(modified_secs.as_secs().into()));
                                        }
                                    }
                                }
                                
                                entries.push(Value::Object(entry_info));
                            } else {
                                // 只包含文件名
                                entries.push(Value::String(file_name_str));
                            }
                        },
                        Err(e) => {
                            return Err(NjilError::ExecutionError(format!("读取目录条目失败: {}", e)));
                        }
                    }
                }
            },
            Err(e) => {
                return Err(NjilError::ExecutionError(format!("读取目录失败: {}", e)));
            }
        }
        
        Ok(Value::Array(entries))
    }
    
    fn name(&self) -> &'static str {
        "system.fs.list"
    }
}

// 导入单元测试
#[cfg(test)]
mod tests; 