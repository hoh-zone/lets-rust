// 第4章：借用机制示例
// 使用命令：cargo run --bin borrowing

fn main() {
    println!("🦀 Rust 基础教程 - 第4章：借用机制");
    println!("================================\n");
    
    // 4.1 引用与借用规则
    println!("📍 4.1 引用与借用规则");
    println!("---------------------");
    reference_and_borrowing();
    println!();
    
    // 4.2 验证借用规则
    println!("📍 4.2 验证借用规则");
    println!("-----------------");
    verify_borrowing_rules();
    println!();
    
    // 4.3 切片
    println!("📍 4.3 切片");
    println!("-----------");
    slices_demo();
    println!();
    
    // 4.4 悬垂引用
    println!("📍 4.4 悬垂引用");
    println!("---------------");
    dangling_references();
    
    println!("\n✅ 第4章示例运行完成！");
}

// 4.1 引用与借用规则
fn reference_and_borrowing() {
    println!("什么是借用？");
    println!("借用允许你访问数据而不获取其所有权");
    
    let s1 = String::from("hello");
    
    // 创建一个引用，不获取所有权
    let len = calculate_length(&s1);
    
    // s1 仍然有效，因为我们只是借用了它
    println!("  '{}' 的长度是 {}", s1, len);
    
    println!("\n引用的基本规则:");
    println!("1. 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用");
    println!("2. 引用必须总是有效的");
    
    basic_reference_rules();
}

fn calculate_length(s: &String) -> usize {
    s.len()
    // s 离开作用域，但因为它没有所有权，所以什么都不会发生
}

fn basic_reference_rules() {
    println!("\n不可变引用:");
    let s = String::from("hello");
    
    // 创建不可变引用
    let r1 = &s;
    let r2 = &s;  // 可以有多个不可变引用
    
    println!("  r1: {}, r2: {}", r1, r2);
    
    println!("\n可变引用:");
    let mut s = String::from("hello");
    
    // 创建可变引用
    let r = &mut s;
    r.push_str(", world");
    
    println!("  修改后: {}", r);
    
    println!("\n引用的作用域:");
    reference_scope_demo();
}

fn reference_scope_demo() {
    let mut s = String::from("hello");
    
    let r1 = &s;     // r1 的作用域开始
    let r2 = &s;     // r2 的作用域开始
    println!("  不可变引用: {} and {}", r1, r2);
    // r1 和 r2 的作用域在这里结束
    
    let r3 = &mut s; // r3 的作用域开始，现在可以创建可变引用
    r3.push_str(", world");
    println!("  可变引用: {}", r3);
    // r3 的作用域结束
}

// 4.2 验证借用规则
fn verify_borrowing_rules() {
    println!("规则验证:");
    
    println!("\n规则一：不可变引用和可变引用不能同时存在");
    rule_one_demo();
    
    println!("\n规则二：同一时间只能有一个可变引用");
    rule_two_demo();
    
    println!("\n数据竞争的预防:");
    data_race_prevention();
    
    println!("\n引用的引用:");
    reference_to_reference();
}

fn rule_one_demo() {
    let mut s = String::from("hello");
    
    let r1 = &s;     // 不可变引用
    let r2 = &s;     // 不可变引用
    // let r3 = &mut s; // 错误！不能在有不可变引用时创建可变引用
    
    println!("  不可变引用: {} and {}", r1, r2);
    // r1 和 r2 不再使用
    
    let r3 = &mut s; // 现在可以创建可变引用
    r3.push_str(" world");
    println!("  可变引用: {}", r3);
}

fn rule_two_demo() {
    let mut s = String::from("hello");
    
    let r1 = &mut s;
    // let r2 = &mut s; // 错误！不能同时有两个可变引用
    
    r1.push_str(" world");
    println!("  第一个可变引用: {}", r1);
    
    // r1 作用域结束后，可以创建新的可变引用
    let r2 = &mut s;
    r2.push_str("!");
    println!("  第二个可变引用: {}", r2);
}

fn data_race_prevention() {
    let mut v = vec![1, 2, 3, 4, 5];
    
    println!("  防止数据竞争:");
    println!("  - 在其他语言中，以下代码可能导致数据竞争");
    println!("  - Rust 编译器会阻止这种情况");
    
    // 这会导致数据竞争（在其他语言中）
    let first = &v[0];      // 不可变引用
    // v.push(6);              // 可变操作，可能导致重新分配
    println!("  第一个元素: {}", first);
    
    // 正确的做法：先使用引用，再修改
    // first 的作用域结束后才修改
    v.push(6);
    println!("  添加元素后: {:?}", v);
}

