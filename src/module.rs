use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};
use serde_json::Value;
use crate::error::NjilError;
use crate::types::{NjimModule, Function};
use crate::debug_println;
use crate::preprocessor::Preprocessor;

/// 模块加载器，负责加载和管理NJIM模块
pub struct ModuleLoader {
    /// 已加载的模块缓存
    loaded_modules: HashMap<String, Arc<NjimModule>>,
    
    /// 模块搜索路径
    search_paths: Vec<PathBuf>,
}

impl ModuleLoader {
    /// 创建一个新的模块加载器
    pub fn new() -> Self {
        Self {
            loaded_modules: HashMap::new(),
            search_paths: vec![PathBuf::from("modules")], // 默认模块目录
        }
    }
    
    /// 添加模块搜索路径
    pub fn add_search_path<P: AsRef<Path>>(&mut self, path: P) {
        self.search_paths.push(path.as_ref().to_path_buf());
    }
    
    /// 加载模块
    pub fn load_module<P: AsRef<Path>>(&mut self, path: P) -> Result<Arc<NjimModule>, NjilError> {
        let path = path.as_ref();
        let module_path = self.resolve_module_path(path)?;
        let file_path_key = module_path.to_string_lossy().to_string();
        
        // 检查模块是否已加载（通过文件路径）
        if let Some(module) = self.loaded_modules.get(&file_path_key) {
            debug_println!("模块已加载(文件路径): {}", file_path_key);
            return Ok(module.clone());
        }
        
        debug_println!("加载模块: {}", file_path_key);
        
        // 使用预处理器处理文件内容，移除注释
        let processed_content = Preprocessor::preprocess_file(&module_path)?;
        
        // 解析模块内容
        let module: NjimModule = match serde_json::from_str(&processed_content) {
            Ok(m) => m,
            Err(e) => {
                return Err(NjilError::ExecutionError(
                    format!("解析NJIM文件失败: {}，请检查文件格式是否正确", e)
                ));
            }
        };
        
        // 获取模块名称和命名空间
        let module_name = module.module.clone();
        let namespace = module.namespace.clone().unwrap_or_else(|| module_name.clone());
        
        debug_println!("模块解析成功，模块名: {}, 命名空间: {}", module_name, namespace);
        
        if let Some(functions) = &module.exports.functions {
            debug_println!("模块导出的函数: {:?}", functions.keys().collect::<Vec<_>>());
        } else {
            debug_println!("模块没有导出函数");
        }
        
        // 验证模块结构
        self.validate_module(&module)?;
        
        // 处理模块依赖
        if let Some(imports) = &module.imports {
            self.process_module_imports(imports, &module_path)?;
        }
        
        // 缓存模块（使用命名空间作为键）
        let module_arc = Arc::new(module);
        
        // 同时使用文件路径和命名空间作为键存储模块
        self.loaded_modules.insert(file_path_key, module_arc.clone());
        self.loaded_modules.insert(namespace.clone(), module_arc.clone());
        
        debug_println!("模块已缓存，命名空间: {}", namespace);
        
        Ok(module_arc)
    }
    
    /// 验证模块结构
    fn validate_module(&self, module: &NjimModule) -> Result<(), NjilError> {
        // 模块名称不能为空
        if module.module.trim().is_empty() {
            return Err(NjilError::ExecutionError(
                "模块名称不能为空".to_string()
            ));
        }
        
        // 验证导出的函数都有返回值
        if let Some(functions) = &module.exports.functions {
            for (fn_name, function) in functions {
                if !self.function_has_return(function) {
                    return Err(NjilError::ExecutionError(
                        format!("模块'{}'中的函数'{}'没有返回值", module.module, fn_name)
                    ));
                }
            }
        }
        
        Ok(())
    }
    
    /// 检查函数是否有返回值
    fn function_has_return(&self, function: &Function) -> bool {
        for stmt in &function.body {
            if let Value::Object(obj) = stmt {
                if obj.contains_key("return") {
                    return true;
                }
            }
        }
        false
    }
    
    /// 处理模块导入
    fn process_module_imports(&mut self, imports: &[Value], current_path: &Path) -> Result<(), NjilError> {
        let parent_dir = current_path.parent().unwrap_or(Path::new("."));
        
        for import in imports {
            if let Value::String(import_path) = import {
                let resolved_path = if import_path.starts_with('/') {
                    // 绝对路径
                    PathBuf::from(import_path.trim_start_matches('/'))
                } else {
                    // 相对路径
                    parent_dir.join(import_path)
                };
                
                // 递归加载依赖模块
                self.load_module(resolved_path)?;
            } else {
                return Err(NjilError::ExecutionError(
                    "模块导入路径必须是字符串".to_string()
                ));
            }
        }
        
        Ok(())
    }
    
