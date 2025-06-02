use crate::error::NjilError;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use serde_json::Value;
use std::env;

/// 处理设置环境变量的操作
pub struct EnvSetHandler;

impl StatementHandler for EnvSetHandler {
    fn name(&self) -> &'static str {
        "system.env.set"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["env.set"]
    }

    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        // 解析环境变量名称和值
        match value {
            Value::Object(obj) => {
                // 获取名称
                let name = if let Some(name_value) = obj.get("name") {
                    let evaluated = interpreter.evaluate_value(name_value)?;
                    if let Value::String(name) = evaluated {
                        name
                    } else {
                        return Err(NjilError::ExecutionError("name参数必须是字符串".to_string()));
                    }
                } else {
                    return Err(NjilError::ExecutionError("缺少name参数".to_string()));
                };
                
                // 获取值
                let value = if let Some(value_obj) = obj.get("value") {
                    let evaluated = interpreter.evaluate_value(value_obj)?;
                    match evaluated {
                        Value::String(s) => s,
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        Value::Null => "null".to_string(),
                        _ => return Err(NjilError::ExecutionError("value参数必须是简单类型（字符串、数字、布尔值或null）".to_string())),
                    }
                } else {
                    return Err(NjilError::ExecutionError("缺少value参数".to_string()));
                };
                
                // 设置环境变量
                env::set_var(&name, &value);
                
                // 返回成功
                Ok(Value::Bool(true))
            },
            _ => Err(NjilError::ExecutionError("参数必须是包含name和value字段的对象".to_string())),
        }
    }
}

/// 静态实例
pub static ENV_SET_HANDLER: EnvSetHandler = EnvSetHandler;

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    
    #[test]
    fn test_env_set() {
        let mut interpreter = Interpreter::new();
        
        // 创建测试参数
        let mut obj = serde_json::Map::new();
        obj.insert("name".to_string(), Value::String("NJIL_TEST_VAR".to_string()));
        obj.insert("value".to_string(), Value::String("test_value".to_string()));
        
        // 设置环境变量
        let result = ENV_SET_HANDLER.handle(
            &mut interpreter, 
            &Value::Object(obj)
        ).unwrap();
        
        assert_eq!(result, Value::Bool(true));
        
        // 验证环境变量已设置
        assert_eq!(env::var("NJIL_TEST_VAR").unwrap(), "test_value");
        
        // 测试设置数字值
        let mut obj = serde_json::Map::new();
        obj.insert("name".to_string(), Value::String("NJIL_TEST_NUM".to_string()));
        obj.insert("value".to_string(), Value::Number(serde_json::Number::from(42)));
        
        let result = ENV_SET_HANDLER.handle(
            &mut interpreter, 
            &Value::Object(obj)
        ).unwrap();
        
        assert_eq!(result, Value::Bool(true));
        assert_eq!(env::var("NJIL_TEST_NUM").unwrap(), "42");
        
        // 清理测试环境变量
        env::remove_var("NJIL_TEST_VAR");
        env::remove_var("NJIL_TEST_NUM");
    }
} 