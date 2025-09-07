### anyhow

`anyhow` 是 Rust 中一个流行的错误处理 crate，它提供了一个灵活的 `Error` 类型，基于 `std::error::Error`  trait 对象，旨在简化应用程序中的错误处理。不同于需要定义自定义错误枚举的库，`anyhow` 允许你轻松传播任何实现 `std::error::Error` 的错误，并添加上下文、回溯和临时错误消息。这使得它特别适合应用程序代码，而非库代码。

#### 1. 安装 anyhow
在你的 `Cargo.toml` 文件中添加依赖：

```toml
[dependencies]
anyhow = "1.0"  # 最新版本可从 crates.io 检查
```

对于 no-std 支持，禁用默认 "std" 特性：

```toml
[dependencies]
anyhow = { version = "1.0", default-features = false }
```

运行 `cargo build` 安装。注意：在 no-std 模式下，需要全局分配器，且在 Rust < 1.81 时，可能需要额外 `.map_err(Error::msg)` 处理非 anyhow 错误。

#### 2. 基本用法
`anyhow` 的核心是 `anyhow::Error`（一个动态错误类型）和 `anyhow::Result<T>`（`Result<T, Error>` 的别名）。语法简单：

- 使用 `Result<T>` 作为函数返回类型。
- 使用 `?` 操作符传播错误。
- 使用 `.context("消息")` 或 `.with_context(|| "动态消息")` 添加上下文。
- 使用 `anyhow!` 宏创建临时错误。
- 使用 `bail!` 宏提前返回错误。
- 使用 `ensure!` 宏检查条件并返回错误。

基本示例：

```rust
use anyhow::{bail, Context, Result};

fn example() -> Result<()> {
    let data = std::fs::read_to_string("file.txt").context("Failed to read file")?;
    if data.is_empty() {
        bail!("File is empty");
    }
    Ok(())
}
```

错误打印时会显示链式原因和上下文。

#### 3. 语义和实现细节
- **错误传播**：`?` 会自动将任何 `std::error::Error` 转换为 `anyhow::Error`。
- **上下文添加**：上下文是字符串或格式化消息，帮助调试。
- **链式错误**：`Error::chain()` 返回错误源的迭代器。
- **回溯**：在 Rust ≥ 1.65 中自动捕获回溯，使用环境变量如 `RUST_BACKTRACE=1` 显示（完整回溯）或 `RUST_LIB_BACKTRACE=1`（仅库回溯）。
- **下转**：使用 `downcast_ref`、`downcast_mut` 或 `downcast` 方法恢复原始错误类型。
- **性能**：`anyhow::Error` 是 trait 对象，分配在堆上；开销小，但对于高性能场景，可能不如枚举。

#### 4. 高级用法
- **自定义错误集成**：与 `thiserror` 结合定义自定义错误，然后传播到 `anyhow::Error`。
- **多线程**：`Error` 实现 `Send + Sync`，适合并发。
- **回溯控制**：自定义回溯捕获，或使用 `anyhow::backtrace` 特性。
- **格式化**：错误支持 `Display` 和 `Debug`，可自定义打印。
- **兼容性**：与 `std::io::Error`、`serde::de::Error` 等无缝集成。
- **no-std**：禁用 "std" 特性，使用全局分配器；回溯不可用。
- **链式上下文**：多次添加上下文，形成错误链。

#### 5. 注意事项
- `anyhow` 适合应用程序，不适合库（库应暴露具体错误类型以便调用者处理）。
- 避免在库中使用 `anyhow::Error` 作为公共 API，返回具体错误。
- 回溯依赖环境变量；生产环境中可能禁用以避免敏感信息泄露。
- 下转可能失败，如果错误类型不匹配。
- 与 `eyre` 类似，但 `anyhow` 更轻量，无需报告钩子。
- 性能开销：trait 对象有虚表调用，但基准显示在大多数情况下可忽略。

