use crate::error::NjilError;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use serde_json::Value;
use std::collections::HashMap;
use super::BuiltinModule;

/// Shell模块，提供终端控制功能
pub struct ShellModule {
    handlers: Vec<&'static dyn StatementHandler>,
}

impl ShellModule {
    pub fn new() -> Self {
        Self {
            handlers: vec![
                &COLOR_HANDLER,
                &CLEAR_LINE_HANDLER,
                &WRITE_HANDLER,
                &WRITE_LINE_HANDLER,
                &OVERWRITE_HANDLER,
                &STYLE_HANDLER,
            ],
        }
    }
}

impl BuiltinModule for ShellModule {
    fn name(&self) -> &'static str {
        "shell"
    }

    fn get_handlers(&self) -> Vec<&'static dyn StatementHandler> {
        self.handlers.clone()
    }

    fn initialize(&self) -> fn(&mut Interpreter) -> Result<(), NjilError> {
        |_| Ok(())
    }
}

// ANSI颜色代码
struct AnsiColor;

impl AnsiColor {
    // 前景色
    const FG_BLACK: &'static str = "\x1b[30m";
    const FG_RED: &'static str = "\x1b[31m";
    const FG_GREEN: &'static str = "\x1b[32m";
    const FG_YELLOW: &'static str = "\x1b[33m";
    const FG_BLUE: &'static str = "\x1b[34m";
    const FG_MAGENTA: &'static str = "\x1b[35m";
    const FG_CYAN: &'static str = "\x1b[36m";
    const FG_WHITE: &'static str = "\x1b[37m";
    const FG_DEFAULT: &'static str = "\x1b[39m";

    // 背景色
    const BG_BLACK: &'static str = "\x1b[40m";
    const BG_RED: &'static str = "\x1b[41m";
    const BG_GREEN: &'static str = "\x1b[42m";
    const BG_YELLOW: &'static str = "\x1b[43m";
    const BG_BLUE: &'static str = "\x1b[44m";
    const BG_MAGENTA: &'static str = "\x1b[45m";
    const BG_CYAN: &'static str = "\x1b[46m";
    const BG_WHITE: &'static str = "\x1b[47m";
    const BG_DEFAULT: &'static str = "\x1b[49m";

    // 样式
    const STYLE_BOLD: &'static str = "\x1b[1m";
    const STYLE_UNDERLINE: &'static str = "\x1b[4m";
    const STYLE_BLINK: &'static str = "\x1b[5m";
    const STYLE_RESET: &'static str = "\x1b[0m";

    // 清除和光标控制
    const CLEAR_LINE: &'static str = "\r\x1b[K";

    // 获取前景色代码
    fn get_fg_color(color: &str) -> &'static str {
        match color.to_lowercase().as_str() {
            "black" => Self::FG_BLACK,
            "red" => Self::FG_RED,
            "green" => Self::FG_GREEN,
            "yellow" => Self::FG_YELLOW,
            "blue" => Self::FG_BLUE,
            "magenta" => Self::FG_MAGENTA,
            "cyan" => Self::FG_CYAN,
            "white" => Self::FG_WHITE,
            _ => Self::FG_DEFAULT,
        }
    }

    // 获取背景色代码
    fn get_bg_color(color: &str) -> &'static str {
        match color.to_lowercase().as_str() {
            "black" => Self::BG_BLACK,
            "red" => Self::BG_RED,
            "green" => Self::BG_GREEN,
            "yellow" => Self::BG_YELLOW,
            "blue" => Self::BG_BLUE,
            "magenta" => Self::BG_MAGENTA,
            "cyan" => Self::BG_CYAN,
            "white" => Self::BG_WHITE,
            _ => Self::BG_DEFAULT,
        }
    }
}

/// 语句处理器：设置文本颜色
pub struct ColorHandler;

// 静态实例
pub static COLOR_HANDLER: ColorHandler = ColorHandler;

impl StatementHandler for ColorHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Object(params) = value {
            // 获取文本内容
            let text = match params.get("text") {
                Some(text_value) => {
                    let evaluated_text = interpreter.evaluate_value(text_value)?;
                    interpreter.value_to_string(&evaluated_text)
                },
                None => return Err(NjilError::ExecutionError("shell.color需要text参数".to_string())),
            };

            // 获取前景色
            let fg_color = if let Some(fg_value) = params.get("fg") {
                let evaluated_fg = interpreter.evaluate_value(fg_value)?;
                let fg_str = interpreter.value_to_string(&evaluated_fg);
                AnsiColor::get_fg_color(&fg_str)
            } else {
                AnsiColor::FG_DEFAULT
            };

            // 获取背景色
            let bg_color = if let Some(bg_value) = params.get("bg") {
                let evaluated_bg = interpreter.evaluate_value(bg_value)?;
                let bg_str = interpreter.value_to_string(&evaluated_bg);
                AnsiColor::get_bg_color(&bg_str)
            } else {
                AnsiColor::BG_DEFAULT
            };

            // 判断是否加粗
            let bold = if let Some(bold_value) = params.get("bold") {
                let evaluated_bold = interpreter.evaluate_value(bold_value)?;
                match evaluated_bold {
                    Value::Bool(b) => b,
                    _ => false,
                }
            } else {
                false
            };

