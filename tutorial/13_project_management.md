# 第13章：项目管理

Rust 提供了强大的模块系统来组织代码，包括 crate、模块、路径和可见性控制。

## 13.1 crate 与模块的基本概念

### 什么是 crate

crate 是 Rust 的编译单元，分为两种类型：

```rust
// 二进制 crate - 有 main 函数，编译成可执行文件
// src/main.rs
fn main() {
    println!("Hello, world!");
}

// 库 crate - 没有 main 函数，编译成库文件
// src/lib.rs
pub fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```

### 包（Package）

一个包包含一个或多个 crate：

```toml
# Cargo.toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
tokio = { version = "1", features = ["full"] }

[dev-dependencies]
criterion = "0.4"

[[bin]]
name = "my_app"
path = "src/main.rs"

[[bin]]
name = "another_app"
path = "src/bin/another.rs"
```

### 模块基础

```rust
// 定义模块
mod front_of_house {
    // 公开的子模块
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("添加到等待列表");
        }
        
        fn seat_at_table() {
            println!("安排座位");
        }
    }
    
    // 私有的子模块
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// 使用模块
pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    
    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
```

### 模块树结构

```rust
// src/lib.rs
mod garden {
    pub mod vegetables {
        #[derive(Debug)]
        pub struct Asparagus {
            pub name: String,
            id: u32,  // 私有字段
        }
        
        impl Asparagus {
            pub fn new(name: &str) -> Self {
                Asparagus {
                    name: name.to_string(),
                    id: 1,
                }
            }
            
            pub fn id(&self) -> u32 {
                self.id
            }
        }
    }
    
    pub mod fruits {
        #[derive(Debug)]
        pub enum Apple {
            Red,
            Green,
        }
    }
}

use garden::vegetables::Asparagus;
use garden::fruits::Apple;

pub fn harvest() {
    let plant = Asparagus::new("绿芦笋");
    println!("收获: {:?}", plant);
    
    let fruit = Apple::Red;
    println!("水果: {:?}", fruit);
}
```

## 13.2 模块的访问控制

### 可见性规则

```rust
mod outer {
    pub fn public_function() {
        println!("从外部可以调用");
    }
    
    fn private_function() {
        println!("只能在模块内部调用");
    }
    
    pub mod inner {
        pub fn inner_public() {
            super::private_function(); // 可以访问父模块的私有项
        }
        
        pub(super) fn inner_restricted() {
            println!("只能在父模块中访问");
        }
        
        pub(crate) fn crate_visible() {
            println!("在整个 crate 内可见");
        }
    }
}

fn main() {
    outer::public_function();        // ✓ 可以
    // outer::private_function();    // ✗ 不可以
    outer::inner::inner_public();    // ✓ 可以
    // outer::inner::inner_restricted(); // ✗ 不可以在这里调用
    outer::inner::crate_visible();   // ✓ 可以（同一个 crate）
}
```

### 结构体和枚举的可见性

```rust
mod shapes {
    // 公开的结构体，但字段可以有不同的可见性
    pub struct Rectangle {
        pub width: f64,     // 公开字段
        pub height: f64,    // 公开字段
        id: u32,           // 私有字段
    }
    
    impl Rectangle {
        pub fn new(width: f64, height: f64) -> Self {
            Rectangle {
                width,
                height,
                id: rand::random(),
            }
        }
        
        pub fn area(&self) -> f64 {
            self.width * self.height
        }
        
        // 私有方法
        fn validate(&self) -> bool {
            self.width > 0.0 && self.height > 0.0
        }
    }
    
    // 枚举的所有变体都与枚举本身有相同的可见性
    pub enum Color {
        Red,
        Green,
        Blue,
        Custom(u8, u8, u8),
    }
}

use shapes::{Rectangle, Color};

fn main() {
    let mut rect = Rectangle::new(10.0, 20.0);
    rect.width = 15.0;  // 可以修改公开字段
    // rect.id = 42;    // 错误！不能访问私有字段
    
    let color = Color::Custom(255, 128, 0);
}
```

### use 和重导出

```rust
mod utils {
    pub mod math {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }
        
        pub fn multiply(a: i32, b: i32) -> i32 {
            a * b
        }
    }
    
    pub mod string {
        pub fn reverse(s: &str) -> String {
            s.chars().rev().collect()
        }
    }
}

// 重导出
pub use utils::math;
pub use utils::string::reverse;

// 使用 as 重命名
use utils::math::add as math_add;

// 导入多个项
use utils::math::{add, multiply};

// 使用 self 和 super
use std::io::{self, Write};

// 嵌套路径
use std::{cmp::Ordering, collections::HashMap};

// glob 导入（谨慎使用）
use utils::math::*;

fn main() {
    let sum = math::add(5, 3);
    let reversed = reverse("hello");
    println!("Sum: {}, Reversed: {}", sum, reversed);
}
```

## 13.3 模块的文件组织

