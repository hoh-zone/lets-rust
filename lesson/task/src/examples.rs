// Rust 基础教程 - 示例代码集合
// 包含所有章节的可运行示例

// 允许未使用的代码，因为这是教学示例
#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

// ============================================================================
// 第1-4章：基础教程示例
// ============================================================================

/// 第1章：变量与常量示例
pub fn variables_and_constants() {
    println!("📝 变量与常量示例");
    
    // 不可变变量
    let x = 5;
    println!("不可变变量 x = {}", x);
    
    // 可变变量
    let mut y = 10;
    println!("可变变量 y = {}", y);
    y = 15;
    println!("修改后 y = {}", y);
    
    // 常量
    const MAX_POINTS: u32 = 100_000;
    println!("常量 MAX_POINTS = {}", MAX_POINTS);
    
    // 变量遮蔽
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("遮蔽后 z = {}", z);
    
    // 类型转换
    let guess: u32 = "42".parse().expect("不是数字！");
    println!("解析的数字 = {}", guess);
}

/// 第2章：数据类型示例
pub fn data_types() {
    println!("🔢 数据类型示例");
    
    // 整数类型
    let a: i32 = 42;
    let b: u64 = 1_000_000;
    println!("整数: i32 = {}, u64 = {}", a, b);
    
    // 浮点数
    let c: f64 = 3.14159;
    let d: f32 = 2.718;
    println!("浮点数: f64 = {}, f32 = {}", c, d);
    
    // 布尔值
    let is_rust_awesome = true;
    let is_learning = false;
    println!("布尔值: {} 和 {}", is_rust_awesome, is_learning);
    
    // 字符
    let heart_eyed_cat = '😻';
    let letter = 'A';
    println!("字符: {} 和 {}", heart_eyed_cat, letter);
    
    // 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("元组解构: x={}, y={}, z={}", x, y, z);
    
    // 数组
    let arr = [1, 2, 3, 4, 5];
    println!("数组第一个元素: {}", arr[0]);
    println!("数组长度: {}", arr.len());
}

/// 第3章：函数示例
pub fn functions_demo() {
    println!("⚙️ 函数示例");
    
    // 基本函数调用
    let result = add_numbers(5, 3);
    println!("5 + 3 = {}", result);
    
    // 有返回值的函数
    let product = multiply(4, 7);
    println!("4 × 7 = {}", product);
    
    // 表达式与语句
    let y = {
        let x = 3;
        x + 1  // 表达式，没有分号
    };
    println!("代码块的值: {}", y);
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

/// 第4章：控制流示例
pub fn control_flow() {
    println!("🔄 控制流示例");
    
    // if 表达式
    let number = 6;
    if number % 4 == 0 {
        println!("数字能被 4 整除");
    } else if number % 3 == 0 {
        println!("数字能被 3 整除");
    } else {
        println!("数字不能被 4 或 3 整除");
    }
    
    // if 作为表达式
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("条件表达式的值: {}", number);
    
    // loop 循环
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop 循环结果: {}", result);
    
    // while 循环
    let mut number = 3;
    while number != 0 {
        println!("倒计时: {}!", number);
        number -= 1;
    }
    println!("发射！🚀");
    
    // for 循环
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("数组元素: {}", element);
    }
    
    // 范围循环
    for number in (1..4).rev() {
        println!("倒序: {}!", number);
    }
}

// ============================================================================
// 第5-15章：进阶教程示例
// ============================================================================

/// 第5章：内存管理示例
pub fn memory_management() {
    println!("🧠 内存管理示例");
    
    // 栈分配
    println!("\n📚 栈内存示例：");
    let stack_var = 42;
    let stack_array = [1, 2, 3, 4, 5];
    println!("栈变量: {}", stack_var);
    println!("栈数组: {:?}", stack_array);
    
    // 堆分配
    println!("\n🏗️ 堆内存示例：");
    let heap_string = String::from("Hello, Heap!");
    let heap_vector = vec![1, 2, 3, 4, 5];
    println!("堆字符串: {}", heap_string);
    println!("堆向量: {:?}", heap_vector);
    
    // 指针类型
    println!("\n👉 指针类型示例：");
    let x = 5;
    let raw_ptr = &x as *const i32;
    println!("原始指针地址: {:p}", raw_ptr);
    
    // 函数调用栈
    println!("\n📞 函数调用栈：");
    demonstrate_call_stack();
}

