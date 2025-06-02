use crate::error::NjilError;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use serde_json::Value;
use std::fs;
use std::path::Path;

// 导出子模块
pub mod exists;
pub mod is_file;
pub mod is_dir;
pub mod mkdir;
pub mod remove;
pub mod copy;
pub mod move_file;
pub mod list;
pub mod process;

// 从各个子模块导出静态处理器实例
pub use exists::FS_EXISTS_HANDLER;
pub use is_file::FS_IS_FILE_HANDLER;
pub use is_dir::FS_IS_DIR_HANDLER;
pub use mkdir::FS_MKDIR_HANDLER;
pub use remove::FS_REMOVE_HANDLER;
pub use copy::FS_COPY_HANDLER;
pub use move_file::FS_MOVE_HANDLER;
pub use list::FS_LIST_HANDLER;

// 从进程模块导出静态处理器实例
pub use process::exec::PROCESS_EXEC_HANDLER;
pub use process::spawn::PROCESS_SPAWN_HANDLER;
pub use process::pid::PROCESS_PID_HANDLER;
pub use process::list::PROCESS_LIST_HANDLER;
pub use process::kill::PROCESS_KILL_HANDLER;

/// System模块，提供系统和环境相关功能
pub struct SystemModule;

impl SystemModule {
    pub fn new() -> Self {
        Self {}
    }
}

impl super::BuiltinModule for SystemModule {
    fn name(&self) -> &'static str {
        "system"
    }
    
    fn get_handlers(&self) -> Vec<&'static dyn StatementHandler> {
        let mut handlers: Vec<&'static dyn StatementHandler> = vec![
            // 文件系统相关处理器
            &FS_EXISTS_HANDLER,
            &FS_IS_FILE_HANDLER,
            &FS_IS_DIR_HANDLER,
            &FS_MKDIR_HANDLER,
            &FS_REMOVE_HANDLER,
            &FS_COPY_HANDLER,
            &FS_MOVE_HANDLER,
            &FS_LIST_HANDLER,
        ];
        
        // 添加进程相关处理器
        let process_handlers: Vec<&'static dyn StatementHandler> = vec![
            &PROCESS_EXEC_HANDLER,
            &PROCESS_SPAWN_HANDLER,
            &PROCESS_PID_HANDLER,
            &PROCESS_LIST_HANDLER,
            &PROCESS_KILL_HANDLER,
        ];
        
        handlers.extend(process_handlers);
        handlers
    }
}

// 辅助函数: 从参数中获取路径
pub fn get_path_param(interpreter: &mut Interpreter, value: &Value) -> Result<String, NjilError> {
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
pub fn copy_dir_recursive(from: &str, to: &str) -> Result<(), NjilError> {
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
    // 测试会在各个子模块中实现
} 