# Result 教程

Rust 中的 `Result<T, E>` 是标准库中的枚举类型，用于表示操作可能成功或失败的情况。它是 Rust 错误处理的核心机制，避免了异常抛出，而是通过返回值强制开发者处理错误。`Result` 有两个变体：`Ok(T)`（成功，持有值）和 `Err(E)`（失败，持有错误）。这与 `Option` 类似，但 `Result` 携带错误信息，便于调试和恢复。
假设你已熟悉 Rust 的基本语法（如枚举、模式匹配）和 `Option`。


## 1. Result 简介

- **定义**：`Result` 是枚举：
  ```rust
  enum Result<T, E> {
      Ok(T),
      Err(E),
  }
  ```
- **为什么使用？**：Rust 无 unchecked exceptions，所有潜在错误通过 Result 返回。编译器强制处理 Err，提升代码鲁棒性。
- **优势**：类型安全、显式错误传播、无运行时开销、易于链式处理。
- **常见场景**：I/O 操作（如文件读取）、解析（如字符串转整数）、网络请求。

### 示例：基本使用
```rust
use std::fs::File;

fn main() {
    let ok_result: Result<i32, String> = Ok(42);
    let err_result: Result<i32, String> = Err(String::from("出错了"));

    println!("{:?}", ok_result);  // 输出: Ok(42)
    println!("{:?}", err_result);  // 输出: Err("出错了")

    // 实际示例：打开文件
    let file = File::open("nonexistent.txt");  // 返回 Result<File, io::Error>
    println!("{:?}", file);  // 可能: Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })
}
```

- **解释**：`T` 是成功类型，`E` 是错误类型（常为 `std::io::Error` 或自定义）。类型注解可选，Rust 可推断。

## 2. 模式匹配处理 Result

用 `match` 处理变体是最直接方式。

### 示例：Match 处理
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file("hello.txt") {
        Ok(content) => println!("内容: {}", content),
        Err(error) => println!("错误: {}", error),
    }
}
```

- **解释**：`match` 穷尽 Ok 和 Err。忽略值用 `_`，但最好处理具体错误（如 error.kind()）。

### if let 简化
```rust
fn main() {
    let result: Result<u32, &str> = Ok(100);
    if let Ok(value) = result {
        println!("值: {}", value);  // 输出: 值: 100
    } else {
        println!("失败");
    }
}
```

- **解释**：`if let` 只处理 Ok，else 处理 Err。更简洁于简单情况。

## 3. ? 操作符：错误传播

`?` 是 Result 的语法糖，用于早返回 Err，而不嵌套 match。

### 示例：使用 ?
```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("username.txt")  // 隐含 ? 等价于 match { Ok(v) => v, Err(e) => return Err(e) }
}

fn main() {
    match read_username_from_file() {
        Ok(username) => println!("用户名: {}", username),
        Err(e) => println!("错误: {}", e),
    }
}
```

- **解释**：`?` 只在返回 Result 的函数中使用。如果 Ok，返回值；如果 Err，早返回。支持链式：`let data = file.open()?; data.read()?;`。错误类型需匹配或实现 From 以转换。

## 4. Result 的方法

Result 有许多方法，避免手动 match。

- **unwrap()**：返回 Ok 值，否则 panic!（不推荐生产）。
- **expect("msg")**：类似 unwrap，但自定义 panic 消息。
- **unwrap_or(default)**：返回 Ok 值或默认（T 类型）。
- **unwrap_or_else(closure)**：懒惰计算默认。
- **map(f)**：转换 Ok 值，Err 保持。
- **map_err(f)**：转换 Err 值，Ok 保持。
- **and_then(f)**：链式操作，返回 Result。
- **or_else(f)**：处理 Err，返回新 Result。
- **is_ok() / is_err()**：检查变体。
- **ok() / err()**：转为 Option。

### 示例：常用方法
```rust
fn main() {
    let ok: Result<i32, &str> = Ok(5);
    let err: Result<i32, &str> = Err("错误");

    // unwrap
    println!("{}", ok.unwrap());  // 5
    // err.unwrap();  // panic!

    // unwrap_or
    println!("{}", err.unwrap_or(0));  // 0

    // map
    let mapped = ok.map(|x| x * 2);
    println!("{:?}", mapped);  // Ok(10)

    // and_then
    let chained = ok.and_then(|x| if x > 0 { Ok(x.to_string()) } else { Err("负数") });
    println!("{:?}", chained);  // Ok("5")

    // map_err
    let err_mapped = err.map_err(|e| format!("新错误: {}", e));
    println!("{:?}", err_mapped);  // Err("新错误: 错误")
}
```

- **解释**：方法链式使用，如 `result.map(...).unwrap_or_else(...)`。map 只影响 Ok。

## 5. 自定义错误和 From trait

为复杂错误定义枚举，实现 From 以支持 ? 的自动转换。

### 示例：自定义错误
```rust
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum MyError {
    Parse(ParseIntError),
    Negative,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::Parse(e) => write!(f, "解析错误: {}", e),
            MyError::Negative => write!(f, "负数无效"),
        }
    }
}

impl From<ParseIntError> for MyError {
    fn from(err: ParseIntError) -> MyError {
        MyError::Parse(err)
    }
}

fn parse_positive(s: &str) -> Result<i32, MyError> {
    let num: i32 = s.parse()?;  // ? 使用 From 转换
    if num < 0 {
        Err(MyError::Negative)
    } else {
        Ok(num)
    }
}

fn main() {
    println!("{:?}", parse_positive("42"));   // Ok(42)
    println!("{:?}", parse_positive("-1"));   // Err(Negative)
    println!("{:?}", parse_positive("abc"));  // Err(Parse(ParseIntError { kind: InvalidDigit }))
}
```

- **解释**：自定义错误枚举，From 允许无缝 ?。实现 Display 和 Error trait 以打印和集成。

## 6. Result 与 Option 的交互

- **ok()**：Result<T, E> 转为 Option<T>（丢弃错误）。
- **transpose()**：Result<Option<T>, E> 转为 Option<Result<T, E>>。

### 示例：Transpose
```rust
fn maybe_parse(s: &str) -> Result<Option<i32>, ParseIntError> {
    if s.is_empty() { Ok(None) } else { s.parse().map(Some) }
}

fn main() {
    let result = maybe_parse("42").transpose();
    println!("{:?}", result);  // Some(Ok(42))

    let empty = maybe_parse("").transpose();
    println!("{:?}", empty);  // Some(None) 等价于 Some(Ok(None))，但 transpose 调整层级
}
```

- **解释**：transpose 交换层级，便于链式处理 Option 和 Result。

## 7. 最佳实践和常见陷阱

- **优先 ?**：简化错误传播，保持代码简洁。
- **自定义错误**：用枚举包装多种错误源，实现 From/Error。
- **避免 unwrap**：生产代码用 match 或 or_else 处理 Err。
- **链式方法**：用 map/and_then 保持函数式风格。
- **常见错误**：
    - 未处理 Result：编译错误（强制）。
    - 错误类型不匹配：用 map_err 或 From 转换。
    - 性能：Result 是零成本枚举。
    - 与 panic!：用 Result 代替，除非不可恢复。
- **标准库**：许多 API 返回 Result，如 str::parse、File::open。
- **crate**：复杂项目用 anyhow（简单错误）或 thiserror（自定义宏）。

## 练习建议
1. 编写函数，读取文件并解析为 Vec<i32>，用 Result 处理错误。
2. 用 and_then 和 map_err 处理链式 Result 操作。
3. 创建自定义错误类型，集成多种 std 错误。

