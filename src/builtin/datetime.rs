use chrono::prelude::*;
use std::time::{SystemTime, Instant};
use std::collections::HashMap;
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
            &NOW_HANDLER,
            &FORMAT_HANDLER,
            &PARSE_HANDLER,
            &MEASURE_HANDLER,
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

/// 获取当前系统时间戳处理器
pub struct NowHandler;

// 静态实例
pub static NOW_HANDLER: NowHandler = NowHandler;

impl StatementHandler for NowHandler {
    fn handle(&self, _interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        // 检查是否指定了单位
        let unit = if let Value::Object(obj) = value {
            if let Some(Value::String(unit_str)) = obj.get("unit") {
                unit_str.as_str()
            } else {
                "ms" // 默认为毫秒
            }
        } else {
            "ms" // 默认为毫秒
        };
        
        // 获取当前系统时间
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| NjilError::ExecutionError(format!("获取系统时间失败: {}", e)))?;
        
        // 根据指定单位返回不同的时间戳
        match unit {
            "s" => Ok(Value::Number(serde_json::Number::from_f64(now.as_secs_f64()).unwrap())),
            "ms" => Ok(Value::Number(serde_json::Number::from(now.as_millis() as u64))),
            "us" | "µs" => Ok(Value::Number(serde_json::Number::from(now.as_micros() as u64))),
            "ns" => Ok(Value::Number(serde_json::Number::from(now.as_nanos() as u64))),
            _ => Err(NjilError::ExecutionError(format!("不支持的时间单位: {}", unit))),
        }
    }
    
    fn name(&self) -> &'static str {
        "datetime.now"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["now"]
    }
}

/// 格式化时间戳处理器
pub struct FormatHandler;

// 静态实例
pub static FORMAT_HANDLER: FormatHandler = FormatHandler;

impl StatementHandler for FormatHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Object(obj) = value {
            // 获取时间戳
            let timestamp = if let Some(timestamp_value) = obj.get("timestamp") {
                let evaluated = interpreter.evaluate_value(timestamp_value)?;
                if let Value::Number(num) = evaluated {
                    match num.as_f64() {
                        Some(n) => n,
                        None => return Err(NjilError::ExecutionError("无法将时间戳转换为浮点数".to_string())),
                    }
                } else if let Value::String(s) = evaluated {
                    match s.parse::<f64>() {
                        Ok(n) => n,
                        Err(_) => return Err(NjilError::ExecutionError("无法将字符串时间戳转换为浮点数".to_string())),
                    }
                } else {
                    return Err(NjilError::ExecutionError("timestamp参数必须是数字或数字字符串".to_string()));
                }
            } else {
                return Err(NjilError::ExecutionError("缺少timestamp参数".to_string()));
            };
            
            // 获取单位
            let unit = if let Some(Value::String(unit_str)) = obj.get("unit") {
                unit_str.as_str()
            } else {
                "s" // 默认为秒
            };
            
            // 获取格式
            let format_str = if let Some(Value::String(fmt)) = obj.get("format") {
                fmt.as_str()
            } else {
                "%Y-%m-%d %H:%M:%S" // 默认格式
            };
            
            // 转换时间戳为DateTime
            let datetime = match unit {
                "s" => {
                    let secs = timestamp as i64;
                    let nanos = ((timestamp - secs as f64) * 1_000_000_000.0) as u32;
                    match DateTime::from_timestamp(secs, nanos) {
                        Some(dt) => dt,
                        None => return Err(NjilError::ExecutionError("无效的时间戳".to_string())),
                    }
                },
                "ms" => {
                    let secs = (timestamp / 1000.0) as i64;
                    let nanos = ((timestamp / 1000.0 - secs as f64) * 1_000_000_000.0) as u32;
                    match DateTime::from_timestamp(secs, nanos) {
                        Some(dt) => dt,
                        None => return Err(NjilError::ExecutionError("无效的时间戳".to_string())),
                    }
                },
                _ => return Err(NjilError::ExecutionError(format!("不支持的时间单位: {}", unit))),
            };
            
            // 格式化为字符串
            let datetime_local = datetime.with_timezone(&Local);
            let formatted = datetime_local.format(format_str).to_string();
            
