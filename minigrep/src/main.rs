//! # Minigrep 主程序
//!
//! 这是 minigrep 项目的主入口点，演示了 Rust 中的各种语法特性：
//! - 变量和常量
//! - 数据类型
//! - 函数定义和调用
//! - 流程控制
//! - 错误处理
//! - 模块使用
//! - 命令行参数处理

// 导入标准库模块 - 演示模块系统
use std::env;           // 环境变量和命令行参数
use std::process;       // 进程控制
use std::io::{self, Write}; // I/O 操作
use std::time::Instant; // 时间测量

// 导入我们的库 - 演示本地模块导入
use minigrep::{
    Config, SearchResult, SearchStats, SearchMode,
    run, example_function
};

// ============================================================================
// 1. 常量定义 - 演示常量和静态变量
// ============================================================================

/// 程序版本 - 编译时常量
const VERSION: &str = "1.0.0";

/// 程序名称
const PROGRAM_NAME: &str = "minigrep";

/// 默认最大结果数
const DEFAULT_MAX_RESULTS: usize = 100;

/// 静态变量 - 程序运行时的全局状态
static mut SEARCH_COUNT: usize = 0;

// ============================================================================
// 2. 主函数 - 程序入口点
// ============================================================================

/// 主函数 - 程序的入口点
/// 
/// 演示了：
/// - 命令行参数处理
/// - 错误处理和传播
/// - 流程控制
/// - 变量绑定和可变性
fn main() {
    // ========================================================================
    // 变量声明和数据类型演示
    // ========================================================================
    
    // 不可变变量 - Rust 默认不可变
    let program_start = Instant::now();
    
    // 可变变量 - 使用 mut 关键字
    let mut exit_code = 0;
    
    // 基本数据类型
    let search_performed: bool = false;
    let _unused_integer: i32 = 42;
    let _unused_float: f64 = 3.14159;
    let _unused_char: char = '🦀';
    
    // 字符串类型
    let greeting = "欢迎使用 Minigrep!"; // &str - 字符串切片
    let _owned_string = String::from("这是一个拥有的字符串"); // String - 拥有的字符串
    
    // 数组和元组
    let _numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let _coordinates: (f64, f64) = (10.0, 20.0);
    
    // 打印欢迎信息
    println!("{}", greeting);
    println!("版本: {}", VERSION);
    println!("{}", "=".repeat(50));
    
    // ========================================================================
    // 命令行参数处理
    // ========================================================================
    
    // 收集命令行参数 - 演示迭代器和集合
    let args: Vec<String> = env::args().collect();
    
    // 检查参数数量 - 演示流程控制
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }
    
    // 检查帮助选项 - 演示模式匹配
    match args.get(1).map(|s| s.as_str()) {
        Some("--help") | Some("-h") => {
            print_help();
            return;
        }
        Some("--version") | Some("-v") => {
            println!("{} {}", PROGRAM_NAME, VERSION);
            return;
        }
        Some("--example") => {
            run_examples();
            return;
        }
        _ => {} // 继续正常执行
    }
    
    // ========================================================================
    // 配置解析和验证
    // ========================================================================
    
    // 构建配置 - 演示 Result 类型和错误处理
    let config = match Config::build(env::args()) {
        Ok(config) => {
            println!("✓ 配置解析成功");
            println!("  查询: '{}'", config.query);
            println!("  文件: '{}'", config.file_path);
            println!("  模式: {:?}", config.search_mode);
            config
        }
        Err(err) => {
            eprintln!("❌ 配置错误: {}", err);
            print_usage();
            process::exit(1);
        }
    };
    
    // ========================================================================
    // 执行搜索
    // ========================================================================
    
    println!("{}", "-".repeat(50));
    println!("🔍 开始搜索...");
    
    // 记录搜索开始时间
    let search_start = Instant::now();
    
    // 执行搜索 - 演示错误处理和匹配
    let search_results = match run(&config) {
        Ok(results) => {
            println!("✓ 搜索完成");
            results
        }
        Err(err) => {
            eprintln!("❌ 搜索失败: {}", err);
            exit_code = 1;
            process::exit(exit_code);
        }
    };
    
    // 计算搜索耗时
    let search_duration = search_start.elapsed();
    
    // 更新搜索计数（演示 unsafe 代码）
    unsafe {
        SEARCH_COUNT += 1;
    }
    
    // ========================================================================
    // 结果处理和显示
    // ========================================================================
    
    // 显示搜索结果 - 演示条件语句和循环
    if search_results.is_empty() {
        println!("❌ 没有找到匹配的结果");
        exit_code = 1;
    } else {
        println!("✓ 找到 {} 个匹配结果:", search_results.len());
        println!("{}", "=".repeat(50));
        
        // 遍历并显示结果 - 演示迭代器和方法链
        for (index, result) in search_results.iter().enumerate() {
            // 格式化输出
            let formatted = result.format_output(config.show_line_numbers);
            
            // 高亮显示匹配部分（简化版）
            let highlighted = highlight_matches(&formatted, &config.query, &config.search_mode);
            
            println!("[{}] {}", index + 1, highlighted);
            
            // 显示匹配位置信息
            if !result.match_positions.is_empty() {
                print!("    匹配位置: ");
                for (start, end) in &result.match_positions {
                    print!("{}:{} ", start, end);
                }
                println!();
            }
        }
    }
    
    // ========================================================================
    // 统计信息和性能分析
    // ========================================================================
    
    println!("{}", "=".repeat(50));
    
    // 读取文件内容进行统计分析
    if let Ok(contents) = std::fs::read_to_string(&config.file_path) {
        let stats = SearchStats::analyze(&contents, &search_results);
        display_statistics(&stats, search_duration);
    }
    
    // 显示程序运行信息
    let total_duration = program_start.elapsed();
    println!("⏱️  总运行时间: {:?}", total_duration);
    println!("🔍 搜索耗时: {:?}", search_duration);
    
    unsafe {
        println!("📊 本次会话搜索次数: {}", SEARCH_COUNT);
    }
    
    // 根据结果设置退出码
    process::exit(exit_code);
}

