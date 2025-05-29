use serde_json::Value;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum NjilError {
    IoError(std::io::Error),
    ParseError(serde_json::Error),
    ParseStringError(String),
    ExecutionError(String),
    ReturnValue(Value),
    LoopBreak,
    LoopContinue,
}

impl fmt::Display for NjilError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NjilError::IoError(err) => write!(f, "IO错误: {}", err),
            NjilError::ParseError(err) => write!(f, "解析错误: {}", err),
            NjilError::ParseStringError(msg) => write!(f, "解析错误: {}", msg),
            NjilError::ExecutionError(msg) => write!(f, "执行错误: {}", msg),
            NjilError::ReturnValue(_) => write!(f, "返回值"),
            NjilError::LoopBreak => write!(f, "循环中断"),
            NjilError::LoopContinue => write!(f, "循环继续"),
        }
    }
}

impl Error for NjilError {}

impl From<std::io::Error> for NjilError {
    fn from(err: std::io::Error) -> Self {
        NjilError::IoError(err)
    }
}

impl From<serde_json::Error> for NjilError {
    fn from(err: serde_json::Error) -> Self {
        NjilError::ParseError(err)
    }
}
