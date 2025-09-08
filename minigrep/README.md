# Minigrep - Rust 学习项目

🦀 一个用于学习 Rust 编程语言的文本搜索工具

## 📖 项目简介

`minigrep` 是一个简化版的 `grep` 命令行工具，专门设计用于学习和演示 Rust 编程语言的核心概念。这个项目涵盖了 Rust 的所有重要特性，从基础语法到高级概念。

## 🎯 学习目标

通过这个项目，你将学习到以下 Rust 概念：

### 1. 变量和数据类型
- **不可变变量**：Rust 默认变量不可变
- **可变变量**：使用 `mut` 关键字
- **基本类型**：`i32`, `f64`, `bool`, `char`
- **复合类型**：元组 `(T1, T2)` 和数组 `[T; N]`
- **字符串类型**：`&str` (字符串切片) 和 `String` (拥有的字符串)

```rust
// 不可变变量
let x = 5;

// 可变变量
let mut y = 10;
y = 15;

// 类型注解
let z: i32 = 42;

// 字符串类型
let slice: &str = "Hello";
let owned: String = String::from("World");
```

### 2. 函数
- **函数定义**：使用 `fn` 关键字
- **参数和返回值**：类型注解和返回类型
- **表达式 vs 语句**：Rust 是基于表达式的语言

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // 表达式，自动返回
}

fn greet(name: &str) {
    println!("Hello, {}!", name);  // 语句
}
```

### 3. 流程控制
- **条件语句**：`if`, `else if`, `else`
- **模式匹配**：`match` 表达式
- **循环**：`for`, `while`, `loop`

```rust
// if 表达式
let number = if condition { 5 } else { 6 };

// match 表达式
match value {
    1 => println!("One"),
    2 | 3 => println!("Two or Three"),
    _ => println!("Something else"),
}

// for 循环
for i in 0..10 {
    println!("{}", i);
}
```

### 4. 所有权系统 (Ownership)
- **所有权规则**：每个值都有一个所有者
- **移动语义**：所有权转移
- **借用**：引用而不获取所有权
- **生命周期**：引用的有效范围

```rust
// 所有权转移
let s1 = String::from("hello");
let s2 = s1;  // s1 不再有效

// 借用
let s3 = String::from("world");
let len = calculate_length(&s3);  // 借用 s3
// s3 仍然有效

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

### 5. 结构体 (Structs)
- **定义结构体**：自定义数据类型
- **方法**：结构体的关联函数
- **关联函数**：类似静态方法

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // 关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

### 6. 枚举和模式匹配
- **枚举定义**：多种可能的值
- **Option 类型**：处理可能为空的值
- **Result 类型**：错误处理

```rust
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Option 类型
let some_number = Some(5);
let absent_number: Option<i32> = None;

// Result 类型
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}
```

### 7. 集合类型
- **Vector**：动态数组
- **HashMap**：键值对映射
- **String**：UTF-8 字符串

```rust
use std::collections::HashMap;

// Vector
let mut v = Vec::new();
v.push(1);
v.push(2);
v.push(3);

// HashMap
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// 迭代器
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
```

### 8. 错误处理
- **panic!**：不可恢复的错误
- **Result<T, E>**：可恢复的错误
- **? 操作符**：错误传播

```rust
use std::fs::File;
use std::io::ErrorKind;

// 使用 match 处理 Result
let f = File::open("hello.txt");
let f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
        ErrorKind::NotFound => {
            panic!("File not found!");
        }
        other_error => {
            panic!("Problem opening file: {:?}", other_error);
        }
    },
};

// 使用 ? 操作符
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

### 9. 泛型、Trait 和生命周期
- **泛型**：代码复用
- **Trait**：共享行为
- **生命周期**：引用有效性

```rust
// 泛型结构体
struct Point<T> {
    x: T,
    y: T,
}

// Trait 定义
trait Summary {
    fn summarize(&self) -> String;
}