// ============================================================================
// 3. 辅助函数 - 演示函数定义、参数和返回值
// ============================================================================

/// 打印程序使用说明
/// 
/// 演示了：
/// - 函数定义
/// - 字符串格式化
/// - 标准输出
fn print_usage() {
    println!("使用方法:");
    println!("  {} <查询字符串> <文件路径> [选项]", PROGRAM_NAME);
    println!();
    println!("选项:");
    println!("  --line-numbers, -n    显示行号");
    println!("  --max=<数量>          限制最大结果数量");
    println!("  --help, -h            显示帮助信息");
    println!("  --version, -v         显示版本信息");
    println!("  --example             运行示例");
    println!();
    println!("环境变量:");
    println!("  IGNORE_CASE=1         启用不区分大小写搜索");
    println!();
    println!("示例:");
    println!("  {} rust poem.txt", PROGRAM_NAME);
    println!("  {} --line-numbers hello story.txt", PROGRAM_NAME);
    println!("  IGNORE_CASE=1 {} RUST poem.txt", PROGRAM_NAME);
}

/// 打印详细帮助信息
fn print_help() {
    println!("{} - Rust 文本搜索工具", PROGRAM_NAME);
    println!("版本: {}", VERSION);
    println!();
    print_usage();
    println!();
    println!("这个程序演示了以下 Rust 概念:");
    println!("• 变量和数据类型");
    println!("• 函数和闭包");
    println!("• 流程控制 (if/else, match, 循环)");
    println!("• 所有权和借用");
    println!("• 结构体和枚举");
    println!("• 模式匹配");
    println!("• 错误处理 (Result, Option)");
    println!("• 集合操作 (Vec, HashMap)");
    println!("• 迭代器和闭包");
    println!("• Trait 和泛型");
    println!("• 生命周期");
    println!("• 模块系统");
    println!("• 测试");
}

