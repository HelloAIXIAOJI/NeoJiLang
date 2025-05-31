// 算术运算模块 - 导出所有算术运算处理器

// 子模块
pub mod add;
pub mod subtract;
pub mod multiply;
pub mod divide;
pub mod modulo;
pub mod compare;

// 重新导出所有处理器，方便其他模块使用
pub use self::add::ADD_HANDLER;
pub use self::subtract::SUBTRACT_HANDLER;
pub use self::multiply::MULTIPLY_HANDLER;
pub use self::divide::DIVIDE_HANDLER;
pub use self::modulo::MODULO_HANDLER;
pub use self::compare::COMPARE_HANDLER;

// 导出所有处理器的集合，便于注册
pub fn get_all_handlers() -> Vec<&'static dyn crate::statements::StatementHandler> {
    vec![
        &ADD_HANDLER,
        &SUBTRACT_HANDLER,
        &MULTIPLY_HANDLER,
        &DIVIDE_HANDLER,
        &MODULO_HANDLER,
        &COMPARE_HANDLER,
    ]
} 