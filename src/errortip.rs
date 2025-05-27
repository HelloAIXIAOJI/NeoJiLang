/// 错误提示信息模块
/// 集中管理所有错误提示，便于国际化和维护

/// 未知指令错误
pub fn unknown_instruction(instruction: &str) -> String {
    format!("未知的指令: {}", instruction)
}

/// 变量相关错误
pub mod var {
    /// 变量未定义错误
    pub fn undefined_variable(name: &str) -> String {
        format!("未定义的变量: {}", name)
    }
    
    /// var.set参数类型错误
    pub fn var_set_requires_object() -> &'static str {
        "var.set需要一个对象参数"
    }
    
    /// var参数类型错误
    pub fn var_requires_string() -> &'static str {
        "var指令需要一个字符串参数"
    }
}

/// 字符串操作相关错误
pub mod string {
    /// 字符串连接参数类型错误
    pub fn concat_requires_array() -> &'static str {
        "字符串连接需要一个数组"
    }
} 