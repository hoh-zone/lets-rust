# 第7章：枚举

枚举（Enum）是 Rust 中定义可能值的集合的方式，它是 Rust 类型系统的核心特性之一。

## 7.1 定义与使用

### 基本枚举

```rust
// 定义枚举
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    // 创建枚举值
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    // 作为函数参数
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) {
    // 处理不同的 IP 类型
}
```

### 枚举与数据关联

```rust
// 枚举可以直接存储数据
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
}

// 不同成员可以有不同类型的数据
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr2::V4(127, 0, 0, 1);
    let loopback = IpAddr2::V6(String::from("::1"));
}
```

### 复杂枚举

```rust
// 枚举成员可以有多种形式
enum Message {
    Quit,                       // 没有数据
    Move { x: i32, y: i32 },   // 命名字段，类似结构体
    Write(String),              // 包含一个 String
    ChangeColor(i32, i32, i32), // 包含三个 i32
}

impl Message {
    // 枚举也可以定义方法
    fn call(&self) {
        match self {
            Message::Quit => println!("退出"),
            Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
            Message::Write(text) => println!("文本消息: {}", text),
            Message::ChangeColor(r, g, b) => println!("改变颜色为 RGB({}, {}, {})", r, g, b),
        }
    }
}

fn main() {
    let m1 = Message::Write(String::from("hello"));
    m1.call();
    
    let m2 = Message::Move { x: 10, y: 20 };
    m2.call();
}
```

### 标准库中的枚举示例

```rust
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

fn main() {
    // 标准库中的 IpAddr 枚举
    let home = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let loopback = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    
    match home {
        IpAddr::V4(addr) => println!("IPv4: {}", addr),
        IpAddr::V6(addr) => println!("IPv6: {}", addr),
    }
}
```

## 7.2 用 match 操作枚举

### 基本 match

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = Coin::Penny;
    println!("硬币价值: {} 美分", value_in_cents(coin));
}
```

### 绑定值的模式

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("幸运便士！");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("州 25 美分硬币来自 {:?}!", state);
            25
        }
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    value_in_cents(coin);
}
```

### 匹配必须穷尽

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(dir: Direction) {
    match dir {
        Direction::Up => println!("向上移动"),
        Direction::Down => println!("向下移动"),
        Direction::Left => println!("向左移动"),
        Direction::Right => println!("向右移动"),
    }
    // 所有情况都已处理
}

// 使用通配符处理剩余情况
fn process_value(x: u8) {
    match x {
        1 => println!("一"),
        3 => println!("三"),
        5 => println!("五"),
        7 => println!("七"),
        _ => println!("其他数字"),  // _ 匹配所有其他值
    }
}
```

### if let 简洁语法

```rust
fn main() {
    let config_max = Some(3u8);
    
    // 使用 match
    match config_max {
        Some(max) => println!("最大值配置为 {}", max),
        _ => (),
    }
    
    // 使用 if let（更简洁）
    if let Some(max) = config_max {
        println!("最大值配置为 {}", max);
    }
    
    // if let 可以有 else
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("州 25 美分硬币来自 {:?}!", state);
    } else {
        count += 1;
    }
}
```

### while let 循环

```rust
fn main() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    // 使用 while let 处理栈
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
```

## 7.3 Option 及常用方法

### Option 枚举定义

```rust
// Option 是标准库中定义的枚举
// enum Option<T> {
//     None,
//     Some(T),
// }

