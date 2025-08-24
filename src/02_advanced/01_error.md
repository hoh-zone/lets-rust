# Rust 错误处理

Rust 语言强调安全性，包括错误处理。它将错误分为两类：**不可恢复错误**（unrecoverable errors，使用 `panic!` 处理）和**可恢复错误**（recoverable errors，使用 `Result` 和 `Option` 类型处理）。这种设计避免了像其他语言中常见的空指针异常或未检查异常，而是通过编译时检查和显式处理来提升代码的健壮性。

## 1. 不可恢复错误：panic!

当程序遇到无法恢复的错误时（如数组越界或断言失败），Rust 使用 `panic!` 宏来终止执行。这会 unwind 栈（清理资源）或直接 abort（不清理，适合嵌入式系统）。

### 示例：简单 panic!
```rust
fn main() {
    panic!("程序崩溃了！");  // 这会立即终止程序
}
```

- **解释**：运行后，程序打印错误消息并退出。panic! 可以接受格式化字符串，如 `panic!("错误: {}", reason);`。
- **自定义 panic!**：在库中常用条件触发，如：
  ```rust
  fn divide(x: i32, y: i32) -> i32 {
      if y == 0 {
          panic!("不能除以零！");
      }
      x / y
  }
  ```

### 配置 panic 行为
- 默认：unwind（清理栈）。
- 在 `Cargo.toml` 中设置 `[profile.release] panic = 'abort'` 来 abort，提高性能但不清理资源。

### 捕捉 panic（高级）
使用 `std::panic::catch_unwind` 可以尝试捕捉，但不推荐滥用，因为 Rust 鼓励显式错误处理。

## 2. 可恢复错误：Result 和 Option

Rust 不使用异常，而是返回枚举类型：
- **Option<T>**：表示可能为空的值。`Some(T)` 或 `None`。
- **Result<T, E>**：表示成功或失败。`Ok(T)` 或 `Err(E)`。

### 示例：使用 Option
```rust
fn find_char(s: &str, c: char) -> Option<usize> {
    for (i, ch) in s.chars().enumerate() {
        if ch == c {
            return Some(i);
        }
    }
    None
}

fn main() {
    match find_char("hello", 'l') {
        Some(pos) => println!("找到位置: {}", pos),
        None => println!("未找到"),
    }
}
```

- **解释**：`match` 处理可能的值。Option 常用于可能失败但无具体错误信息的场景。

### 示例：使用 Result
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;  // 这里使用 ? 操作符，稍后解释
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file("hello.txt") {
        Ok(content) => println!("文件内容: {}", content),
        Err(e) => println!("读取失败: {}", e),
    }
}
```

- **解释**：Result 的 `E` 是错误类型，这里是 `io::Error`。成功返回 `Ok(值)`，失败返回 `Err(错误)`。

### 模式匹配和 unwrap
- **match**：最安全的方式。
- **unwrap()**：如果 Ok 返回值，否则 panic!（不推荐生产环境）。
- **expect("消息")**：类似 unwrap，但自定义 panic 消息。
- **unwrap_or(default)**：为 Option/Result 提供默认值。
- **unwrap_or_else(closure)**：懒惰计算默认值。

## 3. ? 操作符：简化错误传播

`?` 是 Result/Option 的语法糖，用于早返回错误，而不嵌套 match。

### 示例：使用 ?
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;  // 如果失败，早返回 Err
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

- **解释**：`?` 等价于：
  ```rust
  let mut file = match File::open(filename) {
      Ok(f) => f,
      Err(e) => return Err(e),
  };
  ```
- **要求**：函数必须返回 Result/Option。
- **链式使用**：支持多个 ?，错误会向上传播。
- **From trait**：如果错误类型不同，? 会自动转换（如果实现了 From）。

## 4. 自定义错误类型

对于复杂应用，定义自己的错误枚举，结合 thiserror 或 anyhow crate（但本教程用标准库）。

### 示例：自定义错误
```rust
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum MyError {
    Io(std::io::Error),
    Parse(ParseIntError),
    Custom(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "IO 错误: {}", e),
            MyError::Parse(e) => write!(f, "解析错误: {}", e),
            MyError::Custom(s) => write!(f, "自定义错误: {}", s),
        }
    }
}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> MyError {
        MyError::Io(err)
    }
}

impl From<ParseIntError> for MyError {
    fn from(err: ParseIntError) -> MyError {
        MyError::Parse(err)
    }
}

fn parse_number(s: &str) -> Result<i32, MyError> {
    let num: i32 = s.parse().map_err(MyError::Parse)?;  // 手动转换或用 From
    if num < 0 {
        return Err(MyError::Custom("负数无效".to_string()));
    }
    Ok(num)
}

fn main() {
    match parse_number("-5") {
        Ok(n) => println!("数字: {}", n),
        Err(e) => println!("错误: {}", e),
    }
}
```

- **解释**：
    - 枚举包装不同错误源。
    - 实现 `Display` 和 `Debug` 以打印。
    - 实现 `From` 以支持 ? 的自动转换。
    - 这允许统一处理多种错误。

## 5. 错误处理最佳实践

- **使用 Result 而非 panic**：除非确实不可恢复。
- **早失败，早返回**：使用 ? 保持代码简洁。
- **提供上下文**：在 Err 中添加信息，如使用 anyhow::Context。
- **标准库 vs crate**：
    - 简单项目：用 std::io::Error 等。
    - 复杂项目：推荐 anyhow（用户友好错误）或 thiserror（自定义错误宏）。
- **测试错误**：用 `#[should_panic]` 测试 panic，或匹配 Result::Err。
- **性能**：Result 是零成本抽象，不会影响运行时，除非错误发生。
- **常见陷阱**：
    - 忘记处理 Result，导致编译错误（Rust 强制处理）。
    - 过度 unwrap：用在原型中，但生产代码中避免。
    - 错误类型不兼容：确保实现 From 或手动 map_err。

## 练习建议
1. 编写一个函数读取文件并解析为整数列表，使用自定义错误处理解析失败。
2. 修改示例，使用 match 处理多级错误链。
3. 探索 std::error::Error trait 以创建更通用的错误。
