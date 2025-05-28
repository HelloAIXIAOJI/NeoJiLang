use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct NjilProgram {
    pub import: Option<Vec<Value>>,
    pub program: Program,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Program {
    #[serde(flatten)]
    pub functions: HashMap<String, Function>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Function {
    pub body: Vec<Value>,
}
