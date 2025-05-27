use neo_jilang::{run_file, NjilError};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("用法: {} <njil文件路径>", args[0]);
        process::exit(1);
    }
    
    let file_path = &args[1];
    
    match run_file(file_path) {
        Ok(result) => {
            println!("程序执行成功，返回值: {}", result);
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