fn demonstrate_call_stack() {
    println!("  → 进入 demonstrate_call_stack");
    call_level_1();
    println!("  ← 离开 demonstrate_call_stack");
}

fn call_level_1() {
    println!("    → 进入 call_level_1");
    call_level_2();
    println!("    ← 离开 call_level_1");
}

fn call_level_2() {
    println!("      → 进入 call_level_2");
    println!("      ✨ 在最深层函数中");
    println!("      ← 离开 call_level_2");
}

/// 第6章：所有权示例
pub fn ownership() {
    println!("🏠 所有权示例");
    
    // 基本所有权
    println!("\n📦 基本所有权：");
    let s1 = String::from("hello");
    let s2 = s1; // s1 被移动到 s2
    // println!("{}", s1); // 这会报错
    println!("s2: {}", s2);
    
    // Clone 复制
    println!("\n🔄 Clone 复制：");
    let s3 = String::from("world");
    let s4 = s3.clone(); // 深拷贝
    println!("s3: {}, s4: {}", s3, s4);
    
    // Copy trait
    println!("\n📋 Copy trait：");
    let x = 5;
    let y = x; // Copy，不是移动
    println!("x: {}, y: {}", x, y);
    
    // 函数所有权
    println!("\n⚙️ 函数所有权：");
    let s = String::from("function");
    takes_ownership(s);
    // println!("{}", s); // 这会报错，s 已被移动
    
    let x = 5;
    makes_copy(x);
    println!("x 仍然可用: {}", x);
}

fn takes_ownership(some_string: String) {
    println!("函数获得所有权: {}", some_string);
} // some_string 在这里被丢弃

fn makes_copy(some_integer: i32) {
    println!("函数获得副本: {}", some_integer);
} // some_integer 离开作用域，但没有特殊处理

/// 第7章：借用机制示例
pub fn borrowing() {
    println!("🔗 借用机制示例");
    
    // 不可变引用
    println!("\n👀 不可变引用：");
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("字符串 '{}' 的长度是 {}", s1, len);
    
    // 可变引用
    println!("\n✏️ 可变引用：");
    let mut s = String::from("hello");
    change(&mut s);
    println!("修改后的字符串: {}", s);
    
    // 字符串切片
    println!("\n🔪 字符串切片：");
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("切片: '{}' 和 '{}'", hello, world);
    
    // 数组切片
    println!("\n📏 数组切片：");
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..4];
    println!("数组切片: {:?}", slice);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s 离开作用域，但因为它不拥有值，所以什么也不会发生

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/// 第8章：结构体示例
pub fn structs() {
    println!("🏗️ 结构体示例");
    
    // 基本结构体
    println!("\n🏢 基本结构体：");
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("用户: {} ({})", user1.username, user1.email);
    
    // 结构体更新语法
    println!("\n🔄 结构体更新语法：");
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("新用户: {} ({})", user2.username, user2.email);
    
    // 元组结构体
    println!("\n📦 元组结构体：");
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("颜色: ({}, {}, {})", black.0, black.1, black.2);
    println!("点: ({}, {}, {})", origin.0, origin.1, origin.2);
    
    // 方法
    println!("\n⚙️ 方法示例：");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("矩形面积: {}", rect1.area());
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    println!("rect1 能容纳 rect2 吗？{}", rect1.can_hold(&rect2));
    
    // 关联函数
    println!("\n🔧 关联函数：");
    let sq = Rectangle::square(3);
    println!("正方形面积: {}", sq.area());
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

/// 第9章：常用类型示例
pub fn common_types() {
    println!("📦 常用类型示例");
    
    // Vector 示例
    println!("\n📋 Vector 示例：");
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("Vector: {:?}", v);
    
    let v2 = vec![1, 2, 3];
    println!("使用宏创建的 Vector: {:?}", v2);
    
    // String 示例
    println!("\n📝 String 示例：");
    let mut s = String::new();
    s.push_str("hello");
    s.push(' ');
    s.push_str("world");
    println!("构建的字符串: {}", s);
    
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 被移动了，不能再使用
    println!("连接的字符串: {}", s3);
    
    // HashMap 示例
    println!("\n🗺️ HashMap 示例：");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    for (key, value) in &scores {
        println!("队伍 {} 得分 {}", key, value);
    }
    
    // 查找值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(s) => println!("Blue 队得分: {}", s),
        None => println!("Blue 队不存在"),
    }
}

