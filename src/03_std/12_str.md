# Rust std::str 模块教程

Rust 的 `std::str` 模块是标准库中处理字符串切片（&str 和 str）的核心组成部分，提供实用方法用于字符串的解析、搜索、分割、替换、转换和验证等操作。它抽象了 UTF-8 编码的复杂性，确保字符串操作的安全性和效率。`std::str` 的函数和方法多返回 `Result` 或 `Option`，以显式处理无效 UTF-8、边界错误或解析失败。模块强调借用：&str 是借用视图，String 是拥有版本（在 std::string）。`std::str` 与 `std::string`（String 类型）、`std::fmt`（格式化）、`std::io`（读取字符串）和 `std::path`（路径字符串）紧密集成，支持 Unicode 和多字节字符处理。

## 1. std::str 简介

- **导入和基本结构**：通常用 `use std::str;` 或直接方法如 `str::from_utf8`。模块分为函数、trait 和常量三大类。
    - **函数概述**：
        - 创建/验证：`from_utf8`、`from_utf8_unchecked`（unsafe）、`from_boxed_utf8_unchecked`。
        - 解析：`parse`（转为其他类型如 i32/f64）、`from_str` trait。
        - 操作：`split`/`rsplit`（分割）、`replace`/`replacen`（替换）、`trim`/`trim_start`/`trim_end`（去除空白）、`lines`（行迭代）、`chars`/`bytes`（字符/字节迭代）。
        - 搜索：`contains`、`starts_with`/`ends_with`、`find`/`rfind`、`match_indices`。
        - 转换：`to_lowercase`/`to_uppercase`、`to_ascii_lowercase`/`to_ascii_uppercase`、`escape_default`/`escape_debug`/`escape_unicode`。
    - **Trait**：`FromStr`（parse trait）、`Pattern`（split 参数 trait）。
    - **常量**：无直接，但相关如 char::MAX（Unicode 范围）。
- **设计哲学**：`std::str` 是零成本抽象，UTF-8 验证在边界检查；错误通过 Utf8Error/ParseError 处理，避免 panic。字符串不可变，修改用 String。支持 Unicode：char 是 4 字节，迭代 chars() 处理多字节。
- **跨平台注意**：路径字符串用 OsStr（非 UTF-8），str 假设 UTF-8；Windows 文件名可能非 UTF-8，用 from_utf8_lossy。
- **性能基础**：大多数操作 O(n) 于长度，常量时间检查（如 is_empty）；避免频繁 from_utf8 于无效数据。
- **常见用例**：文本解析、字符串清洗、搜索/替换、日志处理、配置读取、Unicode 规范化。
- **扩展概念**：与 std::string::String 集成（as_str() 借用）；与 std::vec::Vec<u8> 转换字节；错误链用 anyhow；与 regex crate 高级模式；Unicode 规范化用 unicode-normalization crate；多线程安全（&str immutable）。

## 2. 创建和验证字符串：from_utf8 等

`std::str` 提供从字节创建 &str 的安全方法。

### 示例：基本 from_utf8（验证字节）
```rust
use std::str;

fn main() {
    let bytes = b"hello";
    match str::from_utf8(bytes) {
        Ok(s) => println!("字符串: {}", s),  // "hello"
        Err(e) => println!("错误: {}", e),
    }
}
```

- **解释**：`from_utf8` 检查 UTF-8，有效返回 &str。性能：O(n) 扫描。

### 示例：from_utf8_unchecked（unsafe 扩展）
```rust
use std::str;

fn main() {
    let bytes = b"valid utf8";
    let s = unsafe { str::from_utf8_unchecked(bytes) };
    println!("unchecked: {}", s);

    // 扩展：无效字节崩溃（debug 模式检查）
    // let invalid = b"\xFF";
    // unsafe { str::from_utf8_unchecked(invalid) };  // 未定义行为
}
```

- **解释**：`from_utf8_unchecked` 假设有效，无检查。unsafe 用于已验证字节。陷阱：无效 UTF-8 未定义行为（panic 或垃圾）。扩展：release 无检查，debug 有运行时断言。

### 示例：from_boxed_utf8_unchecked（Box<[u8]> 扩展）
```rust
use std::str;

fn main() {
    let boxed: Box<[u8]> = Box::from([104, 101, 108, 108, 111]);  // "hello"
    let s = unsafe { str::from_boxed_utf8_unchecked(boxed) };
    println!("从 Box: {}", s);
}
```

- **解释**：`from_boxed_utf8_unchecked` 转为 Box<str>。扩展：用于 Vec<u8> 到 String，避免克隆。

### 示例：Utf8Error 处理（无效字节扩展）
```rust
use std::str;

fn main() {
    let invalid = &[0xE2, 0x82, 0xAC, 0xFF];  // 欧元 + 无效
    if let Err(e) = str::from_utf8(invalid) {
        println!("错误位置: {}", e.valid_up_to());  // 3 (欧元 3 字节有效)
        println!("错误长度: {}", e.error_len().unwrap_or(0));  // 1
    }
}
```

- **解释**：`Utf8Error::valid_up_to` 返回有效字节索引；`error_len` 返回无效长度。用于部分解析。性能：错误时 O(1) 访问。

