# 第12章：错误处理

Rust 将错误分为两类：可恢复错误（Result<T, E>）和不可恢复错误（panic!）。这种设计让错误处理更加明确和安全。

## 12.1 `panic!` 宏与不可恢复错误

### panic! 的基本使用

```rust
fn main() {
    // 直接 panic
    panic!("程序崩溃了！");
    
    // 带格式化的 panic
    let x = 42;
    panic!("发生错误：x 的值是 {}", x);
}

// 数组越界导致 panic
fn array_panic() {
    let v = vec![1, 2, 3];
    v[99];  // panic: index out of bounds
}

// 整数溢出（debug 模式下 panic）
fn overflow_panic() {
    let x: u8 = 255;
    let y = x + 1;  // debug 模式下 panic
}
```

### panic 的处理机制

```rust
use std::panic;

fn main() {
    // 设置 panic hook
    panic::set_hook(Box::new(|panic_info| {
        if let Some(location) = panic_info.location() {
            println!("panic 发生在 {}:{}:{}", 
                location.file(), 
                location.line(), 
                location.column()
            );
        }
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            println!("panic 信息: {}", s);
        }
    }));
    
    // 捕获 panic
    let result = panic::catch_unwind(|| {
        println!("执行可能 panic 的代码");
        panic!("出错了！");
    });
    
    match result {
        Ok(_) => println!("代码正常执行"),
        Err(_) => println!("捕获到 panic"),
    }
    
    println!("程序继续运行");
}
```

### 什么时候使用 panic!

```rust
// 1. 程序处于损坏状态，无法继续
fn corrupt_state() {
    let config = load_config();
    if config.is_corrupted() {
        panic!("配置文件损坏，无法继续运行");
    }
}

// 2. 开发时的断言
fn development_assertions() {
    let x = 5;
    assert!(x > 0, "x 必须是正数");
    assert_eq!(x * 2, 10, "数学计算错误");
    assert_ne!(x, 0, "x 不能为零");
}

// 3. 原型和示例代码
fn prototype_code() {
    let home: Option<String> = std::env::var("HOME").ok();
    let home = home.unwrap();  // 原型代码中可以使用 unwrap
}

// 4. 测试代码
#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "除以零")]
    fn test_divide_by_zero() {
        divide(10, 0);
    }
    
    fn divide(a: i32, b: i32) -> i32 {
        if b == 0 {
            panic!("除以零");
        }
        a / b
    }
}
```

## 12.2 `Result` 类型与可恢复错误

### Result 枚举定义

```rust
// Result 的定义
enum Result<T, E> {
    Ok(T),
    Err(E),
}

use std::fs::File;
use std::io::Error;

fn main() {
    // 打开文件返回 Result
    let file_result: Result<File, Error> = File::open("hello.txt");
    
    match file_result {
        Ok(file) => {
            println!("文件打开成功");
            // 使用 file
        }
        Err(error) => {
            println!("打开文件失败: {}", error);
        }
    }
}
```

### 处理不同类型的错误

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt");
    
    let greeting_file = match greeting_file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("创建文件失败: {:?}", e),
                }
            }
            other_error => {
                panic!("打开文件失败: {:?}", other_error);
            }
        },
    };
}

// 使用闭包简化
fn simplified_error_handling() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("创建文件失败: {:?}", error);
            })
        } else {
            panic!("打开文件失败: {:?}", error);
        }
    });
}
```

### Result 的常用方法

```rust
use std::fs::File;
use std::io::Read;

fn result_methods() -> Result<String, std::io::Error> {
    // unwrap - 成功返回值，失败 panic
    let f = File::open("hello.txt").unwrap();
    
    // expect - 带自定义错误信息的 unwrap
    let f = File::open("hello.txt").expect("无法打开 hello.txt");
    
    // unwrap_or - 提供默认值
    let content = read_file("config.txt").unwrap_or(String::from("默认配置"));
    
    // unwrap_or_else - 使用闭包提供默认值
    let content = read_file("data.txt").unwrap_or_else(|e| {
        eprintln!("读取失败: {}", e);
        String::new()
    });
    
    // map - 转换成功值
    let len = read_file("test.txt").map(|s| s.len());
    
    // map_err - 转换错误类型
    let result = read_file("file.txt")
        .map_err(|e| format!("读取错误: {}", e));
    
    // and_then - 链式操作
    let result = File::open("number.txt")
        .and_then(|mut f| {
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s)
        })
        .and_then(|s| s.trim().parse::<i32>().map_err(|e| e.into()));
    
    Ok(String::from("success"))
}

