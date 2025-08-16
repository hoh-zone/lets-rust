// 教学示例 - 允许未使用的代码
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// 第21章：宏系统
// 演示 Rust 的声明宏（Declarative Macros）

use std::collections::HashMap;

fn main() {
    println!("🦀 第21章：宏系统");
    println!("=====================================");
    
    // 1. 宏与函数的区别
    macro_vs_function_demo();
    
    // 2. 基本宏语法
    basic_macro_syntax_demo();
    
    // 3. 宏参数类型
    macro_parameter_types_demo();
    
    // 4. 重复模式宏
    repetition_patterns_demo();
    
    // 5. HashMap 创建宏
    hashmap_creation_demo();
    
    // 6. 条件编译宏
    conditional_compilation_demo();
    
    // 7. 断言宏
    assertion_macros_demo();
    
    // 8. 日志宏
    logging_macros_demo();
    
    // 9. 计算宏
    calculation_macros_demo();
    
    // 10. 结构体生成宏
    struct_generation_demo();
    
    // 11. 测试宏
    test_macros_demo();
    
    // 12. 配置宏
    config_macros_demo();
    
    println!("\n🎉 第21章宏系统演示完成！");
    println!("📚 您已经学会了声明宏的各种用法");
    println!("💡 下一步：学习第22章过程宏，掌握更强大的元编程");
}

// ============================================================================
// 1. 宏与函数的区别
// ============================================================================

fn macro_vs_function_demo() {
    println!("\n📍 1. 宏与函数的区别");
    println!("{}", "-".repeat(40));
    
    // 函数：运行时调用
    fn add_function(a: i32, b: i32) -> i32 {
        a + b
    }
    
    // 宏：编译时展开
    macro_rules! add_macro {
        ($a:expr, $b:expr) => {
            $a + $b
        };
    }
    
    println!("🔧 函数调用（运行时）：");
    let result1 = add_function(3, 4);
    println!("   add_function(3, 4) = {}", result1);
    
    println!("\n🔧 宏调用（编译时展开）：");
    let result2 = add_macro!(3, 4);
    println!("   add_macro!(3, 4) = {}", result2);
    
    // 宏可以接受不同类型的参数
    let result3 = add_macro!(3.5, 4.2);
    println!("   add_macro!(3.5, 4.2) = {}", result3);
    
    println!("\n💡 关键区别：");
    println!("   • 函数在运行时调用，宏在编译时展开");
    println!("   • 宏可以接受不同类型的参数");
    println!("   • 宏能生成任意代码，函数只能返回值");
    println!("   • 宏可以接受可变数量的参数");
}

// ============================================================================
// 2. 基本宏语法
// ============================================================================

fn basic_macro_syntax_demo() {
    println!("\n📍 2. 基本宏语法");
    println!("{}", "-".repeat(40));
    
    macro_rules! my_macro {
        // 无参数模式
        () => {
            println!("   这是一个简单的宏");
        };
        
        // 带参数的模式
        ($name:expr) => {
            println!("   Hello, {}!", $name);
        };
        
        // 多个参数
        ($name:expr, $age:expr) => {
            println!("   {} is {} years old", $name, $age);
        };
    }
    
    println!("🔧 宏语法演示：");
    my_macro!();
    my_macro!("Alice");
    my_macro!("Bob", 25);
    
    println!("\n💡 语法解释：");
    println!("   • macro_rules! 定义声明宏");
    println!("   • () => {{}} 是模式匹配语法");
    println!("   • $name:expr 捕获表达式");
    println!("   • 不同模式可以有不同行为");
}

// ============================================================================
// 3. 宏参数类型
// ============================================================================

