# 第15章：闭包

闭包（Closure）是 Rust 中的匿名函数，可以捕获其定义时所在环境中的变量。

## 15.1 闭包的定义与基本语法

### 基本语法

```rust
fn main() {
    // 最简单的闭包
    let add_one = |x| x + 1;
    println!("5 + 1 = {}", add_one(5));
    
    // 带类型标注的闭包
    let add: fn(i32, i32) -> i32 = |x, y| x + y;
    println!("3 + 4 = {}", add(3, 4));
    
    // 多行闭包
    let calculate = |x: i32, y: i32| {
        let sum = x + y;
        let product = x * y;
        sum + product
    };
    println!("计算结果: {}", calculate(2, 3));
    
    // 无参数闭包
    let greet = || println!("Hello, World!");
    greet();
}
```

### 闭包与函数的对比

```rust
// 函数定义
fn add_function(x: i32, y: i32) -> i32 {
    x + y
}

// 完整的闭包语法
let add_closure_full = |x: i32, y: i32| -> i32 { x + y };

// 简化的闭包语法（类型推断）
let add_closure = |x, y| x + y;

// 使用示例
fn main() {
    assert_eq!(add_function(2, 3), 5);
    assert_eq!(add_closure_full(2, 3), 5);
    assert_eq!(add_closure(2, 3), 5);
}
```

### 捕获环境变量

```rust
fn main() {
    let multiplier = 5;
    let message = String::from("结果是");
    
    // 闭包捕获环境变量
    let multiply = |x| x * multiplier;
    let show_result = |x| format!("{}: {}", message, x);
    
    println!("{}", multiply(10));  // 50
    println!("{}", show_result(50));  // 结果是: 50
    
    // 函数无法捕获环境变量
    // fn multiply_fn(x: i32) -> i32 {
    //     x * multiplier  // 错误！函数无法访问外部变量
    // }
}
```

### 闭包的类型推断

```rust
fn main() {
    // 第一次调用确定类型
    let identity = |x| x;
    let s = identity(String::from("hello"));  // 推断为 String
    // let n = identity(5);  // 错误！类型已确定为 String
    
    // 显式类型标注
    let identity_num = |x: i32| -> i32 { x };
    let n = identity_num(5);
    
    // 泛型闭包（通过函数包装）
    fn make_identity<T>() -> impl Fn(T) -> T {
        |x| x
    }
    
    let string_identity = make_identity::<String>();
    let num_identity = make_identity::<i32>();
}
```

## 15.2 闭包的常见使用场景

### 迭代器方法

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // map: 转换每个元素
    let squares: Vec<i32> = numbers.iter()
        .map(|x| x * x)
        .collect();
    println!("平方: {:?}", squares);
    
    // filter: 过滤元素
    let evens: Vec<&i32> = numbers.iter()
        .filter(|x| *x % 2 == 0)
        .collect();
    println!("偶数: {:?}", evens);
    
    // fold: 累积计算
    let sum = numbers.iter()
        .fold(0, |acc, x| acc + x);
    println!("总和: {}", sum);
    
    // 链式调用
    let result: Vec<i32> = numbers.iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * x)
        .collect();
    println!("偶数的平方: {:?}", result);
}
```

### 自定义排序

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let mut people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
        Person { name: "Charlie".to_string(), age: 35 },
    ];
    
    // 按年龄排序
    people.sort_by(|a, b| a.age.cmp(&b.age));
    println!("按年龄排序: {:?}", people);
    
    // 按名字长度排序
    people.sort_by_key(|p| p.name.len());
    println!("按名字长度排序: {:?}", people);
    
    // 复杂排序：先按年龄，再按名字
    people.sort_by(|a, b| {
        match a.age.cmp(&b.age) {
            std::cmp::Ordering::Equal => a.name.cmp(&b.name),
            other => other,
        }
    });
}
```

### Option 和 Result 操作

