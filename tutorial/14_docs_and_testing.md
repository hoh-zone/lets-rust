# 第14章：文档与测试

Rust 提供了完善的文档和测试工具，包括文档注释、单元测试、文档测试和集成测试。

## 14.1 注释与文档

### 基本注释

```rust
// 这是行注释
// 可以跨越多行

/* 
   这是块注释
   也可以跨越多行
*/

/* 块注释可以 /* 嵌套 */ 使用 */

fn main() {
    // 函数内的注释
    let x = 5; // 行尾注释
}
```

### 文档注释

```rust
/// 计算两个数的和
/// 
/// # 示例
/// 
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 表示二维空间中的点
/// 
/// # 字段
/// 
/// * `x` - X坐标
/// * `y` - Y坐标
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// 创建一个新的点
    /// 
    /// # 参数
    /// 
    /// * `x` - X坐标值
    /// * `y` - Y坐标值
    /// 
    /// # 返回值
    /// 
    /// 返回一个新的 `Point` 实例
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
    
    /// 计算到原点的距离
    /// 
    /// 使用勾股定理计算：√(x² + y²)
    /// 
    /// # 示例
    /// 
    /// ```
    /// # use crate::Point;
    /// let p = Point::new(3.0, 4.0);
    /// assert_eq!(p.distance_from_origin(), 5.0);
    /// ```
    pub fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
```

### 模块级文档

```rust
//! # 几何计算库
//! 
//! 这个库提供基本的几何形状和计算功能。
//! 
//! ## 主要功能
//! 
//! - 点和向量操作
//! - 基本形状（圆、矩形、三角形）
//! - 面积和周长计算
//! 
//! ## 快速开始
//! 
//! ```
//! use geometry::{Point, Circle};
//! 
//! let center = Point::new(0.0, 0.0);
//! let circle = Circle::new(center, 5.0);
//! println!("圆的面积: {}", circle.area());
//! ```

/// 圆形结构体
pub struct Circle {
    /// 圆心
    pub center: Point,
    /// 半径
    pub radius: f64,
}
```

### 文档属性

```rust
/// 复杂的数学函数
/// 
/// # Panics
/// 
/// 当 `x` 为负数时会 panic
/// 
/// # Errors
/// 
/// 当结果溢出时返回错误
/// 
/// # Safety
/// 
/// 这是一个安全的函数
/// 
/// # Examples
/// 
/// 基本用法：
/// 
/// ```
/// let result = complex_math(5.0).unwrap();
/// assert!(result > 0.0);
/// ```
/// 
/// 处理错误：
/// 
/// ```
/// match complex_math(f64::MAX) {
///     Ok(val) => println!("结果: {}", val),
///     Err(e) => println!("错误: {}", e),
/// }
/// ```
pub fn complex_math(x: f64) -> Result<f64, String> {
    if x < 0.0 {
        panic!("x 不能为负数");
    }
    
    let result = x * x * x;
    if result.is_infinite() {
        Err("结果溢出".to_string())
    } else {
        Ok(result)
    }
}
```

### 隐藏文档测试代码

```rust
/// 使用向量的例子
/// 
/// ```
/// # use std::vec::Vec;
/// # fn main() {
/// let mut vec = Vec::new();
/// vec.push(1);
/// # assert_eq!(vec.len(), 1);
/// # }
/// ```
pub fn vector_example() {
    // 函数实现
}

