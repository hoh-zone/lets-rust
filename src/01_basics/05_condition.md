# 条件分支语句

Rust 是一种系统级编程语言，强调安全性和性能。在 Rust 中，条件分支语句用于根据条件执行不同的代码路径。主要包括 `if`、`else if`、`else` 语句，以及更强大的 `match` 表达式。本教程将从基础开始逐步讲解这些语句的使用方式，包括语法、示例和注意事项。假设你已经安装了 Rust 环境（可以通过 `rustup` 安装），并使用 `cargo` 创建一个新项目来测试代码。

所有示例代码都可以复制到 `src/main.rs` 文件中运行，使用 `cargo run` 执行。

## 1. 基础：`if` 语句

`if` 语句是 Rust 中最简单的条件分支形式。它检查一个布尔表达式（必须返回 `bool` 类型），如果为真，则执行代码块。

### 语法
```rust
if 条件 {
    // 代码块
}
```

### 示例
```rust
fn main() {
    let number = 5;

    if number > 0 {
        println!("数字是正数");
    }

    println!("程序继续执行");  // 这行总是执行
}
```

### 输出
```
数字是正数
程序继续执行
```

### 注意事项
- 条件必须是 `bool` 类型，不能像一些语言那样隐式转换为布尔（例如，不能直接用整数作为条件）。
- 不需要括号包围条件，但代码块必须用大括号 `{}` 包围。
- 如果条件为假，代码块将被跳过。

## 2. `if-else` 语句

添加 `else` 可以处理条件为假的情况。

### 语法
```rust
if 条件 {
    // 真时执行
} else {
    // 假时执行
}
```

### 示例
```rust
fn main() {
    let number = -3;

    if number > 0 {
        println!("数字是正数");
    } else {
        println!("数字是非正数");
    }
}
```

### 输出
```
数字是非正数
```

### 注意事项
- `else` 块是可选的。
- 两个分支的代码块必须用 `{}` 包围，即使只有一行代码。

## 3. `if-else if-else` 链

对于多个条件，可以链式使用 `else if`。

### 语法
```rust
if 条件1 {
    // 条件1 为真
} else if 条件2 {
    // 条件1 为假，条件2 为真
} else {
    // 所有条件为假
}
```

### 示例
```rust
fn main() {
    let score = 85;

    if score >= 90 {
        println!("优秀");
    } else if score >= 80 {
        println!("良好");
    } else if score >= 60 {
        println!("及格");
    } else {
        println!("不及格");
    }
}
```

### 输出
```
良好
```

### 注意事项
- 条件按顺序检查，一旦一个为真，后续分支将被跳过。
- 可以有任意多个 `else if`，但只有一个 `else`。
- 避免过多链式 `else if`，因为这可能导致代码复杂；考虑使用 `match` 代替。

## 4. 在赋值中使用 `if`（作为表达式）

Rust 的 `if` 是一个表达式，可以返回一个值，因此可以用于变量赋值。这类似于其他语言的三元运算符，但更灵活。

### 语法
```rust
let 变量 = if 条件 {
    值1
} else {
    值2
};
```

### 示例
```rust
fn main() {
    let number = 7;

    let description = if number % 2 == 0 {
        "偶数"
    } else {
        "奇数"
    };

    println!("数字是：{}", description);
}
```

### 输出
```
数字是：奇数
```

### 注意事项
- 所有分支必须返回相同类型的值。
- 分支末尾不能有分号 `;`，因为这是表达式而非语句。
- 如果没有 `else`，编译会失败，因为表达式必须有值。

# Rust Match 

Rust 中的 `match` 表达式是处理多分支条件的核心工具，它类似于其他语言的 switch 语句，但更强大、更安全。`match` 支持模式匹配（pattern matching），可以处理枚举、结构体、范围等复杂类型，并要求穷尽所有可能情况（exhaustive matching），以避免运行时错误。`match` 是一个表达式，可以返回值，且编译器会静态检查分支覆盖。本教程从基础到高级逐步讲解 `match` 的使用，包括语法、示例、模式类型、guard 条件、绑定、最佳实践和常见错误。假设你已安装 Rust 环境（通过 `rustup`），并使用 `cargo` 创建项目来测试代码。教程基于 Rust 1.89.0（2025 年 8 月最新稳定版）。所有示例代码可复制到 `src/main.rs` 中，使用 `cargo run` 执行。

## 1. 基础：简单 Match

`match` 检查一个值，并根据第一个匹配的模式执行对应代码。

### 语法
```rust
match 值 {
    模式1 => 表达式1,
    模式2 => 表达式2,
    // ...
    _ => 默认表达式,  // 通配符，处理剩余情况
}
```

- **要求**：必须覆盖所有可能值，否则编译失败。
- **作为表达式**：所有分支返回相同类型的值。

### 示例：整数匹配
```rust
fn main() {
    let number = 3;

    match number {
        1 => println!("一"),
        2 => println!("二"),
        3 => println!("三"),
        _ => println!("其他"),
    }
}
```

### 输出
```
三
```

### 注意事项
- 分支（arms）以逗号 `,` 分隔。
- 通配符 `_` 匹配任意值但不绑定。
- 与 switch 不同，无 fallthrough（自动进入下一分支），每个分支独立执行。