/// 第10章：枚举示例
pub fn enums() {
    println!("🎯 枚举示例");
    
    // 基本枚举
    println!("\n🌐 IP 地址枚举：");
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("IPv4: {:?}", home);
    println!("IPv6: {:?}", loopback);
    
    // Option 枚举
    println!("\n❓ Option 枚举：");
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    println!("Some number: {:?}", some_number);
    println!("Some string: {:?}", some_string);
    println!("Absent number: {:?}", absent_number);
    
    // match 模式匹配
    println!("\n🎯 match 模式匹配：");
    let coin = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(coin);
    println!("硬币价值: {} 美分", value);
    
    // if let 语法
    println!("\n🔍 if let 语法：");
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("最大值配置为 {}", max);
    }
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... 其他州
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
            println!("来自 {:?} 的25美分硬币！", state);
            25
        }
    }
}

/// 第11章：泛型与特征示例
pub fn generics_traits() {
    println!("🔧 泛型与特征示例");
    
    // 泛型函数
    println!("\n🔄 泛型函数：");
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("最大的数字是 {}", result);
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("最大的字符是 {}", result);
    
    // 泛型结构体
    println!("\n📦 泛型结构体：");
    let integer = Point2D { x: 5, y: 10 };
    let float = Point2D { x: 1.0, y: 4.0 };
    println!("整数点: ({}, {})", integer.x, integer.y);
    println!("浮点数点: ({}, {})", float.x, float.y);
    
    // 特征
    println!("\n🎭 特征示例：");
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("当然，就像你可能知道的那样，人们"),
        reply: false,
        retweet: false,
    };
    println!("1 条新推文：{}", tweet.summarize());
    
    let article = NewsArticle {
        headline: String::from("企鹅队再次赢得冠军！"),
        location: String::from("宾夕法尼亚州匹兹堡"),
        author: String::from("Iceburgh"),
        content: String::from("企鹅队再次获得了冠军。"),
    };
    println!("新文章可用！{}", article.summarize());
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[derive(Debug)]
struct Point2D<T> {
    x: T,
    y: T,
}

trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

/// 第12章：生命周期示例
pub fn lifetimes() {
    println!("⏰ 生命周期示例");
    
    // 基本生命周期
    println!("\n🔗 基本生命周期：");
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("最长的字符串是 {}", result);
    
    // 结构体中的生命周期
    println!("\n📚 结构体中的生命周期：");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("重要摘录: {}", i.part);
    
    // 生命周期省略
    println!("\n✂️ 生命周期省略：");
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("第一个单词: {}", word);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

/// 第13章：特征对象示例
pub fn trait_objects() {
    println!("🎭 特征对象示例");
    
    // 基本特征对象
    println!("\n🎨 绘制示例：");
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    
    screen.run();
    
    // 动态分发示例
    println!("\n🔄 动态分发示例：");
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle2D { width: 10.0, height: 5.0 }),
    ];
    
    for shape in shapes {
        println!("面积: {:.2}", shape.area());
    }
}

