// 教学示例 - 允许未使用的代码
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// 第9章：生命周期与标注示例
// 运行命令：cargo run --bin lifetimes

use std::fmt::Display;

fn main() {
    println!("🔹 第9章：生命周期与标注 (Lifetimes)");
    println!("{}", "=".repeat(50));
    
    basic_lifetimes();
    println!();
    
    function_lifetimes();
    println!();
    
    struct_lifetimes();
    println!();
    
    lifetime_elision();
    println!();
    
    static_lifetimes();
    println!();
    
    advanced_lifetimes();
}

/// 9.1 基本生命周期概念
fn basic_lifetimes() {
    println!("📝 9.1 基本生命周期概念");
    println!("{}", "-".repeat(30));
    
    // 正确的生命周期示例
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
    
    println!("x 的生命周期 'b 大于 r 的生命周期 'a，所以代码有效");
    
    // 生命周期与作用域
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest_with_lifetime(string1.as_str(), string2.as_str());
        println!("最长的字符串是 {}", result);
        // result 只能在这个作用域内使用，因为它可能引用 string2
    }
    
    // 字符串字面量有 'static 生命周期
    let s: &'static str = "I have a static lifetime.";
    println!("静态生命周期字符串: {}", s);
}

/// 9.2 函数中的生命周期
fn function_lifetimes() {
    println!("📝 9.2 函数中的生命周期");
    println!("{}", "-".repeat(30));
    
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest_with_lifetime(string1.as_str(), string2);
    println!("最长的字符串是 {}", result);
    
    // 生命周期标注的不同情况
    let first = first_word("hello world");
    println!("第一个单词: {}", first);
    
    let announcement = "今天天气真好！";
    let result = longest_with_announcement(
        string1.as_str(),
        string2,
        announcement,
    );
    println!("带公告的最长字符串: {}", result);
    
    // 返回引用的函数
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    println!("第一句话: {}", first_sentence);
}

// 需要生命周期标注的函数
fn longest_with_lifetime<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 生命周期省略规则适用的函数
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// 复杂的生命周期标注
fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("公告: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// 9.3 结构体中的生命周期
fn struct_lifetimes() {
    println!("📝 9.3 结构体中的生命周期");
    println!("{}", "-".repeat(30));
    
    // 存储引用的结构体需要生命周期标注
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
        
        // 生命周期省略规则适用
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("请注意: {}", announcement);
            self.part
        }
        
        // 明确的生命周期标注
        fn get_part(&self) -> &'a str {
            self.part
        }
    }
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("重要摘录: {}", i.part);
    println!("摘录等级: {}", i.level());
    
    let announcement = "特别通知";
    let part = i.announce_and_return_part(announcement);
    println!("返回的部分: {}", part);
    
    // 多个生命周期参数的结构体
    struct DoubleRef<'a, 'b> {
        first: &'a str,
        second: &'b str,
    }
    
    let first_string = String::from("first");
    let second_string = String::from("second");
    
    let double_ref = DoubleRef {
        first: &first_string,
        second: &second_string,
    };
    
    println!("双引用: {} 和 {}", double_ref.first, double_ref.second);
}

/// 9.4 生命周期省略规则
fn lifetime_elision() {
    println!("📝 9.4 生命周期省略规则");
    println!("{}", "-".repeat(30));
    
    // 规则1：每个引用参数都有自己的生命周期参数
    fn rule1_example(s: &str) -> &str {
        // 编译器推断为: fn rule1_example<'a>(s: &'a str) -> &'a str
        s
    }
    
    // 规则2：如果只有一个输入生命周期参数，它被赋予所有输出生命周期参数
    fn rule2_example(s: &str) -> (&str, &str) {
        // 编译器推断为: fn rule2_example<'a>(s: &'a str) -> (&'a str, &'a str)
        (s, s)
    }
    
    // 规则3：如果方法有多个输入生命周期参数，但其中一个是 &self 或 &mut self
    struct MyStruct<'a> {
        data: &'a str,
    }
    
    impl<'a> MyStruct<'a> {
        fn rule3_example(&self, other: &str) -> &str {
            // 编译器推断 self 的生命周期被赋给返回值
            self.data
        }
    }
    
    let text1 = "hello";
    let result1 = rule1_example(text1);
    println!("规则1示例: {}", result1);
    
    let (r1, r2) = rule2_example(text1);
    println!("规则2示例: {}, {}", r1, r2);
    
    let my_struct = MyStruct { data: "struct data" };
    let result3 = my_struct.rule3_example("other");
    println!("规则3示例: {}", result3);
}

/// 9.5 静态生命周期
fn static_lifetimes() {
    println!("📝 9.5 静态生命周期");
    println!("{}", "-".repeat(30));
    
    // 字符串字面量有 'static 生命周期
    let s: &'static str = "I have a static lifetime.";
    println!("静态字符串: {}", s);
    
    // 静态变量
    static HELLO: &str = "Hello, world!";
    println!("静态变量: {}", HELLO);
    
    // 返回静态生命周期的函数
    fn get_static_str() -> &'static str {
        "This string lives forever!"
    }
    
    let static_str = get_static_str();
    println!("静态函数返回: {}", static_str);
    
    // 注意：不是所有字符串都需要 'static 生命周期
    fn needs_static(s: &'static str) {
        println!("需要静态生命周期: {}", s);
    }
    
    needs_static("字面量可以传递");
    // let dynamic = String::from("动态字符串");
    // needs_static(&dynamic); // 错误！动态字符串没有 'static 生命周期
}

/// 9.6 高级生命周期
fn advanced_lifetimes() {
    println!("📝 9.6 高级生命周期");
    println!("{}", "-".repeat(30));
    
    // 生命周期约束
    fn lifetime_bound<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
        // 'b: 'a 意味着 'b 必须比 'a 活得长
        println!("y: {}", y);
        x
    }
    
    let long_lived = String::from("long lived");
    {
        let short_lived = String::from("short");
        let result = lifetime_bound(&short_lived, &long_lived);
        println!("生命周期约束示例: {}", result);
    }
    
    // 生命周期子类型
    struct Context<'a>(&'a str);
    
    struct Parser<'a> {
        context: &'a Context<'a>,
    }
    
    impl<'a> Parser<'a> {
        fn parse(&self) -> Result<(), &'a str> {
            if self.context.0.is_empty() {
                Err("空上下文")
            } else {
                Ok(())
            }
        }
    }
    
    fn parse_context(context: Context) -> Result<(), String> {
        let parser = Parser { context: &context };
        match parser.parse() {
            Ok(()) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
    
    let ctx = Context("some context");
    match parse_context(ctx) {
        Ok(()) => println!("解析成功"),
        Err(e) => println!("解析失败: {}", e),
    }
    
    // 高阶生命周期边界 (HRTB)
    fn call_with_ref<F>(f: F) 
    where
        F: for<'a> Fn(&'a str) -> &'a str,
    {
        let s = String::from("hello");
        let result = f(&s);
        println!("HRTB 示例结果: {}", result);
    }
    
    call_with_ref(|s| s);
    call_with_ref(|s| &s[1..]);
    
    // 生命周期与闭包
    let mut data = vec![1, 2, 3, 4, 5];
    
    {
        let processor = |slice: &mut [i32]| {
            for item in slice {
                *item *= 2;
            }
        };
        
        processor(&mut data);
    }
    
    println!("处理后的数据: {:?}", data);
} 