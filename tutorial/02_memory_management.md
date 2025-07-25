# 第2章：程序与内存管理

## 2.1 程序的基本执行流程

### 程序的组成

一个 Rust 程序在运行时，内存主要分为以下几个区域：

1. **代码段（Text Segment）**：存储程序的机器码
2. **数据段（Data Segment）**：存储全局变量和静态变量
3. **栈（Stack）**：存储函数调用信息和局部变量
4. **堆（Heap）**：动态分配的内存区域

```rust
// 全局变量/静态变量 - 存储在数据段
static GLOBAL_COUNTER: i32 = 0;
const MAX_SIZE: usize = 100;

fn main() {
    // 局部变量 - 存储在栈上
    let x = 42;
    let y = "hello";
    
    // 动态分配 - 数据存储在堆上
    let v = vec![1, 2, 3, 4, 5];
    let s = String::from("world");
}
```

### 程序执行流程

1. **程序启动**：操作系统加载程序，分配内存空间
2. **初始化**：设置栈指针，初始化全局变量
3. **执行 main 函数**：程序从 main 函数开始执行
4. **函数调用**：通过栈来管理函数调用和返回
5. **程序结束**：清理资源，返回操作系统

```rust
fn main() {
    println!("程序开始");
    
    let result = calculate(10, 20);
    println!("计算结果: {}", result);
    
    println!("程序结束");
}

fn calculate(a: i32, b: i32) -> i32 {
    let sum = add(a, b);
    let product = multiply(a, b);
    sum + product
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}
```

## 2.2 栈与堆

### 栈（Stack）

栈是一种后进先出（LIFO）的数据结构，具有以下特点：

- **快速**：分配和释放都很快，只需移动栈指针
- **有序**：数据按照固定顺序存储
- **大小限制**：栈空间有限（通常几 MB）
- **自动管理**：函数结束时自动清理

```rust
fn main() {
    // 这些数据都存储在栈上
    let a = 5;          // i32 - 4 字节
    let b = true;       // bool - 1 字节
    let c = 'A';        // char - 4 字节（Unicode）
    let d = (1, 2.0);   // 元组 - 12 字节（4 + 8）
    
    // 固定大小的数组也在栈上
    let arr = [1, 2, 3, 4, 5];  // [i32; 5] - 20 字节
}
```

### 堆（Heap）

堆是用于动态内存分配的区域，特点如下：

- **灵活**：可以存储大小可变的数据
- **较慢**：分配需要搜索可用空间
- **无序**：数据可能分散存储
- **手动管理**：需要明确的分配和释放（Rust 通过所有权系统自动管理）

```rust
fn main() {
    // String 类型的数据存储在堆上
    let s1 = String::from("hello");
    
    // Vec 的数据也在堆上
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    
    // Box 显式地将数据放在堆上
    let b = Box::new(5);
}
```

### 栈和堆的对比示例

```rust
fn main() {
    // 栈上的数据
    let x = 5;                    // 栈：5
    let y = x;                    // 栈：复制值，y = 5
    
    // 堆上的数据
    let s1 = String::from("hello"); // 栈：指针、长度、容量
                                    // 堆："hello"
    let s2 = s1;                    // 移动：s1 不再有效
    
    // 使用 clone 进行深拷贝
    let s3 = String::from("world");
    let s4 = s3.clone();            // 堆上复制了数据
    
    println!("s4 = {}", s4);        // s3 和 s4 都有效
    println!("s3 = {}", s3);
}
```

