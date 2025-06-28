// 第6章：常用类型解析示例
// 使用命令：cargo run --bin common_types

fn main() {
    println!("🦀 Rust 基础教程 - 第6章：常用类型解析");
    println!("====================================\n");
    
    // 6.1 Vector
    println!("📍 6.1 Vector");
    println!("-------------");
    vector_demo();
    println!();
    
    // 6.2 String
    println!("📍 6.2 String");
    println!("-------------");
    string_demo();
    println!();
    
    // 6.3 类型比较
    println!("📍 6.3 类型比较");
    println!("---------------");
    type_comparison();
    
    println!("\n✅ 第6章示例运行完成！");
}

// 6.1 Vector
fn vector_demo() {
    println!("创建 Vector:");
    create_vectors();
    
    println!("\n更新 Vector:");
    update_vectors();
    
    println!("\n读取 Vector 的元素:");
    read_vector_elements();
    
    println!("\n遍历 Vector:");
    iterate_vectors();
    
    println!("\n使用枚举存储多种类型:");
    enum_in_vector();
    
    println!("\n Vector 的内存布局:");
    vector_memory_layout();
    
    println!("\n切片（Slices）:");
    vector_slices();
}

fn create_vectors() {
    // 创建空 vector
    let v1: Vec<i32> = Vec::new();
    
    // 使用 vec! 宏
    let v2 = vec![1, 2, 3];
    
    // 创建具有初始容量的 vector
    let mut v3: Vec<i32> = Vec::with_capacity(10);
    
    // 使用迭代器创建
    let v4: Vec<i32> = (0..5).collect();
    println!("  迭代器创建: {:?}", v4);  // [0, 1, 2, 3, 4]
    
    // 创建相同元素的 vector
    let v5 = vec![0; 5];  // [0, 0, 0, 0, 0]
    println!("  相同元素: {:?}", v5);
}

fn update_vectors() {
    let mut v = Vec::new();
    
    // 添加元素
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    
    println!("  添加后: {:?}", v);
    
    // 移除元素
    let last = v.pop();  // 返回 Option<T>
    println!("  弹出: {:?}", last);
    
    // 插入元素
    v.insert(1, 10);  // 在索引 1 处插入 10
    println!("  插入后: {:?}", v);
    
    // 移除指定位置的元素
    let removed = v.remove(1);
    println!("  移除的元素: {}", removed);
    
    // 保留满足条件的元素
    v.retain(|&x| x % 2 == 0);
    println!("  只保留偶数: {:?}", v);
}

fn read_vector_elements() {
    let v = vec![1, 2, 3, 4, 5];
    
    // 使用索引访问
    let third = &v[2];
    println!("  第三个元素是: {}", third);
    
    // 使用 get 方法（返回 Option）
    match v.get(2) {
        Some(third) => println!("  第三个元素是: {}", third),
        None => println!("  没有第三个元素"),
    }
    
    // 越界访问的区别
    // let does_not_exist = &v[100];  // panic!
    let does_not_exist = v.get(100);   // 返回 None
    println!("  索引 100: {:?}", does_not_exist);
}

fn iterate_vectors() {
    let v = vec![100, 32, 57];
    
    // 不可变遍历
    print!("  不可变遍历: ");
    for i in &v {
        print!("{} ", i);
    }
    println!();
    
    // 可变遍历
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("  修改后: {:?}", v);
    
    // 获取所有权的遍历
    let v = vec![1, 2, 3];
    print!("  获取所有权遍历: ");
    for i in v {  // v 被移动
        print!("{} ", i);
    }
    println!();
    // println!("{:?}", v);  // 错误！v 已被移动
}

fn enum_in_vector() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("  整数: {}", i),
            SpreadsheetCell::Float(f) => println!("  浮点数: {}", f),
            SpreadsheetCell::Text(s) => println!("  文本: {}", s),
        }
    }
}

fn vector_memory_layout() {
    use std::mem;
    
    let mut v = Vec::with_capacity(10);
    
    println!("  容量: {}", v.capacity());
    println!("  长度: {}", v.len());
    println!("  Vector 本身大小: {} 字节", mem::size_of_val(&v));
    
    // 添加元素
    for i in 0..5 {
        v.push(i);
        println!("  添加 {} 后 - 长度: {}, 容量: {}", i, v.len(), v.capacity());
    }
    
    // 收缩容量
    v.shrink_to_fit();
    println!("  收缩后容量: {}", v.capacity());
    
    // 预留容量
    v.reserve(20);
    println!("  预留后容量: {}", v.capacity());
}

