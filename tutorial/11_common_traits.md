# 第11章：常用的特征解析

Rust 标准库提供了许多重要的特征，理解和正确使用它们是编写优秀 Rust 代码的关键。

## 11.1 常用的特征（Trait）介绍

### Display 和 Debug

```rust
use std::fmt;

// Display 用于用户友好的输出
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Debug 用于调试输出
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 手动实现 Debug
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

fn main() {
    let p = Point { x: 10, y: 20 };
    println!("Display: {}", p);
    println!("Debug: {:?}", p);
    
    let rect = Rectangle { width: 30, height: 50 };
    println!("Rectangle: {:?}", rect);
}
```

### Clone 和 Copy

```rust
// Clone - 显式深拷贝
#[derive(Clone)]
struct Book {
    title: String,
    author: String,
}

// Copy - 隐式按位复制（只能用于栈上数据）
#[derive(Debug, Copy, Clone)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

// 实现 Copy 的要求
// 1. 所有字段都必须是 Copy
// 2. 不能手动实现 Drop

fn main() {
    // Clone 的使用
    let book1 = Book {
        title: String::from("Rust Programming"),
        author: String::from("Steve Klabnik"),
    };
    let book2 = book1.clone();
    println!("Book1: {} by {}", book1.title, book1.author);
    println!("Book2: {} by {}", book2.title, book2.author);
    
    // Copy 的使用
    let p1 = Point3D { x: 1.0, y: 2.0, z: 3.0 };
    let p2 = p1;  // 自动复制
    println!("p1: {:?}", p1);  // p1 仍然可用
    println!("p2: {:?}", p2);
}
```

### PartialEq 和 Eq

```rust
use std::cmp::Ordering;

// PartialEq - 部分等价关系
#[derive(PartialEq)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

// Eq - 完全等价关系（自反性）
#[derive(PartialEq, Eq)]
struct UserId(u64);

// 手动实现 PartialEq
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

// 浮点数只实现 PartialEq，不实现 Eq
fn float_comparison() {
    let nan = f64::NAN;
    println!("NaN == NaN: {}", nan == nan);  // false
    
    // 这就是为什么 f64 不能实现 Eq
}

fn main() {
    let v1 = Version { major: 1, minor: 0, patch: 0 };
    let v2 = Version { major: 1, minor: 0, patch: 0 };
    let v3 = Version { major: 2, minor: 0, patch: 0 };
    
    println!("v1 == v2: {}", v1 == v2);  // true
    println!("v1 == v3: {}", v1 == v3);  // false
    
    let id1 = UserId(1001);
    let id2 = UserId(1001);
    println!("id1 == id2: {}", id1 == id2);  // true
}
```

### PartialOrd 和 Ord

```rust
use std::cmp::Ordering;

// PartialOrd - 部分排序
#[derive(PartialEq, PartialOrd)]
struct Score(f64);

// Ord - 全序关系
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Priority(i32);

// 手动实现排序
#[derive(PartialEq, Eq)]
struct Person {
    name: String,
    age: u32,
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Person {
    fn cmp(&self, other: &Self) -> Ordering {
        // 先按年龄排序，再按姓名排序
        self.age.cmp(&other.age)
            .then_with(|| self.name.cmp(&other.name))
    }
}

fn main() {
    let mut scores = vec![Score(85.5), Score(92.0), Score(78.5)];
    scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let mut priorities = vec![Priority(3), Priority(1), Priority(2)];
    priorities.sort();  // 可以直接排序
    
    let mut people = vec![
        Person { name: String::from("Alice"), age: 30 },
        Person { name: String::from("Bob"), age: 25 },
        Person { name: String::from("Charlie"), age: 30 },
    ];
    people.sort();
    
    for person in &people {
        println!("{} ({})", person.name, person.age);
    }
}
```

### Hash

```rust
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Hash, PartialEq, Eq)]
struct Key {
    id: u64,
    name: String,
}

// 手动实现 Hash
struct CaseInsensitiveString(String);

impl Hash for CaseInsensitiveString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_lowercase().hash(state);
    }
}

impl PartialEq for CaseInsensitiveString {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}

impl Eq for CaseInsensitiveString {}

fn main() {
    let mut map = HashMap::new();
    
    let key1 = Key { id: 1, name: String::from("Alice") };
    let key2 = Key { id: 2, name: String::from("Bob") };
    
    map.insert(key1, "Value 1");
    map.insert(key2, "Value 2");
    
    // 大小写不敏感的键
    let mut case_map = HashMap::new();
    case_map.insert(CaseInsensitiveString(String::from("Hello")), 1);
    case_map.insert(CaseInsensitiveString(String::from("HELLO")), 2);
    
    println!("Map size: {}", case_map.len());  // 1，因为键相同
}
```

