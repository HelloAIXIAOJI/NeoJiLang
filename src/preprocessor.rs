use std::fs;
use std::path::Path;
use crate::error::NjilError;
use crate::debug_println;

/// 预处理器，用于在解析JSON之前处理文件内容
pub struct Preprocessor;

impl Preprocessor {
    /// 从文件中读取内容并进行预处理
    pub fn preprocess_file<P: AsRef<Path>>(path: P) -> Result<String, NjilError> {
        let content = fs::read_to_string(path)?;
        Self::preprocess_content(&content)
    }
    
    /// 处理内容，移除注释
    pub fn preprocess_content(content: &str) -> Result<String, NjilError> {
        debug_println!("开始预处理内容，移除注释");
        
        let mut result = String::new();
        let mut in_string = false;
        let mut in_single_line_comment = false;
        let mut in_multi_line_comment = false;
        let mut escape_next = false;
        let mut i = 0;
        let chars: Vec<char> = content.chars().collect();
        
        while i < chars.len() {
            let c = chars[i];
            
            // 处理转义字符
            if escape_next {
                if !in_single_line_comment && !in_multi_line_comment {
                    result.push(c);
                }
                escape_next = false;
                i += 1;
                continue;
            }
            
            // 处理字符串内的转义
            if c == '\\' && in_string {
                escape_next = true;
                if !in_single_line_comment && !in_multi_line_comment {
                    result.push(c);
                }
                i += 1;
                continue;
            }
            
            // 处理字符串边界
            if c == '"' && !in_single_line_comment && !in_multi_line_comment {
                in_string = !in_string;
                result.push(c);
                i += 1;
                continue;
            }
            
            // 在字符串内，直接添加字符
            if in_string {
                result.push(c);
                i += 1;
                continue;
            }
            
            // 检测单行注释开始
            if !in_single_line_comment && !in_multi_line_comment && c == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
                in_single_line_comment = true;
                i += 2;
                continue;
            }
            
            // 检测多行注释开始
            if !in_single_line_comment && !in_multi_line_comment && c == '/' && i + 1 < chars.len() && chars[i + 1] == '*' {
                in_multi_line_comment = true;
                i += 2;
                continue;
            }
            
            // 处理单行注释结束
            if in_single_line_comment && (c == '\n' || c == '\r') {
                in_single_line_comment = false;
                // 保留换行符
                result.push(c);
                i += 1;
                continue;
            }
            
            // 处理多行注释结束
            if in_multi_line_comment && c == '*' && i + 1 < chars.len() && chars[i + 1] == '/' {
                in_multi_line_comment = false;
                i += 2;
                continue;
            }
            
            // 非注释内容，添加到结果
            if !in_single_line_comment && !in_multi_line_comment {
                result.push(c);
            }
            
            i += 1;
        }
        
        debug_println!("预处理完成，移除了注释");
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_remove_single_line_comments() {
        let input = r#"{
  // 这是一个单行注释
  "key": "value", // 行尾注释
  "array": [1, 2, 3] // 另一个注释
}"#;
        
        let expected = r#"{
  
  "key": "value", 
  "array": [1, 2, 3] 
}"#;
        
        let result = Preprocessor::preprocess_content(input).unwrap();
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_remove_multi_line_comments() {
        let input = r#"{
  /* 这是一个
     多行注释 */
  "key": "value",
  /* 另一个
     多行注释 */
  "array": [1, 2, 3]
}"#;
        
        let expected = r#"{
  
  "key": "value",
  
  "array": [1, 2, 3]
}"#;
        
        let result = Preprocessor::preprocess_content(input).unwrap();
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_preserve_comments_in_strings() {
        let input = r#"{
  "comment_string": "这不是 // 注释",
  "another_string": "这也不是 /* 注释 */"
}"#;
        
        let result = Preprocessor::preprocess_content(input).unwrap();
        assert_eq!(result, input);
    }
    
    #[test]
    fn test_mixed_comments() {
        let input = r#"{
  // 单行注释
  "key": "value", /* 行内多行注释 */ "key2": "value2"
}"#;
        
        let expected = r#"{
  
  "key": "value",  "key2": "value2"
}"#;
        
        let result = Preprocessor::preprocess_content(input).unwrap();
        assert_eq!(result, expected);
    }
} 