fn read_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}
```

## 12.3 错误传播（? 操作符）

### ? 操作符的使用

```rust
use std::fs::File;
use std::io::{self, Read};

// 手动传播错误
fn read_username_from_file_verbose() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut username = String::new();
    
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// 使用 ? 操作符
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// 链式调用
fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// 最简洁的方式
fn read_username_from_file_shortest() -> Result<String, io::Error> {
    std::fs::read_to_string("hello.txt")
}
```

### ? 与 From trait

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct AppError {
    message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "应用错误: {}", self.message)
    }
}

impl Error for AppError {}

// 实现 From trait 以支持 ? 操作符
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError {
            message: format!("IO 错误: {}", error),
        }
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(error: std::num::ParseIntError) -> Self {
        AppError {
            message: format!("解析错误: {}", error),
        }
    }
}

// 现在可以使用 ? 传播不同类型的错误
fn complex_operation() -> Result<i32, AppError> {
    let contents = std::fs::read_to_string("number.txt")?;
    let number: i32 = contents.trim().parse()?;
    Ok(number * 2)
}
```

### 在 Option 中使用 ?

```rust
fn get_first_char(text: &str) -> Option<char> {
    text.lines().next()?.chars().next()
}

// 混合使用 Result 和 Option
fn parse_config(config: &str) -> Result<Config, Box<dyn Error>> {
    let lines: Vec<&str> = config.lines().collect();
    
    let name = lines.get(0).ok_or("缺少名称")?;
    let age: u32 = lines.get(1)
        .ok_or("缺少年龄")?
        .parse()?;
    
    Ok(Config {
        name: name.to_string(),
        age,
    })
}

struct Config {
    name: String,
    age: u32,
}
```

## 12.4 自定义错误类型

### 基本错误类型

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum DataError {
    NotFound,
    InvalidFormat(String),
    IoError(io::Error),
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataError::NotFound => write!(f, "数据未找到"),
            DataError::InvalidFormat(msg) => write!(f, "无效格式: {}", msg),
            DataError::IoError(e) => write!(f, "IO 错误: {}", e),
        }
    }
}

impl Error for DataError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DataError::IoError(e) => Some(e),
            _ => None,
        }
    }
}
```

### 使用 thiserror 库

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum ApiError {
    #[error("网络错误")]
    Network(#[from] reqwest::Error),
    
    #[error("解析错误: {0}")]
    Parse(String),
    
    #[error("认证失败")]
    Unauthorized,
    
    #[error("未知错误")]
    Unknown,
}

// 使用自定义错误
async fn fetch_data(url: &str) -> Result<String, ApiError> {
    let response = reqwest::get(url).await?;
    
    if response.status() == 401 {
        return Err(ApiError::Unauthorized);
    }
    
    let text = response.text().await?;
    Ok(text)
}
```

### 错误链

```rust
use std::error::Error;

fn print_error_chain(e: &dyn Error) {
    eprintln!("错误: {}", e);
    
    let mut current = e.source();
    while let Some(cause) = current {
        eprintln!("原因: {}", cause);
        current = cause.source();
    }
}

fn process_data() -> Result<(), Box<dyn Error>> {
    match complex_operation() {
        Ok(_) => Ok(()),
        Err(e) => {
            print_error_chain(&e);
            Err(e.into())
        }
    }
}
```

## RWO 权限分析

### panic! 与所有权

```rust
struct Resource {
    name: String,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("释放资源: {}", self.name);
    }
}

fn panic_ownership() {
    let r1 = Resource { name: String::from("R1") };
    let r2 = Resource { name: String::from("R2") };
    
    // panic 时会按 LIFO 顺序调用 drop
    panic!("发生错误");
    
    // 不会执行到这里
    println!("这行不会执行");
}

// panic 安全的函数
fn panic_safe_function(data: &mut Vec<i32>) {
    // 保存原始状态
    let original_len = data.len();
    
    // 尝试进行操作
    data.push(42);
    
    // 如果这里 panic，data 仍处于有效状态
    if data.len() > 10 {
        panic!("向量太大");
    }
}
```

### Result 的权限传递