            // 拼接ANSI代码和文本
            let mut result = String::new();
            result.push_str(fg_color);
            result.push_str(bg_color);
            if bold {
                result.push_str(AnsiColor::STYLE_BOLD);
            }
            result.push_str(&text);
            result.push_str(AnsiColor::STYLE_RESET);

            // 输出到控制台
            print!("{}", result);
            
            Ok(Value::Null)
        } else {
            Err(NjilError::ExecutionError("shell.color需要一个对象作为参数".to_string()))
        }
    }

    fn name(&self) -> &'static str {
        "shell.color"
    }
}

/// 语句处理器：清除当前行
pub struct ClearLineHandler;

// 静态实例
pub static CLEAR_LINE_HANDLER: ClearLineHandler = ClearLineHandler;

impl StatementHandler for ClearLineHandler {
    fn handle(&self, _interpreter: &mut Interpreter, _value: &Value) -> Result<Value, NjilError> {
        // 清除当前行并将光标移到行首
        print!("{}", AnsiColor::CLEAR_LINE);
        
        Ok(Value::Null)
    }

    fn name(&self) -> &'static str {
        "shell.clear_line"
    }
}

/// 语句处理器：写入文本（不换行）
pub struct WriteHandler;

// 静态实例
pub static WRITE_HANDLER: WriteHandler = WriteHandler;

impl StatementHandler for WriteHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        // 评估文本值
        let evaluated_value = interpreter.evaluate_value(value)?;
        let text = interpreter.value_to_string(&evaluated_value);
        
        // 输出到控制台（不换行）
        print!("{}", text);
        
        Ok(Value::Null)
    }

    fn name(&self) -> &'static str {
        "shell.write"
    }
}

/// 语句处理器：写入一行文本（换行）
pub struct WriteLineHandler;

// 静态实例
pub static WRITE_LINE_HANDLER: WriteLineHandler = WriteLineHandler;

impl StatementHandler for WriteLineHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        // 评估文本值
        let evaluated_value = interpreter.evaluate_value(value)?;
        let text = interpreter.value_to_string(&evaluated_value);
        
        // 输出到控制台（换行）
        println!("{}", text);
        
        Ok(Value::Null)
    }

    fn name(&self) -> &'static str {
        "shell.write_line"
    }
}

/// 语句处理器：覆盖当前行
pub struct OverwriteHandler;

// 静态实例
pub static OVERWRITE_HANDLER: OverwriteHandler = OverwriteHandler;

impl StatementHandler for OverwriteHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        // 评估文本值
        let evaluated_value = interpreter.evaluate_value(value)?;
        let text = interpreter.value_to_string(&evaluated_value);
        
        // 清除当前行并写入新文本
        print!("{}{}", AnsiColor::CLEAR_LINE, text);
        
        Ok(Value::Null)
    }

    fn name(&self) -> &'static str {
        "shell.overwrite"
    }
}

/// 语句处理器：设置文本样式
pub struct StyleHandler;

// 静态实例
pub static STYLE_HANDLER: StyleHandler = StyleHandler;

impl StatementHandler for StyleHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Object(params) = value {
            // 获取文本内容
            let text = match params.get("text") {
                Some(text_value) => {
                    let evaluated_text = interpreter.evaluate_value(text_value)?;
                    interpreter.value_to_string(&evaluated_text)
                },
                None => return Err(NjilError::ExecutionError("shell.style需要text参数".to_string())),
            };

            // 判断是否加粗
            let bold = if let Some(bold_value) = params.get("bold") {
                let evaluated_bold = interpreter.evaluate_value(bold_value)?;
                match evaluated_bold {
                    Value::Bool(b) => b,
                    _ => false,
                }
            } else {
                false
            };

            // 判断是否下划线
            let underline = if let Some(underline_value) = params.get("underline") {
                let evaluated_underline = interpreter.evaluate_value(underline_value)?;
                match evaluated_underline {
                    Value::Bool(b) => b,
                    _ => false,
                }
            } else {
                false
            };

            // 判断是否闪烁
            let blink = if let Some(blink_value) = params.get("blink") {
                let evaluated_blink = interpreter.evaluate_value(blink_value)?;
                match evaluated_blink {
                    Value::Bool(b) => b,
                    _ => false,
                }
            } else {
                false
            };

            // 拼接ANSI代码和文本
            let mut result = String::new();
            if bold {
                result.push_str(AnsiColor::STYLE_BOLD);
            }
            if underline {
                result.push_str(AnsiColor::STYLE_UNDERLINE);
            }
            if blink {
                result.push_str(AnsiColor::STYLE_BLINK);
            }
            result.push_str(&text);
            result.push_str(AnsiColor::STYLE_RESET);

            // 输出到控制台
            print!("{}", result);
            
            Ok(Value::Null)
        } else {
            Err(NjilError::ExecutionError("shell.style需要一个对象作为参数".to_string()))
        }
    }

    fn name(&self) -> &'static str {
        "shell.style"
    }
} 