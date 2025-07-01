// 教学示例 - 允许未使用的代码
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// 第3章：所有权机制示例
// 使用命令：cargo run --bin ownership

fn main() {
    println!("🦀 Rust 基础教程 - 第3章：所有权机制");
    println!("==================================\n");
    
    // 3.1 目的与核心思想
    println!("📍 3.1 目的与核心思想");
    println!("---------------------");
    core_concepts();
    println!();
    
    // 3.2 所有权规则
    println!("📍 3.2 所有权规则");
    println!("-----------------");
    ownership_rules();
    println!();
    
    // 3.3 验证规则
    println!("📍 3.3 验证规则");
    println!("---------------");
    verify_rules();
    
    println!("\n✅ 第3章示例运行完成！");
}

// 3.1 目的与核心思想
fn core_concepts() {
    println!("所有权系统的核心思想:");
    println!("每个值都有一个所有者，并且同时只能有一个所有者");
    
    {
        // s 是 "hello" 的所有者
        let s = String::from("hello");
        println!("  s 拥有字符串: {}", s);
        
        // 当所有者离开作用域时，值会被自动清理
    } // s 离开作用域，内存被释放
    println!("  s 已离开作用域，内存被自动释放");
    
    println!("\n内存安全保证:");
    println!("  ✓ 防止使用后释放（Use After Free）");
    println!("  ✓ 防止双重释放（Double Free）");
    println!("  ✓ 防止空悬指针（Dangling Pointer）");
    
    // 展示编译时安全检查
    memory_safety_demo();
}

fn memory_safety_demo() {
    println!("\n编译时安全检查演示:");
    
    // 1. 防止双重释放
    let s1 = String::from("hello");
    let s2 = s1;  // s1 的所有权移动到 s2
    println!("  s2: {} (s1 的所有权已转移)", s2);
    // drop(s1);  // 编译错误！s1 不再拥有值
    drop(s2);     // 只有 s2 能释放内存
    println!("  ✓ 防止了双重释放");
    
    // 2. 防止使用已移动的值
    let s = String::from("world");
    let r = &s;
    // drop(s);  // 如果这样做会编译错误
    println!("  引用 r: {} (s 仍然有效)", r);
}

// 3.2 所有权规则
fn ownership_rules() {
    println!("所有权的三条基本规则:");
    println!("1. 每个值都有一个所有者");
    println!("2. 值在任一时刻只能有一个所有者");
    println!("3. 当所有者离开作用域时，值被丢弃");
    
    println!("\n规则一：每个值都有一个所有者");
    rule_one();
    
    println!("\n规则二：值在任一时刻只能有一个所有者");
    rule_two();
    
    println!("\n规则三：当所有者离开作用域时，值被丢弃");
    rule_three();
}

fn rule_one() {
    let x = 5;           // x 拥有值 5
    let s = String::from("hello");  // s 拥有字符串 "hello"
    let v = vec![1, 2, 3];         // v 拥有向量
    
    println!("  x 拥有值: {}", x);
    println!("  s 拥有字符串: {}", s);
    println!("  v 拥有向量: {:?}", v);
    
    // 复合类型
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }
    
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };  // person 拥有整个结构体，包括其字段
    
    println!("  person 拥有结构体: {:?}", person);
}

fn rule_two() {
    let s1 = String::from("hello");
    let s2 = s1;  // 所有权从 s1 移动到 s2
    
    // println!("{}", s1);  // 编译错误！s1 不再有效
    println!("  s2 现在拥有字符串: {}", s2);
    
    // 移动语义详解
    println!("\n移动语义详解:");
    
    // 对于实现了 Copy trait 的类型，赋值是复制
    let x = 5;
    let y = x;  // x 被复制到 y
    println!("  Copy 类型 - x: {}, y: {} (都可用)", x, y);
    
    // 对于堆分配的类型，赋值是移动
    let s1 = String::from("hello");
    let s2 = s1;  // s1 被移动到 s2
    println!("  移动类型 - s2: {} (s1 不再可用)", s2);
    
    // 函数调用也会发生移动
    let s = String::from("world");
    takes_ownership(s);  // s 的所有权移动到函数
    println!("  函数调用后，s 已被移动");
}

