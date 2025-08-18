// 教学示例 - 允许未使用的代码
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// 第11章：常用的特征解析示例
// 运行命令：cargo run --bin common_traits

use std::fmt::{self, Display, Debug};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;

fn main() {
    println!("🔹 第11章：常用的特征解析 (Common Traits)");
    println!("{}", "=".repeat(50));
    
    display_debug_traits();
    println!();
    
    clone_copy_traits();
    println!();
    
    equality_traits();
    println!();
    
    ordering_traits();
    println!();
    
    hash_trait();
    println!();
    
    iterator_trait();
    println!();
    
    conversion_traits();
    println!();
    
    default_trait();
    println!();
    
    drop_trait();
}

/// 11.1 Display 和 Debug 特征
fn display_debug_traits() {
    println!("📝 11.1 Display 和 Debug 特征");
    println!("{}", "-".repeat(30));
    
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    impl Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    
    let p = Point { x: 10, y: 20 };
    
    // Display 用于用户友好的输出
    println!("Display 格式: {}", p);
    
    // Debug 用于调试输出
    println!("Debug 格式: {:?}", p);
    println!("Debug 美化格式: {:#?}", p);
    
    // 自定义 Debug 实现
    struct Circle {
        center: Point,
        radius: f64,
    }
    
    impl Debug for Circle {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Circle")
                .field("center", &self.center)
                .field("radius", &self.radius)
                .finish()
        }
    }
    
    impl Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "圆心{}, 半径{}", self.center, self.radius)
        }
    }
    
    let circle = Circle {
        center: Point { x: 0, y: 0 },
        radius: 5.0,
    };
    
    println!("圆形 Display: {}", circle);
    println!("圆形 Debug: {:?}", circle);
}

/// 11.2 Clone 和 Copy 特征
fn clone_copy_traits() {
    println!("📝 11.2 Clone 和 Copy 特征");
    println!("{}", "-".repeat(30));
    
    // Copy 类型 - 栈上的简单数据
    #[derive(Debug, Copy, Clone)]
    struct Point2D {
        x: f64,
        y: f64,
    }
    
    let p1 = Point2D { x: 1.0, y: 2.0 };
    let p2 = p1; // 自动复制
    
    println!("Copy 示例:");
    println!("p1: {:?}", p1); // p1 仍然可用
    println!("p2: {:?}", p2);
    
    // Clone 类型 - 需要显式克隆
    #[derive(Debug, Clone)]
    struct Person {
        name: String,
        age: u32,
        hobbies: Vec<String>,
    }
    
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
        hobbies: vec![String::from("读书"), String::from("游泳")],
    };
    
    let person2 = person1.clone(); // 显式克隆
    
    println!("Clone 示例:");
    println!("person1: {:?}", person1);
    println!("person2: {:?}", person2);
    
    // 自定义 Clone 实现
    #[derive(Debug)]
    struct Counter {
        value: i32,
        name: String,
    }
    
    impl Clone for Counter {
        fn clone(&self) -> Self {
            println!("克隆计数器: {}", self.name);
            Counter {
                value: self.value,
                name: format!("{}_copy", self.name),
            }
        }
    }
    
    let counter1 = Counter {
        value: 42,
        name: String::from("main"),
    };
    
    let counter2 = counter1.clone();
    println!("自定义 Clone:");
    println!("counter1: {:?}", counter1);
    println!("counter2: {:?}", counter2);
}

