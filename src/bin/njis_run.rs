use neo_jilang::{njis, NjilError, set_debug_mode};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("用法: {} <njis文件路径> [--njis-debug]", args[0]);
        println!("选项:");
        println!("  --njis-debug  启用调试输出");
        process::exit(1);
    }
    
    // 检查是否有调试标志
    let debug_mode = args.iter().any(|arg| arg == "--njis-debug");
    set_debug_mode(debug_mode);
    
    // 获取文件路径(第一个非选项参数)
    let mut file_path = None;
    for arg in &args[1..] {
        if !arg.starts_with("--") {
            file_path = Some(arg);
            break;
        }
    }
    
    let file_path = match file_path {
        Some(path) => path,
        None => {
            println!("请提供NJIS文件路径");
            process::exit(1);
        }
    };
    
    match njis::run_njis_file(file_path) {
        Ok(result) => {
            if !result.is_null() {
                println!("程序执行成功，返回值: {}", result);
            }
        }
        Err(err) => {
            match err {
                NjilError::ReturnValue(value) => {
                    println!("程序执行成功，返回值: {}", value);
                }
                _ => {
                    eprintln!("错误: {}", err);
                    process::exit(1);
                }
            }
        }
    }
} 