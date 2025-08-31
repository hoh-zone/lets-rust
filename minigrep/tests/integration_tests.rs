//! # Minigrep 集成测试
//!
//! 这个文件演示了 Rust 中的集成测试，包括：
//! - 测试模块组织
//! - 断言宏的使用
//! - 错误情况测试
//! - 性能测试
//! - 文件 I/O 测试

use std::fs;
use std::env;
use std::process::Command;
use minigrep::*;

// ============================================================================
// 1. 基本功能测试
// ============================================================================

#[test]
fn test_config_build_success() {
    // 测试正确的配置构建
    let args = vec![
        "minigrep".to_string(),
        "rust".to_string(),
        "poem.txt".to_string(),
    ];
    
    let config = Config::build(args.into_iter());
    assert!(config.is_ok());
    
    let config = config.unwrap();
    assert_eq!(config.query, "rust");
    assert_eq!(config.file_path, "poem.txt");
    assert_eq!(config.search_mode, SearchMode::CaseSensitive);
}

#[test]
fn test_config_build_insufficient_args() {
    // 测试参数不足的情况
    let args = vec!["minigrep".to_string()];
    
    let config = Config::build(args.into_iter());
    assert!(config.is_err());
    
    if let Err(err) = config {
        assert!(err.contains("Not enough arguments") || err.contains("参数不足"));
    }
}

#[test]
fn test_config_with_options() {
    // 测试带选项的配置
    let args = vec![
        "minigrep".to_string(),
        "rust".to_string(),
        "poem.txt".to_string(),
        "--line-numbers".to_string(),
        "--max=50".to_string(),
    ];
    
    let config = Config::build(args.into_iter());
    assert!(config.is_ok());
    
    let config = config.unwrap();
    assert!(config.show_line_numbers);
    assert_eq!(config.max_results, Some(50));
}

// ============================================================================
// 2. 搜索功能测试
// ============================================================================

#[test]
fn test_search_case_sensitive() {
    // 测试区分大小写搜索
    let query = "duct";
    let contents = "\nRust:\nsafe, fast, productive.\nPick three.\nDuct tape.";
    
    let results = search_case_sensitive(query, contents);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].line, "safe, fast, productive.");
    assert_eq!(results[0].line_number, 3);
}

#[test]
fn test_search_case_insensitive() {
    // 测试不区分大小写搜索
    let query = "rUsT";
    let contents = "\nRust:\nsafe, fast, productive.\nPick three.\nTrust me.";
    
    let results = search_case_insensitive(query, contents);
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].line, "Rust:");
    assert_eq!(results[1].line, "Trust me.");
}

#[test]
fn test_search_exact_match() {
    // 测试精确匹配
    let query = "Rust";
    let contents = "\nRust\nRust is great\nrust\nRUST";
    
    let results = search_exact(query, contents);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].line.trim(), "Rust");
}

#[test]
fn test_search_no_results() {
    // 测试无匹配结果
    let query = "monomorphization";
    let contents = "\nRust:\nsafe, fast, productive.\nPick three.";
    
    let results = search_case_sensitive(query, contents);
    assert_eq!(results.len(), 0);
}

#[test]
fn test_search_multiple_matches_same_line() {
    // 测试同一行多个匹配
    let query = "the";
    let contents = "the quick brown fox jumps over the lazy dog";
    
    let results = search_case_sensitive(query, contents);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].match_positions.len(), 2);
}

// ============================================================================
// 3. 文件操作测试
// ============================================================================

#[test]
fn test_run_with_valid_file() {
    // 创建临时测试文件
    let test_content = "Hello world\nRust is awesome\nProgramming is fun";
    let test_file = "test_poem.txt";
    
    fs::write(test_file, test_content).expect("无法创建测试文件");
    
    // 测试运行
    let config = Config {
        query: "Rust".to_string(),
        file_path: test_file.to_string(),
        search_mode: SearchMode::CaseSensitive,
        show_line_numbers: false,
        max_results: None,
    };
    
    let results = run(&config);
    assert!(results.is_ok());
    
    let results = results.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].line, "Rust is awesome");
    
    // 清理测试文件
    fs::remove_file(test_file).expect("无法删除测试文件");
}

#[test]
fn test_run_with_nonexistent_file() {
    // 测试不存在的文件
    let config = Config {
        query: "test".to_string(),
        file_path: "nonexistent_file.txt".to_string(),
        search_mode: SearchMode::CaseSensitive,
        show_line_numbers: false,
        max_results: None,
    };
    
    let result = run(&config);
    assert!(result.is_err());
}

// ============================================================================
// 4. 统计功能测试
// ============================================================================

#[test]
fn test_search_stats_analysis() {
    // 测试搜索统计分析
    let contents = "Hello world\nRust is great\nProgramming with Rust\nHello again";
    let results = vec![
        SearchResult {
            line: "Rust is great".to_string(),
            line_number: 2,
            match_positions: vec![(0, 4)],
        },
        SearchResult {
            line: "Programming with Rust".to_string(),
            line_number: 1,
            match_positions: vec![(17, 21)],
        },
    ];
    
    let stats = SearchStats::analyze(contents, &results);
    
    assert_eq!(stats.total_lines, 4);
    assert_eq!(stats.matched_lines, 2);
    assert_eq!(stats.total_matches, 2);
    
    // 测试词频统计
    let common_words = stats.most_common_words(3);
    assert!(!common_words.is_empty());
}

