# Rust Trait FromStr

`FromStr` trait 来自 `std::str` 模块，它的主要目的是从字符串解析值。它允许你定义如何从 `&str` 创建类型实例，并处理可能的解析错误，通常通过 `str::parse` 方法隐式调用。 与 `TryFrom<&str>` 类似，但 `FromStr` 是专为字符串解析设计的历史 trait。

## 1. `FromStr` Trait 简介

### 1.1 定义和目的
`FromStr` trait 定义在 `std::str::FromStr` 中，自 Rust 1.0.0 起稳定可用。其语法如下：
```rust
pub trait FromStr: Sized {
    type Err;

    fn from_str(s: &str) -> Result<Self, Self::Err>;
}
```
- **目的**：提供一种从字符串解析值的机制。它允许类型定义如何从 `&str` 创建自身，返回 `Result<Self, Err>` 以处理失败。`Err` 是关联类型，由实现者定义，通常是自定义错误。 这在 CLI、配置解析或用户输入处理中特别有用。

根据官方文档，`FromStr` 的 `from_str` 方法常通过 `str::parse` 隐式调用。输入格式取决于类型，应查阅文档。 它不保证与 `Display` 格式匹配，且 round-trip 可能不 lossless。 标准库为数值类型、网络地址等实现 `FromStr`。

- **为什么需要 `FromStr`？** Rust 强调安全解析。`FromStr` 标准化字符串转换，支持泛型解析，并避免不安全假设，如直接 `unwrap`。 例如，在 CLI 工具中，从参数字符串解析整数。

### 1.2 与相关 Trait 的区别
`FromStr` 与转换 trait 相关，但专为字符串：

- **与 `TryFrom<&str>`**：
    - `FromStr` 等价于 `TryFrom<&str>`，但历史更早，专为字符串。
    - `TryFrom` 更通用，可用于任何类型；`FromStr` 通过 `parse` 方法集成更好。
    - 选择：优先 `FromStr` 以兼容 `parse`；用 `TryFrom` 如果非字符串。

- **与 `From<&str>` / `From<String>`**：
    - `From<&str>` 用于无失败转换；`FromStr` 用于可能失败的解析。
    - `From<String>` 消耗字符串；`FromStr` 用借用 `&str`，更高效。
    - 最佳：实现 `TryFrom<&str>` 和 `FromStr`；仅 `From<String>` 如果消耗。

- **与 `ToString`**：
    - `ToString` 用于转换为字符串；`FromStr` 用于从字符串解析。
    - 常结合使用，但不保证 round-trip。

**何时选择？** 对于字符串解析，用 `FromStr` 以利用 `parse`；对于一般失败转换，用 `TryFrom`。

## 2. 手动实现 `FromStr`

`FromStr` 不能自动派生，必须手动实现。定义 `Err` 和 `from_str`。

### 2.1 基本示例：结构体
从官方示例：
```rust
use std::str::FromStr;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl fmt::Display for ParsePointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid point format")
    }
}

impl Error for ParsePointError {}

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| s.split_once(','))
            .ok_or(ParsePointError)?;

        let x = x.parse::<i32>().map_err(|_| ParsePointError)?;
        let y = y.parse::<i32>().map_err(|_| ParsePointError)?;

        Ok(Point { x, y })
    }
}

fn main() {
    assert_eq!("(1,2)".parse::<Point>(), Ok(Point { x: 1, y: 2 }));
    assert!("(1 2)".parse::<Point>().is_err());
}
```
- 解析 "(x,y)" 格式，使用链式字符串方法和子解析。

### 2.2 枚举
从 GFG 示例：
```rust
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
}

impl FromStr for Day {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "monday" => Ok(Day::Monday),
            "tuesday" => Ok(Day::Tuesday),
            "wednesday" => Ok(Day::Wednesday),
            _ => Err(()),
        }
    }
}

fn main() {
    assert_eq!("Monday".parse::<Day>(), Ok(Day::Monday));
    assert!("Friday".parse::<Day>().is_err());
}
```
- 匹配小写字符串到枚举变体。

### 2.3 泛型类型
```rust
use std::str::FromStr;

#[derive(Debug)]
struct Pair<T>(T, T);

impl<T: FromStr> FromStr for Pair<T> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err(T::Err::from("Invalid format".to_string())); // 假设 Err 可从 String
        }
        let first = parts[0].parse::<T>()?;
        let second = parts[1].parse::<T>()?;
        Ok(Pair(first, second))
    }
}

fn main() {
    assert_eq!("1,2".parse::<Pair<i32>>(), Ok(Pair(1, 2)));
}
```
- 泛型 impl，委托子解析。

### 2.4 自定义错误
使用详细错误：
```rust
#[derive(Debug)]
enum ParseError {
    InvalidFormat,
    ParseInt(std::num::ParseIntError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidFormat => write!(f, "Invalid format"),
            ParseError::ParseInt(e) => write!(f, "Parse int error: {}", e),
        }
    }
}

impl Error for ParseError {}

impl From<std::num::ParseIntError> for ParseError {
    fn from(e: std::num::ParseIntError) -> Self {
        ParseError::ParseInt(e)
    }
}

impl FromStr for Point {
    type Err = ParseError;
    // ... 类似上面，但 map_err 到 ParseError
}
```
- 链错误以提供上下文。

## 3. 与 `parse` 方法的关系

`str::parse` 调用 `FromStr`：
```rust
let num: i32 = "42".parse().unwrap();
```
- Turbofish 语法指定类型：` "42".parse::<i32>() `。

## 4. 高级主题

### 4.1 对于 Trait 对象
实现 `FromStr` 返回 dyn Trait：
```rust
trait Unit {}

struct Time;
impl Unit for Time {}
impl FromStr for Time { /* ... */ }

impl FromStr for dyn Unit {
    type Err = ();

    fn from_str(s: &str) -> Result<Box<dyn Unit>, Self::Err> {
        // 逻辑选择实现
        Ok(Box::new(Time))
    }
}
```
- 使用 Box 返回 trait 对象。

### 4.2 与 `From<&str>` 结合
实现两者：
```rust
impl From<&str> for MyType {
    fn from(s: &str) -> Self {
        s.parse().unwrap() // 但避免 unwrap
    }
}
```
- 但优先 `FromStr` 以处理错误。

### 4.3 第三方 Crate：strum
使用 `strum` 宏自动为枚举实现 `FromStr`。

## 5. 常见用例

- **CLI 参数**：解析命令行字符串。
- **配置文件**：从 TOML/JSON 字符串解析。
- **网络地址**：如 `IpAddr::from_str`。
- **自定义类型**：如日期、颜色。
- **泛型解析**：函数接受 `T: FromStr`。

## 6. 最佳实践

- **实现 `FromStr` 和 `TryFrom<&str>`**：兼容性和通用性。
- **自定义 Err**：实现 `Error` 以详细消息。
- **文档格式**：说明预期输入。
- **测试**：覆盖有效/无效输入。
- **避免消耗**：用 `&str` 而非 `String`。
- **与 Display 一致**：如果可能，确保 round-trip。

## 7. 常见陷阱和错误

- **无 lifetime 参数**：不能为 `&T` 实现。
- **孤儿规则**：不能为外部类型实现。
- **unwrap 滥用**：总是处理 Err。
- **格式不一致**：与 `Display` 不匹配导致混淆。
- **性能**：复杂解析在热路径评估。