```rust
fn main() {
    // Option 的方法
    let some_number = Some(5);
    let none_number: Option<i32> = None;
    
    // map: 转换 Some 中的值
    let doubled = some_number.map(|x| x * 2);
    assert_eq!(doubled, Some(10));
    
    // and_then: 链式操作
    let result = some_number
        .and_then(|x| if x > 0 { Some(x) } else { None })
        .and_then(|x| Some(x * 2));
    
    // unwrap_or_else: 提供默认值
    let value = none_number.unwrap_or_else(|| {
        println!("计算默认值");
        42
    });
    
    // Result 的方法
    let ok_result: Result<i32, &str> = Ok(10);
    let err_result: Result<i32, &str> = Err("错误");
    
    // map_err: 转换错误类型
    let converted = err_result.map_err(|e| format!("转换的错误: {}", e));
    
    // and_then: 链式操作
    let final_result = ok_result
        .and_then(|x| if x > 5 { Ok(x * 2) } else { Err("太小") })
        .map(|x| x + 1);
}
```

### 延迟计算和缓存

```rust
use std::cell::RefCell;

struct Cache<T> {
    calculation: T,
    value: RefCell<Option<i32>>,
}

impl<T> Cache<T>
where
    T: Fn(i32) -> i32,
{
    fn new(calculation: T) -> Cache<T> {
        Cache {
            calculation,
            value: RefCell::new(None),
        }
    }
    
    fn value(&self, arg: i32) -> i32 {
        match *self.value.borrow() {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                *self.value.borrow_mut() = Some(v);
                v
            }
        }
    }
}

fn main() {
    let expensive_closure = |num| {
        println!("计算中...");
        std::thread::sleep(std::time::Duration::from_secs(1));
        num * num
    };
    
    let cache = Cache::new(expensive_closure);
    
    println!("第一次调用: {}", cache.value(5));  // 会打印"计算中..."
    println!("第二次调用: {}", cache.value(5));  // 使用缓存，不会打印
}
```

### 线程和并发

```rust
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    // 在线程中使用闭包
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("线程中: {}", i);
        }
    });
    
    handle.join().unwrap();
    
    // 捕获变量并移动到线程
    let message = String::from("Hello from thread");
    let handle = thread::spawn(move || {
        println!("{}", message);
    });
    
    handle.join().unwrap();
    
    // 共享状态
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("计数器: {}", *counter.lock().unwrap());
}
```

## 15.3 对闭包的深度解析

### 闭包的三种 trait

```rust
// Fn trait 示例
fn apply_fn<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(5)
}

// FnMut trait 示例
fn apply_fn_mut<F>(mut f: F) -> i32
where
    F: FnMut(i32) -> i32,
{
    f(5)
}

// FnOnce trait 示例
fn apply_fn_once<F>(f: F) -> i32
where
    F: FnOnce(i32) -> i32,
{
    f(5)
}

fn main() {
    let x = 10;
    
    // Fn: 不可变借用
    let fn_closure = |y| x + y;
    println!("Fn: {}", apply_fn(fn_closure));
    println!("可以再次使用 x: {}", x);
    
    // FnMut: 可变借用
    let mut count = 0;
    let mut fn_mut_closure = |y| {
        count += 1;
        y + count
    };
    println!("FnMut: {}", apply_fn_mut(&mut fn_mut_closure));
    
    // FnOnce: 获取所有权
    let s = String::from("hello");
    let fn_once_closure = |y| {
        drop(s);  // 消费 s
        y
    };
    println!("FnOnce: {}", apply_fn_once(fn_once_closure));
    // println!("{}", s);  // 错误！s 已被移动
}
```

### 捕获方式详解

```rust
fn main() {
    // 1. 不可变引用捕获
    let x = 5;
    let print_x = || println!("x = {}", x);
    print_x();
    println!("x 仍然可用: {}", x);
    
    // 2. 可变引用捕获
    let mut y = 10;
    let mut add_to_y = |n| y += n;
    add_to_y(5);
    println!("y = {}", y);  // 15
    
    // 3. 值捕获（move）
    let z = vec![1, 2, 3];
    let consume_z = move || {
        println!("z = {:?}", z);
        // z 的所有权已转移到闭包内
    };
    consume_z();
    // println!("{:?}", z);  // 错误！z 已被移动
    
    // 4. 混合捕获
    let a = 5;
    let mut b = 10;
    let c = String::from("hello");
    
    let mut complex_closure = move |x| {
        println!("a = {}", a);  // 复制
        b += x;  // 移动并可变
        println!("c = {}", c);  // 移动
        b
    };
    
    println!("结果: {}", complex_closure(3));
}
```

