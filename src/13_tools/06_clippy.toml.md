# clippy.toml 配置文件详解

clippy.toml 是 Clippy（Rust 语言的 linter 工具）的配置文件，用于自定义特定 lint 的行为，例如调整阈值、允许某些模式或设置最小支持 Rust 版本。它采用 TOML 格式，支持简单的 `key = value` 映射，并可放置在项目根目录或父目录中。Clippy 会按以下顺序搜索配置文件（clippy.toml 或 .clippy.toml）：

1. CLIPPY_CONF_DIR 环境变量指定的目录。
2. CARGO_MANIFEST_DIR 环境变量指定的目录。
3. 当前目录。

配置文件是实验性的，可能在未来被弃用或更改。 对于列表类型配置（如 disallowed-names），可以使用特殊值 `".."` 来扩展默认值而非替换。 示例：
```
disallowed-names = ["bar", ".."]  # 扩展默认 ["foo", "baz", "quux"] 为 ["bar", "foo", "baz", "quux"]
```

Clippy 配置可通过命令行标志、代码属性（如 #[allow(clippy::lint)]）或 Cargo.toml 覆盖，但 clippy.toml 适用于全局 lint 调整。 要禁用 lint 链接消息，设置环境变量 CLIPPY_DISABLE_DOCS_LINKS。

以下是所有主要配置选项的详解，基于官方文档和社区资源组织为类别（如通用、路径、测试相关、MSRV 等）。每个选项包括：

- **描述**：选项的功能说明。
- **类型**：数据类型（e.g., bool, usize, Vec<String>）。
- **默认值**：默认设置。
- **可能值**：允许的值或范围。
- **示例**：配置示例和效果（假设简单代码片段）。
- **注意**：额外说明、受影响 lint 或版本特定。

选项列表综合自官方配置指南，按类别分组。完整列表可在 Clippy 文档中查看。

### 通用选项
这些选项影响整体 lint 行为。

## absolute-paths-allowed-crates
**描述**：允许使用绝对路径的 crate 列表。  
**类型**：Vec<String>  
**默认值**：[]  
**可能值**：crate 名称数组  
**示例**：  
clippy.toml: `absolute-paths-allowed-crates = ["std", "crate1"]`  
代码: use ::std::path::Path; // 允许  
**注意**：用于避免 absolute_paths lint 误报，受影响 lint：absolute_paths。

## absolute-paths-max-segments
**描述**：路径最大段数阈值，超出则 lint。  
**类型**：usize  
**默认值**：2  
**可能值**：任何正整数  
**示例**：  
clippy.toml: `absolute-paths-max-segments = 3`  
代码: use a::b::c::D; // 如果 >3 段，警告  
**注意**：控制路径 lint 严格度。

## accept-comment-above-attributes
**描述**：接受 unsafe 块的安全注释放置在属性上方。  
**类型**：bool  
**默认值**：true  
**可能值**：true, false  
**示例**：  
clippy.toml: `accept-comment-above-attributes = false`  
代码: // safety #[attr] unsafe {} // 如果 false，警告  
**注意**：影响 unsafe_code lint。

## accept-comment-above-statement
**描述**：接受 unsafe 块的安全注释放置在语句上方。  
**类型**：bool  
**默认值**：true  
**可能值**：true, false  
**示例**：  
clippy.toml: `accept-comment-above-statement = false`  
代码: // safety let x = unsafe {}; // 如果 false，警告  
**注意**：用于 unsafe 文档规范。

## allow-comparison-to-zero
**描述**：允许模运算结果与零比较。  
**类型**：bool  
**默认值**：true  
**可能值**：true, false  
**示例**：  
clippy.toml: `allow-comparison-to-zero = false`  
代码: if x % 2 == 0 {} // 如果 false，警告  
**注意**：防止常见数学错误。

## arithmetic-side-effects-allowed
**描述**：忽略算术侧效果的类型列表。  
**类型**：Vec<String>  
**默认值**：[]  
**可能值**：类型名称数组  
**示例**：  
clippy.toml: `arithmetic-side-effects-allowed = ["MyType"]`  
代码: let x = MyType + 1; // 忽略溢出 lint  
**注意**：用于自定义类型，受影响 lint：arithmetic_side_effects。

## avoid-breaking-exported-api
**描述**：避免建议打破导出 API 的变更。  
**类型**：bool  
**默认值**：true  
**可能值**：true, false  
**示例**：  
clippy.toml: `avoid-breaking-exported-api = false`  
代码: pub fn foo() {} // 允许建议更改 public API  
**注意**：库开发中保护 API 稳定性。

## disallowed-names
**描述**：禁止使用的名称列表。  
**类型**：Vec<String>  
**默认值**：["foo", "baz", "quux"]  
**可能值**：名称数组，可用 ".." 扩展默认  
**示例**：  
clippy.toml: `disallowed-names = ["toto", "tata"]`  
代码: let toto = 1; // 警告禁止名称  
**注意**：用于团队命名规范，受影响 lint：disallowed_names。

### 测试相关选项
这些选项控制测试代码中的 lint。

## allow-dbg-in-tests
**描述**：允许 dbg! 在测试函数或 #[cfg(test)] 中使用。  
**类型**：bool  
**默认值**：false  
**可能值**：true, false  
**示例**：  
clippy.toml: `allow-dbg-in-tests = true`  
代码: #[test] fn test() { dbg!(x); } // 允许  
**注意**：调试测试时有用。

## allow-expect-in-tests
**描述**：允许 expect 在测试函数或 #[cfg(test)] 中使用。  
**类型**：bool  
**默认值**：false  
**可能值**：true, false  
**示例**：  
clippy.toml: `allow-expect-in-tests = true`  
代码: #[test] fn test() { expect("msg"); } // 允许  
**注意**：测试中处理 panic。