fn reference_to_reference() {
    let s = String::from("hello");
    let r1 = &s;      // &String
    let r2 = &r1;     // &&String
    let r3 = &r2;     // &&&String
    
    // 自动解引用
    println!("  自动解引用:");
    println!("    s: {}", s);
    println!("    r1: {}", r1);
    println!("    r2: {}", r2);  // 自动解引用
    println!("    r3: {}", r3);  // 自动解引用
    
    // 显式解引用
    println!("  显式解引用:");
    println!("    *r1: {}", *r1);
    println!("    **r2: {}", **r2);
    println!("    ***r3: {}", ***r3);
}

// 4.3 切片
fn slices_demo() {
    println!("字符串切片:");
    string_slices();
    
    println!("\n数组切片:");
    array_slices();
    
    println!("\n可变切片:");
    mutable_slices();
}

fn string_slices() {
    let s = String::from("hello world");
    
    // 字符串切片
    let hello = &s[0..5];   // 或 &s[..5]
    let world = &s[6..11];  // 或 &s[6..]
    let whole = &s[..];     // 整个字符串的切片
    
    println!("  hello: {}", hello);
    println!("  world: {}", world);
    println!("  whole: {}", whole);
    
    // 使用 first_word 函数
    let word = first_word(&s);
    println!("  第一个单词是: {}", word);
    
    // 字符串字面量就是切片
    let s2 = "Hello, world!";  // s2 的类型是 &str
    let word2 = first_word(s2);
    println!("  字面量的第一个单词: {}", word2);
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

fn array_slices() {
    let a = [1, 2, 3, 4, 5];
    
    // 数组切片
    let slice = &a[1..3];  // 包含索引 1 和 2 的元素
    
    println!("  原数组: {:?}", a);
    println!("  切片 [1..3]: {:?}", slice);
    
    // 使用切片
    let sum = sum_slice(&a[..]);  // 传递整个数组的切片
    println!("  数组总和: {}", sum);
}

fn sum_slice(slice: &[i32]) -> i32 {
    let mut sum = 0;
    for &item in slice {
        sum += item;
    }
    sum
}

fn mutable_slices() {
    let mut v = vec![1, 2, 3, 4, 5];
    
    println!("  原向量: {:?}", v);
    
    // 可变切片
    let slice = &mut v[2..4];
    slice[0] = 10;  // 修改第一个元素（原来的 v[2]）
    slice[1] = 20;  // 修改第二个元素（原来的 v[3]）
    
    println!("  修改切片后: {:?}", v);
}

// 4.4 悬垂引用
fn dangling_references() {
    println!("什么是悬垂引用？");
    println!("悬垂引用指向的内存可能已经被分配给其他人");
    
    println!("\nRust 如何防止悬垂引用:");
    
    // 正确的做法：返回 String
    let string = no_dangle();
    println!("  正确返回: {}", string);
    
    println!("\n生命周期基础:");
    lifetime_basics();
    
    println!("\n函数中的生命周期:");
    function_lifetimes();
    
    println!("\n结构体中的引用:");
    struct_references();
    
    println!("\n静态生命周期:");
    static_lifetime();
}

// 正确的做法：返回 String
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // 所有权被移出
}

fn lifetime_basics() {
    // let r;                // r 的生命周期开始
    
    {
        let x = 5;        // x 的生命周期开始
        // r = &x;           // 错误！x 的生命周期太短
        println!("  x 在内部作用域: {}", x);
    }                     // x 的生命周期结束
    
    // println!("r: {}", r); // r 是悬垂引用
    println!("  编译器防止了悬垂引用的创建");
}

fn function_lifetimes() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("  最长的字符串是: {}", result);
}

// 显式生命周期标注
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn struct_references() {
    // 结构体中存储引用需要生命周期标注
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("  重要片段: {}", i.part);
}

fn static_lifetime() {
    // 字符串字面量有 'static 生命周期
    let s: &'static str = "I have a static lifetime.";
    
    // 可以在整个程序运行期间使用
    println!("  静态字符串: {}", s);
    
    let static_str = get_static_str();
    println!("  函数返回的静态字符串: {}", static_str);
}

// 返回静态生命周期的引用
fn get_static_str() -> &'static str {
    "This string lives forever!"
} 