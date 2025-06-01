use std::fs;
use std::path::Path;
use super::system::*;
use crate::interpreter::Interpreter;
use serde_json::Value;

fn setup_test_dir() -> String {
    let test_dir = "target/system_test_dir";
    // 清理已存在的测试目录
    if Path::new(test_dir).exists() {
        let _ = fs::remove_dir_all(test_dir);
    }
    test_dir.to_string()
}

fn cleanup_test_dir(test_dir: &str) {
    if Path::new(test_dir).exists() {
        let _ = fs::remove_dir_all(test_dir);
    }
}

#[test]
fn test_fs_exists() {
    let mut interpreter = Interpreter::new();
    let test_dir = setup_test_dir();
    
    // 创建测试目录
    fs::create_dir_all(&test_dir).unwrap();
    
    // 测试目录存在
    let result = FS_EXISTS_HANDLER.handle(
        &mut interpreter, 
        &Value::String(test_dir.clone())
    ).unwrap();
    
    assert_eq!(result, Value::Bool(true));
    
    // 测试不存在的路径
    let non_existent = format!("{}/non_existent", test_dir);
    let result = FS_EXISTS_HANDLER.handle(
        &mut interpreter, 
        &Value::String(non_existent)
    ).unwrap();
    
    assert_eq!(result, Value::Bool(false));
    
    cleanup_test_dir(&test_dir);
}

#[test]
fn test_fs_is_dir() {
    let mut interpreter = Interpreter::new();
    let test_dir = setup_test_dir();
    
    // 创建测试目录
    fs::create_dir_all(&test_dir).unwrap();
    
    // 创建测试文件
    let test_file = format!("{}/test_file.txt", test_dir);
    fs::write(&test_file, "test content").unwrap();
    
    // 测试目录是目录
    let result = FS_IS_DIR_HANDLER.handle(
        &mut interpreter, 
        &Value::String(test_dir.clone())
    ).unwrap();
    
    assert_eq!(result, Value::Bool(true));
    
    // 测试文件不是目录
    let result = FS_IS_DIR_HANDLER.handle(
        &mut interpreter, 
        &Value::String(test_file)
    ).unwrap();
    
    assert_eq!(result, Value::Bool(false));
    
    cleanup_test_dir(&test_dir);
}

#[test]
fn test_fs_is_file() {
    let mut interpreter = Interpreter::new();
    let test_dir = setup_test_dir();
    
    // 创建测试目录
    fs::create_dir_all(&test_dir).unwrap();
    
    // 创建测试文件
    let test_file = format!("{}/test_file.txt", test_dir);
    fs::write(&test_file, "test content").unwrap();
    
    // 测试文件是文件
    let result = FS_IS_FILE_HANDLER.handle(
        &mut interpreter, 
        &Value::String(test_file.clone())
    ).unwrap();
    
    assert_eq!(result, Value::Bool(true));
    
    // 测试目录不是文件
    let result = FS_IS_FILE_HANDLER.handle(
        &mut interpreter, 
        &Value::String(test_dir)
    ).unwrap();
    
    assert_eq!(result, Value::Bool(false));
    
    cleanup_test_dir(&test_dir);
}

#[test]
fn test_fs_mkdir_and_remove() {
    let mut interpreter = Interpreter::new();
    let test_dir = setup_test_dir();
    let new_dir = format!("{}/new_dir", test_dir);
    let nested_dir = format!("{}/nested/dir", test_dir);
    
    // 创建基础测试目录
    fs::create_dir_all(&test_dir).unwrap();
    
    // 测试创建单层目录
    let result = FS_MKDIR_HANDLER.handle(
        &mut interpreter, 
        &Value::String(new_dir.clone())
    ).unwrap();
    
    assert_eq!(result, Value::Bool(true));
    assert!(Path::new(&new_dir).exists());
    assert!(Path::new(&new_dir).is_dir());
    
    // 测试创建嵌套目录（递归）
    let mut obj = serde_json::Map::new();
    obj.insert("path".to_string(), Value::String(nested_dir.clone()));
    obj.insert("recursive".to_string(), Value::Bool(true));
    
    let result = FS_MKDIR_HANDLER.handle(
        &mut interpreter, 
        &Value::Object(obj)
    ).unwrap();
    
    assert_eq!(result, Value::Bool(true));
    assert!(Path::new(&nested_dir).exists());
    assert!(Path::new(&nested_dir).is_dir());
    
    // 测试删除目录
    let result = FS_REMOVE_HANDLER.handle(
        &mut interpreter, 
        &Value::String(new_dir.clone())
    ).unwrap();
    
    assert_eq!(result, Value::Bool(true));
    assert!(!Path::new(&new_dir).exists());
    
    cleanup_test_dir(&test_dir);
}