/// 11.3 PartialEq 和 Eq 特征
fn equality_traits() {
    println!("📝 11.3 PartialEq 和 Eq 特征");
    println!("{}", "-".repeat(30));
    
    // 自动派生 PartialEq
    #[derive(Debug, PartialEq)]
    struct Version {
        major: u32,
        minor: u32,
        patch: u32,
    }
    
    let v1 = Version { major: 1, minor: 0, patch: 0 };
    let v2 = Version { major: 1, minor: 0, patch: 0 };
    let v3 = Version { major: 2, minor: 0, patch: 0 };
    
    println!("版本比较:");
    println!("v1 == v2: {}", v1 == v2);
    println!("v1 == v3: {}", v1 == v3);
    println!("v1 != v3: {}", v1 != v3);
    
    // 自定义 PartialEq 实现
    #[derive(Debug)]
    struct CaseInsensitiveString(String);
    
    impl PartialEq for CaseInsensitiveString {
        fn eq(&self, other: &Self) -> bool {
            self.0.to_lowercase() == other.0.to_lowercase()
        }
    }
    
    let s1 = CaseInsensitiveString(String::from("Hello"));
    let s2 = CaseInsensitiveString(String::from("HELLO"));
    let s3 = CaseInsensitiveString(String::from("World"));
    
    println!("大小写不敏感字符串比较:");
    println!("\"Hello\" == \"HELLO\": {}", s1 == s2);
    println!("\"Hello\" == \"World\": {}", s1 == s3);
    
    // Eq 特征 - 完全等价关系
    #[derive(Debug, PartialEq, Eq, Hash)]
    struct UserId(u64);
    
    let id1 = UserId(1001);
    let id2 = UserId(1001);
    let id3 = UserId(1002);
    
    println!("用户ID比较:");
    println!("id1 == id2: {}", id1 == id2);
    println!("id1 == id3: {}", id1 == id3);
    
    // 浮点数只实现 PartialEq，不实现 Eq
    let nan = f64::NAN;
    println!("NaN == NaN: {}", nan == nan); // false - 违反自反性
}

/// 11.4 PartialOrd 和 Ord 特征
fn ordering_traits() {
    println!("📝 11.4 PartialOrd 和 Ord 特征");
    println!("{}", "-".repeat(30));
    
    // 自动派生排序
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Priority(i32);
    
    let p1 = Priority(1);
    let p2 = Priority(2);
    let p3 = Priority(1);
    
    println!("优先级比较:");
    println!("p1 < p2: {}", p1 < p2);
    println!("p1 <= p3: {}", p1 <= p3);
    println!("p2 > p1: {}", p2 > p1);
    
    let mut priorities = vec![Priority(3), Priority(1), Priority(2)];
    priorities.sort();
    println!("排序后的优先级: {:?}", priorities);
    
    // 自定义排序实现
    #[derive(Debug, PartialEq, Eq)]
    struct Student {
        name: String,
        age: u32,
        grade: u32, // 使用整数成绩避免浮点数的Eq问题
    }
    
    impl PartialOrd for Student {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    
    impl Ord for Student {
        fn cmp(&self, other: &Self) -> Ordering {
            // 先按成绩排序（降序），再按年龄排序（升序），最后按姓名排序
            other.grade.cmp(&self.grade) // 交换顺序实现降序
                .then_with(|| self.age.cmp(&other.age)) // 年龄升序
                .then_with(|| self.name.cmp(&other.name)) // 姓名升序
        }
    }
    
    let mut students = vec![
        Student { name: "Alice".to_string(), age: 20, grade: 85 },
        Student { name: "Bob".to_string(), age: 19, grade: 92 },
        Student { name: "Charlie".to_string(), age: 20, grade: 85 },
    ];
    
    println!("排序前的学生:");
    for student in &students {
        println!("  {} ({}岁, 成绩: {})", student.name, student.age, student.grade);
    }
    
    students.sort();
    
    println!("排序后的学生 (按成绩降序, 年龄升序, 姓名升序):");
    for student in &students {
        println!("  {} ({}岁, 成绩: {})", student.name, student.age, student.grade);
    }
}

/// 11.5 Hash 特征
fn hash_trait() {
    println!("📝 11.5 Hash 特征");
    println!("{}", "-".repeat(30));
    
    // 自动派生 Hash
    #[derive(Debug, Hash, PartialEq, Eq)]
    struct BookId {
        isbn: String,
        edition: u32,
    }
    
    let mut book_ratings = HashMap::new();
    
    let book1 = BookId {
        isbn: "978-1-234-56789-0".to_string(),
        edition: 1,
    };
    
    let book2 = BookId {
        isbn: "978-1-234-56789-0".to_string(),
        edition: 2,
    };
    
    book_ratings.insert(book1, 4.5);
    book_ratings.insert(book2, 4.8);
    
    println!("图书评分:");
    for (book, rating) in &book_ratings {
        println!("  {:?}: {}", book, rating);
    }
    
    // 自定义 Hash 实现
    #[derive(Debug, PartialEq, Eq)]
    struct CaseInsensitiveKey(String);
    
    impl Hash for CaseInsensitiveKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.to_lowercase().hash(state);
        }
    }
    
    let mut case_map = HashMap::new();
    case_map.insert(CaseInsensitiveKey("Hello".to_string()), 1);
    case_map.insert(CaseInsensitiveKey("HELLO".to_string()), 2);
    case_map.insert(CaseInsensitiveKey("World".to_string()), 3);
    
    println!("大小写不敏感的映射 (应该只有2个条目):");
    for (key, value) in &case_map {
        println!("  {:?}: {}", key, value);
    }
    println!("映射大小: {}", case_map.len());
}

