use std::fs;
use std::io::{self, Write, Read};
use std::fs::OpenOptions;
use std::io::BufWriter;
use serde_json::Value;
use encoding_rs::GBK;
use crate::error::NjilError;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use super::BuiltinModule;

/// IO模块，提供文件读写和标准输入输出功能
pub struct IoModule;

impl IoModule {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuiltinModule for IoModule {
    fn name(&self) -> &'static str {
        "io"
    }
    
    fn get_handlers(&self) -> Vec<&'static dyn StatementHandler> {
        vec![
            &IO_READ_FILE_HANDLER,
            &IO_WRITE_FILE_HANDLER,
            &IO_READ_LINE_HANDLER,
            &IO_INPUT_HANDLER,
        ]
    }
}

/// 读取文件处理器
pub struct IoReadFileHandler;

// 静态实例
pub static IO_READ_FILE_HANDLER: IoReadFileHandler = IoReadFileHandler;

impl StatementHandler for IoReadFileHandler {
    fn handle(&self, _interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        // 检查参数类型
        match value {
            Value::String(file_path) => {
                // 简单形式：直接提供文件路径字符串
                // 读取文件内容（默认UTF-8编码）
                let content = fs::read_to_string(file_path)
                    .map_err(|e| NjilError::IoError(e))?;
                
                Ok(Value::String(content))
            },
            Value::Object(obj) => {
                // 高级形式：提供对象，包含path和可选的encoding
                let path = obj.get("path").ok_or_else(|| 
                    NjilError::ExecutionError("io.readFile需要一个path参数".to_string()))?;
                
                // 解析路径
                let path_str = if let Value::String(p) = path {
                    p
                } else {
                    return Err(NjilError::ExecutionError("io.readFile的path参数必须是字符串".to_string()));
                };
                
                // 检查是否指定了编码
                let encoding = if let Some(enc) = obj.get("encoding") {
                    if let Value::String(enc_str) = enc {
                        enc_str.clone()
                    } else {
                        return Err(NjilError::ExecutionError("io.readFile的encoding参数必须是字符串".to_string()));
                    }
                } else {
                    "utf8".to_string() // 默认使用UTF-8编码
                };
                
                // 根据编码类型选择读取方式
                match encoding.to_lowercase().as_str() {
                    "utf8" | "utf-8" => {
                        // 使用UTF-8编码读取
                        let content = fs::read_to_string(path_str)
                            .map_err(|e| NjilError::IoError(e))?;
                        
                        Ok(Value::String(content))
                    },
                    "gbk" | "gb2312" => {
                        // 读取文件为字节数组
                        let mut file = fs::File::open(path_str)
                            .map_err(|e| NjilError::IoError(e))?;
                        let mut bytes = Vec::new();
                        file.read_to_end(&mut bytes)
                            .map_err(|e| NjilError::IoError(e))?;
                        
                        // 使用encoding_rs库将GBK/GB2312转换为UTF-8
                        let (cow, _encoding_used, had_errors) = GBK.decode(&bytes);
                        if had_errors {
                            return Err(NjilError::ExecutionError("解码GBK/GB2312文件时发生错误".to_string()));
                        }
                        
                        Ok(Value::String(cow.into_owned()))
                    },
                    _ => {
                        Err(NjilError::ExecutionError(format!("不支持的编码类型: {}", encoding)))
                    }
                }
            },
            _ => {
                Err(NjilError::ExecutionError("io.readFile需要一个字符串参数作为文件路径，或者一个包含path的对象".to_string()))
            }
        }
    }
    
    fn name(&self) -> &'static str {
        "io.readFile"
    }
}

/// 写入文件处理器
pub struct IoWriteFileHandler;

// 静态实例
pub static IO_WRITE_FILE_HANDLER: IoWriteFileHandler = IoWriteFileHandler;