#### 6. 替代方案
- **thiserror**：用于定义自定义错误枚举，适合库；与 `anyhow` 结合使用。
- **eyre**：类似 `anyhow`，但有彩色输出和报告钩子。
- **snafu**：用于复杂错误系统，支持上下文和回溯。
- **std::error::Error**：手动实现，但繁琐。
- **failure**：旧库，已被 `anyhow` 取代。
- 对于简单场景，使用 `Box<dyn std::error::Error>`，但缺少上下文支持。

`anyhow` 被视为应用程序错误处理的默认选择。

#### 7. 20 个例子
以下是 20 个例子，从简单到复杂，覆盖文件 I/O、网络、JSON 解析、自定义错误、多线程等场景。每个例子包括代码、预期输出（如果适用）和解释。假设已导入 `use anyhow::{anyhow, bail, ensure, Context, Result};`。

##### 示例 1: 基本文件读取
```rust
fn read_file() -> Result<String> {
    std::fs::read_to_string("nonexistent.txt").context("Failed to read file")
}

fn main() {
    if let Err(e) = read_file() {
        eprintln!("Error: {}", e);
    }
}
```
输出：`Error: Failed to read file\nCaused by:\n    No such file or directory (os error 2)`  
解释：使用 `?` 传播 I/O 错误并添加上下文。

##### 示例 2: JSON 解析
```rust
use serde_json::Value;

fn parse_json() -> Result<Value> {
    let data = r#"{ "key": "value" }"#;
    serde_json::from_str(data).context("Failed to parse JSON")
}

fn main() {
    println!("{:?}", parse_json());
}
```
解释：传播 serde 错误。

##### 示例 3: 临时错误消息
```rust
fn check_value(x: i32) -> Result<()> {
    if x < 0 {
        return Err(anyhow!("Value is negative: {}", x));
    }
    Ok(())
}

fn main() {
    if let Err(e) = check_value(-1) {
        eprintln!("Error: {}", e);
    }
}
```
输出：`Error: Value is negative: -1`  
解释：使用 `anyhow!` 创建 ad-hoc 错误。

##### 示例 4: bail! 宏
```rust
fn process(data: &str) -> Result<()> {
    if data.is_empty() {
        bail!("Data is empty");
    }
    Ok(())
}

fn main() {
    if let Err(e) = process("") {
        eprintln!("Error: {}", e);
    }
}
```
输出：`Error: Data is empty`  
解释：提前返回错误。

##### 示例 5: ensure! 宏
```rust
fn validate_age(age: u32) -> Result<()> {
    ensure!(age >= 18, "Age must be at least 18, got {}", age);
    Ok(())
}

fn main() {
    if let Err(e) = validate_age(17) {
        eprintln!("Error: {}", e);
    }
}
```
输出：`Error: Age must be at least 18, got 17`  
解释：条件检查。

##### 示例 6: 添加动态上下文
```rust
fn read_config(path: &str) -> Result<String> {
    std::fs::read_to_string(path).with_context(|| format!("Failed to read config from {}", path))
}

fn main() {
    if let Err(e) = read_config("config.txt") {
        eprintln!("Error: {}", e);
    }
}
```
解释：使用闭包生成上下文。

##### 示例 7: 下转错误
```rust
use std::io;

fn handle_error() -> Result<()> {
    let err: anyhow::Error = io::Error::new(io::ErrorKind::NotFound, "not found").into();
    if let Some(io_err) = err.downcast_ref::<io::Error>() {
        println!("IO error kind: {:?}", io_err.kind());
    }
    Err(err)
}

fn main() {
    let _ = handle_error();
}
```
解释：恢复原始错误类型。

##### 示例 8: 链式错误迭代
```rust
fn chained_error() -> Result<()> {
    Err(anyhow!("Outer").context("Middle").context("Inner"))
}

fn main() {
    if let Err(e) = chained_error() {
        for cause in e.chain() {
            println!("Cause: {}", cause);
        }
    }
}
```
输出：`Cause: Inner\nCause: Middle\nCause: Outer`  
解释：遍历错误链。