trait Draw {
    fn draw(&self);
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("绘制按钮: '{}' ({}x{})", self.label, self.width, self.height);
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("绘制选择框 ({}x{}) 选项: {:?}", self.width, self.height, self.options);
    }
}

trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Rectangle2D {
    width: f64,
    height: f64,
}

impl Shape for Rectangle2D {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

/// 第14章：常用特征示例
pub fn common_traits() {
    println!("🛠️ 常用特征示例");
    
    // Display 和 Debug
    println!("\n🖨️ Display 和 Debug：");
    let p = PersonDisplay { name: String::from("Alice"), age: 30 };
    println!("Display: {}", p);
    println!("Debug: {:?}", p);
    
    // Clone 和 Copy
    println!("\n📋 Clone 和 Copy：");
    let original = CloneableStruct { data: String::from("original") };
    let cloned = original.clone();
    println!("原始: {}, 克隆: {}", original.data, cloned.data);
    
    // 相等性比较
    println!("\n⚖️ 相等性比较：");
    let p1 = PersonEq { name: String::from("Bob"), age: 25 };
    let p2 = PersonEq { name: String::from("Bob"), age: 25 };
    let p3 = PersonEq { name: String::from("Charlie"), age: 30 };
    
    println!("p1 == p2: {}", p1 == p2);
    println!("p1 == p3: {}", p1 == p3);
    
    // 排序
    println!("\n📊 排序示例：");
    let mut people = vec![
        PersonOrd { name: String::from("Alice"), age: 30 },
        PersonOrd { name: String::from("Bob"), age: 25 },
        PersonOrd { name: String::from("Charlie"), age: 35 },
    ];
    
    people.sort();
    println!("按年龄排序:");
    for person in people {
        println!("  {} ({}岁)", person.name, person.age);
    }
}

#[derive(Debug)]
struct PersonDisplay {
    name: String,
    age: u32,
}

impl std::fmt::Display for PersonDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} ({}岁)", self.name, self.age)
    }
}

#[derive(Clone)]
struct CloneableStruct {
    data: String,
}

#[derive(PartialEq, Eq)]
struct PersonEq {
    name: String,
    age: u32,
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct PersonOrd {
    age: u32,  // 首先按年龄排序
    name: String,  // 然后按姓名排序
}

/// 第15章：错误处理示例
pub fn error_handling() {
    println!("⚠️ 错误处理示例");
    
    // panic! 示例
    println!("\n💥 Panic 示例：");
    println!("注意：这里我们不会真的 panic，只是演示概念");
    // panic!("这是一个 panic!");  // 取消注释会导致程序崩溃
    
    // Result 类型
    println!("\n✅ Result 类型示例：");
    match divide(10.0, 2.0) {
        Ok(result) => println!("10.0 / 2.0 = {}", result),
        Err(e) => println!("错误: {}", e),
    }
    
    match divide(10.0, 0.0) {
        Ok(result) => println!("10.0 / 0.0 = {}", result),
        Err(e) => println!("错误: {}", e),
    }
    
    // 错误传播
    println!("\n🔄 错误传播示例：");
    match read_username_from_file() {
        Ok(username) => println!("用户名: {}", username),
        Err(e) => println!("读取失败: {}", e),
    }
    
    // unwrap 和 expect
    println!("\n🎁 unwrap 和 expect：");
    let good_result: Result<i32, &str> = Ok(42);
    let value = good_result.unwrap();
    println!("unwrap 的值: {}", value);
    
    let another_good_result: Result<i32, &str> = Ok(100);
    let value2 = another_good_result.expect("应该是一个好的结果");
    println!("expect 的值: {}", value2);
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("除数不能为零"))
    } else {
        Ok(a / b)
    }
}

fn read_username_from_file() -> Result<String, String> {
    // 模拟文件读取
    // 在真实场景中，这里会使用 std::fs::read_to_string
    Err(String::from("文件不存在"))
}

