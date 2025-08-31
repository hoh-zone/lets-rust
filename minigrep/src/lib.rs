//! # Minigrep 库
//!
//! 这是一个用于学习 Rust 语法的文本搜索工具库。
//! 本文件演示了 Rust 中的各种核心概念：
//! - 结构体和方法
//! - 枚举和模式匹配
//! - 错误处理
//! - 所有权和借用
//! - 生命周期
//! - Trait 和泛型
//! - 模块系统
//! - 集合操作

use std::error::Error;
use std::fs;
use std::env;
use std::collections::HashMap;

// ============================================================================
// 1. 枚举和模式匹配 - 定义搜索模式
// ============================================================================

/// 搜索模式枚举 - 演示枚举的定义和使用
#[derive(Debug, Clone, PartialEq)]
pub enum SearchMode {
    /// 区分大小写搜索
    CaseSensitive,
    /// 不区分大小写搜索
    CaseInsensitive,
    /// 正则表达式搜索（演示带数据的枚举变体）
    Regex(String),
    /// 精确匹配
    Exact,
}

// 为枚举实现方法
impl SearchMode {
    /// 从环境变量创建搜索模式
    pub fn from_env() -> Self {
        // 演示环境变量处理和模式匹配
        match env::var("IGNORE_CASE") {
            Ok(val) if !val.is_empty() => SearchMode::CaseInsensitive,
            _ => SearchMode::CaseSensitive,
        }
    }
    
    /// 检查是否为大小写敏感模式
    pub fn is_case_sensitive(&self) -> bool {
        matches!(self, SearchMode::CaseSensitive | SearchMode::Exact)
    }
}

// ============================================================================
// 2. 结构体 - 配置结构体
// ============================================================================

/// 配置结构体 - 演示结构体定义、生命周期和所有权
#[derive(Debug, Clone)]
pub struct Config {
    /// 搜索查询字符串
    pub query: String,
    /// 文件路径
    pub file_path: String,
    /// 搜索模式
    pub search_mode: SearchMode,
    /// 是否显示行号
    pub show_line_numbers: bool,
    /// 最大结果数量（演示 Option 类型）
    pub max_results: Option<usize>,
}

// ============================================================================
// 3. 实现块 - 为结构体添加方法
// ============================================================================

impl Config {
    /// 构造函数 - 演示错误处理和 Result 类型
    /// 
    /// # 参数
    /// * `args` - 命令行参数的迭代器
    /// 
    /// # 返回值
    /// * `Result<Config, &'static str>` - 成功返回配置，失败返回错误信息
    /// 
    /// # 示例
    /// ```
    /// use minigrep::Config;
    /// let args = vec!["program".to_string(), "query".to_string(), "file.txt".to_string()];
    /// let config = Config::build(args.into_iter()).unwrap();
    /// ```
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        // 跳过程序名
        args.next();

        // 获取查询字符串 - 演示 Option 和错误处理
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Not enough arguments: missing query string"),
        };

        // 获取文件路径
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Not enough arguments: missing file path"),
        };

        // 从环境变量获取搜索模式
        let search_mode = SearchMode::from_env();

        // 解析其他选项
        let mut show_line_numbers = false;
        let mut max_results = None;
        
        // 处理剩余参数
        for arg in args {
            match arg.as_str() {
                "--line-numbers" | "-n" => show_line_numbers = true,
                arg if arg.starts_with("--max=") => {
                    if let Some(num_str) = arg.strip_prefix("--max=") {
                        max_results = num_str.parse().ok();
                    }
                }
                _ => {} // 忽略未知参数
            }
        }

        Ok(Config {
            query,
            file_path,
            search_mode,
            show_line_numbers,
            max_results,
        })
    }
    
    /// 验证配置 - 演示方法链和错误处理
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.query.is_empty() {
            return Err(ConfigError::EmptyQuery);
        }
        
        if self.file_path.is_empty() {
            return Err(ConfigError::EmptyFilePath);
        }
        
        // 检查文件是否存在
        if !std::path::Path::new(&self.file_path).exists() {
            return Err(ConfigError::FileNotFound(self.file_path.clone()));
        }
        
        Ok(())
    }
}