### 闭包的大小和实现

```rust
use std::mem;

fn main() {
    // 无捕获的闭包
    let no_capture = || 42;
    println!("无捕获闭包大小: {}", mem::size_of_val(&no_capture));
    
    // 捕获一个 i32
    let x = 5;
    let capture_i32 = || x + 1;
    println!("捕获 i32 闭包大小: {}", mem::size_of_val(&capture_i32));
    
    // 捕获多个变量
    let y = 10;
    let z = 15;
    let capture_multiple = || x + y + z;
    println!("捕获多个变量闭包大小: {}", mem::size_of_val(&capture_multiple));
    
    // move 闭包
    let s = String::from("hello");
    let move_string = move || s.len();
    println!("move String 闭包大小: {}", mem::size_of_val(&move_string));
}
```

### 高级闭包模式

```rust
// 返回闭包
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

// 闭包作为泛型参数
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn map<U, F>(self, f: F) -> Container<U>
    where
        F: FnOnce(T) -> U,
    {
        Container { value: f(self.value) }
    }
}

// 递归闭包（需要额外处理）
fn factorial() -> Box<dyn Fn(u32) -> u32> {
    Box::new(|n| {
        if n <= 1 {
            1
        } else {
            n * factorial()(n - 1)
        }
    })
}

fn main() {
    // 使用返回的闭包
    let add_5 = make_adder(5);
    println!("10 + 5 = {}", add_5(10));
    
    // 使用容器的 map
    let container = Container { value: 5 };
    let mapped = container.map(|x| x * 2);
    println!("映射后: {}", mapped.value);
    
    // 使用递归闭包
    let fact = factorial();
    println!("5! = {}", fact(5));
}
```

## RWO 权限分析

### 闭包的捕获权限

```rust
fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let s = String::from("hello");
    let mut counter = 0;
    
    // R: Fn - 不可变借用
    let read_closure = || {
        println!("读取 data: {:?}", data);
        println!("读取 s: {}", s);
    };
    read_closure();
    read_closure();  // 可以多次调用
    println!("data 仍可用: {:?}", data);
    
    // W: FnMut - 可变借用
    let mut write_closure = || {
        counter += 1;
        println!("计数: {}", counter);
    };
    write_closure();
    write_closure();
    // println!("{}", counter);  // 错误！counter 被可变借用
    
    // O: FnOnce - 获取所有权
    let own_closure = move || {
        let owned_data = data;  // 移动 data
        let owned_s = s;        // 移动 s
        println!("拥有: {:?}, {}", owned_data, owned_s);
    };
    own_closure();
    // own_closure();  // 错误！闭包只能调用一次
    // println!("{:?}", data);  // 错误！data 已被移动
}
```

### 闭包与生命周期

```rust
// 闭包的生命周期约束
fn create_closure<'a>(x: &'a str) -> impl Fn() -> &'a str {
    move || x
}

// 捕获引用的闭包
fn capture_reference() {
    let s = String::from("hello");
    let r = &s;
    
    // 闭包捕获引用
    let closure = || {
        println!("引用: {}", r);
    };
    
    closure();
    // drop(s);  // 错误！s 被借用
    closure();  // r 仍然有效
}

// 返回捕获引用的闭包
fn return_closure_with_lifetime<'a>(s: &'a str) -> Box<dyn Fn() + 'a> {
    Box::new(move || {
        println!("捕获的字符串: {}", s);
    })
}

fn main() {
    let closure = create_closure("static string");
    println!("{}", closure());
    
    let owned_string = String::from("owned");
    let closure2 = return_closure_with_lifetime(&owned_string);
    closure2();
}
```

### 闭包的权限转换