fn main() {
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    
    // Option 强制处理空值情况
    // let sum = some_number + 5;  // 错误！不能直接相加
}
```

### 使用 Option

```rust
fn main() {
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;
    
    // 使用 match 处理 Option
    match x {
        Some(value) => println!("x 的值是: {}", value),
        None => println!("x 没有值"),
    }
    
    // 获取 Option 中的值
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("six: {:?}", six);
    println!("none: {:?}", none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
```

### Option 的常用方法

```rust
fn main() {
    let x = Some(2);
    let y: Option<i32> = None;
    
    // unwrap_or：提供默认值
    println!("x: {}", x.unwrap_or(0));
    println!("y: {}", y.unwrap_or(0));
    
    // unwrap_or_else：使用闭包提供默认值
    let k = y.unwrap_or_else(|| 2 * 3);
    println!("k: {}", k);
    
    // map：转换 Option 中的值
    let x_plus_one = x.map(|v| v + 1);
    println!("x_plus_one: {:?}", x_plus_one);
    
    // and_then：链式操作
    let result = Some(2)
        .and_then(|x| Some(x + 1))
        .and_then(|x| Some(x * 2));
    println!("result: {:?}", result);
    
    // filter：根据条件过滤
    let number = Some(4);
    let filtered = number.filter(|&x| x % 2 == 0);
    println!("filtered: {:?}", filtered);
}
```

### Option 的更多方法

```rust
fn main() {
    // is_some 和 is_none
    let x = Some(2);
    let y: Option<i32> = None;
    
    println!("x.is_some(): {}", x.is_some());
    println!("y.is_none(): {}", y.is_none());
    
    // as_ref：将 &Option<T> 转换为 Option<&T>
    let text: Option<String> = Some(String::from("Hello"));
    let text_length: Option<usize> = text.as_ref().map(|s| s.len());
    println!("文本长度: {:?}", text_length);
    println!("原始文本仍然有效: {:?}", text);
    
    // ok_or：将 Option 转换为 Result
    let x = Some("foo");
    assert_eq!(x.ok_or("错误"), Ok("foo"));
    
    let y: Option<&str> = None;
    assert_eq!(y.ok_or("错误"), Err("错误"));
    
    // take：取出 Option 的值，留下 None
    let mut x = Some(2);
    let y = x.take();
    assert_eq!(x, None);
    assert_eq!(y, Some(2));
}
```

### Option 与错误处理

```rust
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn main() {
    let result = divide(2.0, 3.0);
    match result {
        Some(x) => println!("结果: {}", x),
        None => println!("不能除以 0"),
    }
    
    // 使用 ? 操作符（在返回 Option 的函数中）
    fn try_divide() -> Option<f64> {
        let x = divide(10.0, 2.0)?;  // 如果是 None，提前返回
        let y = divide(x, 2.0)?;
        Some(y)
    }
    
    println!("链式除法: {:?}", try_divide());
}
```

## 7.4 内存布局

### 枚举的内存表示

```rust
use std::mem;

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    println!("Message 大小: {} 字节", mem::size_of::<Message>());
    
    // 不同变体的大小
    let quit = Message::Quit;
    let move_msg = Message::Move { x: 10, y: 20 };
    let write = Message::Write(String::from("hello"));
    let color = Message::ChangeColor(255, 0, 0);
    
    // 枚举的大小是最大变体的大小加上标签
}
```

### Option 的优化

```rust
use std::mem;

fn main() {
    // Option 的空指针优化
    println!("Option<i32> 大小: {} 字节", mem::size_of::<Option<i32>>());
    println!("i32 大小: {} 字节", mem::size_of::<i32>());
    
    // 对于引用，Option 可以使用空指针优化
    println!("&i32 大小: {} 字节", mem::size_of::<&i32>());
    println!("Option<&i32> 大小: {} 字节", mem::size_of::<Option<&i32>>());
    
    // Box 也有空指针优化
    println!("Box<i32> 大小: {} 字节", mem::size_of::<Box<i32>>());
    println!("Option<Box<i32>> 大小: {} 字节", mem::size_of::<Option<Box<i32>>>());
}
```

### C 风格枚举

```rust
#[repr(C)]
enum Status {
    Ok = 0,
    Error = 1,
    Pending = 2,
}

#[repr(i32)]
enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    ServerError = 500,
}

fn main() {
    let status = Status::Ok;
    let http = HttpStatus::NotFound;
    
    // 可以转换为整数
    println!("HTTP 状态码: {}", http as i32);
}
```

### 枚举的模式匹配优化

```rust
enum OptimizedEnum {
    A,
    B,
    C,
}

fn process(e: OptimizedEnum) -> u32 {
    // 编译器可能优化为跳转表
    match e {
        OptimizedEnum::A => 1,
        OptimizedEnum::B => 2,
        OptimizedEnum::C => 3,
    }
}

// 复杂匹配
enum ComplexEnum {
    Simple(i32),
    Tuple(i32, i32),
    Struct { x: i32, y: i32 },
}

fn complex_match(e: ComplexEnum) {
    match e {
        ComplexEnum::Simple(x) if x > 0 => println!("正数: {}", x),
        ComplexEnum::Simple(x) => println!("非正数: {}", x),
        ComplexEnum::Tuple(x, y) => println!("元组: ({}, {})", x, y),
        ComplexEnum::Struct { x, y } => println!("结构体: x={}, y={}", x, y),
    }
}
```

### Result 枚举

```rust
// Result 是另一个重要的标准库枚举
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt");
    
    let greeting_file = match greeting_file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("创建文件失败: {:?}", e),
            },
            other_error => {
                panic!("打开文件失败: {:?}", other_error);
            }
        },
    };
}
```

### 自定义迭代器枚举

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

impl List {
    fn new() -> Self {
        Nil
    }
    
    fn prepend(self, elem: i32) -> Self {
        Cons(elem, Box::new(self))
    }
    
    fn len(&self) -> u32 {
        match self {
            Cons(_, tail) => 1 + tail.len(),
            Nil => 0,
        }
    }
}

fn main() {
    let list = List::new()
        .prepend(1)
        .prepend(2)
        .prepend(3);
    
    println!("链表长度: {}", list.len());
}
```

## 小结

本章深入学习了 Rust 的枚举类型：

1. **定义与使用**：枚举可以定义一组可能的值，每个值可以携带不同类型的数据
2. **match 操作**：强大的模式匹配，确保处理所有可能的情况
3. **Option 类型**：Rust 处理空值的标准方式，提供了丰富的方法
4. **内存布局**：了解枚举在内存中的表示和优化

枚举是 Rust 类型系统的核心特性，配合模式匹配，可以编写出安全且表达力强的代码。下一章我们将学习泛型和特征，进一步提升代码的复用性。 