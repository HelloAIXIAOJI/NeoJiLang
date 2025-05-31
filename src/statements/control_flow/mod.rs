// 控制流模块 - 导出所有控制流语句处理器

// 子模块
pub mod if_stmt;
pub mod while_loop;
pub mod for_loop;
pub mod foreach_loop;
pub mod break_continue;

// 重新导出所有控制流语句处理器，方便其他模块使用
pub use self::if_stmt::IF_HANDLER;
pub use self::while_loop::WHILE_LOOP_HANDLER;
pub use self::for_loop::FOR_LOOP_HANDLER;
pub use self::foreach_loop::FOREACH_LOOP_HANDLER;
pub use self::break_continue::{BREAK_HANDLER, CONTINUE_HANDLER};

// 导出所有处理器的集合，便于注册
pub fn get_all_handlers() -> Vec<&'static dyn crate::statements::StatementHandler> {
    vec![
        &IF_HANDLER,
        &WHILE_LOOP_HANDLER,
        &FOR_LOOP_HANDLER,
        &FOREACH_LOOP_HANDLER,
        &BREAK_HANDLER,
        &CONTINUE_HANDLER,
    ]
} 