/// 第13章：项目管理示例
pub fn project_management() {
    println!("🏗️ 第13章：项目管理");
    println!("=====================================");
    println!("💡 运行完整示例：cargo run --bin project_management");
    
    // 简化的模块示例
    println!("\n📦 模块系统演示：");
    println!("  🔸 模块定义和可见性控制");
    println!("  🔸 use 语句和路径");
    println!("  🔸 包和 crate 的概念");
    println!("  🔸 工作空间管理");
    
    // 简单的模块使用示例
    mod simple_module {
        pub fn public_function() {
            println!("    这是一个公开函数");
        }
        
        fn _private_function() {
            println!("    这是一个私有函数");
        }
        
        pub mod nested {
            pub fn nested_function() {
                println!("    这是嵌套模块中的函数");
            }
        }
    }
    
    println!("\n  🔸 调用模块函数：");
    simple_module::public_function();
    simple_module::nested::nested_function();
    
    println!("\n📋 项目管理要点：");
    println!("  • 模块系统帮助组织代码");
    println!("  • pub 关键字控制可见性");
    println!("  • use 语句简化路径");
    println!("  • Cargo.toml 管理依赖");
}

/// 第14章：文档与测试示例
pub fn docs_and_testing() {
    println!("📚 第14章：文档与测试");
    println!("=====================================");
    println!("💡 运行完整示例：cargo run --bin docs_and_testing");
    
    println!("\n📖 文档功能：");
    println!("  🔸 文档注释 (///)");
    println!("  🔸 文档测试");
    println!("  🔸 模块级文档 (//!)");
    println!("  🔸 cargo doc 生成文档");
    
    println!("\n🧪 测试功能：");
    println!("  🔸 单元测试 (#[test])");
    println!("  🔸 集成测试");
    println!("  🔸 断言宏 (assert!, assert_eq!)");
    println!("  🔸 cargo test 运行测试");
    
    // 简单的测试示例（在实际项目中应该在 #[cfg(test)] 模块中）
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    // 模拟测试
    assert_eq!(add(2, 3), 5);
    println!("\n✅ 简单测试通过：add(2, 3) = 5");
    
    println!("\n📋 文档与测试要点：");
    println!("  • 文档注释自动生成API文档");
    println!("  • 文档测试确保示例代码正确");
    println!("  • 单元测试验证函数逻辑");
    println!("  • 集成测试验证模块交互");
}

/// 第15章：闭包示例
pub fn closures() {
    println!("🔒 闭包示例");
    println!("提示：运行 'cargo run --bin closures' 查看完整示例");
    
    println!("\n🎯 闭包基础：");
    let add_one = |x| x + 1;
    println!("闭包计算：{} + 1 = {}", 5, add_one(5));
    
    println!("\n📊 迭代器中的闭包：");
    let numbers = vec![1, 2, 3, 4, 5];
    let squares: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    println!("平方：{:?}", squares);
    
    let evens: Vec<&i32> = numbers.iter().filter(|&x| x % 2 == 0).collect();
    println!("偶数：{:?}", evens);
    
    println!("\n🎭 闭包特征：");
    println!("- Fn：不可变借用");
    println!("- FnMut：可变借用");
    println!("- FnOnce：获取所有权");
}

/// 第16章：迭代器示例
pub fn iterators() {
    println!("🔄 迭代器示例");
    println!("提示：运行 'cargo run --bin iterators' 查看完整示例");
    
    println!("\n📋 迭代器类型：");
    let vec = vec![1, 2, 3, 4, 5];
    
    // iter() - 不可变引用
    println!("iter() - 不可变引用：");
    for item in vec.iter() {
        println!("  {}", item);
    }
    
    // into_iter() - 获取所有权
    let vec2 = vec![1, 2, 3];
    println!("into_iter() - 获取所有权：");
    for item in vec2.into_iter() {
        println!("  {}", item);
    }
    
    println!("\n🔧 迭代器适配器：");
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("翻倍：{:?}", doubled);
    
    let sum: i32 = numbers.iter().sum();
    println!("求和：{}", sum);
}

