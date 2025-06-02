use crate::error::NjilError;
use crate::interpreter::Interpreter;
use crate::statements::StatementHandler;
use serde_json::Value;
use std::env;

/// 处理列出所有环境变量的操作
pub struct EnvListHandler;

impl StatementHandler for EnvListHandler {
    fn name(&self) -> &'static str {
        "system.env.list"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["env.list"]
    }

    fn handle(&self, _interpreter: &mut Interpreter, _value: &Value) -> Result<Value, NjilError> {
        // 获取所有环境变量
        let mut env_vars = serde_json::Map::new();
        
        for (key, value) in env::vars() {
            env_vars.insert(key, Value::String(value));
        }
        
        // 返回环境变量对象
        Ok(Value::Object(env_vars))
    }
}

/// 静态实例
pub static ENV_LIST_HANDLER: EnvListHandler = EnvListHandler;

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    
    #[test]
    fn test_env_list() {
        let mut interpreter = Interpreter::new();
        
        // 设置测试环境变量
        env::set_var("NJIL_TEST_VAR1", "test_value1");
        env::set_var("NJIL_TEST_VAR2", "test_value2");
        
        // 获取环境变量列表
        let result = ENV_LIST_HANDLER.handle(
            &mut interpreter, 
            &Value::Null
        ).unwrap();
        
        // 验证返回值是对象
        if let Value::Object(env_vars) = result {
            // 验证测试环境变量存在
            assert_eq!(env_vars.get("NJIL_TEST_VAR1"), Some(&Value::String("test_value1".to_string())));
            assert_eq!(env_vars.get("NJIL_TEST_VAR2"), Some(&Value::String("test_value2".to_string())));
        } else {
            panic!("Expected object result");
        }
        
        // 清理测试环境变量
        env::remove_var("NJIL_TEST_VAR1");
        env::remove_var("NJIL_TEST_VAR2");
    }
} 