fn macro_parameter_types_demo() {
    println!("\n📍 3. 宏参数类型");
    println!("{}", "-".repeat(40));
    
    macro_rules! demo_types {
        // item: 语法项（函数、结构体等）
        (item: $item:item) => {
            $item
        };
        
        // block: 代码块
        (block: $block:block) => {
            println!("   执行代码块:");
            $block
        };
        
        // expr: 表达式
        (expr: $expr:expr) => {
            println!("   表达式的值: {}", $expr);
        };
        
        // ty: 类型
        (ty: $ty:ty) => {
            let _: $ty;
            println!("   类型: {}", stringify!($ty));
        };
        
        // ident: 标识符
        (ident: $id:ident) => {
            let $id = "identifier";
            println!("   {} = {}", stringify!($id), $id);
        };
    }
    
    println!("🔧 参数类型演示：");
    
    // item
    demo_types! {
        item: fn hello() {
            println!("      Hello from macro-generated function!");
        }
    }
    hello();
    
    // block
    demo_types! {
        block: {
            let x = 5;
            println!("      x = {}", x);
        }
    }
    
    // expr
    demo_types!(expr: 2 + 3);
    
    // ty
    demo_types!(ty: Vec<i32>);
    
    // ident
    demo_types!(ident: my_var);
    
    println!("\n💡 参数类型说明：");
    println!("   • item: 函数、结构体、枚举等语法项");
    println!("   • block: 代码块 {{...}}");
    println!("   • expr: 表达式");
    println!("   • ty: 类型");
    println!("   • ident: 标识符");
    println!("   • pat: 模式");
    println!("   • stmt: 语句");
}

// ============================================================================
// 4. 重复模式宏
// ============================================================================

fn repetition_patterns_demo() {
    println!("\n📍 4. 重复模式宏");
    println!("{}", "-".repeat(40));
    
    // 创建向量的宏
    macro_rules! vec_of {
        ($elem:expr; $n:expr) => {
            {
                let mut v = Vec::new();
                for _ in 0..$n {
                    v.push($elem);
                }
                v
            }
        };
        
        ($($elem:expr),* $(,)?) => {
            {
                let mut v = Vec::new();
                $(v.push($elem);)*
                v
            }
        };
    }
    
    println!("🔧 重复模式演示：");
    
    // 重复相同元素
    let v1 = vec_of![0; 5];
    println!("   vec_of![0; 5] = {:?}", v1);
    
    // 不同元素
    let v2 = vec_of![1, 2, 3, 4, 5];
    println!("   vec_of![1, 2, 3, 4, 5] = {:?}", v2);
    
    // 可选的尾随逗号
    let v3 = vec_of![1, 2, 3,];
    println!("   vec_of![1, 2, 3,] = {:?}", v3);
    
    println!("\n💡 重复模式语法：");
    println!("   • $(...),* 表示重复，用逗号分隔");
    println!("   • $(,)? 表示可选的尾随逗号");
    println!("   • $(...);* 表示重复，用分号分隔");
}

// ============================================================================
// 5. HashMap 创建宏
// ============================================================================

fn hashmap_creation_demo() {
    println!("\n📍 5. HashMap 创建宏");
    println!("{}", "-".repeat(40));
    
    macro_rules! hashmap {
        () => {
            HashMap::new()
        };
        
        ($($key:expr => $value:expr),* $(,)?) => {
            {
                let mut map = HashMap::new();
                $(map.insert($key, $value);)*
                map
            }
        };
    }
    
    println!("🔧 HashMap 宏演示：");
    
    let empty_map: HashMap<&str, i32> = hashmap!();
    println!("   空映射: {:?}", empty_map);
    
    let map = hashmap! {
        "one" => 1,
        "two" => 2,
        "three" => 3,
    };
    println!("   映射: {:?}", map);
    
    println!("\n💡 实用价值：");
    println!("   • 提供类似字面量的语法");
    println!("   • 减少样板代码");
    println!("   • 编译时生成高效代码");
}

// ============================================================================
// 6. 条件编译宏
// ============================================================================

fn conditional_compilation_demo() {
    println!("\n📍 6. 条件编译宏");
    println!("{}", "-".repeat(40));
    
    macro_rules! cfg_if {
        (if #[cfg($meta:meta)] { $($it:item)* }) => {
            $(#[cfg($meta)] $it)*
        };
        
        (
            if #[cfg($i_met:meta)] { $($i_it:item)* }
            else { $($e_it:item)* }
        ) => {
            #[cfg($i_met)]
            $($i_it)*
            
            #[cfg(not($i_met))]
            $($e_it)*
        };
    }
    
    cfg_if! {
        if #[cfg(unix)] {
            fn platform_specific() {
                println!("   这是 Unix 平台特定的代码");
            }
        } else {
            fn platform_specific() {
                println!("   这是其他平台的代码");
            }
        }
    }
    
    println!("🔧 条件编译演示：");
    platform_specific();
    
    println!("\n💡 条件编译用途：");
    println!("   • 平台特定代码");
    println!("   • 功能开关");
    println!("   • 调试 vs 发布版本");
    println!("   • 不同依赖版本适配");
}