fn vector_slices() {
    let v = vec![1, 2, 3, 4, 5];
    
    // 获取切片
    let slice = &v[1..3];
    println!("  切片: {:?}", slice);
    
    // 可变切片
    let mut v = vec![1, 2, 3, 4, 5];
    let slice = &mut v[..];
    slice[0] = 10;
    println!("  修改后的 vector: {:?}", v);
    
    // 切片作为参数
    fn sum_slice(slice: &[i32]) -> i32 {
        slice.iter().sum()
    }
    
    let sum = sum_slice(&v[1..4]);
    println!("  切片 [1..4] 的和: {}", sum);
}

// 6.2 String
fn string_demo() {
    println!("创建 String:");
    create_strings();
    
    println!("\n更新 String:");
    update_strings();
    
    println!("\n索引 String:");
    string_indexing();
    
    println!("\n遍历 String:");
    iterate_strings();
    
    println!("\nString 的方法:");
    string_methods();
}

fn create_strings() {
    // 创建空字符串
    let mut s1 = String::new();
    
    // 从字符串字面量创建
    let s2 = "initial contents".to_string();
    let s3 = String::from("initial contents");
    
    // 从其他类型创建
    let s4 = 42.to_string();
    let s5 = format!("Hello, {}!", "world");
    
    println!("  s5: {}", s5);
}

fn update_strings() {
    let mut s = String::from("foo");
    
    // 追加字符串切片
    s.push_str("bar");
    println!("  push_str 后: {}", s);
    
    // 追加单个字符
    s.push('!');
    println!("  push 后: {}", s);
    
    // 使用 + 运算符
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // 注意 s1 被移动了
    println!("  + 运算符: {}", s3);
    // println!("s1: {}", s1);  // 错误！s1 已被移动
    
    // 使用 format! 宏
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("  format!: {}", s);
    println!("  s1 仍然有效: {}", s1);  // format! 不会获取所有权
}

fn string_indexing() {
    let s = String::from("hello");
    // let h = s[0];  // 错误！String 不支持索引
    
    // 原因：UTF-8 编码
    let hello = String::from("Здравствуйте");
    println!("  俄语字符串长度: {} 字节", hello.len());  // 24 字节，不是 12
    
    // 使用切片（需要小心）
    let s = String::from("hello");
    let h = &s[0..1];  // "h"
    println!("  第一个字母: {}", h);
    
    // UTF-8 字符可能占用多个字节
    let hello = String::from("你好");
    // let h = &hello[0..1];  // panic! 不是字符边界
    let h = &hello[0..3];     // "你"
    println!("  第一个汉字: {}", h);
}

fn iterate_strings() {
    let s = String::from("नमस्ते");
    
    // 遍历字符
    println!("  字符:");
    for c in s.chars() {
        print!("    {}", c);
    }
    println!();
    
    // 遍历字节
    println!("  字节:");
    for b in s.bytes() {
        print!("    {}", b);
    }
    println!();
}

fn string_methods() {
    let mut s = String::from("  Hello, Rust!  ");
    
    // 去除空白
    let trimmed = s.trim();
    println!("  trim: '{}'", trimmed);
    
    // 分割
    let parts: Vec<&str> = trimmed.split(',').collect();
    println!("  分割: {:?}", parts);
    
    // 替换
    let replaced = s.replace("Rust", "World");
    println!("  替换: '{}'", replaced);
    
    // 大小写转换
    println!("  大写: {}", s.to_uppercase());
    println!("  小写: {}", s.to_lowercase());
    
    // 检查前缀和后缀
    let s = String::from("Hello, world!");
    println!("  以 'Hello' 开头? {}", s.starts_with("Hello"));
    println!("  以 '!' 结尾? {}", s.ends_with("!"));
    
    // 查找
    match s.find("world") {
        Some(index) => println!("  'world' 在索引 {} 处", index),
        None => println!("  未找到 'world'"),
    }
}

// 6.3 类型比较
fn type_comparison() {
    println!("String vs &str:");
    string_vs_str();
    
    println!("\nString/&str 内存布局:");
    string_memory_layout();
    
    println!("\n[T; N] vs [T] vs &[T]:");
    array_comparisons();
    
    println!("\n选择合适的类型:");
    type_selection();
    
    println!("\n性能考虑:");
    performance_considerations();
    
    println!("\n常用转换方法:");
    conversion_methods();
}

fn string_vs_str() {
    // String - 堆分配，可变，拥有所有权
    let mut string = String::from("Hello");
    string.push_str(", world!");
    
    // &str - 字符串切片，不可变，借用
    let string_slice: &str = &string[..];
    let literal: &str = "Hello, world!";
    
    // 转换
    let s: String = literal.to_string();
    let s: String = String::from(literal);
    let slice: &str = &s;
    let slice: &str = s.as_str();
    
    // 函数参数建议使用 &str
    fn takes_slice(s: &str) {
        println!("    收到: {}", s);
    }
    
    takes_slice(&string);     // String -> &str
    takes_slice(literal);     // &str
    takes_slice(&s[0..5]);    // 切片
}

