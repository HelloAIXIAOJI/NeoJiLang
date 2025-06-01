use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NjilProgram {
    pub import: Option<Vec<Value>>,
    pub program: Program,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Program {
    #[serde(flatten)]
    pub functions: HashMap<String, Function>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Function {
    pub body: Vec<Value>,
}

/// NeoJi模块(NJIM)的结构定义
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NjimModule {
    /// 模块名称
    pub module: String,
    
    /// 可选的命名空间
    #[serde(default)]
    pub namespace: Option<String>,
    
    /// 可选的模块描述
    #[serde(default)]
    pub description: Option<String>,
    
    /// 可选的作者信息
    #[serde(default)]
    pub author: Option<String>,
    
    /// 可选的版本号
    #[serde(default)]
    pub version: Option<String>,
    
    /// 导出的内容
    pub exports: ModuleExports,
    
    /// 该模块导入的其他模块
    #[serde(default)]
    pub imports: Option<Vec<Value>>,
}

/// 模块导出的内容
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModuleExports {
    /// 导出的常量
    #[serde(default)]
    pub constants: Option<HashMap<String, Value>>,
    
    /// 导出的函数
    #[serde(default)]
    pub functions: Option<HashMap<String, Function>>,
    
    /// 导出的类型定义
    #[serde(default)]
    pub types: Option<HashMap<String, Value>>,
}