```rust
#[derive(Debug)]
struct Data {
    content: String,
}

// R: 只需要读权限
fn validate(data: &Data) -> Result<(), String> {
    if data.content.is_empty() {
        Err(String::from("内容为空"))
    } else {
        Ok(())
    }
}

// W: 需要写权限
fn process(data: &mut Data) -> Result<(), String> {
    validate(data)?;
    data.content.push_str(" [已处理]");
    Ok(())
}

// O: 消费所有权
fn consume_data(data: Data) -> Result<String, String> {
    validate(&data)?;
    Ok(data.content)  // 返回所有权
}

fn permission_example() -> Result<(), String> {
    let mut data = Data { 
        content: String::from("Hello") 
    };
    
    // R: 验证只需要读权限
    validate(&data)?;
    
    // W: 处理需要写权限
    process(&mut data)?;
    
    // O: 最后消费数据
    let result = consume_data(data)?;
    // data 不再可用
    
    println!("结果: {}", result);
    Ok(())
}
```

### 错误处理中的生命周期

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct BorrowError<'a> {
    context: &'a str,
    source: Box<dyn Error + 'a>,
}

impl<'a> fmt::Display for BorrowError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "错误发生在 {}: {}", self.context, self.source)
    }
}

impl<'a> Error for BorrowError<'a> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // 注意生命周期限制
        None
    }
}

// 带引用的错误处理
fn process_with_context<'a>(
    data: &str, 
    context: &'a str
) -> Result<String, BorrowError<'a>> {
    if data.is_empty() {
        return Err(BorrowError {
            context,
            source: Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "数据为空"
            )),
        });
    }
    
    Ok(data.to_uppercase())
}
```

### ? 操作符的权限影响

```rust
use std::fs::File;
use std::io::{self, Read, Write};

// ? 操作符会转移错误的所有权
fn copy_file(from: &str, to: &str) -> Result<u64, io::Error> {
    let mut source = File::open(from)?;  // O: 错误被转移
    let mut dest = File::create(to)?;    // O: 错误被转移
    
    let mut buffer = Vec::new();
    let bytes_read = source.read_to_end(&mut buffer)?;  // W: buffer
    dest.write_all(&buffer)?;  // R: buffer
    
    Ok(bytes_read as u64)
}

// 错误包装保留原始错误
#[derive(Debug)]
struct WrapError {
    inner: io::Error,  // O: 拥有内部错误
}

impl From<io::Error> for WrapError {
    fn from(error: io::Error) -> Self {
        WrapError { inner: error }  // O: 转移所有权
    }
}

// 错误引用避免所有权转移
struct RefError<'a> {
    inner: &'a io::Error,  // R: 只借用错误
}
```

### 错误处理的最佳实践

```rust
// 1. 为库设计合适的错误类型
pub mod my_lib {
    use std::error::Error;
    use std::fmt;
    
    #[derive(Debug)]
    pub enum LibError {
        InvalidInput(String),
        ProcessingFailed { reason: String },
        IoError(std::io::Error),
    }
    
    impl fmt::Display for LibError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                LibError::InvalidInput(msg) => write!(f, "无效输入: {}", msg),
                LibError::ProcessingFailed { reason } => {
                    write!(f, "处理失败: {}", reason)
                }
                LibError::IoError(e) => write!(f, "IO 错误: {}", e),
            }
        }
    }
    
    impl Error for LibError {}
}

// 2. 应用程序使用动态错误
type AppResult<T> = Result<T, Box<dyn Error>>;

fn app_main() -> AppResult<()> {
    let config = load_config()?;
    let data = process_data(&config)?;
    save_results(data)?;
    Ok(())
}

// 3. 提供便利的错误构造函数
impl LibError {
    pub fn invalid_input(msg: impl Into<String>) -> Self {
        LibError::InvalidInput(msg.into())
    }
}

// 4. 使用 Result 类型别名
pub type LibResult<T> = Result<T, LibError>;

fn load_config() -> LibResult<Config> {
    // ...
}
```

## 小结

本章深入学习了 Rust 的错误处理机制：

1. **panic! 宏**：
   - 用于不可恢复的错误
   - 会清理栈并调用析构函数
   - 适用于程序bug和不变量违反

2. **Result 类型**：
   - 用于可恢复的错误
   - 强制处理错误情况
   - 提供丰富的组合器方法

3. **错误传播**：
   - `?` 操作符简化错误传播
   - 自动类型转换（通过 From trait）
   - 支持 Result 和 Option

4. **自定义错误**：
   - 实现 Error trait
   - 错误链支持
   - 考虑使用 thiserror 等库

5. **RWO 权限分析**：
   - panic! 会正确清理所有权（调用 drop）
   - Result 传递错误的所有权
   - 错误处理函数通常只需要读权限
   - `?` 操作符转移错误所有权

良好的错误处理是健壮程序的基础，Rust 的错误处理机制既安全又高效。下一章我们将学习项目管理和模块系统。 