fn string_memory_layout() {
    use std::mem;
    
    // String 的内存布局
    let s = String::from("hello");
    println!("  String 大小: {} 字节", mem::size_of_val(&s));
    println!("  String 容量: {}", s.capacity());
    println!("  String 长度: {}", s.len());
    
    // &str 的内存布局
    let slice: &str = "hello";
    println!("  &str 大小: {} 字节", mem::size_of_val(&slice));
    
    // 内部表示
    println!("\n  String 包含:");
    println!("    - 指向堆数据的指针");
    println!("    - 长度");
    println!("    - 容量");
    
    println!("\n  &str 包含:");
    println!("    - 指向数据的指针");
    println!("    - 长度");
}

fn array_comparisons() {
    use std::mem;
    
    // [T; N] - 固定大小数组
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("  数组大小: {} 字节", mem::size_of_val(&array));
    
    // [T] - 动态大小类型（DST），不能直接使用
    // let slice: [i32] = [1, 2, 3];  // 错误！
    
    // &[T] - 切片引用
    let slice: &[i32] = &array[1..4];
    println!("  切片大小: {} 字节", mem::size_of_val(&slice));
    
    // 转换
    let array = [1, 2, 3, 4, 5];
    let slice: &[i32] = &array;      // 数组到切片
    let slice: &[i32] = &array[..];  // 显式切片
    
    // Vec<T> 到切片
    let vec = vec![1, 2, 3, 4, 5];
    let slice: &[i32] = &vec;
    let slice: &[i32] = vec.as_slice();
}

fn type_selection() {
    // 字符串类型选择
    println!("  字符串类型选择:");
    
    // 使用 String 当你需要：
    // - 拥有字符串数据
    // - 动态构建或修改字符串
    let mut owned = String::new();
    owned.push_str("Hello");
    println!("    String 用于拥有和修改: {}", owned);
    
    // 使用 &str 当你需要：
    // - 只读访问字符串
    // - 作为函数参数
    fn process(s: &str) {
        println!("    &str 用于只读访问: {}", s);
    }
    
    process(&owned);
    process("literal");
    
    // 集合类型选择
    println!("\n  集合类型选择:");
    
    // 使用数组当你需要：
    // - 固定大小的集合
    // - 栈上分配
    let coords: [f64; 3] = [0.0, 1.0, 2.0];
    println!("    数组用于固定大小: {:?}", coords);
    
    // 使用 Vec 当你需要：
    // - 动态大小的集合
    // - 堆上分配
    let mut numbers = Vec::new();
    numbers.push(1);
    println!("    Vec 用于动态大小: {:?}", numbers);
    
    // 使用切片当你需要：
    // - 引用集合的一部分
    // - 通用的集合视图
    fn sum(numbers: &[i32]) -> i32 {
        numbers.iter().sum()
    }
    
    println!("    切片用于通用视图: {}", sum(&numbers));
}

fn performance_considerations() {
    use std::time::Instant;
    
    // String vs &str 性能
    let start = Instant::now();
    let mut s = String::new();
    for _ in 0..1000 {
        s.push_str("hello");
    }
    println!("  String 拼接时间: {:?}", start.elapsed());
    
    // 预分配容量
    let start = Instant::now();
    let mut s = String::with_capacity(5000);
    for _ in 0..1000 {
        s.push_str("hello");
    }
    println!("  预分配 String 时间: {:?}", start.elapsed());
    
    // Vec 性能
    let start = Instant::now();
    let mut v = Vec::new();
    for i in 0..10000 {
        v.push(i);
    }
    println!("  Vec push 时间: {:?}", start.elapsed());
    
    let start = Instant::now();
    let v: Vec<i32> = (0..10000).collect();
    println!("  Vec collect 时间: {:?}", start.elapsed());
}

fn conversion_methods() {
    // String 相关转换
    let s = String::from("hello");
    let bytes = s.as_bytes();           // &[u8]
    let slice = s.as_str();             // &str
    let chars: Vec<char> = s.chars().collect();
    
    println!("  String 转换:");
    println!("    字节数: {}", bytes.len());
    println!("    切片: {}", slice);
    println!("    字符: {:?}", chars);
    
    // Vec 相关转换
    let v = vec![1, 2, 3];
    let slice = v.as_slice();           // &[T]
    
    println!("\n  Vec 转换:");
    println!("    切片: {:?}", slice);
    
    // 从迭代器创建
    let s: String = ['h', 'e', 'l', 'l', 'o'].iter().collect();
    let v: Vec<i32> = (1..=5).collect();
    
    println!("    从迭代器创建 String: {}", s);
    println!("    从迭代器创建 Vec: {:?}", v);
    
    // 字节和字符串转换
    let bytes = b"hello";
    let s = String::from_utf8(bytes.to_vec()).unwrap();
    let s = std::str::from_utf8(bytes).unwrap();
    println!("    从字节转换: {}", s);
} 