### 内存布局示例

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // 栈上的结构体
    let p1 = Point { x: 10, y: 20 };
    
    // 堆上的结构体
    let p2 = Box::new(Point { x: 30, y: 40 });
    
    // String 的内存布局
    let s = String::from("Rust");
    // 栈上：ptr(8字节) + len(8字节) + capacity(8字节) = 24字节
    // 堆上："Rust"(4字节)
    
    println!("字符串长度: {}", s.len());
    println!("字符串容量: {}", s.capacity());
}
```

## 2.3 指针类型

### 引用（References）

引用是 Rust 中最常见的指针类型，使用 `&` 符号：

```rust
fn main() {
    let x = 5;
    let r = &x;        // r 是 x 的不可变引用
    
    println!("x = {}", x);
    println!("r = {}", r);      // 自动解引用
    println!("*r = {}", *r);    // 显式解引用
    
    // 可变引用
    let mut y = 10;
    let m = &mut y;    // m 是 y 的可变引用
    *m += 5;
    println!("y = {}", y);      // 输出 15
}
```

### 引用的规则

```rust
fn main() {
    let mut s = String::from("hello");
    
    // 规则1：可以有多个不可变引用
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    
    // 规则2：只能有一个可变引用
    let r3 = &mut s;
    // let r4 = &mut s;  // 错误，不能同时存在多个可变引用
    r3.push_str(" world");
    println!("{}", r3);
    
    // 规则3：不可变引用和可变引用不能同时存在
    let r4 = &s; 
    println!("{}", r4);
    //错误，此时再次引用r3，会导致在r3还没有失效时存在r4。违背了规则3；
    // println!("{}", r3);  
效
```

### 原始指针（Raw Pointers）

原始指针是不安全的，需要在 `unsafe` 块中使用：

```rust
fn main() {
    let x = 5;
    let raw_ptr = &x as *const i32;  // 不可变原始指针
    
    let mut y = 10;
    let raw_mut_ptr = &mut y as *mut i32;  // 可变原始指针
    
    unsafe {
        println!("*raw_ptr = {}", *raw_ptr);
        *raw_mut_ptr = 20;
        println!("y = {}", y);  // 输出 20
    }
}
```

### 智能指针预览

智能指针是拥有额外元数据和功能的指针：

```rust
use std::rc::Rc;

fn main() {
    // Box<T> - 独占所有权的堆分配
    let b = Box::new(5);
    println!("b = {}", b);
    
    // Rc<T> - 引用计数的共享所有权
    let rc1 = Rc::new(String::from("hello"));
    let rc2 = Rc::clone(&rc1);
    let rc3 = Rc::clone(&rc1);
    
    println!("引用计数: {}", Rc::strong_count(&rc1));  // 输出 3
}
```

## 2.4 函数调用

### 调用栈（Call Stack）

每次函数调用都会在栈上创建一个新的栈帧（Stack Frame）：

```rust
fn main() {
    let a = 10;
    let result = foo(a);
    println!("结果: {}", result);
}

fn foo(x: i32) -> i32 {
    let b = x + 5;
    bar(b)
}

fn bar(y: i32) -> i32 {
    let c = y * 2;
    c
}

// 调用栈：
// 1. main() - 包含变量 a, result
// 2. foo()  - 包含参数 x, 变量 b
// 3. bar()  - 包含参数 y, 变量 c
```

### 参数传递

Rust 中的参数传递遵循所有权规则：

```rust
fn main() {
    // 值传递（Copy 类型）
    let x = 5;
    takes_ownership_copy(x);
    println!("x 仍然可用: {}", x);
    
    // 值传递（移动语义）
    let s = String::from("hello");
    takes_ownership_move(s);
    // println!("{}", s);  // 错误！s 已经被移动
    
    // 引用传递
    let s2 = String::from("world");
    takes_reference(&s2);
    println!("s2 仍然可用: {}", s2);
    
    // 可变引用传递
    let mut s3 = String::from("rust");
    takes_mut_reference(&mut s3);
    println!("s3 修改后: {}", s3);
}

fn takes_ownership_copy(x: i32) {
    println!("收到 Copy 值: {}", x);
}

fn takes_ownership_move(s: String) {
    println!("收到并拥有: {}", s);
}  // s 在这里被释放

fn takes_reference(s: &String) {
    println!("借用字符串: {}", s);
}

fn takes_mut_reference(s: &mut String) {
    s.push_str(" is awesome!");
}
```

### 返回值

函数可以返回值的所有权：

```rust
fn main() {
    let s1 = gives_ownership();
    println!("收到: {}", s1);
    
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("返回: {}", s3);
    
    // 返回多个值
    let s4 = String::from("world");
    let (s5, len) = calculate_length(s4);
    println!("字符串 '{}' 的长度是 {}", s5, len);
}

fn gives_ownership() -> String {
    let s = String::from("yours");
    s  // 返回所有权
}

fn takes_and_gives_back(s: String) -> String {
    s  // 获取所有权并返回
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)  // 返回元组
}
```

### 栈帧和生命周期

```rust
fn main() {
    let x = 5;                      // x 进入作用域
    
    {                               // 新的作用域
        let y = 10;                 // y 进入作用域
        println!("x = {}, y = {}", x, y);
    }                               // y 离开作用域，被释放
    
    // println!("y = {}", y);       // 错误！y 不在作用域内
    
    let result = {
        let temp = x * 2;
        temp + 3                    // 表达式的值被返回
    };                              // temp 被释放
    
    println!("result = {}", result);
}                                   // x 和 result 被释放
```

### 递归和栈溢出

```rust
fn main() {
    // 正确的递归
    let result = factorial(5);
    println!("5! = {}", result);
    
    // 栈溢出的例子（不要运行！）
    // infinite_recursion(0);
}

fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// 这会导致栈溢出
fn infinite_recursion(x: i32) {
    println!("x = {}", x);
    infinite_recursion(x + 1);  // 没有基准情况，无限递归
}
```

### 尾递归优化

Rust 目前不保证尾递归优化，但可以使用迭代来避免栈溢出：

```rust
// 递归版本（可能栈溢出）
fn sum_recursive(n: u64) -> u64 {
    if n == 0 {
        0
    } else {
        n + sum_recursive(n - 1)
    }
}

// 迭代版本（不会栈溢出）
fn sum_iterative(n: u64) -> u64 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}

// 尾递归风格（Rust 不保证优化）
fn sum_tail_recursive(n: u64, acc: u64) -> u64 {
    if n == 0 {
        acc
    } else {
        sum_tail_recursive(n - 1, acc + n)
    }
}

fn main() {
    let n = 100_000;
    // let result = sum_recursive(n);  // 可能栈溢出
    let result = sum_iterative(n);     // 安全
    println!("1 到 {} 的和: {}", n, result);
}
```

## 小结

本章深入探讨了 Rust 程序的内存管理机制：

1. **程序执行流程**：了解程序在内存中的布局和执行过程
2. **栈与堆**：理解两种内存区域的特点和使用场景
3. **指针类型**：掌握引用、原始指针等不同的指针类型
4. **函数调用**：理解调用栈、参数传递和返回值的机制

这些知识是理解 Rust 所有权系统的基础，下一章我们将深入学习 Rust 独特的所有权机制。 