// ============================================================================
// 7. 断言宏
// ============================================================================

fn assertion_macros_demo() {
    println!("\n📍 7. 断言宏");
    println!("{}", "-".repeat(40));
    
    macro_rules! assert_eq_verbose {
        ($left:expr, $right:expr) => {
            {
                let left_val = $left;
                let right_val = $right;
                if left_val != right_val {
                    panic!(
                        "断言失败: {} != {}\n  左值: {:?}\n  右值: {:?}\n  位置: {}:{}:{}",
                        stringify!($left),
                        stringify!($right),
                        left_val,
                        right_val,
                        file!(),
                        line!(),
                        column!()
                    );
                } else {
                    println!("   断言成功: {} == {}", stringify!($left), stringify!($right));
                }
            }
        };
    }
    
    println!("🔧 断言宏演示：");
    let a = 5;
    let b = 5;
    assert_eq_verbose!(a, b);
    
    let x = 10;
    let y = 10; // 确保相等以避免 panic
    assert_eq_verbose!(x, y);
    
    println!("\n💡 断言宏优势：");
    println!("   • 提供详细的错误信息");
    println!("   • 包含源码位置");
    println!("   • 显示实际值和期望值");
    println!("   • 可以自定义错误格式");
}

// ============================================================================
// 8. 日志宏
// ============================================================================

fn logging_macros_demo() {
    println!("\n📍 8. 日志宏");
    println!("{}", "-".repeat(40));
    
    macro_rules! log {
        (ERROR, $($arg:tt)*) => {
            eprintln!("   [ERROR] {}", format!($($arg)*));
        };
        
        (WARN, $($arg:tt)*) => {
            println!("   [WARN]  {}", format!($($arg)*));
        };
        
        (INFO, $($arg:tt)*) => {
            println!("   [INFO]  {}", format!($($arg)*));
        };
        
        (DEBUG, $($arg:tt)*) => {
            #[cfg(debug_assertions)]
            println!("   [DEBUG] {}", format!($($arg)*));
        };
    }
    
    println!("🔧 日志宏演示：");
    log!(INFO, "应用程序启动");
    log!(WARN, "这是一个警告: {}", "内存使用较高");
    log!(ERROR, "发生错误: {}", "文件未找到");
    log!(DEBUG, "调试信息: 变量 x = {}", 42);
    
    println!("\n💡 日志宏特点：");
    println!("   • 统一的日志格式");
    println!("   • 不同级别的处理");
    println!("   • DEBUG 只在调试模式编译");
    println!("   • 支持格式化参数");
}

// ============================================================================
// 9. 计算宏
// ============================================================================

fn calculation_macros_demo() {
    println!("\n📍 9. 计算宏");
    println!("{}", "-".repeat(40));
    
    macro_rules! add {
        ($a:expr, $b:expr) => {
            $a + $b
        };
    }
    
    macro_rules! multiply {
        ($a:expr, $b:expr) => {
            $a * $b
        };
    }
    
    println!("🔧 计算宏演示：");
    println!("   add!(2, 3) = {}", add!(2, 3));
    println!("   multiply!(5, 6) = {}", multiply!(5, 6));
    println!("   标准运算：10 - 4 = {}", 10 - 4);
    println!("   标准运算：15 / 3 = {}", 15 / 3);
    
    // 复杂表达式
    println!("   复杂运算：(2 + 3) * 4 = {}", (2 + 3) * 4);
    
    println!("\n💡 计算宏用途：");
    println!("   • 编译时计算");
    println!("   • 类型安全的运算");
    println!("   • 自定义运算符");
    println!("   • 表达式简化");
}

// ============================================================================
// 10. 结构体生成宏
// ============================================================================