## 2. Match 作为表达式（返回值）

`match` 可以用于变量赋值或函数返回。

### 示例
```rust
fn describe_day(day: u8) -> &'static str {
    match day {
        1 => "星期一",
        2 => "星期二",
        3 => "星期三",
        4 => "星期四",
        5 => "星期五",
        _ => "周末",
    }
}

fn main() {
    let today = 4;
    println!("今天是：{}", describe_day(today));
}
```

### 输出
```
今天是：星期四
```

- **注意**：分支末尾无分号 `;` 以返回值为表达式；多行代码用 `{}` 包围。

## 3. 模式匹配（Patterns）

`match` 支持多种模式，如范围、枚举、解构等。

### 范围模式（Ranges）
```rust
fn main() {
    let score = 85;

    match score {
        90..=100 => println!("优秀"),
        80..=89 => println!("良好"),
        60..=79 => println!("及格"),
        _ => println!("不及格"),
    }
}
```

- **输出**：`良好`
- **注意**：`..=` 包含边界；`..` 排除。

### 枚举匹配
枚举常与 `match` 结合使用。
```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(u8),  // 带数据
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("州代码：{}", state);
            25
        },
    }
}

fn main() {
    let coin = Coin::Quarter(50);
    println!("价值：{} 美分", value_in_cents(coin));
}
```

- **输出**：
```
州代码：50
价值：25 美分
```
- **解构**：`Coin::Quarter(state)` 绑定 `state` 到枚举数据。

### 结构体解构
```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("在 x 轴：{}", x),
        Point { x: 0, y } => println!("在 y 轴：{}", y),
        Point { x, y } => println!("在 ({}, {})", x, y),
    }
}
```

- **输出**：`在 y 轴：7`
- **注意**：用 `..` 忽略字段，如 `Point { x, .. }`。

## 4. Guard 条件（if Guards）

在模式后添加 `if` 条件进行额外过滤。

### 示例
```rust
fn main() {
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("偶数：{}", x),
        Some(x) => println!("奇数：{}", x),
        None => println!("无值"),
    }
}
```

- **输出**：`偶数：4`
- **注意**：guard 不影响穷尽性；仍需覆盖所有模式。

## 5. 绑定与 @ 运算符

使用 `@` 将匹配值绑定到变量。

### 示例
```rust
fn main() {
    let msg = "hello";

    match msg {
        x @ "hello" => println!("匹配：{}", x),
        _ => println!("其他"),
    }
}
```

- **输出**：`匹配：hello`
- **高级**：结合范围，如 `x @ 1..=5 => println!("范围：{}", x)`。

## 6. 多模式与 OR

使用 `|` 分隔多个模式。

### 示例
```rust
fn main() {
    let x = 1;

    match x {
        1 | 2 => println!("一或二"),
        3..=5 => println!("三到五"),
        _ => println!("其他"),
    }
}
```

- **输出**：`一或二`

## 7. if let 简化（Match 的简化形式）

对于单模式匹配，使用 `if let` 避免完整 `match`。

### 示例
```rust
fn main() {
    let some_value = Some(3);

    if let Some(x) = some_value {
        println!("值：{}", x);
    } else {
        println!("无值");
    }
}
```

- **注意**：相当于 `match` 的单臂版本；可选 `else` 处理剩余情况。

## 8. 常见错误与最佳实践

使用表格总结常见问题：

| 问题 | 原因 | 解决方案 |
|------|------|----------|
| 未穷尽匹配 | 遗漏模式 | 添加 `_` 或覆盖所有枚举变体 |
| 类型不匹配 | 分支返回不同类型 | 确保所有分支返回相同类型 |
| Fallthrough 期望 | 期望自动进入下一分支 | Rust 无 fallthrough；显式在分支中处理 |
| 性能问题 | 过于复杂模式 | 对于简单条件，用 if/else 替代；match 适合枚举 |
| 绑定失败 | 未使用 @ 或解构 | 用 `@` 绑定或正确解构变量 |

- **最佳实践**：优先用 `match` 处理枚举和 Option/Result；用 if/else 处理布尔条件。利用编译器检查提升代码安全性。保持模式简洁，提高可读性。
- **与 Switch 比较**：`match` 更安全（强制穷尽、无隐式转换、无 fallthrough），更灵活（支持模式匹配、解构）。

## 9. 练习

1. 编写函数，使用 `match` 处理 Option<i32>，如果 Some，返回其平方，否则返回 0。
2. 定义枚举 TrafficLight { Red, Yellow, Green }，用 `match` 输出对应描述（如 "停止"）。
3. 使用 guard 和范围实现 FizzBuzz：遍历 1-100，3 的倍数打印 "Fizz"，5 "Buzz"，两者 "FizzBuzz"，否则数字。
4. 解构一个 Vec<i32>，用 `match` 根据长度和首元素分支处理（如空、单元素、多元素）。

通过这些示例，你应该能熟练使用 Rust 的 `match` 表达式。作为 switch 的强大替代，它是 Rust 模式匹配的核心。更多细节可参考 Rust 官方书籍（The Rust Programming Language）。如果有疑问，欢迎提供具体代码反馈！