/// 忽略某些测试
/// 
/// ```ignore
/// // 这段代码不会运行
/// let result = expensive_operation();
/// ```
/// 
/// ```no_run
/// // 这段代码会编译但不会运行
/// # fn main() {
/// loop {
///     println!("无限循环");
/// }
/// # }
/// ```
pub fn special_examples() {}
```

## 14.2 单元测试

### 基本单元测试

```rust
// src/lib.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("除数不能为零".to_string())
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }
    
    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
        assert_eq!(divide(1.0, 3.0).unwrap(), 1.0 / 3.0);
    }
    
    #[test]
    fn test_divide_by_zero() {
        assert_eq!(divide(5.0, 0.0), Err("除数不能为零".to_string()));
    }
    
    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("这个测试应该 panic");
    }
    
    #[test]
    #[should_panic(expected = "除数不能为零")]
    fn test_panic_with_message() {
        divide(5.0, 0.0).unwrap(); // 这会 panic
    }
}
```

### 测试组织

```rust
pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }
    
    pub fn area(&self) -> f64 {
        self.width * self.height
    }
    
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // 辅助函数
    fn create_test_rectangles() -> (Rectangle, Rectangle, Rectangle) {
        let rect1 = Rectangle::new(8.0, 7.0);
        let rect2 = Rectangle::new(5.0, 3.0);
        let rect3 = Rectangle::new(9.0, 6.0);
        (rect1, rect2, rect3)
    }
    
    #[test]
    fn test_area() {
        let rect = Rectangle::new(4.0, 5.0);
        assert_eq!(rect.area(), 20.0);
    }
    
    #[test]
    fn test_can_hold() {
        let (rect1, rect2, rect3) = create_test_rectangles();
        
        assert!(rect1.can_hold(&rect2));
        assert!(!rect1.can_hold(&rect3));
        assert!(!rect2.can_hold(&rect1));
    }
    
    // 使用 Result 的测试
    #[test]
    fn test_with_result() -> Result<(), String> {
        let rect = Rectangle::new(4.0, 5.0);
        if rect.area() == 20.0 {
            Ok(())
        } else {
            Err("面积计算错误".to_string())
        }
    }
}
```

### 测试私有函数

```rust
// 私有函数
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_internal() {
        // 可以测试私有函数
        assert_eq!(internal_adder(2, 3), 5);
    }
    
    #[test]
    fn test_public() {
        assert_eq!(add_two(3), 5);
    }
}
```

### 忽略测试

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn fast_test() {
        assert_eq!(1 + 1, 2);
    }
    
    #[test]
    #[ignore]
    fn expensive_test() {
        // 模拟耗时操作
        use std::thread;
        use std::time::Duration;
        
        thread::sleep(Duration::from_secs(1));
        assert_eq!(1 + 1, 2);
    }
    
    #[test]
    #[ignore = "需要外部服务"]
    fn integration_test() {
        // 需要数据库连接的测试
    }
}

// 运行所有测试：cargo test
// 只运行被忽略的测试：cargo test -- --ignored
// 运行所有测试（包括被忽略的）：cargo test -- --include-ignored
```

## 14.3 文档测试

### 基本文档测试

```rust
/// 斐波那契数列计算
/// 
/// # 示例
/// 
/// ```
/// # use crate::fibonacci;
/// assert_eq!(fibonacci(0), 0);
/// assert_eq!(fibonacci(1), 1);
/// assert_eq!(fibonacci(5), 5);
/// assert_eq!(fibonacci(10), 55);
/// ```
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

/// 字符串处理工具
/// 
/// # 示例
/// 
/// 基本使用：
/// 
/// ```
/// # use crate::StringProcessor;
/// let processor = StringProcessor::new("Hello");
/// assert_eq!(processor.reverse(), "olleH");
/// ```
/// 
/// 链式调用：
/// 
/// ```
/// # use crate::StringProcessor;
/// let result = StringProcessor::new("hello")
///     .to_uppercase()
///     .add_suffix("!")
///     .get();
/// assert_eq!(result, "HELLO!");
/// ```
pub struct StringProcessor {
    data: String,
}

impl StringProcessor {
    pub fn new(s: &str) -> Self {
        StringProcessor { data: s.to_string() }
    }
    
    pub fn reverse(&self) -> String {
        self.data.chars().rev().collect()
    }
    
    pub fn to_uppercase(mut self) -> Self {
        self.data = self.data.to_uppercase();
        self
    }
    
    pub fn add_suffix(mut self, suffix: &str) -> Self {
        self.data.push_str(suffix);
        self
    }
    
