pub mod error;
pub mod types;
pub mod interpreter;
pub mod statements;
pub mod errortip;
pub mod njis;
pub mod builtin;
pub mod utils;

use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

/// 全局调试模式标志
static DEBUG_MODE: AtomicBool = AtomicBool::new(false);

/// 设置调试模式
pub fn set_debug_mode(enabled: bool) {
    DEBUG_MODE.store(enabled, Ordering::SeqCst);
}

/// 检查调试模式是否开启
pub fn is_debug_mode() -> bool {
    DEBUG_MODE.load(Ordering::SeqCst)
}

/// 条件打印调试信息，只有在调试模式开启时才打印
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        if $crate::is_debug_mode() {
            println!($($arg)*);
        }
    };
}

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