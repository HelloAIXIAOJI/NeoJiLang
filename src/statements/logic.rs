use crate::error::NjilError;
use serde_json::Value;
use crate::interpreter::Interpreter;
use crate::errortip;
use super::StatementHandler;

/// 逻辑与操作处理器
pub struct LogicAndHandler;

// 静态实例
pub static LOGIC_AND_HANDLER: LogicAndHandler = LogicAndHandler;

impl StatementHandler for LogicAndHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Array(operands) = value {
            if operands.is_empty() {
                return Err(NjilError::ExecutionError(
                    errortip::logic::and_requires_operands().to_string()
                ));
            }

            // 逻辑与：所有操作数都为真时结果为真
            for operand in operands {
                // 评估操作数
                let operand_value = match interpreter.evaluate_value(operand) {
                    Ok(value) => value,
                    Err(NjilError::ExecutionError(msg)) => {
                        // 检查是否是变量未定义错误
                        if msg.starts_with("未定义的变量:") {
                            Value::Null // 变量不存在时，视为null（假）
                        } else {
                            return Err(NjilError::ExecutionError(msg));
                        }
                    },
                    Err(e) => return Err(e),
                };

                // 检查操作数是否为真
                let is_true = match operand_value {
                    Value::Bool(b) => b,
                    Value::Number(n) => n.as_f64().unwrap_or(0.0) != 0.0,
                    Value::String(s) => !s.is_empty(),
                    Value::Array(a) => !a.is_empty(),
                    Value::Object(o) => !o.is_empty(),
                    Value::Null => false,
                };

                // 短路求值：如果有一个操作数为假，整个表达式为假
                if !is_true {
                    return Ok(Value::Bool(false));
                }
            }

            // 所有操作数都为真，结果为真
            Ok(Value::Bool(true))
        } else {
            Err(NjilError::ExecutionError(
                errortip::logic::and_requires_array().to_string()
            ))
        }
    }

    fn name(&self) -> &'static str {
        "logic.and"
    }
}

/// 逻辑或操作处理器
pub struct LogicOrHandler;

// 静态实例
pub static LOGIC_OR_HANDLER: LogicOrHandler = LogicOrHandler;

impl StatementHandler for LogicOrHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Array(operands) = value {
            if operands.is_empty() {
                return Err(NjilError::ExecutionError(
                    errortip::logic::or_requires_operands().to_string()
                ));
            }

            // 逻辑或：任一操作数为真时结果为真
            for operand in operands {
                // 评估操作数
                let operand_value = match interpreter.evaluate_value(operand) {
                    Ok(value) => value,
                    Err(NjilError::ExecutionError(msg)) => {
                        // 检查是否是变量未定义错误
                        if msg.starts_with("未定义的变量:") {
                            Value::Null // 变量不存在时，视为null（假）
                        } else {
                            return Err(NjilError::ExecutionError(msg));
                        }
                    },
                    Err(e) => return Err(e),
                };

                // 检查操作数是否为真
                let is_true = match operand_value {
                    Value::Bool(b) => b,
                    Value::Number(n) => n.as_f64().unwrap_or(0.0) != 0.0,
                    Value::String(s) => !s.is_empty(),
                    Value::Array(a) => !a.is_empty(),
                    Value::Object(o) => !o.is_empty(),
                    Value::Null => false,
                };

                // 短路求值：如果有一个操作数为真，整个表达式为真
                if is_true {
                    return Ok(Value::Bool(true));
                }
            }

            // 所有操作数都为假，结果为假
            Ok(Value::Bool(false))
        } else {
            Err(NjilError::ExecutionError(
                errortip::logic::or_requires_array().to_string()
            ))
        }
    }

    fn name(&self) -> &'static str {
        "logic.or"
    }
}

/// 逻辑非操作处理器
pub struct LogicNotHandler;

// 静态实例
pub static LOGIC_NOT_HANDLER: LogicNotHandler = LogicNotHandler;

impl StatementHandler for LogicNotHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        // 评估操作数
        let operand_value = match interpreter.evaluate_value(value) {
            Ok(value) => value,
            Err(NjilError::ExecutionError(msg)) => {
                // 检查是否是变量未定义错误
                if msg.starts_with("未定义的变量:") {
                    Value::Null // 变量不存在时，视为null（假）
                } else {
                    return Err(NjilError::ExecutionError(msg));
                }
            },
            Err(e) => return Err(e),
        };

        // 检查操作数是否为真
        let is_true = match operand_value {
            Value::Bool(b) => b,
            Value::Number(n) => n.as_f64().unwrap_or(0.0) != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            Value::Object(o) => !o.is_empty(),
            Value::Null => false,
        };

        // 返回逻辑非结果
        Ok(Value::Bool(!is_true))
    }

    fn name(&self) -> &'static str {
        "logic.not"
    }
} 