fn struct_generation_demo() {
    println!("\n📍 10. 结构体生成宏");
    println!("{}", "-".repeat(40));
    
    macro_rules! create_struct {
        (
            $vis:vis struct $name:ident {
                $($field_name:ident: $field_type:ty),* $(,)?
            }
        ) => {
            $vis struct $name {
                $($field_name: $field_type,)*
            }
            
            impl $name {
                pub fn new($($field_name: $field_type),*) -> Self {
                    Self {
                        $($field_name),*
                    }
                }
                
                // 为每个字段生成 getter
                $(
                    paste::paste! {
                        pub fn [<get_ $field_name>](&self) -> &$field_type {
                            &self.$field_name
                        }
                    }
                )*
            }
        };
    }
    
    // 简化版本（不使用 paste）
    macro_rules! simple_struct {
        (
            $vis:vis struct $name:ident {
                $($field_name:ident: $field_type:ty),* $(,)?
            }
        ) => {
            $vis struct $name {
                $($field_name: $field_type,)*
            }
            
            impl $name {
                pub fn new($($field_name: $field_type),*) -> Self {
                    Self {
                        $($field_name),*
                    }
                }
            }
        };
    }
    
    simple_struct! {
        pub struct Person {
            name: String,
            age: u32,
            email: String,
        }
    }
    
    println!("🔧 结构体生成宏演示：");
    let person = Person::new(
        "Alice".to_string(),
        30,
        "alice@example.com".to_string(),
    );
    
    println!("   姓名: {}", person.name);
    println!("   年龄: {}", person.age);
    println!("   邮箱: {}", person.email);
    
    println!("\n💡 结构体宏优势：");
    println!("   • 自动生成构造器");
    println!("   • 减少重复代码");
    println!("   • 一致的接口");
    println!("   • 可扩展的模式");
}

// ============================================================================
// 11. 测试宏
// ============================================================================

fn test_macros_demo() {
    println!("\n📍 11. 测试宏");
    println!("{}", "-".repeat(40));
    
    macro_rules! test_case {
        ($name:ident: $input:expr => $expected:expr) => {
            fn $name() -> bool {
                let result = $input;
                if result == $expected {
                    println!("   ✅ {}: {} == {}", stringify!($name), result, $expected);
                    true
                } else {
                    println!("   ❌ {}: {} != {}", stringify!($name), result, $expected);
                    false
                }
            }
        };
        
        ($name:ident: $input:expr => $expected:expr, $description:expr) => {
            fn $name() -> bool {
                let result = $input;
                if result == $expected {
                    println!("   ✅ {}: {}", $description, stringify!($input));
                    true
                } else {
                    println!("   ❌ {}: {} 预期 {} 但得到 {}", $description, stringify!($input), $expected, result);
                    false
                }
            }
        };
    }
    
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
    
    test_case!(test_add_positive: add(2, 3) => 5);
    test_case!(test_add_negative: add(-1, -1) => -2);
    test_case!(test_multiply: multiply(4, 5) => 20, "乘法测试");
    
    println!("🔧 测试宏演示：");
    test_add_positive();
    test_add_negative();
    test_multiply();
    
    println!("\n💡 测试宏价值：");
    println!("   • 简化测试编写");
    println!("   • 统一测试格式");
    println!("   • 自动生成测试名");
    println!("   • 详细的失败信息");
}

// ============================================================================
// 12. 配置宏
// ============================================================================

fn config_macros_demo() {
    println!("\n📍 12. 配置宏");
    println!("{}", "-".repeat(40));
    
    macro_rules! config {
        (
            $(
                $key:ident = $value:expr
            ),* $(,)?
        ) => {
            pub struct Config {
                $(pub $key: String),*
            }
            
            impl Config {
                pub fn new() -> Self {
                    Self {
                        $($key: $value.to_string()),*
                    }
                }
                
                pub fn from_env() -> Self {
                    Self {
                        $(
                            $key: std::env::var(stringify!($key))
                                .unwrap_or_else(|_| $value.to_string())
                        ),*
                    }
                }
                
                pub fn print(&self) {
                    println!("   配置:");
                    $(println!("     {}: {}", stringify!($key), self.$key);)*
                }
            }
        };
    }
    
    config! {
        database_url = "localhost:5432",
        api_key = "default_key",
        log_level = "info",
        max_connections = "100",
    }
    
    println!("🔧 配置宏演示：");
    let config = Config::new();
    config.print();
    
    println!("\n💡 配置宏优势：");
    println!("   • 声明式配置定义");
    println!("   • 自动环境变量支持");
    println!("   • 类型安全");
    println!("   • 默认值处理");
}