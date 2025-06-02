use crate::error::NjilError;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use serde_json::Value;
use std::env;

/// 处理获取环境变量的操作
pub struct EnvGetHandler;

impl StatementHandler for EnvGetHandler {
    fn name(&self) -> &'static str {
        "system.env.get"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["env.get"]
    }

    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        // 解析环境变量名称
        let var_name = match value {
            Value::String(name) => name.clone(),
            Value::Object(obj) => {
                if let Some(name_value) = obj.get("name") {
                    let evaluated = interpreter.evaluate_value(name_value)?;
                    if let Value::String(name) = evaluated {
                        name
                    } else {
                        return Err(NjilError::ExecutionError("name参数必须是字符串".to_string()));
                    }
                } else {
                    return Err(NjilError::ExecutionError("缺少name参数".to_string()));
                }
            },
            _ => return Err(NjilError::ExecutionError("参数必须是字符串或包含name字段的对象".to_string())),
        };
        
        // 获取环境变量
        match env::var(&var_name) {
            Ok(value) => Ok(Value::String(value)),
            Err(_) => Ok(Value::Null), // 如果环境变量不存在，返回null而不是错误
        }
    }
}

/// 静态实例
pub static ENV_GET_HANDLER: EnvGetHandler = EnvGetHandler;

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    
    #[test]
    fn test_env_get() {
        let mut interpreter = Interpreter::new();
        
        // 设置测试环境变量
        env::set_var("NJIL_TEST_VAR", "test_value");
        
        // 测试字符串参数
        let result = ENV_GET_HANDLER.handle(
            &mut interpreter, 
            &Value::String("NJIL_TEST_VAR".to_string())
        ).unwrap();
        
        assert_eq!(result, Value::String("test_value".to_string()));
        
        // 测试对象参数
        let mut obj = serde_json::Map::new();
        obj.insert("name".to_string(), Value::String("NJIL_TEST_VAR".to_string()));
        
        let result = ENV_GET_HANDLER.handle(
            &mut interpreter, 
            &Value::Object(obj)
        ).unwrap();
        
        assert_eq!(result, Value::String("test_value".to_string()));
        
        // 测试不存在的环境变量
        let result = ENV_GET_HANDLER.handle(
            &mut interpreter, 
            &Value::String("NJIL_NON_EXISTENT_VAR".to_string())
        ).unwrap();
        
        assert_eq!(result, Value::Null);
        
        // 清理测试环境变量
        env::remove_var("NJIL_TEST_VAR");
    }
} 