// ============================================================================
// 5. 错误处理测试
// ============================================================================

#[test]
fn test_search_result_formatting() {
    // 测试搜索结果格式化
    let result = SearchResult {
        line: "Hello, Rust world!".to_string(),
        line_number: 1,
        match_positions: vec![(7, 11)],
    };
    
    // 测试不显示行号
    let formatted = result.format_output(false);
    assert_eq!(formatted, "Hello, Rust world!");
    
    // 测试显示行号
    let formatted = result.format_output(true);
    assert_eq!(formatted, "1: Hello, Rust world!");
}

#[test]
fn test_search_mode_debug() {
    // 测试搜索模式的调试输出
    let mode1 = SearchMode::CaseSensitive;
    let mode2 = SearchMode::CaseInsensitive;
    let mode3 = SearchMode::Exact;
    let mode4 = SearchMode::Regex("test.*pattern".to_string());
    
    // 确保所有模式都能正确格式化
    assert!(!format!("{:?}", mode1).is_empty());
    assert!(!format!("{:?}", mode2).is_empty());
    assert!(!format!("{:?}", mode3).is_empty());
    assert!(!format!("{:?}", mode4).is_empty());
}

// ============================================================================
// 6. 性能测试
// ============================================================================

#[test]
fn test_large_file_performance() {
    // 创建大文件进行性能测试
    let mut large_content = String::new();
    for i in 0..1000 {
        large_content.push_str(&format!("Line {} with some Rust content\n", i));
    }
    
    let start = std::time::Instant::now();
    let results = search_case_sensitive("Rust", &large_content);
    let duration = start.elapsed();
    
    // 验证结果正确性
    assert_eq!(results.len(), 1000);
    
    // 性能检查：应该在合理时间内完成（这里设为1秒，实际应该更快）
    assert!(duration.as_secs() < 1, "搜索耗时过长: {:?}", duration);
}

// ============================================================================
// 7. 边界条件测试
// ============================================================================

#[test]
fn test_empty_file() {
    // 测试空文件
    let results = search_case_sensitive("anything", "");
    assert_eq!(results.len(), 0);
}

#[test]
fn test_empty_query() {
    // 测试空查询
    let contents = "Some content here";
    let results = search_case_sensitive("", contents);
    // 空查询应该返回0个结果
    assert_eq!(results.len(), 0);
}

#[test]
fn test_single_character_query() {
    // 测试单字符查询
    let contents = "a\nb\nc\na";
    let results = search_case_sensitive("a", contents);
    assert_eq!(results.len(), 2);
}

#[test]
fn test_unicode_content() {
    // 测试 Unicode 内容
    let contents = "Hello 世界\nRust 编程\n🦀 螃蟹";
    let results = search_case_sensitive("世界", contents);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].line, "Hello 世界");
}

// ============================================================================
// 8. 命令行集成测试
// ============================================================================

#[test]
fn test_command_line_integration() {
    // 创建测试文件
    let test_content = "Hello world\nRust is great\nProgramming is fun";
    let test_file = "integration_test.txt";
    
    fs::write(test_file, test_content).expect("无法创建测试文件");
    
    // 测试命令行调用
    let output = Command::new("cargo")
        .args(&["run", "--", "Rust", test_file])
        .output();
    
    match output {
        Ok(output) => {
            // 检查程序是否成功运行
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("Rust is great") || output.status.success());
        }
        Err(_) => {
            // 如果无法运行 cargo，跳过此测试
            println!("跳过命令行集成测试：无法运行 cargo");
        }
    }
    
    // 清理测试文件
    let _ = fs::remove_file(test_file);
}

// ============================================================================
// 9. 辅助测试函数
// ============================================================================

/// 创建临时测试文件的辅助函数
fn create_temp_file(name: &str, content: &str) -> String {
    let temp_path = format!("temp_{}", name);
    fs::write(&temp_path, content).expect("无法创建临时文件");
    temp_path
}

/// 清理临时文件的辅助函数
fn cleanup_temp_file(path: &str) {
    let _ = fs::remove_file(path);
}

// ============================================================================
// 10. 模块级测试设置和清理
// ============================================================================

/// 测试模块的设置函数
#[cfg(test)]
fn setup() {
    // 设置测试环境
    env::set_var("RUST_LOG", "debug");
}

/// 测试模块的清理函数
#[cfg(test)]
fn teardown() {
    // 清理测试环境
    // 这里可以添加清理逻辑
}

// 使用 std::sync::Once 确保设置只运行一次
use std::sync::Once;
static INIT: Once = Once::new();

/// 确保测试环境只初始化一次
fn ensure_test_setup() {
    INIT.call_once(|| {
        setup();
    });
}

// 在每个需要设置的测试中调用 ensure_test_setup()