### 基本文件结构

```rust
// 文件结构：
// src/
// ├── main.rs
// ├── lib.rs
// ├── config.rs
// └── network/
//     ├── mod.rs
//     ├── client.rs
//     └── server.rs

// src/main.rs
mod config;
mod network;

use config::Config;
use network::client::connect;

fn main() {
    let config = Config::load();
    connect(&config.server_url);
}

// src/config.rs
pub struct Config {
    pub server_url: String,
    pub timeout: u64,
}

impl Config {
    pub fn load() -> Self {
        Config {
            server_url: "localhost:8080".to_string(),
            timeout: 30,
        }
    }
}

// src/network/mod.rs
pub mod client;
pub mod server;

// src/network/client.rs
pub fn connect(url: &str) {
    println!("连接到 {}", url);
}

// src/network/server.rs
pub fn start(port: u16) {
    println!("服务器启动在端口 {}", port);
}
```

### 新的模块文件命名方式

```rust
// 新的文件结构（Rust 2018+）：
// src/
// ├── main.rs
// └── network/
//     ├── client.rs
//     └── server.rs
// └── network.rs  // 代替 network/mod.rs

// src/network.rs
pub mod client;
pub mod server;

pub fn init() {
    println!("初始化网络模块");
}
```

### 工作空间（Workspace）

```toml
# 根目录的 Cargo.toml
[workspace]
members = [
    "app",
    "core",
    "utils",
]

# app/Cargo.toml
[package]
name = "app"
version = "0.1.0"

[dependencies]
core = { path = "../core" }
utils = { path = "../utils" }

# core/Cargo.toml
[package]
name = "core"
version = "0.1.0"

[dependencies]
utils = { path = "../utils" }
```

## RWO 权限分析

### 模块中的所有权规则

```rust
mod library {
    pub struct Book {
        title: String,
        available: bool,
    }
    
    impl Book {
        pub fn new(title: String) -> Self {
            Book {
                title,  // O: 获取 title 的所有权
                available: true,
            }
        }
        
        // R: 只需要读权限
        pub fn title(&self) -> &str {
            &self.title
        }
        
        // W: 需要写权限
        pub fn checkout(&mut self) {
            self.available = false;
        }
        
        // O: 消费所有权
        pub fn destroy(self) -> String {
            format!("销毁图书: {}", self.title)
        }
    }
    
    pub struct Library {
        books: Vec<Book>,
    }
    
    impl Library {
        pub fn new() -> Self {
            Library { books: Vec::new() }
        }
        
        // O: 获取 book 的所有权
        pub fn add_book(&mut self, book: Book) {
            self.books.push(book);
        }
        
        // R: 返回不可变引用
        pub fn find_book(&self, title: &str) -> Option<&Book> {
            self.books.iter().find(|b| b.title() == title)
        }
        
        // W: 返回可变引用
        pub fn find_book_mut(&mut self, title: &str) -> Option<&mut Book> {
            self.books.iter_mut().find(|b| b.title() == title)
        }
        
        // O: 移除并返回所有权
        pub fn remove_book(&mut self, title: &str) -> Option<Book> {
            let pos = self.books.iter().position(|b| b.title() == title)?;
            Some(self.books.remove(pos))
        }
    }
}

use library::{Book, Library};

fn library_example() {
    let mut lib = Library::new();
    
    // O: 创建并转移所有权
    let book1 = Book::new("Rust Programming".to_string());
    lib.add_book(book1);
    // book1 不再可用
    
    // R: 只读访问
    if let Some(book) = lib.find_book("Rust Programming") {
        println!("找到书籍: {}", book.title());
    }
    
    // W: 可变访问
    if let Some(book) = lib.find_book_mut("Rust Programming") {
        book.checkout();
    }
    
    // O: 取回所有权
    if let Some(book) = lib.remove_book("Rust Programming") {
        let msg = book.destroy();
        println!("{}", msg);
    }
}
```

### 模块间的权限传递

```rust
mod database {
    pub struct Connection {
        url: String,
    }
    
    impl Connection {
        pub fn new(url: String) -> Self {
            Connection { url }
        }
        
        // R: 借用 self，返回借用
        pub fn query<'a>(&'a self, sql: &str) -> QueryResult<'a> {
            println!("执行查询: {}", sql);
            QueryResult {
                conn: self,
                data: vec!["结果1".to_string(), "结果2".to_string()],
            }
        }
    }
    
    pub struct QueryResult<'a> {
        conn: &'a Connection,  // R: 保持对连接的引用
        data: Vec<String>,     // O: 拥有数据
    }
    
    impl<'a> QueryResult<'a> {
        pub fn fetch_all(self) -> Vec<String> {
            self.data  // O: 转移数据所有权
        }
    }
}

mod service {
    use super::database::{Connection, QueryResult};
    
    pub struct UserService {
        db: Connection,  // O: 拥有数据库连接
    }
    
    impl UserService {
        pub fn new(db_url: String) -> Self {
            UserService {
                db: Connection::new(db_url),
            }
        }
        
        // R: 只需要不可变引用
        pub fn get_users(&self) -> Vec<String> {
            let result = self.db.query("SELECT * FROM users");
            result.fetch_all()
        }
    }
}
```