    pub fn get(self) -> String {
        self.data
    }
}
```

### 文档测试的特殊语法

```rust
/// 复杂的示例
/// 
/// ```rust
/// # // 隐藏的设置代码
/// # let secret_key = "secret";
/// # 
/// // 用户看到的代码
/// let encoder = Encoder::new();
/// let encoded = encoder.encode("message", secret_key);
/// # assert!(!encoded.is_empty());
/// ```
/// 
/// 编译失败的例子：
/// 
/// ```compile_fail
/// let x: i32 = "not a number"; // 这会编译失败
/// ```
/// 
/// 应该 panic 的例子：
/// 
/// ```should_panic
/// # use crate::divide;
/// divide(10, 0); // 这应该 panic
/// ```
pub struct Encoder;

impl Encoder {
    pub fn new() -> Self {
        Encoder
    }
    
    pub fn encode(&self, message: &str, key: &str) -> String {
        format!("{}-{}", message, key)
    }
}

pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("除数不能为零");
    }
    a / b
}
```

## 14.4 集成测试

### 基本集成测试

```rust
// tests/integration_test.rs
use my_crate;

#[test]
fn test_public_api() {
    let result = my_crate::add(2, 3);
    assert_eq!(result, 5);
}

#[test]
fn test_complex_workflow() {
    // 测试多个模块的交互
    let processor = my_crate::StringProcessor::new("hello");
    let reversed = processor.reverse();
    assert_eq!(reversed, "olleh");
    
    let point = my_crate::Point::new(3.0, 4.0);
    assert_eq!(point.distance_from_origin(), 5.0);
}
```

### 集成测试的模块化

```rust
// tests/common/mod.rs
pub fn setup() -> TestContext {
    println!("设置测试环境");
    TestContext {
        temp_dir: create_temp_dir(),
        config: load_test_config(),
    }
}

pub struct TestContext {
    pub temp_dir: String,
    pub config: Config,
}

pub struct Config {
    pub test_mode: bool,
}

fn create_temp_dir() -> String {
    "/tmp/test".to_string()
}

fn load_test_config() -> Config {
    Config { test_mode: true }
}

// tests/test_with_setup.rs
mod common;

#[test]
fn test_with_context() {
    let ctx = common::setup();
    assert!(ctx.config.test_mode);
    
    // 使用测试环境进行测试
}
```

### 自定义测试框架

```rust
// tests/custom_test.rs
// 使用自定义测试宏
macro_rules! test_case {
    ($name:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let result = process($input);
            assert_eq!(result, $expected);
        }
    };
}

fn process(input: &str) -> String {
    input.to_uppercase()
}

test_case!(test_hello, "hello", "HELLO");
test_case!(test_world, "world", "WORLD");
test_case!(test_empty, "", "");

// 参数化测试
#[cfg(test)]
mod parameterized_tests {
    use super::*;
    
    #[test]
    fn test_multiple_cases() {
        let test_cases = vec![
            ("hello", "HELLO"),
            ("world", "WORLD"),
            ("Rust", "RUST"),
            ("", ""),
        ];
        
        for (input, expected) in test_cases {
            assert_eq!(process(input), expected, 
                      "Failed for input: {}", input);
        }
    }
}
```

## RWO 权限分析

### 测试中的所有权

```rust
#[derive(Debug, PartialEq)]
pub struct Resource {
    data: String,
}

impl Resource {
    pub fn new(data: String) -> Self {
        Resource { data }
    }
    
    // R: 不可变借用
    pub fn read(&self) -> &str {
        &self.data
    }
    
    // W: 可变借用
    pub fn write(&mut self, data: String) {
        self.data = data;
    }
    
    // O: 获取所有权
    pub fn consume(self) -> String {
        self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_read_permission() {
        let resource = Resource::new("test".to_string());
        
        // R: 可以多次借用读取
        let data1 = resource.read();
        let data2 = resource.read();
        assert_eq!(data1, data2);
        assert_eq!(data1, "test");
    }
    
    #[test]
    fn test_write_permission() {
        let mut resource = Resource::new("old".to_string());
        
        // W: 需要可变借用
        resource.write("new".to_string());
        assert_eq!(resource.read(), "new");
    }
    
    #[test]
    fn test_ownership_transfer() {
        let resource = Resource::new("data".to_string());
        
        // O: 消费所有权
        let data = resource.consume();
        assert_eq!(data, "data");
        
        // resource 不再可用
        // let _ = resource.read(); // 编译错误
    }
}
```

### 测试辅助函数的权限设计

```rust
#[cfg(test)]
mod test_helpers {
    use super::*;
    
