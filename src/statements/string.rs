use crate::error::NjilError;
use serde_json::Value;
use crate::interpreter::Interpreter;
use crate::errortip;
use super::StatementHandler;

/// 字符串连接处理器，同时处理string.concat和txtlink
pub struct StringConcatHandler;

// 静态实例
pub static STRING_CONCAT_HANDLER: StringConcatHandler = StringConcatHandler;

impl StringConcatHandler {
    pub fn concat_strings(interpreter: &mut Interpreter, parts: &[Value]) -> Result<Value, NjilError> {
        let mut result = String::new();
        for part in parts {
            let evaluated = interpreter.evaluate_value(part)?;
            result.push_str(&interpreter.value_to_string(&evaluated));
        }
        Ok(Value::String(result))
    }
}

impl StatementHandler for StringConcatHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Array(parts) = value {
            Self::concat_strings(interpreter, parts)
        } else {
            Err(NjilError::ExecutionError(errortip::string::concat_requires_array().to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "string.concat"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["string.concat", "txtlink"]
    }
}

/// 字符串分割处理器
pub struct StringSplitHandler;

// 静态实例
pub static STRING_SPLIT_HANDLER: StringSplitHandler = StringSplitHandler;

impl StatementHandler for StringSplitHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Object(obj) = value {
            // 获取要分割的字符串
            if !obj.contains_key("string") {
                return Err(NjilError::ExecutionError("string.split需要string参数".to_string()));
            }
            let string_value = interpreter.evaluate_value(obj.get("string").unwrap())?;
            let string = match string_value {
                Value::String(s) => s,
                _ => interpreter.value_to_string(&string_value),
            };
            
            // 获取分隔符
            if !obj.contains_key("separator") {
                return Err(NjilError::ExecutionError("string.split需要separator参数".to_string()));
            }
            let separator_value = interpreter.evaluate_value(obj.get("separator").unwrap())?;
            let separator = match separator_value {
                Value::String(s) => s,
                _ => interpreter.value_to_string(&separator_value),
            };
            
            // 分割字符串并返回数组
            let parts: Vec<Value> = string.split(&separator)
                .map(|s| Value::String(s.to_string()))
                .collect();
            
            Ok(Value::Array(parts))
        } else {
            Err(NjilError::ExecutionError("string.split需要一个对象参数".to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "string.split"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec![]
    }
}

/// 字符串替换处理器
pub struct StringReplaceHandler;

// 静态实例
pub static STRING_REPLACE_HANDLER: StringReplaceHandler = StringReplaceHandler;

impl StatementHandler for StringReplaceHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Object(obj) = value {
            // 获取原始字符串
            if !obj.contains_key("string") {
                return Err(NjilError::ExecutionError("string.replace需要string参数".to_string()));
            }
            let string_value = interpreter.evaluate_value(obj.get("string").unwrap())?;
            let string = match string_value {
                Value::String(s) => s,
                _ => interpreter.value_to_string(&string_value),
            };
            
            // 获取要替换的内容
            if !obj.contains_key("old") {
                return Err(NjilError::ExecutionError("string.replace需要old参数".to_string()));
            }
            let old_value = interpreter.evaluate_value(obj.get("old").unwrap())?;
            let old = match old_value {
                Value::String(s) => s,
                _ => interpreter.value_to_string(&old_value),
            };
            
            // 获取新内容
            if !obj.contains_key("new") {
                return Err(NjilError::ExecutionError("string.replace需要new参数".to_string()));
            }
            let new_value = interpreter.evaluate_value(obj.get("new").unwrap())?;
            let new = match new_value {
                Value::String(s) => s,
                _ => interpreter.value_to_string(&new_value),
            };
            
            // 执行替换（默认替换所有匹配）
            let result = string.replace(&old, &new);
            
            Ok(Value::String(result))
        } else {
            Err(NjilError::ExecutionError("string.replace需要一个对象参数".to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "string.replace"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec![]
    }
}

/// 字符串修剪处理器
pub struct StringTrimHandler;

// 静态实例
pub static STRING_TRIM_HANDLER: StringTrimHandler = StringTrimHandler;

impl StatementHandler for StringTrimHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        // 支持直接传入字符串或包含string属性的对象
        let input_string = match value {
            Value::String(_) => {
                // 直接评估字符串值
                let evaluated = interpreter.evaluate_value(value)?;
                match evaluated {
                    Value::String(s) => s,
                    _ => interpreter.value_to_string(&evaluated),
                }
            },
            Value::Object(obj) => {
                if !obj.contains_key("string") {
                    return Err(NjilError::ExecutionError("string.trim需要string参数".to_string()));
                }
                let string_value = interpreter.evaluate_value(obj.get("string").unwrap())?;
                match string_value {
                    Value::String(s) => s,
                    _ => interpreter.value_to_string(&string_value),
                }
            },
            _ => return Err(NjilError::ExecutionError("string.trim需要字符串参数或包含string属性的对象".to_string())),
        };
        
        // 执行修剪操作
        let result = input_string.trim().to_string();
        
        Ok(Value::String(result))
    }
    
    fn name(&self) -> &'static str {
        "string.trim"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec![]
    }
}

