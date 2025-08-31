# Option 教程

Rust 中的 `Option<T>` 是标准库中的枚举类型，用于表示一个值可能存在或不存在的情况。它是 Rust 处理“空值”的方式，避免了像其他语言中常见的 null 指针异常。`Option` 强制开发者显式处理“无值”情况，提升代码安全性。`Option<T>` 有两个变体：`Some(T)`（有值）和 `None`（无值）。

## 1. Option 简介

- **定义**：`Option` 是枚举：
  ```rust
  enum Option<T> {
      Some(T),
      None,
  }
  ```
- **为什么使用？**：Rust 无 null，所有可能为空的值用 Option 包装。编译器强制处理 None ケース，防止运行时错误。
- **优势**：类型安全、显式错误处理、无运行时开销。
- **常见场景**：函数返回可能失败的值（如查找）、可选配置。

### 示例：基本使用
```rust
fn main() {
    let some_number: Option<i32> = Some(5);
    let none_number: Option<i32> = None;

    println!("{:?}", some_number);  // 输出: Some(5)
    println!("{:?}", none_number);  // 输出: None
}
```

- **解释**：`Some(T)` 持有值，`None` 表示无值。类型注解可选，Rust 可推断。

## 2. 模式匹配处理 Option

最常见方式是用 `match` 处理变体。

### 示例：Match 处理
```rust
fn divide(dividend: f64, divisor: f64) -> Option<f64> {
    if divisor == 0.0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

fn main() {
    match divide(10.0, 2.0) {
        Some(result) => println!("结果: {}", result),  // 输出: 结果: 5.0
        None => println!("不能除以零！"),
    }

    match divide(10.0, 0.0) {
        Some(_) => {},  // 未发生
        None => println!("不能除以零！"),  // 输出
    }
}
```

- **解释**：`match` 穷尽所有变体，必须处理 Some 和 None。忽略值用 `_`。

### if let 简化
```rust
fn main() {
    let config: Option<u32> = Some(42);
    if let Some(value) = config {
        println!("配置值: {}", value);  // 输出: 配置值: 42
    } else {
        println!("无配置");
    }
}
```

- **解释**：`if let` 只处理 Some，else 处理 None。更简洁于 match。

## 3. Option 的方法

Option 有许多实用方法，避免手动 match。

- **unwrap()**：返回 Some 值，否则 panic!（不推荐生产）。
- **expect("msg")**：类似 unwrap，但自定义 panic 消息。
- **unwrap_or(default)**：返回 Some 值或默认值。
- **unwrap_or_else(closure)**：懒惰计算默认值。
- **map(f)**：转换 Some 值，None 保持。
- **and_then(f)**：链式操作，返回 Option。
- **ok_or(err)**：转为 Result。
- **is_some() / is_none()**：检查变体。
- **as_ref() / as_mut()**：借用内部值。

### 示例：常用方法
```rust
fn main() {
    let some = Some(5);
    let none: Option<i32> = None;

    // unwrap
    println!("{}", some.unwrap());  // 5
    // none.unwrap();  // panic!

    // unwrap_or
    println!("{}", none.unwrap_or(0));  // 0

    // map
    let mapped = some.map(|x| x * 2);
    println!("{:?}", mapped);  // Some(10)

    // and_then
    let chained = some.and_then(|x| if x > 0 { Some(x.to_string()) } else { None });
    println!("{:?}", chained);  // Some("5")
}
```

- **解释**：方法链式使用，如 `option.map(...).unwrap_or(...)`。map 只影响 Some。

## 4. Option 与函数

函数常返回 Option 处理可选值。

### 示例：查找函数
```rust
fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return Some(offset);
        }
    }
    None
}

fn main() {
    let position = find("hello", 'l');
    if let Some(pos) = position {
        println!("找到于位置: {}", pos);  // 输出: 找到于位置: 2
    }
}
```

- **解释**：返回 Some(位置) 或 None。调用者必须处理。

## 5. Option 与集合

Option 常用于 Vec 或 HashMap 的 get 方法。

### 示例：集合集成
```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Alice", 50);

    let alice_score = scores.get("Alice");
    println!("{:?}", alice_score);  // Some(50)

    let bob_score = scores.get("Bob").copied().unwrap_or(0);
    println!("{}", bob_score);  // 0
}
```

- **解释**：`get` 返回 &Option<V>。copied() 用于 Copy 类型。

## 6. 高级主题：Option 与 Result

- **transpose()**：Option<Result<T, E>> 转为 Result<Option<T>, E>。
- **flatten()**：Option<Option<T>> 转为 Option<T>。
- **zip(other)**：结合两个 Option 为 Option<(T, U)>。

### 示例：Transpose
```rust
use std::num::ParseIntError;

fn parse(s: &str) -> Option<Result<i32, ParseIntError>> {
    if s.is_empty() { None } else { Some(s.parse()) }
}

fn main() {
    let result = parse("42").transpose();
    println!("{:?}", result);  // Ok(Some(42))

    let empty = parse("").transpose();
    println!("{:?}", empty);  // Ok(None)
}
```

- **解释**：transpose 交换层级，便于错误处理链。

## 7. 最佳实践和常见陷阱

- **避免 unwrap**：生产代码用 match 或 unwrap_or 处理 None。
- **链式方法**：用 map/and_then 保持函数式风格。
- **默认值**：用 unwrap_or 而非 if let，当默认简单时。
- **常见错误**：
    - 忘记处理 None：编译错误（非穷尽 match）。
    - 借用问题：Option<&T> vs &Option<T>（用 as_ref()）。
    - 性能：Option 是零成本，枚举优化为标签 + 值。
    - 与 ? 操作符：? 在返回 Option 的函数中传播 None。
- **derive**：#[derive(PartialEq, Debug)] 等用于自定义类型中的 Option。
- **标准库**：许多 API 返回 Option，如 str::find、Vec::get。

## 练习建议
1. 编写函数，返回字符串中第一个元音的位置（Option<usize>）。
2. 用 map 和 unwrap_or 处理 Option<Vec<i32>>，计算平均值或默认 0.0。
3. 实现一个解析可选命令行参数的简单 CLI，使用 Option。