// Trait 实现
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// 生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

### 10. 模块系统
- **包 (Package)**：Cargo 功能，包含一个或多个 crate
- **Crate**：模块树，产生库或可执行文件
- **模块 (Module)**：组织代码，控制私有性
- **路径 (Path)**：命名项的方式

```rust
// lib.rs
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    
    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
```

## 🚀 快速开始

### 安装 Rust

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 重新加载环境
source ~/.cargo/env

# 验证安装
rustc --version
cargo --version
```

### 运行项目

```bash
# 克隆或下载项目
cd minigrep

# 构建项目
cargo build

# 运行测试
cargo test

# 运行程序
cargo run -- rust poem.txt

# 查看帮助
cargo run -- --help

# 运行示例
cargo run -- --example
```

## 📋 使用方法

### 基本搜索

```bash
# 在文件中搜索 "rust"
cargo run -- rust poem.txt

# 不区分大小写搜索
IGNORE_CASE=1 cargo run -- RUST poem.txt

# 显示行号
cargo run -- --line-numbers rust poem.txt

# 限制结果数量
cargo run -- --max=5 rust poem.txt
```

### 高级功能

```bash
# 精确匹配
cargo run -- --exact "Rust" poem.txt

# 查看版本
cargo run -- --version

# 查看帮助
cargo run -- --help

# 运行语法示例
cargo run -- --example
```

## 🏗️ 项目结构

```
minigrep/
├── Cargo.toml          # 项目配置文件
├── README.md           # 项目文档
├── src/
│   ├── lib.rs          # 库代码 - 核心功能
│   └── main.rs         # 主程序 - 命令行接口
├── tests/
│   └── integration_tests.rs  # 集成测试
├── poem.txt            # 示例文本文件
└── story.txt           # 另一个示例文件
```

### 核心模块说明

#### `lib.rs` - 核心库
- **Config 结构体**：配置管理
- **SearchResult 结构体**：搜索结果
- **SearchMode 枚举**：搜索模式
- **SearchStats 结构体**：统计信息
- **搜索函数**：核心搜索逻辑
- **错误处理**：自定义错误类型

#### `main.rs` - 主程序
- **命令行参数解析**
- **用户界面**
- **程序流程控制**
- **示例代码演示**
- **性能测试**

## 🧪 测试

项目包含全面的测试套件：

```bash
# 运行所有测试
cargo test

# 运行单元测试
cargo test --lib

# 运行集成测试
cargo test --test integration_tests

# 运行特定测试
cargo test test_search_case_sensitive

# 显示测试输出
cargo test -- --nocapture

# 运行文档测试
cargo test --doc
```

### 测试覆盖的功能
- ✅ 配置解析和验证
- ✅ 各种搜索模式
- ✅ 文件操作
- ✅ 错误处理
- ✅ 边界条件
- ✅ 性能测试
- ✅ Unicode 支持

## 📚 Rust 概念详解

### 内存安全

Rust 通过所有权系统在编译时保证内存安全：

```rust
// 这会编译错误 - 使用已移动的值
let s1 = String::from("hello");
let s2 = s1;
// println!("{}", s1); // 错误！s1 已被移动

// 正确的方式 - 借用
let s1 = String::from("hello");
let s2 = &s1;  // 借用
println!("{} {}", s1, s2);  // 正确！
```

### 零成本抽象

Rust 的抽象不会带来运行时开销：

```rust
// 这个迭代器链会被编译器优化为简单的循环
let sum: i32 = (0..1_000_000)
    .filter(|x| x % 2 == 0)
    .map(|x| x * x)
    .sum();
```

### 并发安全

Rust 的类型系统防止数据竞争：

```rust
use std::thread;
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}
```

## 🔧 高级特性

### 自定义 Derive

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
```

### 宏 (Macros)

```rust
// 声明式宏
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

### 异步编程

```rust
use tokio;