// ============================================================================
// 4. 自定义错误类型 - 演示错误处理和 Trait 实现
// ============================================================================

/// 配置错误枚举 - 演示自定义错误类型
#[derive(Debug, Clone)]
pub enum ConfigError {
    EmptyQuery,
    EmptyFilePath,
    FileNotFound(String),
    InvalidMaxResults,
}

// 为错误类型实现 Display trait
impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::EmptyQuery => write!(f, "查询字符串不能为空"),
            ConfigError::EmptyFilePath => write!(f, "文件路径不能为空"),
            ConfigError::FileNotFound(path) => write!(f, "文件未找到: {}", path),
            ConfigError::InvalidMaxResults => write!(f, "无效的最大结果数量"),
        }
    }
}

// 实现 Error trait
impl Error for ConfigError {}

// ============================================================================
// 5. 搜索结果结构体 - 演示生命周期
// ============================================================================

/// 搜索结果 - 演示生命周期参数
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// 匹配的行内容
    pub line: String,
    /// 行号（从1开始）
    pub line_number: usize,
    /// 匹配的位置（字符索引）
    pub match_positions: Vec<(usize, usize)>,
}

impl SearchResult {
    /// 创建新的搜索结果
    pub fn new(line: String, line_number: usize, match_positions: Vec<(usize, usize)>) -> Self {
        Self {
            line,
            line_number,
            match_positions,
        }
    }
    
    /// 格式化输出 - 演示字符串操作
    pub fn format_output(&self, show_line_numbers: bool) -> String {
        if show_line_numbers {
            format!("{}: {}", self.line_number, self.line)
        } else {
            self.line.clone()
        }
    }
}

// ============================================================================
// 6. 泛型和 Trait - 搜索策略
// ============================================================================

/// 搜索策略 trait - 演示 trait 定义
pub trait SearchStrategy {
    /// 执行搜索
    fn search(&self, query: &str, contents: &str) -> Vec<SearchResult>;
    
    /// 获取策略名称
    fn name(&self) -> &'static str;
}

/// 区分大小写搜索策略
pub struct CaseSensitiveSearch;

impl SearchStrategy for CaseSensitiveSearch {
    fn search(&self, query: &str, contents: &str) -> Vec<SearchResult> {
        search_case_sensitive(query, contents)
    }
    
    fn name(&self) -> &'static str {
        "区分大小写搜索"
    }
}

/// 不区分大小写搜索策略
pub struct CaseInsensitiveSearch;

impl SearchStrategy for CaseInsensitiveSearch {
    fn search(&self, query: &str, contents: &str) -> Vec<SearchResult> {
        search_case_insensitive(query, contents)
    }
    
    fn name(&self) -> &'static str {
        "不区分大小写搜索"
    }
}

// ============================================================================
// 7. 主要功能函数 - 演示所有权、借用和错误处理
// ============================================================================

/// 运行程序的主要逻辑
/// 
/// # 参数
/// * `config` - 配置对象的引用
/// 
/// # 返回值
/// * `Result<Vec<SearchResult>, Box<dyn Error>>` - 搜索结果或错误
/// 
/// # 错误
/// 当文件读取失败或其他 I/O 错误时返回错误
pub fn run(config: &Config) -> Result<Vec<SearchResult>, Box<dyn Error>> {
    // 验证配置
    config.validate()?;
    
    // 读取文件内容 - 演示错误传播
    let contents = fs::read_to_string(&config.file_path)?;
    
    // 根据搜索模式选择搜索策略 - 演示模式匹配和 trait 对象
    let results = match &config.search_mode {
        SearchMode::CaseSensitive => {
            let strategy = CaseSensitiveSearch;
            strategy.search(&config.query, &contents)
        }
        SearchMode::CaseInsensitive => {
            let strategy = CaseInsensitiveSearch;
            strategy.search(&config.query, &contents)
        }
        SearchMode::Exact => search_exact(&config.query, &contents),
        SearchMode::Regex(pattern) => search_regex(pattern, &contents)?,
    };
    
    // 应用最大结果限制 - 演示 Option 和迭代器
    let limited_results = match config.max_results {
        Some(max) => results.into_iter().take(max).collect(),
        None => results,
    };
    
    Ok(limited_results)
}