    // R: 返回共享引用
    pub fn get_test_data() -> &'static str {
        "test data"
    }
    
    // O: 创建并返回所有权
    pub fn create_test_resource() -> Resource {
        Resource::new("test resource".to_string())
    }
    
    // R: 借用参数进行验证
    pub fn assert_resource_valid(resource: &Resource) {
        assert!(!resource.read().is_empty());
    }
    
    // W: 修改测试对象
    pub fn modify_for_test(resource: &mut Resource) {
        resource.write("modified".to_string());
    }
    
    // O: 构建器模式，链式调用转移所有权
    pub struct TestBuilder {
        resources: Vec<Resource>,
    }
    
    impl TestBuilder {
        pub fn new() -> Self {
            TestBuilder { resources: Vec::new() }
        }
        
        // O: 获取资源所有权
        pub fn add_resource(mut self, resource: Resource) -> Self {
            self.resources.push(resource);
            self
        }
        
        // O: 构建并返回所有权
        pub fn build(self) -> Vec<Resource> {
            self.resources
        }
    }
}

#[cfg(test)]
mod tests_with_helpers {
    use super::*;
    use super::test_helpers::*;
    
    #[test]
    fn test_with_builder() {
        let resources = TestBuilder::new()
            .add_resource(create_test_resource())
            .add_resource(Resource::new("another".to_string()))
            .build();
        
        assert_eq!(resources.len(), 2);
    }
}
```

### Mock 对象的权限管理

```rust
// 定义 trait
pub trait Database {
    fn fetch(&self, id: u32) -> Option<String>;
    fn store(&mut self, id: u32, data: String);
    fn delete(&mut self, id: u32) -> bool;
}

// 真实实现
pub struct RealDatabase {
    data: std::collections::HashMap<u32, String>,
}

impl Database for RealDatabase {
    fn fetch(&self, id: u32) -> Option<String> {
        self.data.get(&id).cloned()
    }
    
    fn store(&mut self, id: u32, data: String) {
        self.data.insert(id, data);
    }
    
    fn delete(&mut self, id: u32) -> bool {
        self.data.remove(&id).is_some()
    }
}

#[cfg(test)]
mod mocks {
    use super::*;
    use std::cell::RefCell;
    
    // Mock 实现
    pub struct MockDatabase {
        // 使用 RefCell 实现内部可变性
        fetch_calls: RefCell<Vec<u32>>,
        store_calls: RefCell<Vec<(u32, String)>>,
        fetch_responses: std::collections::HashMap<u32, String>,
    }
    
    impl MockDatabase {
        pub fn new() -> Self {
            MockDatabase {
                fetch_calls: RefCell::new(Vec::new()),
                store_calls: RefCell::new(Vec::new()),
                fetch_responses: std::collections::HashMap::new(),
            }
        }
        
        // 设置模拟响应
        pub fn with_fetch_response(mut self, id: u32, response: String) -> Self {
            self.fetch_responses.insert(id, response);
            self
        }
        
        // R: 验证调用（不可变借用）
        pub fn verify_fetch_called(&self, id: u32) -> bool {
            self.fetch_calls.borrow().contains(&id)
        }
        
        pub fn verify_store_called(&self, id: u32, data: &str) -> bool {
            self.store_calls.borrow()
                .iter()
                .any(|(i, d)| i == &id && d == data)
        }
    }
    
    impl Database for MockDatabase {
        fn fetch(&self, id: u32) -> Option<String> {
            // R: 记录调用（通过 RefCell）
            self.fetch_calls.borrow_mut().push(id);
            self.fetch_responses.get(&id).cloned()
        }
        
        fn store(&mut self, id: u32, data: String) {
            // W: 记录调用
            self.store_calls.borrow_mut().push((id, data));
        }
        
        fn delete(&mut self, id: u32) -> bool {
            true // 简单返回成功
        }
    }
}

// 使用 Database 的服务
pub struct UserService<D: Database> {
    db: D,
}

impl<D: Database> UserService<D> {
    pub fn new(db: D) -> Self {
        UserService { db }
    }
    