## 11.2 特征实现与应用场景

### Iterator 特征

```rust
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// 使用 Iterator 的强大功能
fn iterator_examples() {
    let counter = Counter::new(5);
    
    // map
    let squares: Vec<u32> = counter.map(|x| x * x).collect();
    println!("Squares: {:?}", squares);
    
    // filter
    let evens: Vec<u32> = Counter::new(10)
        .filter(|x| x % 2 == 0)
        .collect();
    println!("Evens: {:?}", evens);
    
    // fold
    let sum = Counter::new(100).fold(0, |acc, x| acc + x);
    println!("Sum: {}", sum);
}

fn main() {
    iterator_examples();
}
```

### From 和 Into

```rust
use std::convert::From;

#[derive(Debug)]
struct Millimeters(f64);

#[derive(Debug)]
struct Meters(f64);

impl From<Meters> for Millimeters {
    fn from(m: Meters) -> Self {
        Millimeters(m.0 * 1000.0)
    }
}

// From 自动实现了 Into
fn convert_example() {
    let m = Meters(1.5);
    
    // 使用 From
    let mm = Millimeters::from(m);
    println!("{:?}", mm);
    
    // 使用 Into
    let m2 = Meters(2.0);
    let mm2: Millimeters = m2.into();
    println!("{:?}", mm2);
}

// 错误转换
#[derive(Debug)]
enum MyError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        MyError::Io(err)
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(err: std::num::ParseIntError) -> Self {
        MyError::Parse(err)
    }
}

fn error_conversion() -> Result<(), MyError> {
    let _file = std::fs::File::open("foo.txt")?;  // ? 自动转换错误类型
    let _number: i32 = "42".parse()?;
    Ok(())
}
```

### Default

```rust
#[derive(Default)]
struct Config {
    timeout: u64,
    retries: u32,
    verbose: bool,
}

// 手动实现 Default
impl Default for Point {
    fn default() -> Self {
        Point { x: 0, y: 0 }
    }
}

// 使用 Default 的构建器模式
#[derive(Default)]
struct ServerBuilder {
    host: Option<String>,
    port: Option<u16>,
    threads: Option<usize>,
}

impl ServerBuilder {
    fn new() -> Self {
        Default::default()
    }
    
    fn host(mut self, host: String) -> Self {
        self.host = Some(host);
        self
    }
    
    fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
    
    fn build(self) -> Server {
        Server {
            host: self.host.unwrap_or_else(|| "localhost".to_string()),
            port: self.port.unwrap_or(8080),
            threads: self.threads.unwrap_or(4),
        }
    }
}

struct Server {
    host: String,
    port: u16,
    threads: usize,
}

fn main() {
    let config = Config::default();
    println!("Default config: timeout={}, retries={}, verbose={}", 
             config.timeout, config.retries, config.verbose);
    
    let server = ServerBuilder::new()
        .host("127.0.0.1".to_string())
        .port(3000)
        .build();
}
```

### Drop

```rust
struct FileWrapper {
    name: String,
    // 实际应用中这里会有文件句柄
}

impl Drop for FileWrapper {
    fn drop(&mut self) {
        println!("Closing file: {}", self.name);
        // 清理资源
    }
}

// RAII 模式
struct Guard<'a> {
    value: &'a mut i32,
    original: i32,
}

impl<'a> Guard<'a> {
    fn new(value: &'a mut i32) -> Self {
        let original = *value;
        *value = 0;  // 重置值
        Guard { value, original }
    }
}

impl<'a> Drop for Guard<'a> {
    fn drop(&mut self) {
        *self.value = self.original;  // 恢复原值
        println!("Guard dropped, value restored to {}", self.original);
    }
}

fn main() {
    {
        let _file = FileWrapper { 
            name: String::from("data.txt") 
        };
        println!("Using file...");
    }  // 自动调用 drop
    
    let mut value = 42;
    {
        let _guard = Guard::new(&mut value);
        println!("Value during guard: {}", value);  // 0
    }
    println!("Value after guard: {}", value);  // 42
}
```

## RWO 权限分析

### Display/Debug 的权限要求

```rust
use std::fmt;

struct Data {
    value: String,
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // R: 只需要读权限
        write!(f, "{}", self.value)
    }
}

fn display_analysis() {
    let data = Data { value: String::from("Hello") };
    
    // R: println! 只需要不可变引用
    println!("{}", &data);
    
    // 可以同时有多个读取
    let r1 = &data;
    let r2 = &data;
    println!("{} {}", r1, r2);
}
```

### Clone/Copy 的权限影响