impl StatementHandler for IoWriteFileHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        // 期望参数是一个对象，包含path和content
        if let Value::Object(obj) = value {
            let path = obj.get("path").ok_or_else(|| 
                NjilError::ExecutionError("io.writeFile需要一个path参数".to_string()))?;
                
            let content = obj.get("content").ok_or_else(|| 
                NjilError::ExecutionError("io.writeFile需要一个content参数".to_string()))?;
            
            // 解析路径
            let path_str = if let Value::String(p) = path {
                p
            } else {
                return Err(NjilError::ExecutionError("io.writeFile的path参数必须是字符串".to_string()));
            };
            
            // 检查是否指定了编码
            let encoding = if let Some(enc) = obj.get("encoding") {
                if let Value::String(enc_str) = enc {
                    enc_str.clone()
                } else {
                    return Err(NjilError::ExecutionError("io.writeFile的encoding参数必须是字符串".to_string()));
                }
            } else {
                "utf8".to_string() // 默认使用UTF-8编码
            };
            
            // 使用解释器的内容解析方法处理内容
            let content_str = interpreter.parse_content(content)?;
            
            // 根据编码类型选择写入方式
            match encoding.to_lowercase().as_str() {
                "utf8" | "utf-8" => {
                    // 使用UTF-8编码写入
                    let file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open(path_str)
                        .map_err(|e| NjilError::IoError(e))?;
                        
                    let mut writer = BufWriter::new(file);
                    writer.write_all(content_str.as_bytes())
                        .map_err(|e| NjilError::IoError(e))?;
                    writer.flush()
                        .map_err(|e| NjilError::IoError(e))?;
                },
                "gbk" | "gb2312" => {
                    // 使用encoding_rs将UTF-8转换为GBK/GB2312
                    let (bytes, _encoding_used, had_errors) = GBK.encode(&content_str);
                    if had_errors {
                        return Err(NjilError::ExecutionError("编码为GBK/GB2312时发生错误".to_string()));
                    }
                    
                    // 写入文件
                    let file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open(path_str)
                        .map_err(|e| NjilError::IoError(e))?;
                        
                    let mut writer = BufWriter::new(file);
                    writer.write_all(&bytes)
                        .map_err(|e| NjilError::IoError(e))?;
                    writer.flush()
                        .map_err(|e| NjilError::IoError(e))?;
                },
                _ => {
                    return Err(NjilError::ExecutionError(format!("不支持的编码类型: {}", encoding)));
                }
            }
            
            Ok(Value::Bool(true))
        } else {
            Err(NjilError::ExecutionError("io.writeFile需要一个对象，包含path和content属性".to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "io.writeFile"
    }
}

/// 读取单行输入处理器
pub struct IoReadLineHandler;

// 静态实例
pub static IO_READ_LINE_HANDLER: IoReadLineHandler = IoReadLineHandler;

impl StatementHandler for IoReadLineHandler {
    fn handle(&self, _interpreter: &mut Interpreter, _value: &Value) -> Result<Value, NjilError> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .map_err(|e| NjilError::IoError(e))?;
        
        // 去除末尾的换行符
        if input.ends_with('\n') {
            input.pop();
            if input.ends_with('\r') {
                input.pop();
            }
        }
        
        Ok(Value::String(input))
    }
    
    fn name(&self) -> &'static str {
        "io.readLine"
    }
}

/// 输入处理器（带提示）
pub struct IoInputHandler;

// 静态实例
pub static IO_INPUT_HANDLER: IoInputHandler = IoInputHandler;

impl StatementHandler for IoInputHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        // 首先打印提示（如果有）
        if !value.is_null() {
            let prompt = interpreter.value_to_string(value);
            print!("{}", prompt);
            io::stdout().flush().map_err(|e| NjilError::IoError(e))?;
        }
        
        // 读取输入
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .map_err(|e| NjilError::IoError(e))?;
        
        // 去除末尾的换行符
        if input.ends_with('\n') {
            input.pop();
            if input.ends_with('\r') {
                input.pop();
            }
        }
        
        Ok(Value::String(input))
    }
    
    fn name(&self) -> &'static str {
        "io.input"
    }
} 