    pub fn get_user(&self, id: u32) -> Option<String> {
        self.db.fetch(id)
    }
    
    pub fn save_user(&mut self, id: u32, name: String) {
        self.db.store(id, name);
    }
}

#[cfg(test)]
mod service_tests {
    use super::*;
    use super::mocks::MockDatabase;
    
    #[test]
    fn test_get_user() {
        let mock_db = MockDatabase::new()
            .with_fetch_response(1, "Alice".to_string());
        
        let service = UserService::new(mock_db);
        let user = service.get_user(1);
        
        assert_eq!(user, Some("Alice".to_string()));
        assert!(service.db.verify_fetch_called(1));
    }
    
    #[test]
    fn test_save_user() {
        let mock_db = MockDatabase::new();
        let mut service = UserService::new(mock_db);
        
        service.save_user(1, "Bob".to_string());
        
        assert!(service.db.verify_store_called(1, "Bob"));
    }
}
```

### 并发测试中的权限

```rust
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone)]
pub struct ConcurrentCounter {
    count: Arc<Mutex<i32>>,
}

impl ConcurrentCounter {
    pub fn new() -> Self {
        ConcurrentCounter {
            count: Arc::new(Mutex::new(0)),
        }
    }
    
    // 通过 Arc 共享所有权
    pub fn increment(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
    }
    
    pub fn get(&self) -> i32 {
        *self.count.lock().unwrap()
    }
}

#[cfg(test)]
mod concurrent_tests {
    use super::*;
    
    #[test]
    fn test_concurrent_increment() {
        let counter = ConcurrentCounter::new();
        let mut handles = vec![];
        
        for _ in 0..10 {
            // 克隆 Arc，共享所有权
            let counter_clone = counter.clone();
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    counter_clone.increment();
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(counter.get(), 1000);
    }
}
```

### 生命周期测试

```rust
pub struct Container<'a> {
    data: &'a str,
}

impl<'a> Container<'a> {
    pub fn new(data: &'a str) -> Self {
        Container { data }
    }
    
    pub fn get(&self) -> &'a str {
        self.data
    }
}

#[cfg(test)]
mod lifetime_tests {
    use super::*;
    
    #[test]
    fn test_lifetime_constraints() {
        let data = String::from("test data");
        let container = Container::new(&data);
        
        assert_eq!(container.get(), "test data");
        
        // 生命周期保证数据有效
        let reference = container.get();
        drop(container); // container 被释放
        assert_eq!(reference, "test data"); // reference 仍然有效
        
        // drop(data); // 如果释放 data，reference 将无效
    }
    
    #[test]
    fn test_scoped_lifetime() {
        let container;
        {
            let data = String::from("scoped");
            container = Container::new(&data);
            assert_eq!(container.get(), "scoped");
        } // data 在这里被释放
        
        // 以下代码会编译错误，因为 data 已经超出作用域
        // let _ = container.get();
    }
}
```

## 小结

本章深入学习了 Rust 的文档和测试系统：

1. **文档**：
   - 使用 `///` 编写文档注释
   - 支持 Markdown 格式
   - 文档示例会作为测试运行
   - 使用 `//!` 编写模块级文档

2. **单元测试**：
   - 使用 `#[test]` 标记测试函数
   - 在 `#[cfg(test)]` 模块中组织测试
   - 支持 `should_panic` 和 `ignore` 属性
   - 可以测试私有函数

3. **文档测试**：
   - 文档中的代码示例自动成为测试
   - 支持隐藏设置代码
   - 可以标记 `no_run`、`ignore`、`compile_fail`

4. **集成测试**：
   - 放在 `tests` 目录
   - 只能测试公开 API
   - 可以共享测试工具代码

5. **RWO 权限分析**：
   - **R**：测试中频繁使用不可变借用验证状态
   - **W**：Mock 对象常用 `RefCell` 实现内部可变性
   - **O**：测试构建器模式转移所有权
   - 并发测试使用 `Arc` 共享所有权

良好的测试和文档是高质量代码的重要组成部分。Rust 的工具让编写和维护测试变得简单高效。下一章我们将学习闭包。