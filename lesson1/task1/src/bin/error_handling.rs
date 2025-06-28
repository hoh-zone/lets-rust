// 第12章：错误处理
// Rust 提供了强大的错误处理机制，包括 panic! 和 Result 类型

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

// 1. 自定义错误类型
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "不能除以零"),
            MathError::NegativeSquareRoot => write!(f, "负数不能开平方根"),
            MathError::Overflow => write!(f, "数值溢出"),
        }
    }
}

impl Error for MathError {}

// 2. 数学运算函数
fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn safe_sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

fn safe_multiply(a: i32, b: i32) -> Result<i32, MathError> {
    match a.checked_mul(b) {
        Some(result) => Ok(result),
        None => Err(MathError::Overflow),
    }
}

// 3. 复杂的自定义错误类型
#[derive(Debug)]
struct ValidationError {
    field: String,
    message: String,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "字段 '{}' 验证失败: {}", self.field, self.message)
    }
}

impl Error for ValidationError {}

#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Parse(ParseIntError),
    Validation(ValidationError),
    Math(MathError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "IO 错误: {}", err),
            AppError::Parse(err) => write!(f, "解析错误: {}", err),
            AppError::Validation(err) => write!(f, "验证错误: {}", err),
            AppError::Math(err) => write!(f, "数学错误: {}", err),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Io(err) => Some(err),
            AppError::Parse(err) => Some(err),
            AppError::Validation(err) => Some(err),
            AppError::Math(err) => Some(err),
        }
    }
}

// 自动转换
impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError::Parse(err)
    }
}

impl From<ValidationError> for AppError {
    fn from(err: ValidationError) -> Self {
        AppError::Validation(err)
    }
}

impl From<MathError> for AppError {
    fn from(err: MathError) -> Self {
        AppError::Math(err)
    }
}

// 4. 用户验证
fn validate_age(age: i32) -> Result<i32, ValidationError> {
    if age < 0 {
        Err(ValidationError {
            field: "age".to_string(),
            message: "年龄不能为负数".to_string(),
        })
    } else if age > 150 {
        Err(ValidationError {
            field: "age".to_string(),
            message: "年龄不能超过150岁".to_string(),
        })
    } else {
        Ok(age)
    }
}

fn validate_email(email: &str) -> Result<&str, ValidationError> {
    if email.contains('@') {
        Ok(email)
    } else {
        Err(ValidationError {
            field: "email".to_string(),
            message: "邮箱必须包含 @ 符号".to_string(),
        })
    }
}