    /// 解析模块路径
    fn resolve_module_path(&self, path: &Path) -> Result<PathBuf, NjilError> {
        // 如果是绝对路径，直接返回
        if path.is_absolute() {
            debug_println!("检查绝对路径: {}", path.display());
            if path.exists() {
                return Ok(path.to_path_buf());
            }
        } else {
            // 如果是相对路径，在搜索路径中查找
            for search_path in &self.search_paths {
                let full_path = search_path.join(path);
                debug_println!("检查搜索路径组合: {}", full_path.display());
                if full_path.exists() {
                    return Ok(full_path);
                }
            }
            
            // 直接检查当前路径
            debug_println!("检查当前路径: {}", path.display());
            if path.exists() {
                return Ok(path.to_path_buf());
            }
            
            // 检查是否需要添加.njim扩展名
            let with_ext = if path.extension().is_some() {
                path.to_path_buf()
            } else {
                path.with_extension("njim")
            };
            
            debug_println!("检查带扩展名的路径: {}", with_ext.display());
            if with_ext.exists() {
                return Ok(with_ext);
            }
            
            // 在搜索路径中查找带扩展名的路径
            for search_path in &self.search_paths {
                let full_path = search_path.join(&with_ext);
                debug_println!("检查带扩展名的搜索路径组合: {}", full_path.display());
                if full_path.exists() {
                    return Ok(full_path);
                }
            }
        }
        
        debug_println!("无法找到模块: {}", path.display());
        Err(NjilError::ExecutionError(
            format!("找不到模块: {}", path.display())
        ))
    }
    
    /// 获取模块
    pub fn get_module(&self, module_path: &str) -> Option<Arc<NjimModule>> {
        self.loaded_modules.get(module_path).cloned()
    }
    
    /// 获取模块函数
    pub fn get_module_function(&self, module_path: &str, function_name: &str) -> Option<Function> {
        if let Some(module) = self.get_module(module_path) {
            if let Some(functions) = &module.exports.functions {
                return functions.get(function_name).cloned();
            }
        }
        None
    }
    
    /// 获取模块常量
    pub fn get_module_constant(&self, module_path: &str, constant_name: &str) -> Option<Value> {
        if let Some(module) = self.get_module(module_path) {
            if let Some(constants) = &module.exports.constants {
                return constants.get(constant_name).cloned();
            }
        }
        None
    }
    
    /// 获取所有已加载模块的路径
    pub fn get_loaded_module_paths(&self) -> Vec<String> {
        self.loaded_modules.keys().cloned().collect()
    }
}

// 全局模块加载器实例
static GLOBAL_MODULE_LOADER: OnceLock<Mutex<ModuleLoader>> = OnceLock::new();

/// 获取全局模块加载器
pub fn get_module_loader() -> &'static Mutex<ModuleLoader> {
    GLOBAL_MODULE_LOADER.get_or_init(|| Mutex::new(ModuleLoader::new()))
}

/// 加载模块，使用全局模块加载器
pub fn load_module<P: AsRef<Path>>(path: P) -> Result<Arc<NjimModule>, NjilError> {
    debug_println!("全局加载模块: {}", path.as_ref().display());
    let mut loader = get_module_loader().lock().unwrap();
    let result = loader.load_module(path);
    
    // 打印已加载的模块信息
    if result.is_ok() {
        let loaded_paths = loader.get_loaded_module_paths();
        debug_println!("全局已加载的模块路径: {:?}", loaded_paths);
    }
    
    result
}

/// 添加模块搜索路径
pub fn add_module_search_path<P: AsRef<Path>>(path: P) {
    let mut loader = get_module_loader().lock().unwrap();
    loader.add_search_path(path);
}

/// 获取模块函数
pub fn get_module_function(module_path: &str, function_name: &str) -> Option<Function> {
    let loader = get_module_loader().lock().unwrap();
    debug_println!("尝试获取模块函数: {}.{}", module_path, function_name);
    debug_println!("已加载的模块路径: {:?}", loader.get_loaded_module_paths());
    
    if let Some(module) = loader.get_module(module_path) {
        debug_println!("找到模块: {}", module_path);
        if let Some(functions) = &module.exports.functions {
            debug_println!("模块导出的函数: {:?}", functions.keys().collect::<Vec<_>>());
            let function = functions.get(function_name).cloned();
            if function.is_some() {
                debug_println!("找到函数: {}", function_name);
            } else {
                debug_println!("在模块中未找到函数: {}", function_name);
            }
            return function;
        } else {
            debug_println!("模块没有导出函数");
        }
    } else {
        debug_println!("未找到模块: {}", module_path);
    }
    None
}

/// 获取模块常量
pub fn get_module_constant(module_path: &str, constant_name: &str) -> Option<Value> {
    let loader = get_module_loader().lock().unwrap();
    loader.get_module_constant(module_path, constant_name)
} 