## allow-indexing-slicing-in-tests
**描述**：允许索引/切片在测试函数或 #[cfg(test)] 中使用。  
**类型**：bool  
**默认值**：false  
**可能值**：true, false  
**示例**：  
clippy.toml: `allow-indexing-slicing-in-tests = true`  
代码: #[test] fn test() { vec[0]; } // 允许  
**注意**：测试中忽略边界检查 lint。

## allow-panic-in-tests
**描述**：允许 panic 在测试函数或 #[cfg(test)] 中使用。  
**类型**：bool  
**默认值**：false  
**可能值**：true, false  
**示例**：  
clippy.toml: `allow-panic-in-tests = true`  
代码: #[test] fn test() { panic!(); } // 允许  
**注意**：测试失败处理。

### MSRV 选项
这些选项与最小支持 Rust 版本相关。

## msrv
**描述**：指定项目的最小支持 Rust 版本，影响版本相关 lint。  
**类型**：String  
**默认值**：None  
**可能值**：Rust 版本字符串，如 "1.70.0"（补丁可选）  
**示例**：  
clippy.toml: `msrv = "1.64.0"`  
代码: 使用 post-1.64 特性 // lint 考虑版本  
**注意**：可通过 #![clippy::msrv = "1.64.0"] 属性设置（需启用 unstable custom_inner_attributes）。与 Cargo.toml rust-version 同步。

### 路径和名称选项
这些选项控制路径和名称 lint。

## allow-exact-repetitions
**描述**：允许项名称与包含模块名称相同。  
**类型**：bool  
**默认值**：true  
**可能值**：true, false  
**示例**：  
clippy.toml: `allow-exact-repetitions = false`  
代码: mod foo { pub struct Foo; } // 如果 false，警告  
**注意**：命名风格。

## allow-one-hash-in-raw-strings
**描述**：允许 r#""# 当 r"" 可用时使用。  
**类型**：bool  
**默认值**：false  
**可能值**：true, false  
**示例**：  
clippy.toml: `allow-one-hash-in-raw-strings = true`  
代码: r#""#; // 允许  
**注意**：字符串字面量优化。

### 格式和表达式选项
这些选项影响表达式和格式 lint。

## allow-mixed-uninlined-format-args
**描述**：允许混合未内联格式参数，如 format!("{} {}", a, foo.bar)。  
**类型**：bool  
**默认值**：true  
**可能值**：true, false  
**示例**：  
clippy.toml: `allow-mixed-uninlined-format-args = false`  
代码: format!("{} {}", a, b.c); // 如果 false，警告  
**注意**：格式字符串风格。

## allow-expect-in-consts
**描述**：允许 expect 在编译时评估的代码中使用。  
**类型**：bool  
**默认值**：true  
**可能值**：true, false  
**示例**：  
clippy.toml: `allow-expect-in-consts = false`  
代码: const X: () = expect("msg"); // 如果 false，警告  
**注意**：常量表达式 lint。

### 阈值和限制选项
这些选项设置 lint 触发阈值。

## enum-variant-name-threshold
**描述**：枚举变体名称 lint 触发阈值。  
**类型**：usize  
**默认值**：3（推测）  
**可能值**：任何正整数  
**示例**：  
clippy.toml: `enum-variant-name-threshold = 5`  
代码: enum E { A, B, C, D, E } // >5 变体时 lint  
**注意**：控制 enum_variant_names lint。

## too-large-for-stack
**描述**：栈上类型大小阈值。  
**类型**：usize  
**默认值**：4096  
**可能值**：任何正整数  
**示例**：  
clippy.toml: `too-large-for-stack = 2048`  
代码: let x: [u8; 3000] = ...; // >2048 警告  
**注意**：性能优化，受影响 lint：large_stack_arrays。

## cognitive-complexity-threshold
**描述**：认知复杂度阈值。  
**类型**：usize  
**默认值**：25（推测）  
**可能值**：任何正整数  
**示例**：  
clippy.toml: `cognitive-complexity-threshold = 30`  
代码: 嵌套 if/loop // >30 警告  
**注意**：受影响 lint：cognitive_complexity。

## max-suggested-slice-size
**描述**：切片大小建议阈值。  
**类型**：usize  
**默认值**：8  
**可能值**：任何正整数  
**示例**：  
clippy.toml: `max-suggested-slice-size = 16`  
代码: &vec[0..10] // >16 建议 Vec  
**注意**：内存优化。

### 文档和注释选项
这些选项控制文档 lint。

## doc-valid-idents
**描述**：文档中有效标识符列表。  
**类型**：Vec<String>  
**默认值**：默认列表  
**可能值**：标识符数组，可用 ".." 扩展  
**示例**：  
clippy.toml: `doc-valid-idents = ["MyTerm", ".."]`  
代码: /// MyTerm // 允许自定义术语  
**注意**：避免 doc_valid_idents lint 误报。

### 其他选项
这些选项杂类 lint 配置。

## disallow-unstable-features
**描述**：禁止不稳定特性。  
**类型**：bool  
**默认值**：false  
**可能值**：true, false  
**示例**：  
clippy.toml: `disallow-unstable-features = true`  
代码: #![feature(foo)] // 警告  
**注意**：用于稳定代码基地。

## 结语
这些选项覆盖了 clippy.toml 的主要配置。通过组合使用，你可以定制 Clippy 以适应项目需求。更多选项和更新请参考官方文档。 如需启用特定 lint 组（如 pedantic），使用 --warn clippy::pedantic 或在 Cargo.toml 配置。 对于大型项目，考虑将 clippy.toml 置于 workspace 根目录以共享配置。