```rust
#[derive(Clone)]
struct Heavy {
    data: Vec<i32>,
}

#[derive(Copy, Clone)]
struct Light {
    x: i32,
    y: i32,
}

fn clone_copy_analysis() {
    // Clone: 显式创建新的所有权
    let h1 = Heavy { data: vec![1, 2, 3] };
    let h2 = h1.clone();  // O: h2 获得新的所有权
    // h1 和 h2 都拥有各自的数据
    
    // Copy: 隐式复制
    let l1 = Light { x: 10, y: 20 };
    let l2 = l1;  // 自动复制，l1 仍然有效
    println!("l1: ({}, {})", l1.x, l1.y);  // R: 仍可读取
    
    // 函数调用时的区别
    fn take_heavy(h: Heavy) {
        // O: 获得所有权
    }
    
    fn take_light(l: Light) {
        // 获得副本，不影响调用者
    }
    
    take_heavy(h1);  // O: h1 所有权转移
    // take_heavy(h1);  // 错误！h1 已经移动
    
    take_light(l1);  // 复制
    take_light(l1);  // 可以再次调用
}
```

### Iterator 的权限模式

```rust
struct Container {
    items: Vec<i32>,
}

impl Container {
    // R: 不可变迭代器
    fn iter(&self) -> std::slice::Iter<'_, i32> {
        self.items.iter()
    }
    
    // W: 可变迭代器
    fn iter_mut(&mut self) -> std::slice::IterMut<'_, i32> {
        self.items.iter_mut()
    }
    
    // O: 消费迭代器
    fn into_iter(self) -> std::vec::IntoIter<i32> {
        self.items.into_iter()
    }
}

fn iterator_permissions() {
    let mut container = Container { items: vec![1, 2, 3, 4, 5] };
    
    // R: 只读遍历
    for item in container.iter() {
        println!("Read: {}", item);
    }
    
    // W: 可变遍历
    for item in container.iter_mut() {
        *item *= 2;  // W: 修改元素
    }
    
    // O: 消费容器
    for item in container.into_iter() {
        println!("Owned: {}", item);
    }
    // container 不再可用
}
```

### Drop 与所有权转移

```rust
struct Resource {
    name: String,
}

impl Drop for Resource {
    fn drop(&mut self) {
        // W: drop 需要可变引用
        println!("Dropping resource: {}", self.name);
        self.name.clear();  // 可以修改
    }
}

fn drop_analysis() {
    let r1 = Resource { name: String::from("R1") };
    
    // 显式 drop
    drop(r1);  // O: 获得所有权并立即释放
    // println!("{}", r1.name);  // 错误！r1 已被移动
    
    let r2 = Resource { name: String::from("R2") };
    {
        let r3 = Resource { name: String::from("R3") };
        // r3 在作用域结束时自动 drop
    }
    
    // 手动调用 drop 方法
    let mut r4 = Resource { name: String::from("R4") };
    // r4.drop();  // 错误！不能直接调用
    drop(r4);  // 正确方式
}
```

### 特征的权限最佳实践

```rust
// 1. 只读特征
trait ReadOnly {
    fn read(&self) -> &str;
}

// 2. 可变特征
trait Mutable {
    fn modify(&mut self);
}

// 3. 消费特征
trait Consume {
    fn consume(self) -> String;
}

struct Example {
    data: String,
}

impl ReadOnly for Example {
    fn read(&self) -> &str {
        &self.data  // R: 返回不可变引用
    }
}

impl Mutable for Example {
    fn modify(&mut self) {
        self.data.push_str(" modified");  // W: 需要可变访问
    }
}

impl Consume for Example {
    fn consume(self) -> String {
        self.data  // O: 消费并返回所有权
    }
}

fn trait_permissions() {
    let mut ex = Example { data: String::from("test") };
    
    // R: 多次读取
    println!("{}", ex.read());
    println!("{}", ex.read());
    
    // W: 修改
    ex.modify();
    
    // O: 消费
    let result = ex.consume();
    // ex 不再可用
}
```

## 小结

本章深入学习了 Rust 标准库中的常用特征：

1. **基础特征**：
   - Display/Debug：格式化输出（R）
   - Clone/Copy：复制语义（O）
   - PartialEq/Eq：相等性比较（R）
   - PartialOrd/Ord：排序比较（R）
   - Hash：哈希计算（R）

2. **高级特征**：
   - Iterator：迭代器模式（R/W/O）
   - From/Into：类型转换（O）
   - Default：默认值（O）
   - Drop：资源清理（W）

3. **RWO 权限分析**：
   - 大多数特征只需要读权限（R）
   - Drop 和可变方法需要写权限（W）
   - Clone/From/Into 等涉及所有权转移（O）
   - 正确的权限设计可以提高代码的灵活性和安全性

理解这些特征的语义和权限要求，是写出地道 Rust 代码的基础。下一章我们将学习错误处理机制。 