## 3. 解析字符串：parse 和 FromStr

`parse` 是泛型方法，转为实现 FromStr 的类型。

### 示例：基本 parse（数字扩展）
```rust
use std::str::FromStr;

fn main() {
    let num: i32 = "42".parse().unwrap();
    println!("解析: {}", num);

    let float = f64::from_str("3.14").unwrap();
    println!("from_str: {}", float);
}
```

- **解释**：`parse` 调用 FromStr::from_str。unwrap 处理 Err(ParseIntError 等)。

### 示例：自定义 FromStr（扩展类型）
```rust
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug)]
struct Point(i32, i32);

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')').split(',').collect();
        let x = coords[0].trim().parse()?;
        let y = coords[1].trim().parse()?;
        Ok(Point(x, y))
    }
}

fn main() {
    let p: Point = "(3, 4)".parse().unwrap();
    println!("点: {:?}", p);
}
```

- **解释**：自定义解析逻辑。`type Err` 定义错误。扩展：用 nom crate 复杂解析。

### 示例：ParseError 处理（无效输入扩展）
```rust
fn main() {
    match "abc".parse::<i32>() {
        Ok(n) => println!("数字: {}", n),
        Err(e) => {
            println!("种类: {:?}", e.kind());  // InvalidDigit
            println!("描述: {}", e);
        }
    }
}
```

- **解释**：`ParseIntError::kind` 分类（Empty/InvalidDigit/Overflow/Underflow）。用于用户友好错误。

## 4. 字符串操作：Split、Replace、Trim

操作返回迭代器或新字符串。

### 示例：Split 和 Rsplit（分割扩展）
```rust
fn main() {
    let s = "a,b,c,d";
    let parts: Vec<&str> = s.split(',').collect();
    println!("split: {:?}", parts);  // ["a", "b", "c", "d"]

    let rparts: Vec<&str> = s.rsplit(',').collect();
    println!("rsplit: {:?}", rparts);  // ["d", "c", "b", "a"]
}
```

- **解释**：`split` 从左；`rsplit` 从右。收集到 Vec。性能：懒惰迭代。

### 示例：SplitN 和 SplitOnce（有限分割扩展）
```rust
fn main() {
    let s = "key:value:extra";
    let parts: Vec<&str> = s.splitn(2, ':').collect();
    println!("splitn: {:?}", parts);  // ["key", "value:extra"]

    if let Some((key, value)) = s.split_once(':') {
        println!("key: {}, value: {}", key, value);  // key: key, value: value:extra
    }
}
```

- **解释**：`splitn` 限 n 次；`split_once` 单次返回 Option<(&str, &str)>。扩展：用 split_terminator 忽略 trailing 分隔。

### 示例：Replace 和 Replacen（替换扩展）
```rust
fn main() {
    let s = "foo foo foo";
    let replaced = s.replace("foo", "bar");
    println!("replace: {}", replaced);  // "bar bar bar"

    let replacen = s.replacen("foo", "bar", 2);
    println!("replacen: {}", replacen);  // "bar bar foo"
}
```

- **解释**：`replace` 全部；`replacen` 限 n 次。返回 String（分配）。性能：O(n) 扫描。陷阱：模式重叠未处理，用 regex 复杂。

### 示例：Trim 变体（去除空白扩展）
```rust
fn main() {
    let s = "  hello   ";
    println!("trim: '{}'", s.trim());  // 'hello'

    println!("trim_start: '{}'", s.trim_start());  // 'hello   '
    println!("trim_end: '{}'", s.trim_end());      // '  hello'

    // 扩展：自定义谓词
    let trimmed = s.trim_matches(|c: char| c.is_whitespace() || c == '*');
    println!("自定义 trim: '{}'", trimmed);
}
```

- **解释**：`trim` 移除前后空白。`trim_matches` 用闭包自定义。扩展：用 strip_prefix/strip_suffix 移除特定前/后缀。

## 5. 迭代字符串：Chars、Bytes、Lines

迭代返回 char/u8 或 &str。

### 示例：Chars 和 Bytes（字符/字节扩展）
```rust
fn main() {
    let s = "café";
    let chars: Vec<char> = s.chars().collect();
    println!("chars: {:?}", chars);  // ['c', 'a', 'f', 'é'] (é 是 2 字节)

    let bytes: Vec<u8> = s.bytes().collect();
    println!("bytes: {:?}", bytes);  // [99, 97, 102, 195, 169]

    // 扩展：计数和反转
    println!("char 数: {}", s.chars().count());  // 4
    let rev_chars: String = s.chars().rev().collect();
    println!("反转: {}", rev_chars);  // éfac
}
```

- **解释**：`chars` 解码 UTF-8，返回 char（1-4 字节）。`bytes` 返回 u8。性能：O(n) 解码。陷阱：索引字节不等于 char（如 s.as_bytes()[3] 是 é 的部分）。