/// 11.6 Iterator 特征
fn iterator_trait() {
    println!("📝 11.6 Iterator 特征");
    println!("{}", "-".repeat(30));
    
    // 自定义迭代器
    struct Counter {
        current: u32,
        max: u32,
    }
    
    impl Counter {
        fn new(max: u32) -> Self {
            Counter { current: 0, max }
        }
    }
    
    impl Iterator for Counter {
        type Item = u32;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.max {
                self.current += 1;
                Some(self.current)
            } else {
                None
            }
        }
    }
    
    let counter = Counter::new(5);
    println!("计数器迭代:");
    for num in counter {
        println!("  {}", num);
    }
    
    // 迭代器方法链
    let numbers: Vec<i32> = (1..=10).collect();
    println!("原始数字: {:?}", numbers);
    
    let result: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0) // 过滤偶数
        .map(|&x| x * x) // 平方
        .collect();
    
    println!("偶数的平方: {:?}", result);
    
    // fold 和 reduce
    let sum: i32 = (1..=10).fold(0, |acc, x| acc + x);
    println!("1到10的和: {}", sum);
    
    let product: Option<i32> = (1..=5).reduce(|acc, x| acc * x);
    println!("1到5的积: {:?}", product);
    
    // 自定义迭代器适配器
    struct Skip<I> {
        iter: I,
        n: usize,
    }
    
    impl<I: Iterator> Iterator for Skip<I> {
        type Item = I::Item;
        
        fn next(&mut self) -> Option<Self::Item> {
            for _ in 0..self.n {
                self.iter.next()?;
            }
            self.iter.next()
        }
    }
    
    let numbers: Vec<i32> = (1..=10).collect();
    let skip_iter = Skip { iter: numbers.iter(), n: 2 };
    let skipped: Vec<&i32> = skip_iter.collect();
    println!("每隔2个取一个: {:?}", skipped);
}

/// 11.7 From 和 Into 特征
fn conversion_traits() {
    println!("📝 11.7 From 和 Into 特征");
    println!("{}", "-".repeat(30));
    
    // 基本类型转换
    let num: i32 = 42;
    let big_num: i64 = i64::from(num);
    println!("i32 转 i64: {} -> {}", num, big_num);
    
    // 自定义转换
    #[derive(Debug)]
    struct Celsius(f64);
    
    #[derive(Debug)]
    struct Fahrenheit(f64);
    
    impl From<Celsius> for Fahrenheit {
        fn from(c: Celsius) -> Self {
            Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
        }
    }
    
    impl From<Fahrenheit> for Celsius {
        fn from(f: Fahrenheit) -> Self {
            Celsius((f.0 - 32.0) * 5.0 / 9.0)
        }
    }
    
    let temp_c = Celsius(25.0);
    let temp_f = Fahrenheit::from(temp_c);
    println!("摄氏度转华氏度: {:?} -> {:?}", Celsius(25.0), temp_f);
    
    // Into 自动实现
    let temp_c2 = Celsius(0.0);
    let temp_f2: Fahrenheit = temp_c2.into();
    println!("使用 Into: {:?} -> {:?}", Celsius(0.0), temp_f2);
    
    // 错误转换
    #[derive(Debug)]
    enum ParseError {
        InvalidFormat,
        OutOfRange,
    }
    
    impl std::fmt::Display for ParseError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                ParseError::InvalidFormat => write!(f, "无效格式"),
                ParseError::OutOfRange => write!(f, "超出范围"),
            }
        }
    }
    
    impl std::error::Error for ParseError {}
    
    #[derive(Debug)]
    enum MyError {
        Parse(ParseError),
        Io(std::io::Error),
    }
    
    impl From<ParseError> for MyError {
        fn from(err: ParseError) -> Self {
            MyError::Parse(err)
        }
    }
    
    fn might_fail() -> Result<i32, MyError> {
        // ? 操作符自动使用 From 转换错误类型
        Err(ParseError::InvalidFormat)?
    }
    
    match might_fail() {
        Ok(val) => println!("成功: {}", val),
        Err(e) => println!("错误转换示例: {:?}", e),
    }
}

