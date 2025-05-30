use crate::error::NjilError;
use serde_json::Value;
use crate::interpreter::Interpreter;
use super::StatementHandler;
use crate::utils::type_convert;

/// 类型转换处理器
pub struct TypeConvertHandler;

// 静态实例
pub static TYPE_CONVERT_HANDLER: TypeConvertHandler = TypeConvertHandler;

impl StatementHandler for TypeConvertHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        if let Value::Object(convert_obj) = value {
            // 检查必要的字段
            if !convert_obj.contains_key("value") || !convert_obj.contains_key("type") {
                return Err(NjilError::ExecutionError(
                    "类型转换需要value和type字段".to_string()
                ));
            }
            
            // 获取要转换的值
            let source_value = interpreter.evaluate_value(convert_obj.get("value").unwrap())?;
            
            // 获取目标类型
            let target_type = match convert_obj.get("type") {
                Some(Value::String(t)) => t,
                _ => return Err(NjilError::ExecutionError("type字段必须是字符串".to_string())),
            };
            
            // 执行类型转换
            Ok(type_convert::convert_to_type(&source_value, target_type))
        } else {
            Err(NjilError::ExecutionError(
                "类型转换需要一个对象参数".to_string()
            ))
        }
    }
    
    fn name(&self) -> &'static str {
        "type.convert"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["type.convert", "convert"]
    }
}

/// 转换为布尔类型处理器
pub struct ToBoolHandler;

// 静态实例
pub static TO_BOOL_HANDLER: ToBoolHandler = ToBoolHandler;

impl StatementHandler for ToBoolHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        let evaluated = interpreter.evaluate_value(value)?;
        Ok(Value::Bool(type_convert::to_bool(&evaluated)))
    }
    
    fn name(&self) -> &'static str {
        "type.bool"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["type.bool", "to_bool", "bool"]
    }
}

/// 转换为数字类型处理器
pub struct ToNumberHandler;

// 静态实例
pub static TO_NUMBER_HANDLER: ToNumberHandler = ToNumberHandler;

impl StatementHandler for ToNumberHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        let evaluated = interpreter.evaluate_value(value)?;
        
        if let Some(num) = type_convert::to_number(&evaluated) {
            if let Some(json_num) = serde_json::Number::from_f64(num) {
                return Ok(Value::Number(json_num));
            }
        }
        
        // 无法转换为数字时返回null
        Ok(Value::Null)
    }
    
    fn name(&self) -> &'static str {
        "type.number"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["type.number", "to_number", "number"]
    }
}

/// 转换为字符串类型处理器
pub struct ToStringHandler;

// 静态实例
pub static TO_STRING_HANDLER: ToStringHandler = ToStringHandler;

impl StatementHandler for ToStringHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        let evaluated = interpreter.evaluate_value(value)?;
        Ok(Value::String(type_convert::to_string(&evaluated)))
    }
    
    fn name(&self) -> &'static str {
        "type.string"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["type.string", "to_string", "string"]
    }
}

/// 转换为数组类型处理器
pub struct ToArrayHandler;

// 静态实例
pub static TO_ARRAY_HANDLER: ToArrayHandler = ToArrayHandler;

impl StatementHandler for ToArrayHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        let evaluated = interpreter.evaluate_value(value)?;
        Ok(Value::Array(type_convert::to_array(&evaluated)))
    }
    
    fn name(&self) -> &'static str {
        "type.array"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["type.array", "to_array", "array"]
    }
}

/// 转换为对象类型处理器
pub struct ToObjectHandler;

// 静态实例
pub static TO_OBJECT_HANDLER: ToObjectHandler = ToObjectHandler;

impl StatementHandler for ToObjectHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        let evaluated = interpreter.evaluate_value(value)?;
        Ok(Value::Object(type_convert::to_object(&evaluated)))
    }
    
    fn name(&self) -> &'static str {
        "type.object"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["type.object", "to_object", "object"]
    }
}

/// 获取值的类型处理器
pub struct TypeOfHandler;

// 静态实例
pub static TYPE_OF_HANDLER: TypeOfHandler = TypeOfHandler;

impl StatementHandler for TypeOfHandler {
    fn handle(&self, interpreter: &mut Interpreter, value: &Value) -> Result<Value, NjilError> {
        let evaluated = interpreter.evaluate_value(value)?;
        
        let type_name = match evaluated {
            Value::Null => "null",
            Value::Bool(_) => "boolean",
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
        };
        
        Ok(Value::String(type_name.to_string()))
    }
    
    fn name(&self) -> &'static str {
        "type.of"
    }
    
    fn aliases(&self) -> Vec<&'static str> {
        vec!["type.of", "typeof"]
    }
} 