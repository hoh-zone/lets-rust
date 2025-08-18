// 教学示例 - 允许未使用的代码
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// 第2章：程序与内存管理示例
// 使用命令：cargo run --bin memory_management

use task::examples;

fn main() {
    println!("🦀 Rust 基础教程 - 第2章：程序与内存管理");
    println!("==========================================\n");
    
    // 2.1 程序的基本执行流程
    println!("📍 2.1 程序的基本执行流程");
    println!("-------------------------");
    program_execution_flow();
    println!();
    
    // 2.2 栈与堆
    println!("📍 2.2 栈与堆");
    println!("-------------");
    stack_and_heap();
    println!();
    
    // 2.3 指针类型
    println!("📍 2.3 指针类型");
    println!("---------------");
    pointer_types();
    println!();
    
    // 2.4 函数调用
    println!("📍 2.4 函数调用");
    println!("---------------");
    function_calls();
    
    println!("\n✅ 第2章示例运行完成！");
}

// 2.1 程序的基本执行流程
fn program_execution_flow() {
    // 全局变量/静态变量 - 存储在数据段
    static GLOBAL_COUNTER: i32 = 0;
    const MAX_SIZE: usize = 100;
    
    println!("全局常量 MAX_SIZE: {}", MAX_SIZE);
    println!("静态变量 GLOBAL_COUNTER: {}", GLOBAL_COUNTER);
    
    // 局部变量 - 存储在栈上
    let x = 42;
    let y = "hello";
    
    // 动态分配 - 数据存储在堆上
    let v = vec![1, 2, 3, 4, 5];
    let s = String::from("world");
    
    println!("栈上变量 x: {}, y: {}", x, y);
    println!("堆上数据 - vector: {:?}", v);
    println!("堆上数据 - string: {}", s);
    
    // 程序执行流程示例
    println!("\n程序执行流程:");
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

// 2.2 栈与堆
fn stack_and_heap() {
    println!("栈上的数据:");
    // 这些数据都存储在栈上
    let a = 5;          // i32 - 4 字节
    let b = true;       // bool - 1 字节
    let c = 'A';        // char - 4 字节（Unicode）
    let d = (1, 2.0);   // 元组 - 12 字节（4 + 8）
    
    // 固定大小的数组也在栈上
    let arr = [1, 2, 3, 4, 5];  // [i32; 5] - 20 字节
    
    println!("  a: {}, b: {}, c: {}", a, b, c);
    println!("  元组 d: {:?}", d);
    println!("  数组 arr: {:?}", arr);
    
    println!("\n堆上的数据:");
    // String 类型的数据存储在堆上
    let s1 = String::from("hello");
    
    // Vec 的数据也在堆上
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    
    // Box 显式地将数据放在堆上
    let b = Box::new(5);
    
    println!("  String s1: {}", s1);
    println!("  Vector v: {:?}", v);
    println!("  Box b: {}", b);
    
    // 栈和堆的对比示例
    println!("\n栈和堆的对比:");
    // 栈上的数据
    let x = 5;                    // 栈：5
    let y = x;                    // 栈：复制值，y = 5
    println!("  栈上复制 - x: {}, y: {}", x, y);
    
    // 堆上的数据
    let s1 = String::from("hello"); // 栈：指针、长度、容量；堆："hello"
    let s2 = s1;                    // 移动：s1 不再有效
    println!("  堆上移动 - s2: {}", s2);
    // println!("s1: {}", s1); // 这会报错
    
    // 使用 clone 进行深拷贝
    let s3 = String::from("world");
    let s4 = s3.clone();            // 堆上复制了数据
    println!("  深拷贝 - s3: {}, s4: {}", s3, s4);
    
    // 内存布局示例
    println!("\n内存布局示例:");
    let s = String::from("Rust");
    println!("  字符串 '{}' 的内存信息:", s);
    println!("  - 长度: {} 字节", s.len());
    println!("  - 容量: {} 字节", s.capacity());
    println!("  - String 本身在栈上占用 24 字节（指针8 + 长度8 + 容量8）");
}

// 2.3 指针类型
fn pointer_types() {
    println!("引用（References）:");
    let x = 5;
    let r = &x;        // r 是 x 的不可变引用
    
    println!("  x = {}", x);
    println!("  r = {}", r);      // 自动解引用
    println!("  *r = {}", *r);    // 显式解引用
    
    // 可变引用
    let mut y = 10;
    let m = &mut y;    // m 是 y 的可变引用
    *m += 5;
    println!("  修改后 y = {}", y);      // 输出 15
    
    println!("\n引用的规则演示:");
    let mut s = String::from("hello");
    
    // 规则1：可以有多个不可变引用
    let r1 = &s;
    let r2 = &s;
    println!("  多个不可变引用: {} and {}", r1, r2);
    
    // 规则2：只能有一个可变引用
    let r3 = &mut s;
    r3.push_str(" world");
    println!("  可变引用修改后: {}", r3);
    
    println!("\n智能指针预览:");
    // Box<T> - 独占所有权的堆分配
    let b = Box::new(5);
    println!("  Box<T>: {}", b);
    
    // Rc<T> - 引用计数的共享所有权
    use std::rc::Rc;
    let rc1 = Rc::new(String::from("hello"));
    let rc2 = Rc::clone(&rc1);
    let rc3 = Rc::clone(&rc1);
    
    println!("  Rc<T> 引用计数: {}", Rc::strong_count(&rc1));  // 输出 3
}

// 2.4 函数调用
fn function_calls() {
    println!("调用栈示例:");
    let a = 10;
    let result = foo(a);
    println!("  最终结果: {}", result);
    
    println!("\n参数传递:");
    // 值传递（Copy 类型）
    let x = 5;
    takes_ownership_copy(x);
    println!("  x 仍然可用: {}", x);
    
    // 值传递（移动语义）
    let s = String::from("hello");
    takes_ownership_move(s);
    // println!("{}", s);  // 错误！s 已经被移动
    
    // 引用传递
    let s2 = String::from("world");
    takes_reference(&s2);
    println!("  s2 仍然可用: {}", s2);
    
    // 可变引用传递
    let mut s3 = String::from("rust");
    takes_mut_reference(&mut s3);
    println!("  s3 修改后: {}", s3);
    
    println!("\n返回值:");
    let s1 = gives_ownership();
    println!("  收到: {}", s1);
    
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("  返回: {}", s3);
    
    // 返回多个值
    let s4 = String::from("world");
    let (s5, len) = calculate_length(s4);
    println!("  字符串 '{}' 的长度是 {}", s5, len);
    
    println!("\n递归示例:");
    let result = factorial(5);
    println!("  5! = {}", result);
    
    // 迭代版本避免栈溢出
    let n = 100;
    let result = sum_iterative(n);
    println!("  1 到 {} 的和: {}", n, result);
}

fn foo(x: i32) -> i32 {
    let b = x + 5;
    bar(b)
}

fn bar(y: i32) -> i32 {
    let c = y * 2;
    c
}

fn takes_ownership_copy(x: i32) {
    println!("    收到 Copy 值: {}", x);
}

fn takes_ownership_move(s: String) {
    println!("    收到并拥有: {}", s);
}  // s 在这里被释放

fn takes_reference(s: &String) {
    println!("    借用字符串: {}", s);
}

fn takes_mut_reference(s: &mut String) {
    s.push_str(" is awesome!");
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

fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn sum_iterative(n: u64) -> u64 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
} 