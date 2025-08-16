// 教学示例 - 允许未使用的代码
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// 第7章：枚举
// 枚举是一种定义类型的方式，该类型可以是几个可能的变体之一

use std::fmt;

// 基本枚举定义
#[derive(Debug, Clone, PartialEq)]
enum IpAddrKind {
    V4,
    V6,
}

// 带数据的枚举
#[derive(Debug, Clone)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 复杂枚举
#[derive(Debug, Clone)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 为枚举实现方法
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("退出消息"),
            Message::Move { x, y } => println!("移动到坐标 ({}, {})", x, y),
            Message::Write(text) => println!("写入文本: {}", text),
            Message::ChangeColor(r, g, b) => println!("改变颜色为 RGB({}, {}, {})", r, g, b),
        }
    }
    
    fn is_quit(&self) -> bool {
        matches!(self, Message::Quit)
    }
}

// 自定义错误类型
#[derive(Debug, Clone)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "不能除以零"),
            MathError::NegativeSquareRoot => write!(f, "负数不能开平方根"),
            MathError::Overflow => write!(f, "数值溢出"),
        }
    }
}

// 使用 Result 的计算函数
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

// 状态机示例
#[derive(Debug, Clone, PartialEq)]
enum State {
    Idle,
    Running,
    Paused,
    Stopped,
}

struct StateMachine {
    state: State,
    name: String,
}

impl StateMachine {
    fn new(name: &str) -> Self {
        StateMachine {
            state: State::Idle,
            name: name.to_string(),
        }
    }
    
    fn start(&mut self) {
        match self.state {
            State::Idle => {
                self.state = State::Running;
                println!("{} 开始运行", self.name);
            }
            State::Paused => {
                self.state = State::Running;
                println!("{} 恢复运行", self.name);
            }
            _ => println!("{} 无法从当前状态启动", self.name),
        }
    }
    
    fn pause(&mut self) {
        if self.state == State::Running {
            self.state = State::Paused;
            println!("{} 暂停", self.name);
        } else {
            println!("{} 无法暂停", self.name);
        }
    }
    
    fn stop(&mut self) {
        match self.state {
            State::Running | State::Paused => {
                self.state = State::Stopped;
                println!("{} 停止", self.name);
            }
            _ => println!("{} 无法停止", self.name),
        }
    }
}