```rust
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // 使用 Rc 共享所有权
    let shared_data = Rc::new(vec![1, 2, 3]);
    let data_clone = Rc::clone(&shared_data);
    
    let closure = move || {
        println!("共享数据: {:?}", data_clone);
    };
    
    closure();
    println!("原始数据: {:?}", shared_data);
    
    // 使用 RefCell 实现内部可变性
    let mutable_data = Rc::new(RefCell::new(0));
    let data_clone = Rc::clone(&mutable_data);
    
    let increment = move || {
        *data_clone.borrow_mut() += 1;
    };
    
    increment();
    increment();
    println!("值: {}", mutable_data.borrow());
    
    // 组合使用
    let complex_data = Rc::new(RefCell::new(vec![1, 2, 3]));
    let data_clone = Rc::clone(&complex_data);
    
    let modify = move |x| {
        data_clone.borrow_mut().push(x);
    };
    
    modify(4);
    modify(5);
    println!("修改后: {:?}", complex_data.borrow());
}
```

### 闭包在 API 设计中的权限考虑

```rust
// 灵活的回调设计
struct EventHandler<T> {
    handlers: Vec<Box<dyn Fn(&T)>>,
}

impl<T> EventHandler<T> {
    fn new() -> Self {
        EventHandler { handlers: Vec::new() }
    }
    
    // R: 只需要不可变引用的处理器
    fn add_handler<F>(&mut self, handler: F)
    where
        F: Fn(&T) + 'static,
    {
        self.handlers.push(Box::new(handler));
    }
    
    fn trigger(&self, event: &T) {
        for handler in &self.handlers {
            handler(event);
        }
    }
}

// 构建器模式中的闭包
struct Builder {
    value: i32,
}

impl Builder {
    fn new() -> Self {
        Builder { value: 0 }
    }
    
    // W: 可变借用 self
    fn with<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Self),
    {
        f(&mut self);
        self
    }
    
    fn build(self) -> i32 {
        self.value
    }
}

fn main() {
    // 使用事件处理器
    let mut handler = EventHandler::new();
    
    let prefix = String::from("事件: ");
    handler.add_handler(move |msg: &String| {
        println!("{}{}", prefix, msg);
    });
    
    handler.trigger(&String::from("测试"));
    
    // 使用构建器
    let result = Builder::new()
        .with(|b| b.value = 10)
        .with(|b| b.value *= 2)
        .build();
    
    println!("构建结果: {}", result);
}
```

### 异步闭包和权限

```rust
use std::future::Future;
use std::pin::Pin;

// 异步闭包的类型
type AsyncClosure<T> = Box<
    dyn Fn() -> Pin<Box<dyn Future<Output = T> + Send>> + Send + Sync
>;

// 创建异步闭包
fn make_async_closure(msg: String) -> AsyncClosure<String> {
    Box::new(move || {
        let msg = msg.clone();
        Box::pin(async move {
            // 模拟异步操作
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            format!("异步消息: {}", msg)
        })
    })
}

// 使用异步闭包
async fn use_async_closure() {
    let closure = make_async_closure(String::from("Hello"));
    
    let result = closure().await;
    println!("{}", result);
    
    // 可以多次调用
    let result2 = closure().await;
    println!("{}", result2);
}

// 捕获异步上下文
async fn capture_async_context() {
    let data = vec![1, 2, 3];
    
    let process = || async {
        // 在异步块中使用捕获的数据
        for &item in &data {
            println!("处理: {}", item);
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
    };
    
    process().await;
}
```

## 小结

本章深入学习了 Rust 的闭包系统：

1. **基本语法**：
   - 使用 `|参数| 表达式` 定义闭包
   - 支持类型推断
   - 可以捕获环境变量

2. **常见场景**：
   - 迭代器方法（map、filter、fold）
   - 自定义排序
   - Option/Result 操作
   - 延迟计算和缓存
   - 线程和并发

3. **深度解析**：
   - 三种 trait：Fn、FnMut、FnOnce
   - 捕获方式：不可变引用、可变引用、移动
   - 闭包的大小和实现细节
   - 高级模式：返回闭包、递归闭包

4. **RWO 权限分析**：
   - **R (Fn)**：不可变借用，可多次调用
   - **W (FnMut)**：可变借用，可多次调用但独占
   - **O (FnOnce)**：获取所有权，只能调用一次
   - 使用 Rc/RefCell 进行权限转换
   - 异步闭包的特殊考虑

闭包是 Rust 中强大而灵活的特性，理解其捕获机制和 trait 系统对于编写高效、安全的代码至关重要。下一章我们将学习迭代器。