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

/// 控制流相关错误
pub mod control_flow {
    /// if语句参数类型错误
    pub fn if_requires_object() -> &'static str {
        "if语句需要一个对象参数"
    }
    
    /// if语句缺少必要字段
    pub fn if_missing_fields() -> &'static str {
        "if语句需要condition和then字段"
    }
    
    /// while循环参数类型错误
    pub fn while_requires_object() -> &'static str {
        "while循环需要一个对象参数"
    }
    
    /// while循环缺少必要字段
    pub fn while_missing_fields() -> &'static str {
        "while循环需要condition和body字段"
    }
    
    /// for循环参数类型错误
    pub fn for_requires_object() -> &'static str {
        "for循环需要一个对象参数"
    }
    
    /// for循环缺少必要字段
    pub fn for_missing_fields() -> &'static str {
        "for循环需要count和body字段"
    }
    
    /// foreach循环参数类型错误
    pub fn foreach_requires_object() -> &'static str {
        "foreach循环需要一个对象参数"
    }
    
    /// foreach循环缺少必要字段
    pub fn foreach_missing_fields() -> &'static str {
        "foreach循环需要collection、var和body字段"
    }
    
    /// 循环变量名类型错误
    pub fn var_name_requires_string() -> &'static str {
        "循环变量名必须是字符串"
    }
    
    /// 循环次数类型错误
    pub fn count_requires_number() -> &'static str {
        "循环次数必须是数字"
    }
    
    /// 集合类型错误
    pub fn collection_requires_array_or_object() -> &'static str {
        "foreach循环的集合必须是数组或对象"
    }
}

/// 逻辑运算相关错误
pub mod logic {
    /// 逻辑与操作参数类型错误
    pub fn and_requires_array() -> &'static str {
        "逻辑与操作需要一个数组参数"
    }
    
    /// 逻辑与操作操作数不足错误
    pub fn and_requires_operands() -> &'static str {
        "逻辑与操作需要至少一个操作数"
    }
    
    /// 逻辑或操作参数类型错误
    pub fn or_requires_array() -> &'static str {
        "逻辑或操作需要一个数组参数"
    }
    
    /// 逻辑或操作操作数不足错误
    pub fn or_requires_operands() -> &'static str {
        "逻辑或操作需要至少一个操作数"
    }
}

/// 延时相关错误信息
pub mod sleep {
    /// 延时参数无效
    pub fn invalid_sleep_duration() -> String {
        "延时参数无效，必须是正数或带有duration字段的对象".to_string()
    }
    
    /// 时间单位无效
    pub fn invalid_time_unit(unit: &str) -> String {
        format!("无效的时间单位: {}，支持的单位有: ms (毫秒), s (秒), m (分钟)", unit)
    }
    
    /// 延时值必须是数字
    pub fn duration_must_be_number() -> String {
        "延时时间必须是数字".to_string()
    }
    
    /// 延时值不能为负
    pub fn duration_cannot_be_negative() -> String {
        "延时时间不能为负数".to_string()
    }
}

/// 异常处理相关错误信息
pub mod exception {
    /// throw语句参数错误
    pub fn throw_requires_value() -> &'static str {
        "throw语句需要一个异常值参数"
    }
    
    /// try/catch语句参数类型错误
    pub fn try_catch_requires_object() -> &'static str {
        "try/catch语句需要一个对象参数"
    }
    
    /// try/catch语句缺少必要字段
    pub fn try_catch_missing_fields() -> &'static str {
        "try/catch语句需要try字段，且可选地包含catch字段"
    }
    
    /// catch块var字段错误
    pub fn catch_var_requires_string() -> &'static str {
        "catch块的var字段（异常变量）必须是字符串"
    }
} 