##### 示例 9: 与 thiserror 集成
```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("Invalid input")]
    Invalid,
}

fn custom() -> Result<()> {
    Err(MyError::Invalid.into()).context("Custom error occurred")
}

fn main() {
    if let Err(e) = custom() {
        eprintln!("Error: {}", e);
    }
}
```
解释：自定义错误传播。

##### 示例 10: 网络请求
```rust
use reqwest;

fn fetch_url(url: &str) -> Result<String> {
    reqwest::blocking::get(url)?.text().context("Failed to fetch URL")
}

fn main() {
    if let Err(e) = fetch_url("https://example.com") {
        eprintln!("Error: {}", e);
    }
}
```
解释：处理 reqwest 错误。

##### 示例 11: 多线程错误传播
```rust
use std::thread;

fn threaded() -> Result<()> {
    let handle = thread::spawn(|| -> Result<()> {
        bail!("Thread error");
    });
    handle.join().unwrap()?;
    Ok(())
}

fn main() {
    if let Err(e) = threaded() {
        eprintln!("Error: {}", e);
    }
}
```
解释：线程中错误。

##### 示例 12: 数据库查询（模拟）
```rust
fn query_db() -> Result<()> {
    Err(anyhow!("DB connection failed")).context("Query failed")
}

fn main() {
    if let Err(e) = query_db() {
        eprintln!("Error: {}", e);
    }
}
```
解释：模拟数据库错误。

##### 示例 13: 环境变量读取
```rust
fn get_env() -> Result<String> {
    std::env::var("MISSING_VAR").context("Env var missing")
}

fn main() {
    if let Err(e) = get_env() {
        eprintln!("Error: {}", e);
    }
}
```
解释：处理 env 错误。

##### 示例 14: 数学计算错误
```rust
fn divide(a: f64, b: f64) -> Result<f64> {
    ensure!(b != 0.0, "Division by zero");
    Ok(a / b)
}

fn main() {
    if let Err(e) = divide(1.0, 0.0) {
        eprintln!("Error: {}", e);
    }
}
```
解释：自定义条件错误。

##### 示例 15: PyO3 集成（模拟）
```rust
fn py_call() -> Result<()> {
    Err(anyhow!("PyError")).context("Python call failed")
}

fn main() {
    let _ = py_call();
}
```
解释：与 pyo3 错误集成。

##### 示例 16: 回溯启用
```rust
// 运行时设置 RUST_BACKTRACE=1
fn backtrace() -> Result<()> {
    Err(anyhow!("Error with backtrace"))
}

fn main() {
    if let Err(e) = backtrace() {
        eprintln!("{:?}", e);
    }
}
```
解释：显示回溯。

##### 示例 17: 泛型函数错误
```rust
fn generic<T: std::str::FromStr>(s: &str) -> Result<T> {
    s.parse().map_err(|_| anyhow!("Parse failed"))
}

fn main() {
    if let Err(e) = generic::<i32>("abc") {
        eprintln!("Error: {}", e);
    }
}
```
解释：泛型解析。

##### 示例 18: 库中错误（与 trait）
```rust
trait MyTrait {
    fn do_something() -> Result<()>;
}

impl MyTrait for () {
    fn do_something() -> Result<()> {
        bail!("Trait error");
    }
}

fn main() {
    let _ = <() as MyTrait>::do_something();
}
```
解释：trait 中使用。

##### 示例 19: 多个错误源
```rust
fn multi_error() -> Result<()> {
    let _ = std::fs::read("a").context("First")?;
    let _ = std::fs::read("b").context("Second")?;
    Ok(())
}

fn main() {
    if let Err(e) = multi_error() {
        eprintln!("Error: {}", e);
    }
}
```
解释：链式多个上下文。

##### 示例 20: 高级下转与匹配
```rust
use std::io;

fn advanced_downcast() -> Result<()> {
    let err: anyhow::Error = io::Error::new(io::ErrorKind::InvalidData, "bad data").into();
    match err.downcast::<io::Error>() {
        Ok(io_err) => println!("IO error: {}", io_err),
        Err(e) => return Err(e),
    }
    Ok(())
}

fn main() {
    let _ = advanced_downcast();
}
```