#[test]
fn test_fs_copy_and_move() {
    let mut interpreter = Interpreter::new();
    let test_dir = setup_test_dir();
    
    // 创建测试目录
    fs::create_dir_all(&test_dir).unwrap();
    
    // 创建测试文件
    let test_file = format!("{}/source.txt", test_dir);
    let copy_dest = format!("{}/copied.txt", test_dir);
    let move_dest = format!("{}/moved.txt", test_dir);
    
    fs::write(&test_file, "test content").unwrap();
    
    // 测试复制文件
    let mut copy_obj = serde_json::Map::new();
    copy_obj.insert("from".to_string(), Value::String(test_file.clone()));
    copy_obj.insert("to".to_string(), Value::String(copy_dest.clone()));
    
    let result = FS_COPY_HANDLER.handle(
        &mut interpreter, 
        &Value::Object(copy_obj)
    ).unwrap();
    
    assert_eq!(result, Value::Bool(true));
    assert!(Path::new(&copy_dest).exists());
    assert!(Path::new(&test_file).exists());  // 原文件仍存在
    
    // 测试移动文件
    let mut move_obj = serde_json::Map::new();
    move_obj.insert("from".to_string(), Value::String(test_file.clone()));
    move_obj.insert("to".to_string(), Value::String(move_dest.clone()));
    
    let result = FS_MOVE_HANDLER.handle(
        &mut interpreter, 
        &Value::Object(move_obj)
    ).unwrap();
    
    assert_eq!(result, Value::Bool(true));
    assert!(Path::new(&move_dest).exists());
    assert!(!Path::new(&test_file).exists());  // 原文件不存在
    
    cleanup_test_dir(&test_dir);
}

#[test]
fn test_fs_list() {
    let mut interpreter = Interpreter::new();
    let test_dir = setup_test_dir();
    
    // 创建测试目录和文件
    fs::create_dir_all(&test_dir).unwrap();
    fs::write(format!("{}/file1.txt", test_dir), "content1").unwrap();
    fs::write(format!("{}/file2.txt", test_dir), "content2").unwrap();
    fs::create_dir(format!("{}/subdir", test_dir)).unwrap();
    
    // 测试列出目录内容
    let result = FS_LIST_HANDLER.handle(
        &mut interpreter, 
        &Value::String(test_dir.clone())
    ).unwrap();
    
    if let Value::Array(entries) = result {
        assert_eq!(entries.len(), 3);  // 应该有3个条目
        
        // 检查是否包含预期文件名
        let names: Vec<String> = entries.iter()
            .filter_map(|e| if let Value::String(name) = e { Some(name.clone()) } else { None })
            .collect();
        
        assert!(names.contains(&"file1.txt".to_string()));
        assert!(names.contains(&"file2.txt".to_string()));
        assert!(names.contains(&"subdir".to_string()));
    } else {
        panic!("Expected array result from fs.list");
    }
    
    // 测试包含详细信息
    let mut obj = serde_json::Map::new();
    obj.insert("path".to_string(), Value::String(test_dir.clone()));
    obj.insert("includeInfo".to_string(), Value::Bool(true));
    
    let result = FS_LIST_HANDLER.handle(
        &mut interpreter, 
        &Value::Object(obj)
    ).unwrap();
    
    if let Value::Array(entries) = result {
        assert_eq!(entries.len(), 3);  // 应该有3个条目
        
        // 检查条目是否包含详细信息
        for entry in entries {
            if let Value::Object(entry_obj) = entry {
                assert!(entry_obj.contains_key("name"));
                assert!(entry_obj.contains_key("type"));
                
                // 文件还应该有大小
                if let Value::String(typ) = &entry_obj["type"] {
                    if typ == "file" {
                        assert!(entry_obj.contains_key("size"));
                    }
                }
            } else {
                panic!("Expected object entry with includeInfo=true");
            }
        }
    } else {
        panic!("Expected array result from fs.list");
    }
    
    cleanup_test_dir(&test_dir);
} 