// ============================================================================
// 8. 搜索函数实现 - 演示迭代器、闭包和集合操作
// ============================================================================

/// 区分大小写搜索
/// 
/// # 参数
/// * `query` - 搜索查询
/// * `contents` - 文件内容
/// 
/// # 返回值
/// 包含匹配行的向量
/// 
/// # 示例
/// ```
/// use minigrep::search_case_sensitive;
/// let results = search_case_sensitive("rust", "Rust is great\nrust is awesome");
/// assert_eq!(results.len(), 1);
/// ```
pub fn search_case_sensitive(query: &str, contents: &str) -> Vec<SearchResult> {
    contents
        .lines()                    // 迭代器：按行分割
        .enumerate()                // 添加行号
        .filter_map(|(line_num, line)| {  // 过滤和映射
            // 查找所有匹配位置
            let match_positions = find_all_matches(line, query, true);
            if !match_positions.is_empty() {
                Some(SearchResult::new(
                    line.to_string(),
                    line_num + 1,  // 行号从1开始
                    match_positions,
                ))
            } else {
                None
            }
        })
        .collect()                  // 收集结果
}

/// 不区分大小写搜索
pub fn search_case_insensitive(query: &str, contents: &str) -> Vec<SearchResult> {
    let query = query.to_lowercase();  // 转换为小写
    
    contents
        .lines()
        .enumerate()
        .filter_map(|(line_num, line)| {
            let line_lower = line.to_lowercase();
            let match_positions = find_all_matches(&line_lower, &query, false);
            if !match_positions.is_empty() {
                Some(SearchResult::new(
                    line.to_string(),
                    line_num + 1,
                    match_positions,
                ))
            } else {
                None
            }
        })
        .collect()
}

/// 精确匹配搜索
pub fn search_exact(query: &str, contents: &str) -> Vec<SearchResult> {
    contents
        .lines()
        .enumerate()
        .filter_map(|(line_num, line)| {
            if line.trim() == query {
                Some(SearchResult::new(
                    line.to_string(),
                    line_num + 1,
                    vec![(0, line.len())],
                ))
            } else {
                None
            }
        })
        .collect()
}

/// 正则表达式搜索（简化版）
pub fn search_regex(pattern: &str, contents: &str) -> Result<Vec<SearchResult>, Box<dyn Error>> {
    // 这里使用简单的字符串匹配模拟正则表达式
    // 在实际项目中，你会使用 regex crate
    if pattern.contains('*') {
        // 简单的通配符支持
        let prefix = pattern.trim_end_matches('*');
        Ok(contents
            .lines()
            .enumerate()
            .filter_map(|(line_num, line)| {
                if line.starts_with(prefix) {
                    Some(SearchResult::new(
                        line.to_string(),
                        line_num + 1,
                        vec![(0, prefix.len())],
                    ))
                } else {
                    None
                }
            })
            .collect())
    } else {
        // 回退到普通搜索
        Ok(search_case_sensitive(pattern, contents))
    }
}

// ============================================================================
// 9. 辅助函数 - 演示字符串操作和算法
// ============================================================================

/// 查找字符串中所有匹配的位置
fn find_all_matches(text: &str, pattern: &str, case_sensitive: bool) -> Vec<(usize, usize)> {
    let mut matches = Vec::new();
    
    if pattern.is_empty() {
        return matches;
    }
    
    let search_text = if case_sensitive { text.to_string() } else { text.to_lowercase() };
    let search_pattern = if case_sensitive { pattern.to_string() } else { pattern.to_lowercase() };
    
    let mut start = 0;
    while let Some(pos) = search_text[start..].find(&search_pattern) {
        let actual_pos = start + pos;
        let end_pos = actual_pos + search_pattern.chars().count();
        matches.push((actual_pos, end_pos));
        
        // 安全地移动到下一个字符位置
        if let Some((next_start, _)) = search_text[actual_pos..].char_indices().nth(1) {
            start = actual_pos + next_start;
        } else {
            break;
        }
    }
    
    matches
}