/// 字符串格式化处理器
pub struct StringFormatHandler;

// 静态实例
pub static STRING_FORMAT_HANDLER: StringFormatHandler = StringFormatHandler;

impl StatementHandler for StringFormatHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Object(obj) = value {
            // 获取格式字符串
            if !obj.contains_key("template") {
                return Err(NjilError::ExecutionError("string.format需要template参数".to_string()));
            }
            let template_value = interpreter.evaluate_value(obj.get("template").unwrap())?;
            let template = match template_value {
                Value::String(s) => s,
                _ => interpreter.value_to_string(&template_value),
            };
            
            // 获取参数对象
            if !obj.contains_key("params") {
                return Err(NjilError::ExecutionError("string.format需要params参数".to_string()));
            }
            let params_value = interpreter.evaluate_value(obj.get("params").unwrap())?;
            
            if let Value::Object(params) = params_value {
                // 使用参数替换模板中的占位符 {name}
                let mut result = template.clone();
                
                for (key, val) in params {
                    let placeholder = format!("{{{}}}", key);
                    
                    // 首先评估参数值，确保变量被解析
                    let evaluated_val = interpreter.evaluate_value(&val)?;
                    let value_str = interpreter.value_to_string(&evaluated_val);
                    
                    result = result.replace(&placeholder, &value_str);
                }
                
                Ok(Value::String(result))
            } else {
                Err(NjilError::ExecutionError("string.format的params必须是对象".to_string()))
            }
        } else {
            Err(NjilError::ExecutionError("string.format需要一个对象参数".to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "string.format"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_string_split() {
        let mut interpreter = Interpreter::new();
        let handler = StringSplitHandler;
        
        // 测试基本分割
        let value = json!({
            "string": "a,b,c",
            "separator": ","
        });
        let result = handler.handle(&mut interpreter, &value).unwrap();
        assert_eq!(result, json!(["a", "b", "c"]));
        
        // 测试空分隔符
        let value = json!({
            "string": "abc",
            "separator": ""
        });
        let result = handler.handle(&mut interpreter, &value).unwrap();
        assert_eq!(result, json!(["a", "b", "c"]));
    }
    
    #[test]
    fn test_string_replace() {
        let mut interpreter = Interpreter::new();
        let handler = StringReplaceHandler;
        
        // 测试基本替换
        let value = json!({
            "string": "hello world",
            "old": "world",
            "new": "NeoJiLang"
        });
        let result = handler.handle(&mut interpreter, &value).unwrap();
        assert_eq!(result, json!("hello NeoJiLang"));
        
        // 测试替换所有匹配
        let value = json!({
            "string": "hello hello hello",
            "old": "hello",
            "new": "hi"
        });
        let result = handler.handle(&mut interpreter, &value).unwrap();
        assert_eq!(result, json!("hi hi hi"));
    }
    
    #[test]
    fn test_string_trim() {
        let mut interpreter = Interpreter::new();
        let handler = StringTrimHandler;
        
        // 测试直接传入字符串
        let value = json!("  hello  ");
        let result = handler.handle(&mut interpreter, &value).unwrap();
        assert_eq!(result, json!("hello"));
        
        // 测试使用对象参数
        let value = json!({
            "string": "  hello world  "
        });
        let result = handler.handle(&mut interpreter, &value).unwrap();
        assert_eq!(result, json!("hello world"));
    }
    
    #[test]
    fn test_string_format() {
        let mut interpreter = Interpreter::new();
        let handler = StringFormatHandler;
        
        // 测试基本格式化
        let value = json!({
            "template": "Hello, {name}! Your score is {score}.",
            "params": {
                "name": "User",
                "score": 100
            }
        });
        let result = handler.handle(&mut interpreter, &value).unwrap();
        assert_eq!(result, json!("Hello, User! Your score is 100."));
        
        // 测试不存在的参数（保留占位符）
        let value = json!({
            "template": "Hello, {name}! {missing}",
            "params": {
                "name": "User"
            }
        });
        let result = handler.handle(&mut interpreter, &value).unwrap();
        assert_eq!(result, json!("Hello, User! {missing}"));
    }
} 