/// 第17章：智能指针示例
pub fn smart_pointers() {
    println!("📦 第17章：智能指针");
    println!("=====================================");
    println!("💡 运行完整示例：cargo run --bin smart_pointers");
    
    println!("\n📦 Box<T> 示例：");
    let b = Box::new(5);
    println!("  Box 中的值：{}", b);
    
    // 递归类型示例
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    
    use List::{Cons, Nil};
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("  递归链表：{:?}", list);
    
    println!("\n🎭 智能指针特性：");
    println!("  🔸 Deref trait 允许智能指针表现得像引用");
    println!("  🔸 Drop trait 允许自定义清理代码");
    println!("  🔸 自动解引用强制转换");
    
    println!("\n📋 智能指针类型：");
    println!("  • Box<T> - 堆分配");
    println!("  • Rc<T> - 引用计数");
    println!("  • RefCell<T> - 内部可变性");
    println!("  • Arc<T> - 原子引用计数");
    println!("  • Mutex<T> - 互斥锁");
}

/// 第18章：常用智能指针示例
pub fn common_smart_pointers() {
    println!("🐄 第18章：常用智能指针");
    println!("=====================================");
    println!("💡 运行完整示例：cargo run --bin common_smart_pointers");
    
    println!("\n🐄 Cow (Clone on Write)：");
    use std::borrow::Cow;
    
    let borrowed: Cow<str> = "hello".into();
    let owned: Cow<str> = String::from("world").into();
    
    println!("  借用的字符串: {:?}", borrowed);
    println!("  拥有的字符串: {:?}", owned);
    
    println!("\n🔗 Weak 弱引用：");
    println!("  🔸 避免循环引用");
    println!("  🔸 缓存场景应用");
    println!("  🔸 观察者模式");
    
    println!("\n📌 Pin 固定指针：");
    println!("  🔸 防止值移动");
    println!("  🔸 自引用结构体");
    println!("  🔸 异步编程中的应用");
    
    println!("\n📋 应用场景：");
    println!("  • Cow - 延迟克隆优化");
    println!("  • Weak - 打破循环引用");
    println!("  • Pin - 异步和自引用");
}

/// 第19章：并发编程示例
pub fn concurrency() {
    println!("🧵 第19章：并发编程");
    println!("=====================================");
    println!("💡 运行完整示例：cargo run --bin concurrency");
    
    println!("\n🧵 线程基础：");
    use std::thread;
    use std::time::Duration;
    
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("  子线程: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    for i in 1..=2 {
        println!("  主线程: {}", i);
        thread::sleep(Duration::from_millis(150));
    }
    
    handle.join().unwrap();
    
    println!("\n📨 消息传递：");
    use std::sync::mpsc;
    
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        tx.send("Hello from thread").unwrap();
    });
    
    let received = rx.recv().unwrap();
    println!("  收到消息: {}", received);
    
    println!("\n📋 并发概念：");
    println!("  • 线程创建和同步");
    println!("  • 消息传递通信");
    println!("  • 共享状态管理");
    println!("  • 原子操作");
}

/// 第20章：Unsafe Rust示例
pub fn unsafe_rust() {
    println!("⚠️ 第20章：Unsafe Rust");
    println!("=====================================");
    println!("💡 运行完整示例：cargo run --bin unsafe_rust");
    
    println!("\n🎯 原始指针：");
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    unsafe {
        println!("  r1 指向的值: {}", *r1);
        println!("  r2 指向的值: {}", *r2);
    }
    
    println!("\n⚠️ 不安全函数：");
    unsafe fn dangerous() {
        println!("  这是一个不安全函数");
    }
    
    unsafe {
        dangerous();
    }
    
    println!("\n📋 Unsafe 能力：");
    println!("  • 解引用原始指针");
    println!("  • 调用不安全函数");
    println!("  • 访问可变静态变量");
    println!("  • 实现不安全 trait");
    
    println!("\n⚠️ 安全原则：");
    println!("  • 最小化 unsafe 代码");
    println!("  • 在安全抽象中包装");
    println!("  • 仔细验证内存安全");
}