/// 运行示例代码
/// 
/// 演示了：
/// - 函数调用
/// - 字符串操作
/// - 集合创建和操作
fn run_examples() {
    println!("🚀 运行 Rust 语法示例...");
    println!("{}", "=".repeat(50));
    
    // ========================================================================
    // 1. 变量和数据类型示例
    // ========================================================================
    
    println!("📝 1. 变量和数据类型:");
    
    // 基本类型
    let integer: i32 = 42;
    let float: f64 = 3.14159;
    let boolean: bool = true;
    let character: char = '🦀';
    
    println!("  整数: {}", integer);
    println!("  浮点数: {:.2}", float);
    println!("  布尔值: {}", boolean);
    println!("  字符: {}", character);
    
    // 复合类型
    let tuple: (i32, f64, char) = (42, 3.14, '🦀');
    let array: [i32; 3] = [1, 2, 3];
    
    println!("  元组: {:?}", tuple);
    println!("  数组: {:?}", array);
    
    // 字符串类型
    let string_slice: &str = "Hello, Rust!";
    let owned_string: String = String::from("Hello, World!");
    
    println!("  字符串切片: {}", string_slice);
    println!("  拥有的字符串: {}", owned_string);
    
    println!();
    
    // ========================================================================
    // 2. 函数示例
    // ========================================================================
    
    println!("🔧 2. 函数示例:");
    
    // 调用库中的示例函数
    let result = example_function("Rust");
    println!("  函数调用结果: {}", result);
    
    // 调用本地函数
    let sum = add_numbers(10, 20);
    println!("  加法结果: {}", sum);
    
    // 高阶函数示例
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("  映射结果: {:?}", doubled);
    
    println!();
    
    // ========================================================================
    // 3. 流程控制示例
    // ========================================================================
    
    println!("🔄 3. 流程控制示例:");
    
    // if/else 语句
    let number = 42;
    if number > 0 {
        println!("  {} 是正数", number);
    } else if number < 0 {
        println!("  {} 是负数", number);
    } else {
        println!("  {} 是零", number);
    }
    
    // match 表达式
    let grade = 'A';
    let description = match grade {
        'A' => "优秀",
        'B' => "良好",
        'C' => "及格",
        'D' | 'F' => "不及格",
        _ => "未知等级",
    };
    println!("  等级 {} 对应: {}", grade, description);
    
    // 循环示例
    print!("  计数循环: ");
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();
    
    println!();
    
    // ========================================================================
    // 4. 所有权和借用示例
    // ========================================================================
    
    println!("🔒 4. 所有权和借用示例:");
    
    // 所有权转移
    let s1 = String::from("Hello");
    let s2 = s1; // s1 的所有权转移给 s2
    // println!("{}", s1); // 这行会编译错误
    println!("  所有权转移后: {}", s2);
    
    // 借用
    let s3 = String::from("World");
    let len = calculate_length(&s3); // 借用 s3
    println!("  字符串 '{}' 的长度是 {}", s3, len); // s3 仍然有效
    
    println!();
    
    // ========================================================================
    // 5. 结构体和枚举示例
    // ========================================================================
    
    println!("📦 5. 结构体和枚举示例:");
    
    // 结构体
    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64,
    }
    
    let point = Point { x: 3.0, y: 4.0 };
    println!("  点坐标: {:?}", point);
    
    // 枚举
    #[derive(Debug)]
    enum Color {
        Red,
        Green,
        Blue,
        RGB(u8, u8, u8),
    }
    
    let color1 = Color::Red;
    let color2 = Color::RGB(255, 128, 0);
    println!("  颜色1: {:?}", color1);
    println!("  颜色2: {:?}", color2);
    
    println!();
    
    // ========================================================================
    // 6. 错误处理示例
    // ========================================================================
    
    println!("⚠️  6. 错误处理示例:");
    
    // Result 类型
    let result = divide(10.0, 2.0);
    match result {
        Ok(value) => println!("  除法结果: {}", value),
        Err(err) => println!("  除法错误: {}", err),
    }
    
    // Option 类型
    let numbers = vec![1, 2, 3, 4, 5];
    match numbers.get(2) {
        Some(value) => println!("  索引2的值: {}", value),
        None => println!("  索引2不存在"),
    }
    
    println!();
    
    println!("✅ 示例运行完成!");
}

/// 简单的加法函数 - 演示函数参数和返回值
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b // 表达式作为返回值
}

/// 计算字符串长度 - 演示借用
fn calculate_length(s: &String) -> usize {
    s.len()
}

/// 除法函数 - 演示错误处理
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("除数不能为零".to_string())
    } else {
        Ok(a / b)
    }
}