            Ok(Value::String(formatted))
        } else {
            Err(NjilError::ExecutionError("参数必须是包含timestamp字段的对象".to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "datetime.format"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["format"]
    }
}

/// 解析时间字符串处理器
pub struct ParseHandler;

// 静态实例
pub static PARSE_HANDLER: ParseHandler = ParseHandler;

impl StatementHandler for ParseHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Object(obj) = value {
            // 获取时间字符串
            let time_str = if let Some(str_value) = obj.get("time") {
                let evaluated = interpreter.evaluate_value(str_value)?;
                if let Value::String(s) = evaluated {
                    s
                } else {
                    return Err(NjilError::ExecutionError("time参数必须是字符串".to_string()));
                }
            } else {
                return Err(NjilError::ExecutionError("缺少time参数".to_string()));
            };
            
            // 获取格式
            let format_str = if let Some(format_value) = obj.get("format") {
                let evaluated = interpreter.evaluate_value(format_value)?;
                if let Value::String(fmt) = evaluated {
                    fmt
                } else {
                    return Err(NjilError::ExecutionError("format参数必须是字符串".to_string()));
                }
            } else {
                "%Y-%m-%d %H:%M:%S".to_string() // 默认格式
            };
            
            // 获取返回单位
            let unit = if let Some(Value::String(unit_str)) = obj.get("unit") {
                unit_str.as_str()
            } else {
                "s" // 默认为秒
            };
            
            // 解析时间字符串
            let datetime = match NaiveDateTime::parse_from_str(&time_str, &format_str) {
                Ok(dt) => dt,
                Err(e) => return Err(NjilError::ExecutionError(format!("解析时间字符串失败: {}", e))),
            };
            
            // 转换为时间戳，使用UTC标准时间
            let utc_datetime = datetime.and_utc();
            let timestamp = utc_datetime.timestamp() as f64 + utc_datetime.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
            
            // 根据指定单位返回
            match unit {
                "s" => Ok(Value::Number(serde_json::Number::from_f64(timestamp).unwrap())),
                "ms" => Ok(Value::Number(serde_json::Number::from_f64(timestamp * 1000.0).unwrap())),
                "us" | "µs" => Ok(Value::Number(serde_json::Number::from_f64(timestamp * 1_000_000.0).unwrap())),
                "ns" => Ok(Value::Number(serde_json::Number::from_f64(timestamp * 1_000_000_000.0).unwrap())),
                _ => Err(NjilError::ExecutionError(format!("不支持的时间单位: {}", unit))),
            }
        } else {
            Err(NjilError::ExecutionError("参数必须是包含time字段的对象".to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "datetime.parse"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["parse"]
    }
}

/// 测量代码执行时间处理器
pub struct MeasureHandler;

// 静态实例
pub static MEASURE_HANDLER: MeasureHandler = MeasureHandler;

// 存储测量点的全局变量
thread_local! {
    static MEASURE_POINTS: std::cell::RefCell<HashMap<String, Instant>> = std::cell::RefCell::new(HashMap::new());
}

impl StatementHandler for MeasureHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Object(obj) = value {
            // 获取操作类型
            let action = if let Some(Value::String(action_str)) = obj.get("action") {
                action_str.as_str()
            } else {
                return Err(NjilError::ExecutionError("缺少action参数".to_string()));
            };
            
            // 获取测量点名称
            let name = if let Some(name_value) = obj.get("name") {
                let evaluated = interpreter.evaluate_value(name_value)?;
                if let Value::String(name_str) = evaluated {
                    name_str
                } else {
                    return Err(NjilError::ExecutionError("name参数必须是字符串".to_string()));
                }
            } else {
                "default".to_string() // 默认测量点名称
            };
            
            match action {
                "start" => {
                    // 开始计时
                    MEASURE_POINTS.with(|points| {
                        points.borrow_mut().insert(name, Instant::now());
                    });
                    Ok(Value::Null)
                },
                "end" => {
                    // 结束计时并返回耗时
                    MEASURE_POINTS.with(|points| {
                        let mut points_mut = points.borrow_mut();
                        if let Some(start_time) = points_mut.remove(&name) {
                            let elapsed = start_time.elapsed();
                            
                            // 获取返回单位
                            let unit = if let Some(Value::String(unit_str)) = obj.get("unit") {
                                unit_str.as_str()
                            } else {
                                "ms" // 默认为毫秒
                            };
                            
                            // 根据指定单位返回
                            match unit {
                                "s" => Ok(Value::Number(serde_json::Number::from_f64(elapsed.as_secs_f64()).unwrap())),
                                "ms" => Ok(Value::Number(serde_json::Number::from(elapsed.as_millis() as u64))),
                                "us" | "µs" => Ok(Value::Number(serde_json::Number::from(elapsed.as_micros() as u64))),
                                "ns" => Ok(Value::Number(serde_json::Number::from(elapsed.as_nanos() as u64))),
                                _ => Err(NjilError::ExecutionError(format!("不支持的时间单位: {}", unit))),
                            }
                        } else {
                            Err(NjilError::ExecutionError(format!("未找到测量点: {}", name)))
                        }
                    })
                },
                "reset" => {
                    // 重置指定的测量点
                    MEASURE_POINTS.with(|points| {
                        let mut points_mut = points.borrow_mut();
                        points_mut.remove(&name);
                    });
                    Ok(Value::Null)
                },
                "clear" => {
                    // 清除所有测量点
                    MEASURE_POINTS.with(|points| {
                        points.borrow_mut().clear();
                    });
                    Ok(Value::Null)
                },
                _ => Err(NjilError::ExecutionError(format!("不支持的操作: {}", action))),
            }
        } else {
            Err(NjilError::ExecutionError("参数必须是包含action字段的对象".to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "datetime.measure"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["measure"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    
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
    
    #[test]
    fn test_now_handler() {
        let mut interpreter = Interpreter::new();
        let result = NOW_HANDLER.handle(&mut interpreter, &Value::Null).unwrap();
        
        if let Value::Number(_) = result {
            // 成功获取时间戳
        } else {
            panic!("Expected number result");
        }
        
        // 测试不同单位
        let mut obj = serde_json::Map::new();
        obj.insert("unit".to_string(), Value::String("s".to_string()));
        let result = NOW_HANDLER.handle(&mut interpreter, &Value::Object(obj)).unwrap();
        
        if let Value::Number(_) = result {
            // 成功获取秒级时间戳
        } else {
            panic!("Expected number result");
        }
    }
    
    #[test]
    fn test_format_handler() {
        let mut interpreter = Interpreter::new();
        let mut obj = serde_json::Map::new();
        obj.insert("timestamp".to_string(), Value::Number(serde_json::Number::from(1622505600))); // 2021-06-01 00:00:00
        obj.insert("format".to_string(), Value::String("%Y-%m-%d".to_string()));
        
        let result = FORMAT_HANDLER.handle(&mut interpreter, &Value::Object(obj)).unwrap();
        
        if let Value::String(formatted) = result {
            assert!(formatted.contains("2021"));
        } else {
            panic!("Expected string result");
        }
    }
    
    #[test]
    fn test_parse_handler() {
        let mut interpreter = Interpreter::new();
        let mut obj = serde_json::Map::new();
        obj.insert("time".to_string(), Value::String("2021-06-01 00:00:00".to_string()));
        obj.insert("format".to_string(), Value::String("%Y-%m-%d %H:%M:%S".to_string()));
        
        let result = PARSE_HANDLER.handle(&mut interpreter, &Value::Object(obj)).unwrap();
        
        if let Value::Number(timestamp) = result {
            assert!(timestamp.as_f64().unwrap() > 1600000000.0);
        } else {
            panic!("Expected number result");
        }
    }
    
    #[test]
    fn test_measure_handler() {
        let mut interpreter = Interpreter::new();
        
        // 开始计时
        let mut start_obj = serde_json::Map::new();
        start_obj.insert("action".to_string(), Value::String("start".to_string()));
        start_obj.insert("name".to_string(), Value::String("test".to_string()));
        
        let start_result = MEASURE_HANDLER.handle(&mut interpreter, &Value::Object(start_obj)).unwrap();
        assert_eq!(start_result, Value::Null);
        
        // 模拟耗时操作
        thread::sleep(Duration::from_millis(10));
        
        // 结束计时
        let mut end_obj = serde_json::Map::new();
        end_obj.insert("action".to_string(), Value::String("end".to_string()));
        end_obj.insert("name".to_string(), Value::String("test".to_string()));
        
        let end_result = MEASURE_HANDLER.handle(&mut interpreter, &Value::Object(end_obj)).unwrap();
        
        if let Value::Number(elapsed) = end_result {
            assert!(elapsed.as_u64().unwrap() >= 10);
        } else {
            panic!("Expected number result");
        }
    }
} 