// ============================================================================
// 10. 统计和分析功能 - 演示集合操作和高级特性
// ============================================================================

/// 搜索统计信息
#[derive(Debug, Default)]
pub struct SearchStats {
    pub total_lines: usize,
    pub matched_lines: usize,
    pub total_matches: usize,
    pub word_frequency: HashMap<String, usize>,
}

impl SearchStats {
    /// 分析搜索结果并生成统计信息
    pub fn analyze(contents: &str, results: &[SearchResult]) -> Self {
        let total_lines = contents.lines().count();
        let matched_lines = results.len();
        let total_matches = results.iter().map(|r| r.match_positions.len()).sum();
        
        // 词频统计 - 演示 HashMap 和迭代器链
        let word_frequency = contents
            .split_whitespace()
            .map(|word| word.to_lowercase().trim_matches(|c: char| !c.is_alphabetic()).to_string())
            .filter(|word| !word.is_empty())
            .fold(HashMap::new(), |mut acc, word| {
                *acc.entry(word).or_insert(0) += 1;
                acc
            });
        
        SearchStats {
            total_lines,
            matched_lines,
            total_matches,
            word_frequency,
        }
    }
    
    /// 获取最常见的词汇
    pub fn most_common_words(&self, n: usize) -> Vec<(String, usize)> {
        let mut words: Vec<_> = self.word_frequency.iter().collect();
        words.sort_by(|a, b| b.1.cmp(a.1));  // 按频率降序排序
        words.into_iter()
            .take(n)
            .map(|(word, count)| (word.clone(), *count))
            .collect()
    }
}

// ============================================================================
// 11. 测试模块 - 演示单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_case_sensitive() {
        let query = "duct";
        let contents = "\nRust:\nsafe, fast, productive.\nPick three.\nDuct tape.";

        let results = search_case_sensitive(query, contents);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].line, "safe, fast, productive.");
        assert_eq!(results[0].line_number, 3);
    }

    #[test]
    fn test_search_case_insensitive() {
        let query = "rUsT";
        let contents = "\nRust:\nsafe, fast, productive.\nPick three.\nTrust me.";

        let results = search_case_insensitive(query, contents);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].line, "Rust:");
        assert_eq!(results[1].line, "Trust me.");
    }

    #[test]
    fn test_config_build() {
        let args = vec![
            "program".to_string(),
            "query".to_string(),
            "filename".to_string(),
        ];
        
        let config = Config::build(args.into_iter()).unwrap();
        assert_eq!(config.query, "query");
        assert_eq!(config.file_path, "filename");
    }

    #[test]
    fn test_search_mode_from_env() {
        // 测试环境变量
        env::set_var("IGNORE_CASE", "1");
        let mode = SearchMode::from_env();
        assert_eq!(mode, SearchMode::CaseInsensitive);
        
        env::remove_var("IGNORE_CASE");
        let mode = SearchMode::from_env();
        assert_eq!(mode, SearchMode::CaseSensitive);
    }

    #[test]
    fn test_find_all_matches() {
        let text = "hello world hello rust";
        let pattern = "hello";
        let matches = find_all_matches(text, pattern, true);
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0], (0, 5));
        assert_eq!(matches[1], (12, 17));
    }

    #[test]
    fn test_search_stats() {
        let contents = "hello world\nhello rust\nworld peace";
        let results = search_case_sensitive("hello", contents);
        let stats = SearchStats::analyze(contents, &results);
        
        assert_eq!(stats.total_lines, 3);
        assert_eq!(stats.matched_lines, 2);
        assert_eq!(stats.total_matches, 2);
    }
}

// ============================================================================
// 12. 文档测试示例
// ============================================================================

/// 演示文档测试的函数
/// 
/// # 示例
/// 
/// ```
/// use minigrep::example_function;
/// let result = example_function("test");
/// assert_eq!(result, "Hello, test!");
/// ```
pub fn example_function(name: &str) -> String {
    format!("Hello, {}!", name)
}