### 可见性与权限

```rust
mod secure {
    pub struct SecureData {
        data: String,  // 私有字段
    }
    
    impl SecureData {
        // O: 构造函数获取所有权
        pub fn new(data: String) -> Self {
            SecureData { data }
        }
        
        // R: 只读访问（受控）
        pub fn view(&self) -> &str {
            // 可以在这里添加访问控制逻辑
            &self.data
        }
        
        // 没有提供可变访问方法，保护数据完整性
        
        // O: 消费并转换
        pub fn into_bytes(self) -> Vec<u8> {
            self.data.into_bytes()
        }
    }
    
    // 使用 newtype 模式控制权限
    pub struct ReadOnly<T>(T);
    
    impl<T> ReadOnly<T> {
        pub fn new(value: T) -> Self {
            ReadOnly(value)
        }
        
        // 只提供不可变访问
        pub fn get(&self) -> &T {
            &self.0
        }
        
        // 不提供可变访问
    }
}
```

### 模块级别的权限设计

```rust
// API 设计示例
pub mod api {
    use std::sync::{Arc, Mutex};
    
    // 共享状态
    pub struct SharedState {
        inner: Arc<Mutex<StateInner>>,
    }
    
    struct StateInner {
        count: u64,
        data: Vec<String>,
    }
    
    impl SharedState {
        pub fn new() -> Self {
            SharedState {
                inner: Arc::new(Mutex::new(StateInner {
                    count: 0,
                    data: Vec::new(),
                })),
            }
        }
        
        // R: 克隆 Arc，共享所有权
        pub fn clone_handle(&self) -> Self {
            SharedState {
                inner: Arc::clone(&self.inner),
            }
        }
        
        // W: 通过 Mutex 提供安全的可变访问
        pub fn increment(&self) {
            let mut state = self.inner.lock().unwrap();
            state.count += 1;
        }
        
        // R: 只读访问
        pub fn count(&self) -> u64 {
            let state = self.inner.lock().unwrap();
            state.count
        }
    }
}

// 权限边界示例
mod boundary {
    // 入口点控制权限
    pub fn process_data(input: String) -> Result<String, String> {
        // O: 获取输入的所有权
        let validated = validate(input)?;
        let processed = transform(validated);
        Ok(finalize(processed))
    }
    
    // 内部函数不公开
    fn validate(data: String) -> Result<String, String> {
        if data.is_empty() {
            Err("数据不能为空".to_string())
        } else {
            Ok(data)
        }
    }
    
    fn transform(data: String) -> String {
        data.to_uppercase()
    }
    
    fn finalize(data: String) -> String {
        format!("[已处理] {}", data)
    }
}
```

### 生命周期与模块

```rust
mod cache {
    use std::collections::HashMap;
    
    pub struct Cache<'a> {
        data: HashMap<String, &'a str>,
    }
    
    impl<'a> Cache<'a> {
        pub fn new() -> Self {
            Cache {
                data: HashMap::new(),
            }
        }
        
        // R: 存储引用，不获取所有权
        pub fn insert(&mut self, key: String, value: &'a str) {
            self.data.insert(key, value);
        }
        
        // R: 返回引用的引用
        pub fn get(&self, key: &str) -> Option<&&'a str> {
            self.data.get(key)
        }
    }
    
    // 生命周期边界
    pub fn with_cache<'a, F>(values: &'a [&'a str], f: F) 
    where 
        F: FnOnce(&mut Cache<'a>)
    {
        let mut cache = Cache::new();
        for (i, &value) in values.iter().enumerate() {
            cache.insert(format!("item_{}", i), value);
        }
        f(&mut cache);
    }
}
```

## 小结

本章深入学习了 Rust 的项目管理和模块系统：

1. **基本概念**：
   - crate 是编译单元
   - package 包含一个或多个 crate
   - 模块用于组织代码结构

2. **访问控制**：
   - 默认私有，使用 `pub` 公开
   - 可以使用 `pub(crate)`、`pub(super)` 等细粒度控制
   - 结构体字段可以有独立的可见性

3. **文件组织**：
   - 模块可以内联定义或放在单独文件
   - 使用 `mod` 声明子模块
   - 工作空间管理多个相关包

4. **RWO 权限分析**：
   - **R**：模块边界不影响引用的传递
   - **W**：可变引用的传递需要考虑可见性
   - **O**：所有权可以跨模块转移
   - 良好的模块设计可以强制执行权限边界

模块系统是组织大型 Rust 项目的基础，合理的模块划分和权限控制可以提高代码的可维护性和安全性。下一章我们将学习文档和测试。 