// 5. 使用 ? 操作符的函数
fn read_file_to_string(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_and_double(s: &str) -> Result<i32, AppError> {
    let num: i32 = s.parse()?;
    let doubled = safe_multiply(num, 2)?;
    Ok(doubled)
}

// 6. 链式错误处理
fn complex_calculation(input: &str) -> Result<f64, AppError> {
         let num: f64 = input.parse()
         .map_err(|_: std::num::ParseFloatError| ValidationError {
             field: "input".to_string(),
             message: "无法解析为数字".to_string(),
         })?;
    
    let divided = safe_divide(num, 2.0)?;
    let sqrt_result = safe_sqrt(divided)?;
    Ok(sqrt_result)
}

// 7. Option 的错误处理
fn find_user_by_id(id: u32) -> Option<String> {
    let users = vec![
        (1, "Alice"),
        (2, "Bob"),
        (3, "Charlie"),
    ];
    
    users.iter()
        .find(|(user_id, _)| *user_id == id)
        .map(|(_, name)| name.to_string())
}

fn get_user_email(user_id: u32) -> Result<String, AppError> {
    let username = find_user_by_id(user_id)
        .ok_or_else(|| ValidationError {
            field: "user_id".to_string(),
            message: format!("用户 ID {} 不存在", user_id),
        })?;
    
    Ok(format!("{}@example.com", username.to_lowercase()))
}

// 8. 恢复策略
fn divide_with_default(a: f64, b: f64, default: f64) -> f64 {
    safe_divide(a, b).unwrap_or(default)
}

fn safe_operation_with_retry<F>(operation: F, max_retries: u32) -> Result<String, AppError>
where
    F: Fn() -> Result<String, AppError>,
{
    let mut attempts = 0;
    loop {
        match operation() {
            Ok(result) => return Ok(result),
            Err(e) => {
                attempts += 1;
                if attempts >= max_retries {
                    return Err(e);
                }
                println!("操作失败，重试第 {} 次", attempts);
            }
        }
    }
}

// 9. 错误聚合
fn validate_user_data(name: &str, age_str: &str, email: &str) -> Result<(String, i32, String), Vec<AppError>> {
    let mut errors = Vec::new();
    
    let name = if name.trim().is_empty() {
        errors.push(ValidationError {
            field: "name".to_string(),
            message: "姓名不能为空".to_string(),
        }.into());
        None
    } else {
        Some(name.to_string())
    };
    
    let age = match age_str.parse::<i32>() {
        Ok(age) => match validate_age(age) {
            Ok(age) => Some(age),
            Err(e) => {
                errors.push(e.into());
                None
            }
        },
        Err(e) => {
            errors.push(e.into());
            None
        }
    };
    
    let email = match validate_email(email) {
        Ok(email) => Some(email.to_string()),
        Err(e) => {
            errors.push(e.into());
            None
        }
    };
    
    if errors.is_empty() {
        Ok((name.unwrap(), age.unwrap(), email.unwrap()))
    } else {
        Err(errors)
    }
}

// 10. panic 的使用场景
fn demonstrate_panic() {
    println!("演示 panic 的使用场景：");
    
    // 1. 使用 unwrap（可能 panic）
    let some_value = Some(42);
    let value = some_value.unwrap();
    println!("unwrap 成功: {}", value);
    
    // 2. 使用 expect（带自定义消息的 panic）
    let result: Result<i32, &str> = Ok(100);
    let value = result.expect("这个结果应该是 Ok");
    println!("expect 成功: {}", value);
    
    // 3. 断言
    assert_eq!(2 + 2, 4);
    assert!(value > 0, "值应该是正数");
    
    println!("所有断言都通过了");
}

// 11. 错误处理的最佳实践
fn best_practices_example() -> Result<String, AppError> {
    // 1. 使用 ? 操作符进行错误传播
         let content = read_file_to_string("example.txt")
         .or_else(|_: io::Error| -> Result<String, io::Error> {
             // 如果文件不存在，使用默认内容
             println!("文件不存在，使用默认内容");
             Ok("default content".to_string())
         })?;
    
    // 2. 链式操作
    let processed = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<&str>>()
        .join("\n");
    
    Ok(processed)
}

fn main() {
    println!("=== 第12章：错误处理 ===\n");
    
    // 1. 基本的 Result 使用
    println!("1. 基本的 Result 使用：");
    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("10.0 / 2.0 = {}", result),
        Err(e) => println!("错误: {}", e),
    }
    
    match safe_divide(10.0, 0.0) {
        Ok(result) => println!("10.0 / 0.0 = {}", result),
        Err(e) => println!("错误: {}", e),
    }
    println!();
    
    // 2. 链式操作
    println!("2. 链式操作：");
    let result = safe_divide(16.0, 4.0)
        .and_then(|x| safe_sqrt(x))
        .and_then(|x| safe_divide(x, 2.0));
    
    match result {
        Ok(value) => println!("链式计算结果: {}", value),
        Err(e) => println!("链式计算错误: {}", e),
    }
    println!();
    
    // 3. map 和 map_err 的使用
    println!("3. map 和 map_err 的使用：");
    let result = safe_divide(20.0, 4.0)
        .map(|x| x * 2.0)
        .map_err(|e| format!("计算失败: {}", e));
    
    println!("map 结果: {:?}", result);
    
    let error_result = safe_divide(10.0, 0.0)
        .map(|x| x * 2.0)
        .map_err(|e| format!("计算失败: {}", e));
    
    println!("map_err 结果: {:?}", error_result);
    println!();
    
    // 4. 使用 ? 操作符
    println!("4. 使用 ? 操作符：");
    match parse_and_double("42") {
        Ok(result) => println!("解析并翻倍: {}", result),
        Err(e) => println!("错误: {}", e),
    }
    
    match parse_and_double("abc") {
        Ok(result) => println!("解析并翻倍: {}", result),
        Err(e) => println!("错误: {}", e),
    }
    println!();
    
    // 5. 复杂错误处理
    println!("5. 复杂错误处理：");
    let test_cases = vec!["16", "4", "-4", "abc"];
    
    for input in test_cases {
        match complex_calculation(input) {
            Ok(result) => println!("复杂计算 '{}' = {}", input, result),
            Err(e) => println!("复杂计算 '{}' 错误: {}", input, e),
        }
    }
    println!();
    
    // 6. Option 的错误处理
    println!("6. Option 的错误处理：");
    let user_ids = vec![1, 2, 5];
    
    for id in user_ids {
        match get_user_email(id) {
            Ok(email) => println!("用户 {} 的邮箱: {}", id, email),
            Err(e) => println!("获取用户 {} 邮箱失败: {}", id, e),
        }
    }
    println!();
    
    // 7. 恢复策略
    println!("7. 恢复策略：");
    println!("除法结果（带默认值）: {}", divide_with_default(10.0, 0.0, -1.0));
    println!("除法结果（正常）: {}", divide_with_default(10.0, 2.0, -1.0));
    
         // 模拟重试操作
     use std::cell::RefCell;
     let attempt_count = RefCell::new(0);
     let retry_operation = || {
         *attempt_count.borrow_mut() += 1;
         if *attempt_count.borrow() < 3 {
             Err(ValidationError {
                 field: "network".to_string(),
                 message: "网络连接失败".to_string(),
             }.into())
         } else {
             Ok("操作成功".to_string())
         }
     };
    
    match safe_operation_with_retry(retry_operation, 5) {
        Ok(result) => println!("重试操作成功: {}", result),
        Err(e) => println!("重试操作最终失败: {}", e),
    }
    println!();
    
    // 8. 错误聚合
    println!("8. 错误聚合：");
    let test_data = vec![
        ("Alice", "25", "alice@example.com"),
        ("", "30", "bob@example.com"),
        ("Charlie", "-5", "charlie@example.com"),
        ("David", "25", "invalid-email"),
        ("", "abc", "invalid"),
    ];
    
    for (name, age, email) in test_data {
        match validate_user_data(name, age, email) {
            Ok((name, age, email)) => {
                println!("用户验证成功: {} ({}) - {}", name, age, email);
            }
            Err(errors) => {
                println!("用户验证失败:");
                for error in errors {
                    println!("  - {}", error);
                }
            }
        }
    }
    println!();
    
    // 9. panic 的使用场景
    demonstrate_panic();
    println!();
    
    // 10. 最佳实践示例
    println!("10. 最佳实践示例：");
    match best_practices_example() {
        Ok(content) => println!("处理后的内容: {}", content),
        Err(e) => println!("处理失败: {}", e),
    }
    println!();
    
    // 11. unwrap_or 和 unwrap_or_else
    println!("11. unwrap_or 和 unwrap_or_else：");
    let good_result: Result<i32, &str> = Ok(42);
    let bad_result: Result<i32, &str> = Err("错误");
    
    println!("good_result.unwrap_or(0): {}", good_result.unwrap_or(0));
    println!("bad_result.unwrap_or(0): {}", bad_result.unwrap_or(0));
    
    let expensive_default = || {
        println!("计算昂贵的默认值");
        100
    };
    
    println!("good_result.unwrap_or_else: {}", good_result.unwrap_or_else(|_| expensive_default()));
    println!("bad_result.unwrap_or_else: {}", bad_result.unwrap_or_else(|_| expensive_default()));
    println!();
    
    // 12. 错误链
    println!("12. 错误链：");
    let error = AppError::Validation(ValidationError {
        field: "test".to_string(),
        message: "测试错误".to_string(),
    });
    
    println!("错误: {}", error);
    if let Some(source) = error.source() {
        println!("错误源: {}", source);
    }
    
    // 13. 自定义 Result 类型
    println!("13. 自定义 Result 类型：");
    type AppResult<T> = Result<T, AppError>;
    
    fn custom_operation() -> AppResult<String> {
        Ok("自定义操作成功".to_string())
    }
    
    match custom_operation() {
        Ok(result) => println!("自定义操作: {}", result),
        Err(e) => println!("自定义操作失败: {}", e),
    }
    
    println!("\n=== 第12章完成 ===");
}

