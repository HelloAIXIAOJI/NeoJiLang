use std::collections::HashMap;
use crate::error::NjilError;
use crate::interpreter::Interpreter;

pub mod io;

/// 内置模块特性，定义了内置模块应该实现的方法
pub trait BuiltinModule {
    /// 获取模块名称
    fn name(&self) -> &'static str;
    
    /// 获取模块中所有的语句处理器
    fn get_handlers(&self) -> Vec<&'static dyn crate::statements::StatementHandler>;
    
    /// 初始化模块
    /// 由于借用冲突问题，这个方法不直接操作解释器
    /// 而是返回一个可由调用者执行的初始化函数
    fn initialize(&self) -> fn(&mut Interpreter) -> Result<(), NjilError> {
        |_| Ok(())
    }
}

/// 内置模块注册表，用于管理和查找内置模块
pub struct BuiltinModuleRegistry {
    modules: HashMap<String, Box<dyn BuiltinModule>>,
}

impl BuiltinModuleRegistry {
    /// 创建一个新的内置模块注册表
    pub fn new() -> Self {
        let mut registry = Self {
            modules: HashMap::new(),
        };
        
        // 注册所有内置模块
        registry.register_module(Box::new(io::IoModule::new()));
        
        registry
    }
    
    /// 注册一个内置模块
    pub fn register_module(&mut self, module: Box<dyn BuiltinModule>) {
        let name = module.name().to_string();
        self.modules.insert(name, module);
    }
    
    /// 获取指定名称的内置模块
    pub fn get_module(&self, name: &str) -> Option<&Box<dyn BuiltinModule>> {
        self.modules.get(name)
    }
    
    /// 获取所有内置模块的名称
    pub fn get_module_names(&self) -> Vec<&String> {
        self.modules.keys().collect()
    }
} 