#[tokio::main]
async fn main() {
    let result = fetch_data().await;
    println!("Result: {:?}", result);
}

async fn fetch_data() -> Result<String, Box<dyn std::error::Error>> {
    // 异步操作
    Ok("Data".to_string())
}
```

## 🎨 最佳实践

### 1. 错误处理

```rust
// 使用 Result 类型
fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse()
}

// 使用 ? 操作符传播错误
fn process_numbers(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let num1 = parse_number(input)?;
    let num2 = parse_number("42")?;
    Ok(num1 + num2)
}
```

### 2. 迭代器使用

```rust
// 函数式编程风格
let results: Vec<_> = data
    .iter()
    .filter(|&&x| x > 0)
    .map(|&x| x * 2)
    .collect();

// 链式调用
let sum: i32 = numbers
    .iter()
    .filter(|&&x| x % 2 == 0)
    .sum();
```

### 3. 模式匹配

```rust
// 解构
match point {
    Point { x: 0, y } => println!("On the y axis at {}", y),
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
}

// if let 简化
if let Some(value) = optional_value {
    println!("Got: {}", value);
}
```

## 🔍 性能优化

### 1. 避免不必要的分配

```rust
// 好：使用字符串切片
fn process_text(text: &str) -> &str {
    text.trim()
}

// 避免：不必要的 String 分配
fn process_text_bad(text: &str) -> String {
    text.trim().to_string()  // 不必要的分配
}
```

### 2. 使用适当的集合类型

```rust
// 已知大小时使用数组
let fixed_size: [i32; 5] = [1, 2, 3, 4, 5];

// 动态大小时使用 Vec
let mut dynamic: Vec<i32> = Vec::with_capacity(100);  // 预分配容量

// 键值对使用 HashMap
use std::collections::HashMap;
let mut map: HashMap<String, i32> = HashMap::new();
```

## 🛠️ 开发工具

### Cargo 命令

```bash
# 创建新项目
cargo new my_project
cargo new --lib my_library

# 构建和运行
cargo build          # 调试构建
cargo build --release  # 发布构建
cargo run            # 运行
cargo test           # 测试

# 代码检查
cargo check          # 快速检查
cargo clippy         # 代码质量检查
cargo fmt            # 代码格式化

# 文档
cargo doc            # 生成文档
cargo doc --open     # 生成并打开文档
```

### 有用的工具

```bash
# 安装额外工具
rustup component add clippy  # 代码检查工具
rustup component add rustfmt # 代码格式化工具

# 性能分析
cargo install flamegraph
cargo flamegraph --bin minigrep

# 基准测试
cargo install criterion
```

## 📖 学习资源

### 官方资源
- [Rust 官方网站](https://www.rust-lang.org/)
- [Rust 程序设计语言](https://doc.rust-lang.org/book/)
- [Rust 标准库文档](https://doc.rust-lang.org/std/)
- [Rust 参考手册](https://doc.rust-lang.org/reference/)

### 社区资源
- [Rust 用户论坛](https://users.rust-lang.org/)
- [Rust 官方 Discord](https://discord.gg/rust-lang)
- [This Week in Rust](https://this-week-in-rust.org/)
- [Awesome Rust](https://github.com/rust-unofficial/awesome-rust)

### 练习项目
- [Rustlings](https://github.com/rust-lang/rustlings) - 小练习
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - 示例代码
- [Exercism Rust Track](https://exercism.org/tracks/rust) - 编程练习

## 🤝 贡献

欢迎贡献代码、报告问题或提出改进建议！

### 开发流程

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 打开 Pull Request

### 代码规范

```bash
# 运行所有检查
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- Rust 团队和社区
- 《Rust 程序设计语言》一书的作者
- 所有为 Rust 生态系统做出贡献的开发者

---

**Happy Coding with Rust! 🦀**

如果你觉得这个项目有帮助，请给它一个 ⭐️！