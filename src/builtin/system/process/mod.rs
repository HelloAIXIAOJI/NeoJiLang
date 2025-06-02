use crate::error::NjilError;
use crate::statements::StatementHandler;

// 导出子模块
pub mod exec;
pub mod spawn;
pub mod pid;
pub mod list;
pub mod kill;

// 从各个子模块导出静态处理器实例
pub use exec::PROCESS_EXEC_HANDLER;
pub use spawn::PROCESS_SPAWN_HANDLER;
pub use pid::PROCESS_PID_HANDLER;
pub use list::PROCESS_LIST_HANDLER;
pub use kill::PROCESS_KILL_HANDLER;

// 注册所有进程相关处理器
pub fn register_handlers() -> Vec<&'static dyn StatementHandler> {
    vec![
        &PROCESS_EXEC_HANDLER,
        &PROCESS_SPAWN_HANDLER,
        &PROCESS_PID_HANDLER,
        &PROCESS_LIST_HANDLER,
        &PROCESS_KILL_HANDLER,
    ]
} 