/// 高亮显示匹配的文本 - 演示字符串操作和模式匹配
/// 
/// # 参数
/// * `text` - 要处理的文本
/// * `query` - 搜索查询
/// * `mode` - 搜索模式
/// 
/// # 返回值
/// 高亮后的文本字符串
fn highlight_matches(text: &str, query: &str, mode: &SearchMode) -> String {
    // 简化的高亮实现
    match mode {
        SearchMode::CaseSensitive => {
            text.replace(query, &format!("[{}]", query))
        }
        SearchMode::CaseInsensitive => {
            // 简化处理：直接替换
            let lower_text = text.to_lowercase();
            let lower_query = query.to_lowercase();
            if lower_text.contains(&lower_query) {
                format!("[匹配] {}", text)
            } else {
                text.to_string()
            }
        }
        SearchMode::Exact => {
            if text.trim() == query {
                format!("[精确匹配] {}", text)
            } else {
                text.to_string()
            }
        }
        SearchMode::Regex(pattern) => {
            format!("[正则: {}] {}", pattern, text)
        }
    }
}

/// 显示统计信息 - 演示结构体使用和格式化输出
/// 
/// # 参数
/// * `stats` - 搜索统计信息
/// * `duration` - 搜索耗时
fn display_statistics(stats: &SearchStats, duration: std::time::Duration) {
    println!("📊 搜索统计:");
    println!("  总行数: {}", stats.total_lines);
    println!("  匹配行数: {}", stats.matched_lines);
    println!("  总匹配数: {}", stats.total_matches);
    
    // 计算匹配率
    let match_rate = if stats.total_lines > 0 {
        (stats.matched_lines as f64 / stats.total_lines as f64) * 100.0
    } else {
        0.0
    };
    println!("  匹配率: {:.1}%", match_rate);
    
    // 显示最常见的词汇
    let common_words = stats.most_common_words(5);
    if !common_words.is_empty() {
        println!("  最常见词汇:");
        for (word, count) in common_words {
            println!("    '{}': {} 次", word, count);
        }
    }
    
    // 性能信息
    println!("  搜索速度: {:.2} 行/秒", 
        stats.total_lines as f64 / duration.as_secs_f64());
}

// ============================================================================
// 4. 条件编译示例 - 演示特性标志
// ============================================================================

#[cfg(feature = "color")]
fn colorize_text(text: &str, _color: &str) -> String {
    // 在实际实现中，这里会添加 ANSI 颜色代码
    format!("\x1b[32m{}\x1b[0m", text) // 绿色文本
}

#[cfg(not(feature = "color"))]
fn colorize_text(text: &str, _color: &str) -> String {
    text.to_string()
}

// ============================================================================
// 5. 测试模块 - 演示集成测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_numbers() {
        assert_eq!(add_numbers(2, 3), 5);
        assert_eq!(add_numbers(-1, 1), 0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0).unwrap(), 5.0);
        assert!(divide(10.0, 0.0).is_err());
    }

    #[test]
    fn test_calculate_length() {
        let s = String::from("hello");
        assert_eq!(calculate_length(&s), 5);
    }

    #[test]
    fn test_highlight_matches() {
        let result = highlight_matches("hello world", "hello", &SearchMode::CaseSensitive);
        assert_eq!(result, "[hello] world");
    }
}

// ============================================================================
// 6. 文档注释和示例
// ============================================================================

/// 演示文档测试的函数
/// 
/// # 参数
/// * `input` - 输入字符串
/// 
/// # 返回值
/// 处理后的字符串
/// 
/// # 示例
/// 
/// ```
/// // 这个测试会在 `cargo test` 时运行
/// let result = minigrep::main(); // 注意：这只是示例，实际不会这样调用
/// // assert_eq!(result, expected);
/// ```
/// 
/// # 错误
/// 
/// 当输入为空时可能返回错误。
/// 
/// # 安全性
/// 
/// 这个函数是安全的，不会导致内存安全问题。
pub fn process_input(input: &str) -> Result<String, &'static str> {
    if input.is_empty() {
        Err("输入不能为空")
    } else {
        Ok(format!("处理后的: {}", input))
    }
}