### 示例：Lines 和 LineWrap（行迭代扩展）
```rust
fn main() {
    let s = "line1\nline2\r\nline3";
    let lines: Vec<&str> = s.lines().collect();
    println!("lines: {:?}", lines);  // ["line1", "line2", "line3"]

    // 扩展：处理 trailing \n
    let with_trailing = "trailing\n";
    println!("最后行: {:?}", with_trailing.lines().last());  // Some("trailing")
}
```

- **解释**：`lines` 处理 \n 和 \r\n，去除换行。扩展：用 split('\n') 保留 trailing 空行。

## 6. 搜索和匹配：Contains、Find、MatchIndices

搜索返回 bool、Option<usize> 或迭代器。

### 示例：Contains 和 StartsWith（检查扩展）
```rust
fn main() {
    let s = "hello world";
    println!("包含 'world'？{}", s.contains("world"));  // true
    println!("以 'hello' 开头？{}", s.starts_with("hello"));  // true
    println!("以 char 开头？{}", s.starts_with(char::is_whitespace));  // false (闭包)

    // 扩展：忽略大小写
    println!("忽略案包含 'WORLD'？{}", s.to_lowercase().contains("world"));  // true
}
```

- **解释**：`contains` 检查子串/char/闭包。`starts_with`/`ends_with` 类似。性能：O(n) 最坏。

### 示例：Find 和 Rfind（位置扩展）
```rust
fn main() {
    let s = "hello hello";
    println!("第一个 'ello'：{:?}", s.find("ello"));  // Some(1)
    println!("最后一个 'ello'：{:?}", s.rfind("ello"));  // Some(7)

    // 扩展：用闭包
    let vowel_pos = s.find(|c: char| "aeiou".contains(c));
    println!("第一个元音: {:?}", vowel_pos);  // Some(1) 'e'
}
```

- **解释**：`find` 返回第一个匹配索引。`rfind` 从右。闭包支持自定义。

### 示例：MatchIndices（多匹配扩展）
```rust
fn main() {
    let s = "ababa";
    let matches: Vec<(usize, &str)> = s.match_indices("aba").collect();
    println!("匹配: {:?}", matches);  // [(0, "aba"), (2, "aba")]
}
```

- **解释**：`match_indices` 返回 (index, match) 迭代器。扩展：用 matches 检查存在。

## 7. 转换字符串：Case、Escape

转换返回 ToString Iterator 或 String。

### 示例：Case 转换（大小写扩展）
```rust
fn main() {
    let s = "Hello, World!";
    let lower: String = s.to_lowercase();
    println!("小写: {}", lower);  // "hello, world!"

    let upper: String = s.to_uppercase();
    println!("大写: {}", upper);  // "HELLO, WORLD!"

    // 扩展：ASCII 变体
    let ascii_lower = s.to_ascii_lowercase();
    println!("ASCII 小写: {}", ascii_lower);  // "hello, world!"
}
```

- **解释**：`to_lowercase` 处理 Unicode（如 ß -> ss）。`to_ascii_lowercase` 只 ASCII，快。性能：O(n) 分配。

### 示例：Escape 序列（转义扩展）
```rust
fn main() {
    let s = "hello\tworld\n";
    println!("default: {}", s.escape_default());  // hello\tworld\n
    println!("debug: {}", s.escape_debug());     // "hello\tworld\n"
    println!("unicode: {}", s.escape_unicode()); // \u{68}\u{65}...
}
```

- **解释**：`escape_default` 转义非打印。`escape_debug` 调试友好。`escape_unicode` 全 Unicode 转义。扩展：用 for char in s.chars() 自定义。

## 8. 最佳实践和常见陷阱

- **UTF-8 最佳实践**：总是 from_utf8 检查；用 chars() 处理 Unicode，避免字节索引。
- **性能陷阱**：频繁 parse 慢，用预验证；长链 split.collect 分配，用 iter 懒惰。
- **错误最佳实践**：分类 ParseError kind；重试 InvalidDigit 用 trim 清洗。
- **安全性**：sanitize 输入避免注入；Unicode 等价用 normalization。
- **跨平台扩展**：路径用 OsStr，非 str；测试多字节 OS 文件名。
- **测试扩展**：用 assert_eq 测试 parse；fuzz 测试无效 UTF-8 用 proptest。
- **资源管理**：str 无资源，但与 String 结合时管理所有权。
- **常见错误扩展**：
    - Utf8Error：valid_up_to 恢复部分，用 &bytes[..valid]。
    - Parse 溢出：用 checked_parse 或 BigInt crate。
    - Unicode 长度：len() 是字节，chars().count() 是 char 数。
    - Case 变体：to_lowercase 可能变长（如德文 ß）。

## 9. 练习建议

1. 编写 CSV 解析器：用 split(',') 和 trim 清洗，parse 到 Vec<f64>。
2. 实现自定义 splitter：用 Pattern trait 支持 regex-like 分割。
3. 创建 Unicode 规范化：用 to_nfc (外部 crate) + to_lowercase。
4. 处理大字符串：用 lines() map parse，fold 累积统计。
5. 基准测试：比较 split vs regex::split 大文本时间，用 Instant。
6. 与 io 集成：用 BufReader lines() map trim.collect 读取文件。
7. 错误模拟：用 mock 无效字符串测试 from_utf8 重试逻辑。