fn main() {
    println!("=== 第7章：枚举 ===\n");
    
    // 1. 基本枚举使用
    println!("1. 基本枚举使用：");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("IPv4: {:?}", four);
    println!("IPv6: {:?}", six);
    println!();
    
    // 2. 带数据的枚举
    println!("2. 带数据的枚举：");
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("本地地址: {:?}", home);
    println!("回环地址: {:?}", loopback);
    println!();
    
    // 3. 复杂枚举和方法
    println!("3. 复杂枚举和方法：");
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello, Rust!")),
        Message::ChangeColor(255, 0, 0),
    ];
    
    for message in &messages {
        message.call();
        println!("是否为退出消息: {}", message.is_quit());
    }
    println!();
    
    // 4. 模式匹配
    println!("4. 模式匹配：");
    for message in &messages {
        match message {
            Message::Quit => println!("处理退出"),
            Message::Move { x, y } if *x > 5 => println!("大幅移动到 ({}, {})", x, y),
            Message::Move { x, y } => println!("小幅移动到 ({}, {})", x, y),
            Message::Write(text) if text.len() > 10 => println!("长文本: {}", text),
            Message::Write(text) => println!("短文本: {}", text),
            Message::ChangeColor(r, g, b) => {
                let color_name = match (r, g, b) {
                    (255, 0, 0) => "红色",
                    (0, 255, 0) => "绿色",
                    (0, 0, 255) => "蓝色",
                    _ => "其他颜色",
                };
                println!("改变为{}", color_name);
            }
        }
    }
    println!();
    
    // 5. Option 的使用
    println!("5. Option 的使用：");
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    // 使用 match 处理 Option
    match some_number {
        Some(n) => println!("数字是: {}", n),
        None => println!("没有数字"),
    }
    
    // 使用 if let
    if let Some(s) = some_string {
        println!("字符串是: {}", s);
    }
    
    // 使用 Option 的方法
    let doubled = some_number.map(|x| x * 2);
    println!("翻倍后: {:?}", doubled);
    
    let default_value = absent_number.unwrap_or(42);
    println!("默认值: {}", default_value);
    println!();
    
    // 6. Result 的使用
    println!("6. Result 的使用：");
    let calculations = vec![
        (10.0, 2.0),
        (5.0, 0.0),   // 除零错误
        (16.0, 4.0),
    ];
    
    for (a, b) in calculations {
        match safe_divide(a, b) {
            Ok(result) => {
                println!("{} / {} = {}", a, b, result);
                
                // 链式操作：先除法，再开平方根
                match safe_sqrt(result) {
                    Ok(sqrt_result) => println!("√{} = {}", result, sqrt_result),
                    Err(e) => println!("开平方根错误: {}", e),
                }
            }
            Err(e) => println!("除法错误: {}", e),
        }
    }
    println!();
    
    // 7. Result 的链式操作
    println!("7. Result 的链式操作：");
    let chain_result = safe_divide(16.0, 4.0)
        .and_then(|x| safe_sqrt(x))
        .map(|x| x * 2.0);
    
    match chain_result {
        Ok(result) => println!("链式计算结果: {}", result),
        Err(e) => println!("链式计算错误: {}", e),
    }
    
    // 使用 ? 操作符的函数示例
    fn calculate_and_sqrt(a: f64, b: f64) -> Result<f64, MathError> {
        let division_result = safe_divide(a, b)?;
        let sqrt_result = safe_sqrt(division_result)?;
        Ok(sqrt_result * 2.0)
    }
    
    match calculate_and_sqrt(36.0, 4.0) {
        Ok(result) => println!("使用 ? 操作符的结果: {}", result),
        Err(e) => println!("使用 ? 操作符的错误: {}", e),
    }
    println!();
    
    // 8. 状态机示例
    println!("8. 状态机示例：");
    let mut machine = StateMachine::new("任务处理器");
    println!("初始状态: {:?}", machine.state);
    
    machine.start();
    println!("当前状态: {:?}", machine.state);
    
    machine.pause();
    println!("当前状态: {:?}", machine.state);
    
    machine.start();
    println!("当前状态: {:?}", machine.state);
    
    machine.stop();
    println!("当前状态: {:?}", machine.state);
    println!();
    
    // 9. 枚举的内存布局
    println!("9. 枚举的内存布局：");
    println!("IpAddrKind 大小: {} bytes", std::mem::size_of::<IpAddrKind>());
    println!("IpAddr 大小: {} bytes", std::mem::size_of::<IpAddr>());
    println!("Message 大小: {} bytes", std::mem::size_of::<Message>());
    println!("Option<i32> 大小: {} bytes", std::mem::size_of::<Option<i32>>());
    println!("Result<i32, String> 大小: {} bytes", std::mem::size_of::<Result<i32, String>>());
    println!();
    
    // 10. 实用的枚举模式
    println!("10. 实用的枚举模式：");
    
    // 使用枚举表示配置
    #[derive(Debug)]
    enum LogLevel {
        Debug,
        Info,
        Warning,
        Error,
    }
    
    fn log_message(level: LogLevel, message: &str) {
        let prefix = match level {
            LogLevel::Debug => "[DEBUG]",
            LogLevel::Info => "[INFO]",
            LogLevel::Warning => "[WARNING]",
            LogLevel::Error => "[ERROR]",
        };
        println!("{} {}", prefix, message);
    }
    
    log_message(LogLevel::Info, "程序启动");
    log_message(LogLevel::Warning, "配置文件不存在，使用默认配置");
    log_message(LogLevel::Error, "连接数据库失败");
    
    println!("\n=== 第7章完成 ===");
} 