/// 11.8 Default 特征
fn default_trait() {
    println!("📝 11.8 Default 特征");
    println!("{}", "-".repeat(30));
    
    // 自动派生 Default
    #[derive(Debug, Default)]
    struct Config {
        host: String,
        port: u16,
        timeout: u64,
        debug: bool,
    }
    
    let default_config = Config::default();
    println!("默认配置: {:?}", default_config);
    
    // 自定义 Default 实现
    #[derive(Debug)]
    struct DatabaseConfig {
        url: String,
        max_connections: u32,
        timeout: std::time::Duration,
    }
    
    impl Default for DatabaseConfig {
        fn default() -> Self {
            DatabaseConfig {
                url: "localhost:5432".to_string(),
                max_connections: 10,
                timeout: std::time::Duration::from_secs(30),
            }
        }
    }
    
    let db_config = DatabaseConfig::default();
    println!("数据库默认配置: {:?}", db_config);
    
    // 构建器模式使用 Default
    #[derive(Debug, Default)]
    struct ServerBuilder {
        host: Option<String>,
        port: Option<u16>,
        workers: Option<usize>,
    }
    
    impl ServerBuilder {
        fn new() -> Self {
            Self::default()
        }
        
        fn host(mut self, host: String) -> Self {
            self.host = Some(host);
            self
        }
        
        fn port(mut self, port: u16) -> Self {
            self.port = Some(port);
            self
        }
        
        fn workers(mut self, workers: usize) -> Self {
            self.workers = Some(workers);
            self
        }
        
        fn build(self) -> ServerConfig {
            ServerConfig {
                host: self.host.unwrap_or_else(|| "127.0.0.1".to_string()),
                port: self.port.unwrap_or(8080),
                workers: self.workers.unwrap_or(4),
            }
        }
    }
    
    #[derive(Debug)]
    struct ServerConfig {
        host: String,
        port: u16,
        workers: usize,
    }
    
    let server = ServerBuilder::new()
        .host("0.0.0.0".to_string())
        .port(3000)
        .build();
    
    println!("服务器配置: {:?}", server);
}

/// 11.9 Drop 特征
fn drop_trait() {
    println!("📝 11.9 Drop 特征");
    println!("{}", "-".repeat(30));
    
    struct FileWrapper {
        name: String,
    }
    
    impl FileWrapper {
        fn new(name: String) -> Self {
            println!("打开文件: {}", name);
            FileWrapper { name }
        }
    }
    
    impl Drop for FileWrapper {
        fn drop(&mut self) {
            println!("关闭文件: {}", self.name);
        }
    }
    
    {
        let _file1 = FileWrapper::new("config.txt".to_string());
        let _file2 = FileWrapper::new("data.txt".to_string());
        println!("文件使用中...");
    } // 文件按 LIFO 顺序自动关闭
    
    println!("文件已关闭");
    
    // RAII 模式示例
    struct Guard<'a> {
        name: &'a str,
        value: &'a mut i32,
        original: i32,
    }
    
    impl<'a> Guard<'a> {
        fn new(name: &'a str, value: &'a mut i32) -> Self {
            let original = *value;
            *value = 0; // 重置值
            println!("Guard '{}' 激活，原值: {}", name, original);
            Guard { name, value, original }
        }
    }
    
    impl<'a> Drop for Guard<'a> {
        fn drop(&mut self) {
            *self.value = self.original; // 恢复原值
            println!("Guard '{}' 释放，恢复值: {}", self.name, self.original);
        }
    }
    
    let mut important_value = 42;
    println!("重要值: {}", important_value);
    
    {
        let _guard = Guard::new("保护器", &mut important_value);
        println!("保护期间值: {}", *_guard.value);
    }
    
    println!("保护结束后值: {}", important_value);
} 