/// 第21章：宏系统示例
pub fn macros() {
    println!("🎭 第21章：宏系统");
    println!("=====================================");
    println!("💡 运行完整示例：cargo run --bin macros");
    
    // 简单演示一些宏的概念
    macro_rules! say_hello {
        () => {
            println!("  Hello from a simple macro!");
        };
        ($name:expr) => {
            println!("  Hello, {}!", $name);
        };
    }
    
    println!("\n🎯 声明宏演示：");
    say_hello!();
    say_hello!("Rust");
    
    // vec! 宏演示
    let v = vec![1, 2, 3, 4, 5];
    println!("  vec! 宏创建的向量: {:?}", v);
    
    // println! 宏演示
    println!("  println! 宏支持格式化：{} + {} = {}", 2, 3, 2 + 3);
    
    // 自定义重复模式宏
    macro_rules! create_function {
        ($func_name:ident) => {
            fn $func_name() {
                println!("  函数 {} 被调用了", stringify!($func_name));
            }
        };
    }
    
    create_function!(foo);
    create_function!(bar);
    
    println!("\n🔧 宏生成的函数：");
    foo();
    bar();
    
    println!("\n📚 宏系统的特点：");
    println!("  • 编译时代码生成");
    println!("  • 模式匹配语法");
    println!("  • 元编程能力");
    println!("  • 代码复用和简化");
    println!("  • 卫生宏系统");
    
    println!("\n🎯 宏的类型：");
    println!("  • 声明宏 (macro_rules!)");
    println!("  • 过程宏 (proc_macro)");
    println!("  • 派生宏 (#[derive])");
    println!("  • 属性宏 (#[attribute])");
}

/// 第22章：过程宏示例
pub fn procedural_macros() {
    println!("🔮 第22章：过程宏深入解析");
    println!("=====================================");
    println!("💡 运行完整示例：cargo run --bin procedural_macros");
    
    // 模拟过程宏的效果
    #[derive(Debug)]
    struct ExampleStruct {
        name: String,
        value: i32,
    }
    
    let example = ExampleStruct {
        name: "Test".to_string(),
        value: 42,
    };
    
    println!("\n🎯 派生宏演示：");
    println!("  #[derive(Debug)] 宏自动实现了 Debug trait");
    println!("  示例结构体: {:?}", example);
    
    // 模拟 Builder 模式
    println!("\n🏗️ Builder 模式概念：");
    println!("  // 使用派生宏自动生成 Builder");
    println!("  #[derive(Builder)]");
    println!("  struct User {{");
    println!("      name: String,");
    println!("      email: String,");
    println!("  }}");
    println!("  // 生成：UserBuilder, name(), email(), build()");
    
    println!("\n🔧 属性宏概念：");
    println!("  // 给函数添加计时功能");
    println!("  #[timing]");
    println!("  fn expensive_function() {{ ... }}");
    println!("  // 自动添加性能监控代码");
    
    println!("\n📚 过程宏的类型：");
    println!("  • 派生宏 (Derive Macros): #[derive(MyTrait)]");
    println!("  • 属性宏 (Attribute Macros): #[my_attribute]");
    println!("  • 函数式宏 (Function-like Macros): my_macro!()");
    
    println!("\n🔧 过程宏的特点：");
    println!("  • 操作 TokenStream");
    println!("  • 生成任意复杂代码");
    println!("  • 需要独立的 proc-macro crate");
    println!("  • 比声明宏更强大");
    println!("  • 使用 syn、quote、proc-macro2 库");
    
    println!("\n🎯 应用场景：");
    println!("  • ORM 框架 (如 Diesel)");
    println!("  • 序列化库 (如 Serde)");
    println!("  • Web 框架 (如 Actix)");
    println!("  • 自定义 DSL");
    println!("  • 代码生成工具");
} 