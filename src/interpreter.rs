use crate::error::NjilError;
use crate::types::{Function, NjilProgram};
use crate::statements;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct Interpreter {
    pub(crate) variables: HashMap<String, Value>,
    returning: bool,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            returning: false,
        }
    }

    pub fn load_file<P: AsRef<Path>>(&self, path: P) -> Result<NjilProgram, NjilError> {
        let content = fs::read_to_string(path)?;
        let program: NjilProgram = serde_json::from_str(&content)?;
        Ok(program)
    }

    pub fn execute(&mut self, program: &NjilProgram) -> Result<Value, NjilError> {
        // 我们现在只处理program部分，忽略import
        if let Some(main_fn) = program.program.functions.get("main") {
            self.execute_function(main_fn)
        } else {
            Err(NjilError::ExecutionError("找不到main函数".to_string()))
        }
    }

    fn execute_function(&mut self, function: &Function) -> Result<Value, NjilError> {
        for statement in &function.body {
            match self.execute_statement(statement) {
                Ok(_) => {},
                Err(NjilError::ReturnValue(value)) => {
                    self.returning = true;
                    return Ok(value);
                }
                Err(e) => return Err(e),
            }
        }
        
        Err(NjilError::ExecutionError("函数没有返回值".to_string()))
    }

    fn execute_statement(&mut self, statement: &Value) -> Result<Value, NjilError> {
        // 使用语句处理函数处理语句
        statements::handle_statement(self, statement)
    }

    pub fn evaluate_value(&mut self, value: &Value) -> Result<Value, NjilError> {
        match value {
            Value::String(_) => Ok(value.clone()),
            Value::Object(_) => {
                // 如果是对象，尝试执行它
                self.execute_statement(value)
            }
            Value::Array(_) => Ok(value.clone()),
            Value::Null => Ok(value.clone()),
            Value::Bool(_) => Ok(value.clone()),
            Value::Number(_) => Ok(value.clone()),
        }
    }

    pub fn value_to_string(&self, value: &Value) -> String {
        match value {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => "null".to_string(),
            Value::Array(arr) => {
                let mut result = String::new();
                for (i, item) in arr.iter().enumerate() {
                    if i > 0 {
                        result.push(' ');
                    }
                    result.push_str(&self.value_to_string(item));
                }
                result
            }
            Value::Object(_) => "[对象]".to_string(),
        }
    }
    
    #[allow(dead_code)]
    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
    
    #[allow(dead_code)]
    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    /// 检查是否正在返回
    pub fn is_returning(&self) -> bool {
        self.returning
    }
    
    /// 设置返回状态
    pub fn set_returning(&mut self, returning: bool) {
        self.returning = returning;
    }
}

/// 从文件加载并执行NJIL程序
pub fn run_file<P: AsRef<Path>>(file_path: P) -> Result<Value, NjilError> {
    let mut interpreter = Interpreter::new();
    let program = interpreter.load_file(file_path)?;
    interpreter.execute(&program)
} 