fn takes_ownership(some_string: String) {
    println!("    函数内部: {}", some_string);
}  // some_string 离开作用域，内存被释放

fn rule_three() {
    {
        let s = String::from("hello");  // s 在这里有效
        println!("  s 在作用域内: {}", s);
        // 使用 s
    }  // s 离开作用域，drop 函数被自动调用
    println!("  s 已离开作用域，内存被释放");
    
    // Drop trait 示例
    println!("\nDrop trait 示例:");
    drop_example();
}

fn drop_example() {
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("    Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("  CustomSmartPointers created.");
    // d 先被 drop，然后是 c（LIFO 顺序）
}

// 3.3 验证规则
fn verify_rules() {
    println!("Copy 和 Clone:");
    copy_and_clone();
    
    println!("\n所有权和函数:");
    ownership_and_functions();
    
    println!("\n所有权链:");
    ownership_chain();
    
    println!("\n部分移动:");
    partial_move();
}

fn copy_and_clone() {
    // Copy trait
    println!("  Copy trait 示例:");
    let x = 5;
    let y = x;
    println!("    x = {}, y = {} (都可用)", x, y);
    
    // 浮点数、布尔值、字符也是 Copy
    let a = 3.14;
    let b = a;
    
    let flag1 = true;
    let flag2 = flag1;
    
    let ch1 = 'A';
    let ch2 = ch1;
    
    // 元组（如果所有元素都是 Copy）
    let tup1 = (1, 2.0, true);
    let tup2 = tup1;
    println!("    元组复制: {:?} (原始仍可用)", tup1);
    
    // Clone trait
    println!("\n  Clone trait 示例:");
    let s1 = String::from("hello");
    let s2 = s1.clone();  // 深拷贝
    
    println!("    s1 = {}, s2 = {} (都可用)", s1, s2);
}

fn ownership_and_functions() {
    let s = String::from("hello");
    let len = calculate_length(&s);  // 传递引用，不转移所有权
    println!("  '{}' 的长度是 {} (s 仍然可用)", s, len);
    
    let s2 = String::from("world");
    let s3 = take_and_return(s2);  // s2 的所有权被转移并返回
    // println!("{}", s2);  // 错误！
    println!("  返回的字符串: {} (s2 已被移动)", s3);
}

fn calculate_length(s: &String) -> usize {
    s.len()  // 不拥有所有权，只是借用
}

fn take_and_return(s: String) -> String {
    s  // 返回所有权
}

fn ownership_chain() {
    let s1 = give_ownership();         // give_ownership 移动返回值到 s1
    let s2 = String::from("hello");    // s2 进入作用域
    let s3 = takes_and_gives_back(s2); // s2 被移动到函数，返回值移动到 s3
    
    println!("  s1: {}", s1);
    // println!("{}", s2);  // 错误！s2 已被移动
    println!("  s3: {} (s2 已被移动)", s3);
}

fn give_ownership() -> String {
    let some_string = String::from("yours");
    some_string  // 返回并移动所有权
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // 返回并移动所有权
}

fn partial_move() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }
    
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    // 移动 name 字段
    let name = person.name;
    
    // person 部分移动了，不能整体使用
    // println!("{:?}", person);  // 错误！
    
    // 但可以使用未移动的字段
    println!("  年龄: {} (name 字段已被移动)", person.age);
    println!("  姓名: {} (已移动到新变量)", name);
    
    // 解构时的部分移动
    let person2 = Person {
        name: String::from("Bob"),
        age: 25,
    };
    
    let Person { name, age } = person2;
    println!("  解构 - 姓名: {}, 年龄: {}", name, age);
    // person2 完全移动了
} 