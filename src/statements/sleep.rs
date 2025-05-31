use crate::error::NjilError;
use serde_json::Value;
use crate::interpreter::Interpreter;
use crate::errortip;
use crate::debug_println;
use super::StatementHandler;
use std::thread;
use std::time::Duration;

/// 延时执行语句处理器
pub struct SleepHandler;

// 静态实例
pub static SLEEP_HANDLER: SleepHandler = SleepHandler;

impl StatementHandler for SleepHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        // 先评估传入的值，处理可能的变量引用
        let evaluated_value = interpreter.evaluate_value(value)?;
        
        // 解析延时时间
        let duration_ms = match &evaluated_value {
            // 数字形式，直接作为毫秒数
            Value::Number(n) => {
                if let Some(ms) = n.as_u64() {
                    ms
                } else if let Some(ms) = n.as_f64() {
                    if ms >= 0.0 {
                        ms as u64
                    } else {
                        return Err(NjilError::ExecutionError(errortip::sleep::duration_cannot_be_negative()));
                    }
                } else {
                    return Err(NjilError::ExecutionError(errortip::sleep::invalid_sleep_duration()));
                }
            },
            // 对象形式，可以指定时间单位
            Value::Object(obj) => {
                // 获取持续时间
                let duration = if let Some(duration_value) = obj.get("duration") {
                    // 评估持续时间，处理可能的变量引用
                    let evaluated_duration = interpreter.evaluate_value(duration_value)?;
                    match evaluated_duration {
                        Value::Number(n) => {
                            if let Some(d) = n.as_u64() {
                                d
                            } else if let Some(d) = n.as_f64() {
                                if d >= 0.0 {
                                    d as u64
                                } else {
                                    return Err(NjilError::ExecutionError(errortip::sleep::duration_cannot_be_negative()));
                                }
                            } else {
                                return Err(NjilError::ExecutionError(errortip::sleep::invalid_sleep_duration()));
                            }
                        },
                        _ => return Err(NjilError::ExecutionError(errortip::sleep::duration_must_be_number())),
                    }
                } else {
                    return Err(NjilError::ExecutionError("sleep对象必须包含duration字段".to_string()));
                };
                
                // 获取时间单位
                let unit = if let Some(unit_value) = obj.get("unit") {
                    // 评估单位，处理可能的变量引用
                    let evaluated_unit = interpreter.evaluate_value(unit_value)?;
                    match evaluated_unit {
                        Value::String(u) => u.clone(), // 克隆字符串以避免借用问题
                        _ => return Err(NjilError::ExecutionError("时间单位必须是字符串".to_string())),
                    }
                } else {
                    "ms".to_string() // 默认单位为毫秒
                };
                
                // 根据单位转换为毫秒
                match unit.as_str() {
                    "ms" => duration,
                    "s" => duration * 1000,
                    "m" => duration * 60 * 1000,
                    _ => return Err(NjilError::ExecutionError(errortip::sleep::invalid_time_unit(&unit))),
                }
            },
            // 其他类型，尝试转换为数字
            _ => {
                let str_value = interpreter.value_to_string(&evaluated_value);
                // 尝试解析为数字
                match str_value.parse::<u64>() {
                    Ok(ms) => ms,
                    Err(_) => return Err(NjilError::ExecutionError(errortip::sleep::invalid_sleep_duration())),
                }
            }
        };
        
        debug_println!("执行延时: {}毫秒", duration_ms);
        
        // 执行延时
        thread::sleep(Duration::from_millis(duration_ms));
        
        Ok(Value::Null)
    }
    
    fn name(&self) -> &'static str {
        "sleep"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["delay", "wait"]
    }
} 