// 14. 错误处理的性能考虑
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn test_error_performance() {
        // Result 的成功路径应该很快
        let start = Instant::now();
        for i in 0..1000000 {
            let _ = safe_divide(10.0, 2.0);
        }
        let success_duration = start.elapsed();
        
        // 错误路径可能稍慢
        let start = Instant::now();
        for i in 0..1000000 {
            let _ = safe_divide(10.0, 0.0);
        }
        let error_duration = start.elapsed();
        
        println!("成功路径耗时: {:?}", success_duration);
        println!("错误路径耗时: {:?}", error_duration);
        
        // 在 Rust 中，错误处理的开销很小
        assert!(error_duration.as_millis() < success_duration.as_millis() * 10);
    }
    
    #[test]
    fn test_error_conversion() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "文件未找到");
        let app_error: AppError = io_error.into();
        
        match app_error {
            AppError::Io(_) => assert!(true),
            _ => assert!(false, "应该是 IO 错误"),
        }
    }
    
    #[test]
    fn test_question_mark_operator() {
        fn test_function() -> Result<i32, AppError> {
            let result = "42".parse::<i32>()?;
            Ok(result * 2)
        }
        
        assert_eq!(test_function().unwrap(), 84);
        
        fn test_error_function() -> Result<i32, AppError> {
            let result = "abc".parse::<i32>()?;
            Ok(result * 2)
        }
        
        assert!(test_error_function().is_err());
    }
} 