use std::fs;
use std::path::Path;
use serde_json::Value;
use crate::error::NjilError;
use crate::interpreter::Interpreter;

/// NJIS (NeoJi Script) 文件处理模块
/// NJIS 是NJIL的简化版本，使用JSON数组直接表示语句序列

/// 从.njis文件加载并执行脚本
pub fn run_njis_file<P: AsRef<Path>>(file_path: P) -> Result<Value, NjilError> {
    // 读取文件内容
    let content = fs::read_to_string(file_path)
        .map_err(|e| NjilError::IoError(e))?;
    
    // 解析NJIS文件内容
    let statements: Value = serde_json::from_str(&content)
        .map_err(|e| NjilError::ParseError(e))?;
    
    // 验证NJIS结构（必须是数组）
    if !statements.is_array() {
        return Err(NjilError::ParseStringError("NJIS文件必须是JSON数组".to_string()));
    }
    
    // 创建解释器并执行语句
    let mut interpreter = Interpreter::new();
    
    // NJIS不支持import，所以自动导入所有内置模块
    interpreter.import_all_builtin_modules()?;
    
    let mut result = Value::Null;
    
    if let Value::Array(statements) = statements {
        for statement in statements {
            result = interpreter.evaluate_value(&statement)?;
            
            // 如果遇到返回语句，提前结束执行
            if interpreter.is_returning() {
                break;
            }
        }
    }
    
    Ok(result)
}

/// 从字符串执行NJIS脚本
pub fn run_njis_str(content: &str) -> Result<Value, NjilError> {
    // 解析NJIS内容
    let statements: Value = serde_json::from_str(content)
        .map_err(|e| NjilError::ParseError(e))?;
    
    // 验证NJIS结构（必须是数组）
    if !statements.is_array() {
        return Err(NjilError::ParseStringError("NJIS字符串必须是JSON数组".to_string()));
    }
    
    // 创建解释器并执行语句
    let mut interpreter = Interpreter::new();
    
    // NJIS不支持import，所以自动导入所有内置模块
    interpreter.import_all_builtin_modules()?;
    
    let mut result = Value::Null;
    
    if let Value::Array(statements) = statements {
        for statement in statements {
            result = interpreter.evaluate_value(&statement)?;
            
            // 如果遇到返回语句，提前结束执行
            if interpreter.is_returning() {
                break;
            }
        }
    }
    
    Ok(result)
} 