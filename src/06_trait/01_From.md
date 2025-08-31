# Trait Error

`Error` trait 来自 `std::error` 模块，它是 Rust 错误处理的核心，用于定义错误类型的基本期望。它要求错误类型实现 `Debug` 和 `Display`，并提供方法来描述错误、其来源和上下文。 与其他格式化 trait 不同，`Error` 专注于错误的值语义和链式处理。

## 1. `Error` Trait 简介

### 1.1 定义和目的
`Error` trait 定义在 `std::error::Error` 中，自 Rust 1.0.0 起可用。它代表 `Result<T, E>` 中 `E` 类型的基本期望：错误值应可调试、可显示，并可选地提供来源或上下文。 其目的是标准化错误处理，使不同库的错误类型可互操作，尤其在 trait 对象（如 `Box<dyn Error>`）中使用。

根据官方文档，`Error` 要求实现 `Debug` 和 `Display`，错误消息应简洁、小写、无尾随标点。 它促进错误链（error chaining），允许追踪错误根源，而不丢失上下文。

- **为什么需要 `Error`？** Rust 的错误处理强调可恢复性（recoverable errors）。实现 `Error` 允许你的自定义错误类型与标准库（如 `io::Error`）无缝集成，支持泛型错误处理、日志记录和用户反馈。 它也启用 `?` 操作符在多错误类型间的传播。

### 1.2 与其他 Trait 的区别
`Error` 与 `Debug` 和 `Display` 紧密相关，但专注于错误语义：

- **与 `Debug` 和 `Display`**：
    - `Error` 以它们为超 trait（supertraits），要求实现。`Debug` 用于开发者诊断（详细结构），`Display` 用于用户消息（简洁字符串）。
    - 区别：`Error` 添加错误特定方法如 `source`，支持链式和上下文提取。

- **与 `From` 和 `Into`**：
    - `Error` 常与 `From` 结合，用于错误转换（如 `From<io::Error> for MyError`）。这简化 `?` 操作符的使用。
    - 区别：`From` 是通用转换 trait；`Error` 专为错误标准化。

- **与 `std::io::Error`**：
    - `io::Error` 是具体类型，实现 `Error`。它用于 I/O 操作，而 `Error` 是通用接口。

**何时选择？** 为所有自定义错误类型实现 `Error`，尤其在库中。使用 `Box<dyn Error>` 处理未知错误类型。 最佳实践：库使用枚举错误（enum errors）以暴露变体；应用使用不透明错误（opaque errors）以隐藏细节。

## 2. 手动实现 `Error`

`Error` 不能自动派生（derive），必须手动实现。但你可以派生 `Debug` 和 `Display`，然后空实现 `Error`。

### 2.1 基本示例：结构体
```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyError {
    message: String,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for MyError {}
```
- 这里，`Error` 实现为空，因为没有来源。使用时：`Err(MyError { message: "oops".into() })`。

### 2.2 枚举错误
```rust
use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Parse(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "IO error: {}", err),
            AppError::Parse(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}
```
- 支持错误转换和来源链。

### 2.3 泛型错误
```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct GenericError<T: fmt::Debug> {
    inner: T,
}

impl<T: fmt::Debug> fmt::Display for GenericError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Generic error: {:?}", self.inner)
    }
}

impl<T: fmt::Debug + 'static> Error for GenericError<T> {}
```
- 约束确保 `T` 可调试和静态。

## 3. 使用辅助 Crate：thiserror 和 anyhow

### 3.1 thiserror：库中枚举错误
`thiserror` 简化 boilerplate，用于暴露变体的库。
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] std::io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
}
```
- 自动实现 `Display`、`Error` 和 `From`。

### 3.2 anyhow：应用中不透明错误
`anyhow` 提供 `anyhow::Error`（`Box<dyn Error + Send + Sync>` 的包装）。
```rust
use anyhow::{Context, Result};

fn read_config() -> Result<String> {
    std::fs::read_to_string("config.toml").context("Failed to read config")
}
```
- 使用 `context` 添加上下文。

## 4. 高级主题

### 4.1 错误链和 `source` 方法
实现 `source` 返回下层错误：
```rust
impl Error for SuperError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.sidekick)
    }
}
```
- 支持追踪链。

### 4.2 Backtrace 和上下文（Nightly）
使用 `provide`（实验）提取 backtrace：
```rust
#![feature(error_generic_member_access)]
impl std::error::Error for Error {
    fn provide<'a>(&'a self, request: &mut Request<'a>) {
        request.provide_ref::<MyBacktrace>(&self.backtrace);
    }
}
```
- 需要 nightly 和 feature。

### 4.3 与 Trait 对象：`Box<dyn Error>`
用于聚合多种错误：
```rust
type BoxError = Box<dyn std::error::Error>;
fn run() -> Result<(), BoxError> { ... }
```
- 擦除类型，但丢失具体变体。

### 4.4 与 `From` 结合
实现 `From` 以支持 `?`：
- 如上枚举示例。

## 5. 常见用例

- **库**：定义枚举错误，暴露变体以允许匹配。
- **应用**：使用不透明错误，焦点在报告而非处理。
- **CLI**：结合 `Display` 打印用户友好消息。
- **Web 服务**：链错误以日志记录根源。
- **测试**：断言具体错误变体。

## 6. 最佳实践

- **实现所有方法**：即使 `source` 为 None，也提供以支持生态。
- **使用 thiserror/anyhow**：减少 boilerplate；库用 thiserror，应用用 anyhow。
- **错误消息**：简洁、小写、无标点；本地化时分离。
- **避免 panic**：除非不可恢复；优先 `Result`。
- **测试错误**：编写测试匹配变体和消息。
- **文档**：说明错误何时发生及如何处理。

## 7. 常见陷阱和错误

- **忘记超 trait**：必须实现 `Debug` 和 `Display`。
- **孤儿规则**：不能为外部类型实现 `From` 外部 trait。
- **弃用方法**：避免 `description` 和 `cause`；用 `Display` 和 `source`。
- **类型擦除**：`dyn Error` 丢失匹配能力；用枚举保留。
- **性能**：`Box<dyn Error>` 有分配开销；在热路径避免。
- **不一致消息**：确保 `Display` 用户友好，非本地化。

## 8. 更多示例和资源

- **官方文档**：`std::error::Error` 页面。

