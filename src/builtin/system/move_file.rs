use crate::error::NjilError;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use serde_json::Value;
use std::fs;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builtin::system::copy::FS_COPY_HANDLER;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;
    
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
} 