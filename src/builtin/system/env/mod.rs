use crate::statements::StatementHandler;

// 导出子模块
pub mod get;
pub mod set;
pub mod list;

// 从各个子模块导出静态处理器实例
pub use get::ENV_GET_HANDLER;
pub use set::ENV_SET_HANDLER;
pub use list::ENV_LIST_HANDLER;

// 注册所有环境变量相关处理器
pub fn register_handlers() -> Vec<&'static dyn StatementHandler> {
    vec![
        &ENV_GET_HANDLER,
        &ENV_SET_HANDLER,
        &ENV_LIST_HANDLER,
    ]
} 