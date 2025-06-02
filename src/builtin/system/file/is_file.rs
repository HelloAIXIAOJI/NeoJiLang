use crate::error::NjilError;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use serde_json::Value;
use std::path::Path;
use super::get_path_param;

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