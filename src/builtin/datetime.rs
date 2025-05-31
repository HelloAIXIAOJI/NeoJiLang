use chrono::prelude::*;
use serde_json::Value;
use crate::error::NjilError;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use super::BuiltinModule;

/// DateTime模块，提供日期和时间相关功能
pub struct DateTimeModule;

impl DateTimeModule {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuiltinModule for DateTimeModule {
    fn name(&self) -> &'static str {
        "datetime"
    }
    
    fn get_handlers(&self) -> Vec<&'static dyn StatementHandler> {
        vec![
            &DATE_HANDLER,
            &TIME_HANDLER,
        ]
    }
}

/// 获取当前日期处理器
pub struct DateHandler;

// 静态实例
pub static DATE_HANDLER: DateHandler = DateHandler;

impl StatementHandler for DateHandler {
    fn handle(&self, _interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        let local: DateTime<Local> = Local::now();
        
        // 检查是否提供了格式参数
        let format_str = if let Value::String(fmt) = value {
            fmt.clone()
        } else if let Value::Object(obj) = value {
            if let Some(Value::String(fmt)) = obj.get("format") {
                fmt.clone()
            } else {
                "%Y-%m-%d".to_string() // 默认格式 YYYY-MM-DD
            }
        } else {
            "%Y-%m-%d".to_string() // 默认格式 YYYY-MM-DD
        };
        
        // 格式化日期
        let formatted_date = local.format(&format_str).to_string();
        
        Ok(Value::String(formatted_date))
    }
    
    fn name(&self) -> &'static str {
        "datetime.date"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["date"]
    }
}

/// 获取当前时间处理器
pub struct TimeHandler;

// 静态实例
pub static TIME_HANDLER: TimeHandler = TimeHandler;

impl StatementHandler for TimeHandler {
    fn handle(&self, _interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        let local: DateTime<Local> = Local::now();
        
        // 检查是否提供了格式参数
        let format_str = if let Value::String(fmt) = value {
            fmt.clone()
        } else if let Value::Object(obj) = value {
            if let Some(Value::String(fmt)) = obj.get("format") {
                fmt.clone()
            } else {
                "%H:%M:%S".to_string() // 默认格式 HH:MM:SS
            }
        } else {
            "%H:%M:%S".to_string() // 默认格式 HH:MM:SS
        };
        
        // 格式化时间
        let formatted_time = local.format(&format_str).to_string();
        
        Ok(Value::String(formatted_time))
    }
    
    fn name(&self) -> &'static str {
        "datetime.time"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["time"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_date_default_format() {
        let mut interpreter = Interpreter::new();
        let result = DATE_HANDLER.handle(&mut interpreter, &Value::Null).unwrap();
        
        if let Value::String(date_str) = result {
            // 检查格式是否为 YYYY-MM-DD
            assert!(date_str.matches('-').count() == 2);
            assert_eq!(date_str.len(), 10);
        } else {
            panic!("Expected string result");
        }
    }
    
    #[test]
    fn test_time_default_format() {
        let mut interpreter = Interpreter::new();
        let result = TIME_HANDLER.handle(&mut interpreter, &Value::Null).unwrap();
        
        if let Value::String(time_str) = result {
            // 检查格式是否为 HH:MM:SS
            assert!(time_str.matches(':').count() == 2);
            assert_eq!(time_str.len(), 8);
        } else {
            panic!("Expected string result");
        }
    }
    
    #[test]
    fn test_date_custom_format() {
        let mut interpreter = Interpreter::new();
        let format = Value::String("%d/%m/%Y".to_string());
        let result = DATE_HANDLER.handle(&mut interpreter, &format).unwrap();
        
        if let Value::String(date_str) = result {
            // 检查格式是否为 DD/MM/YYYY
            assert!(date_str.matches('/').count() == 2);
        } else {
            panic!("Expected string result");
        }
    }
    
    #[test]
    fn test_time_custom_format() {
        let mut interpreter = Interpreter::new();
        let format = Value::String("%H-%M-%S".to_string());
        let result = TIME_HANDLER.handle(&mut interpreter, &format).unwrap();
        
        if let Value::String(time_str) = result {
            // 检查格式是否为 HH-MM-SS
            assert!(time_str.matches('-').count() == 2);
        } else {
            panic!("Expected string result");
        }
    }
} 