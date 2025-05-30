pub mod error;
pub mod types;
pub mod interpreter;
pub mod statements;
pub mod errortip;
pub mod njis;
pub mod builtin;
pub mod utils;

use std::path::Path;

/// 运行一个NJIL或NJIS文件
pub fn run_file<P: AsRef<Path>>(file_path: P) -> Result<serde_json::Value, error::NjilError> {
    let path = file_path.as_ref();
    
    // 根据文件扩展名选择适当的解析器
    if let Some(extension) = path.extension() {
        if extension == "njis" {
            // 使用NJIS解析器
            return njis::run_njis_file(path);
        } else if extension == "njil" {
            // 使用NJIL解析器
            return interpreter::run_file(path);
        }
    }
    
    // 默认使用NJIL解析器
    interpreter::run_file(path)
}

// 重新导出常用类型
pub use crate::error::NjilError;
pub use crate::types::{NjilProgram, Program, Function};
pub use crate::interpreter::Interpreter; 