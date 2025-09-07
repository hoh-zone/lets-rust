# Rustfmt

Rustfmt 是 Rust 编程语言的官方代码格式化工具，用于自动标准化 Rust 源代码的风格，确保一致性和可读性。它基于 Rust 工具链，支持命令行选项和配置文件（rustfmt.toml），并可集成到 Cargo 中。本教程基于官方文档和社区资源，提供超级扩展的指导，分为50个独立教程部分，每个部分聚焦一个关键命令、选项或配置使用场景。每个教程包括：

- **描述**：选项或配置的功能说明。
- **语法**：基本命令格式。
- **示例**：实际命令和预期效果（假设有一个简单的 `main.rs` 文件：`fn main() { println!("Hello"); }`）。
- **高级提示**：扩展用法或注意事项。

这些教程从基础开始，逐步深入，适合初学者到高级用户。安装 Rust 后，通过 `rustup component add rustfmt` 添加组件，然后在终端运行 `rustfmt` 即可开始实验。注意：rustfmt 通常处理 .rs 文件，支持 stdin/stdout。

## 教程1: 获取帮助信息 (--help)
**描述**：显示 rustfmt 的所有命令行选项和简要说明，帮助快速上手。  
**语法**：`rustfmt --help`。  
**示例**：  
`rustfmt --help`  
输出：列出选项如 `--check`、`--config` 等。  
**高级提示**：结合 `--version`：`rustfmt --help --version` 检查版本。

## 教程2: 查看版本信息 (--version)
**描述**：打印当前 rustfmt 版本，便于检查兼容性。  
**语法**：`rustfmt --version`。  
**示例**：  
`rustfmt --version`  
输出：`rustfmt 1.7.0-stable (2024-09-04)`（版本依安装而定）。  
**高级提示**：用于确认与 Rust 版本匹配。

## 教程3: 基本格式化文件
**描述**：默认模式下，格式化指定文件，就地覆盖。  
**语法**：`rustfmt <file.rs>`。  
**示例**：  
`rustfmt main.rs`  
格式化 `main.rs`，添加空格和换行。  
**高级提示**：多文件：`rustfmt src/*.rs`。

## 教程4: 检查模式 (--check)
**描述**：检查代码是否符合格式，而不修改文件。  
**语法**：`rustfmt --check <file.rs>`。  
**示例**：  
`rustfmt --check main.rs`  
如果不一致，返回非零退出码。  
**高级提示**：用于 CI：`rustfmt --check src/**/*.rs`。

## 教程5: 输出到 stdout (--emit=stdout)
**描述**：将格式化结果输出到标准输出，而不覆盖文件。  
**语法**：`rustfmt --emit=stdout <file.rs>`。  
**示例**：  
`rustfmt --emit=stdout main.rs`  
打印格式化代码。  
**高级提示**：重定向：`rustfmt --emit=stdout main.rs > formatted.rs`。

## 教程6: 输出到文件 (--emit=files)
**描述**：格式化并覆盖原文件（默认行为）。  
**语法**：`rustfmt --emit=files <file.rs>`。  
**示例**：  
`rustfmt --emit=files main.rs`  
覆盖 `main.rs`。  
**高级提示**：安全备份：结合 `--backup`。

## 教程7: 输出差异 (--emit=diff)
**描述**：显示格式化前后的差异。  
**语法**：`rustfmt --emit=diff <file.rs>`。  
**示例**：  
`rustfmt --emit=diff main.rs`  
输出 diff 格式。  
**高级提示**：用于审查：`rustfmt --emit=diff --check`。

## 教程8: 指定配置文件 (--config-path)
**描述**：使用自定义配置文件路径。  
**语法**：`rustfmt --config-path <path> <file.rs>`。  
**示例**：  
`rustfmt --config-path custom.toml main.rs`  
应用自定义配置。  
**高级提示**：项目根目录默认搜索 rustfmt.toml。

## 教程9: 内联配置 (--config)
**描述**：命令行覆盖配置选项。  
**语法**：`rustfmt --config <key=value> <file.rs>`。  
**示例**：  
`rustfmt --config indent_style=tab main.rs`  
使用 tab 缩进。  
**高级提示**：多配置：`--config key1=val1,key2=val2`。

## 教程10: 备份文件 (--backup)
**描述**：格式化前备份原文件。  
**语法**：`rustfmt --backup <file.rs>`。  
**示例**：  
`rustfmt --backup main.rs`  
生成 main.rs.orig。  
**高级提示**：结合 `--emit=files` 确保安全。

## 教程11: 跳过子目录 (--skip-children)
**描述**：在递归模式下跳过子目录。  
**语法**：`rustfmt --skip-children <dir>`。  
**示例**：  
`rustfmt --skip-children src/`  
只格式化 src/ 根文件。  
**高级提示**：用于大型项目。

## 教程12: 详细输出 (--verbose)
**描述**：启用详细日志。  
**语法**：`rustfmt --verbose <file.rs>`。  
**示例**：  
`rustfmt -v main.rs`  
显示处理细节。  
**高级提示**：调试配置问题。

## 教程13: 安静模式 (--quiet)
**描述**：抑制非错误输出。  
**语法**：`rustfmt --quiet <file.rs>`。  
**示例**：  
`rustfmt -q main.rs`  
安静格式化。  
**高级提示**：脚本中使用。

## 教程14: 颜色输出 (--color)
**描述**：控制输出颜色。  
**语法**：`rustfmt --color <always|never|auto> <file.rs>`。  
**示例**：  
`rustfmt --color always main.rs`  
始终使用颜色。  
**高级提示**：默认 auto。

## 教程15: 集成 Cargo (cargo fmt)
**描述**：通过 Cargo 调用 rustfmt 格式化项目。  
**语法**：`cargo fmt`。  
**示例**：  
`cargo fmt`  
格式化整个项目。  
**高级提示**：`cargo fmt -- --check` 检查模式。

## 教程16: 配置 indent_style
**描述**：设置缩进风格（tab 或 space）。  
**语法**：`rustfmt --config indent_style=block <file.rs>`。  
**示例**：  
`rustfmt --config indent_style=block main.rs`  
使用块缩进。  
**高级提示**：选项：block/visual，默认 block。

## 教程17: 配置 tab_spaces
**描述**：设置 tab 宽度（空格数）。  
**语法**：`rustfmt --config tab_spaces=4 <file.rs>`。  
**示例**：  
`rustfmt --config tab_spaces=4 main.rs`  
4 个空格缩进。  
**高级提示**：结合 indent_style=space。

## 教程18: 配置 newline_style
**描述**：设置换行符风格。  
**语法**：`rustfmt --config newline_style=unix <file.rs>`。  
**示例**：  
`rustfmt --config newline_style=unix main.rs`  
使用 LF 换行。  
**高级提示**：选项：unix/windows/auto。

## 教程19: 配置 fn_single_line
**描述**：允许单行函数。  
**语法**：`rustfmt --config fn_single_line=true <file.rs>`。  
**示例**：  
`rustfmt --config fn_single_line=true main.rs`  
保持简单 fn 在单行。  
**高级提示**：默认 false。

## 教程20: 配置 struct_lit_single_line
**描述**：允许单行结构体字面量。  
**语法**：`rustfmt --config struct_lit_single_line=true <file.rs>`。  
**示例**：  
`rustfmt --config struct_lit_single_line=true main.rs`  
单行 struct。  
**高级提示**：用于紧凑代码。

## 教程21: 配置 where_single_line
**描述**：允许单行 where 子句。  
**语法**：`rustfmt --config where_single_line=true <file.rs>`。  
**示例**：  
`rustfmt --config where_single_line=true main.rs`  
紧凑 where。  
**高级提示**：默认 false。

## 教程22: 配置 use_small_heuristics
**描述**：使用小型启发式格式化。  
**语法**：`rustfmt --config use_small_heuristics=default <file.rs>`。  
**示例**：  
`rustfmt --config use_small_heuristics=max main.rs`  
最大启发式。  
**高级提示**：选项：off/default/max。

## 教程23: 配置 max_width
**描述**：设置最大行宽。  
**语法**：`rustfmt --config max_width=100 <file.rs>`。  
**示例**：  
`rustfmt --config max_width=100 main.rs`  
100 字符限。  
**高级提示**：默认 100。

## 教程24: 配置 chain_width
**描述**：设置方法链换行宽度。  
**语法**：`rustfmt --config chain_width=60 <file.rs>`。  
**示例**：  
`rustfmt --config chain_width=60 main.rs`  
链式调用换行。  
**高级提示**：小于 max_width。

## 教程25: 配置 single_line_if_else_max_width
**描述**：单行 if-else 最大宽度。  
**语法**：`rustfmt --config single_line_if_else_max_width=50 <file.rs>`。  
**示例**：  
`rustfmt --config single_line_if_else_max_width=50 main.rs`  
紧凑 if。  
**高级提示**：0 禁用。

## 教程26: 配置 array_width
**描述**：数组字面量最大宽度。  
**语法**：`rustfmt --config array_width=60 <file.rs>`。  
**示例**：  
`rustfmt --config array_width=60 main.rs`  
数组换行。  
**高级提示**：默认无限。

## 教程27: 配置 attr_fn_like_width
**描述**：属性函数式宏宽度。  
**语法**：`rustfmt --config attr_fn_like_width=70 <file.rs>`。  
**示例**：  
`rustfmt --config attr_fn_like_width=70 main.rs`  
属性格式。  
**高级提示**：用于 derive。

## 教程28: 配置 struct_lit_width
**描述**：结构体字面量宽度。  
**语法**：`rustfmt --config struct_lit_width=18 <file.rs>`。  
**示例**：  
`rustfmt --config struct_lit_width=18 main.rs`  
struct 换行。  
**高级提示**：默认 18。

## 教程29: 配置 struct_variant_width
**描述**：枚举变体宽度。  
**语法**：`rustfmt --config struct_variant_width=35 <file.rs>`。  
**示例**：  
`rustfmt --config struct_variant_width=35 main.rs`  
enum 格式。  
**高级提示**：默认 35。

## 教程30: 配置 enum_discrim_align_threshold
**描述**：枚举判别式对齐阈值。  
**语法**：`rustfmt --config enum_discrim_align_threshold=30 <file.rs>`。  
**示例**：  
`rustfmt --config enum_discrim_align_threshold=30 main.rs`  
对齐 enum。  
**高级提示**：0 禁用。

## 教程31: 配置 match_block_trailing_comma
**描述**：match 块尾随逗号。  
**语法**：`rustfmt --config match_block_trailing_comma=true <file.rs>`。  
**示例**：  
`rustfmt --config match_block_trailing_comma=true main.rs`  
添加逗号。  
**高级提示**：默认 false。

## 教程32: 配置 force_explicit_abi
**描述**：强制显式 ABI。  
**语法**：`rustfmt --config force_explicit_abi=true <file.rs>`。  
**示例**：  
`rustfmt --config force_explicit_abi=true main.rs`  
添加 extern。  
**高级提示**：FFI 使用。

## 教程33: 配置 force_multiline_blocks
**描述**：强制多行块。  
**语法**：`rustfmt --config force_multiline_blocks=true <file.rs>`。  
**示例**：  
`rustfmt --config force_multiline_blocks=true main.rs`  
展开块。  
**高级提示**：默认 false。

## 教程34: 配置 use_field_init_shorthand
**描述**：使用字段初始化简写。  
**语法**：`rustfmt --config use_field_init_shorthand=true <file.rs>`。  
**示例**：  
`rustfmt --config use_field_init_shorthand=true main.rs`  
简写 struct。  
**高级提示**：默认 false。

## 教程35: 配置 use_try_shorthand
**描述**：使用 try 简写。  
**语法**：`rustfmt --config use_try_shorthand=true <file.rs>`。  
**示例**：  
`rustfmt --config use_try_shorthand=true main.rs`  
转换为 ?。  
**高级提示**：默认 false。

## 教程36: 配置 imports_granularity
**描述**：导入粒度。  
**语法**：`rustfmt --config imports_granularity=module <file.rs>`。  
**示例**：  
`rustfmt --config imports_granularity=module main.rs`  
按模块分组。  
**高级提示**：选项：preserve/one/crate/module/item。

## 教程37: 配置 group_imports
**描述**：分组导入。  
**语法**：`rustfmt --config group_imports=std_external_crate <file.rs>`。  
**示例**：  
`rustfmt --config group_imports=std_external_crate main.rs`  
分组 std 和外部。  
**高级提示**：选项：preserve/std_external_crate/one。

## 教程38: 配置 reorder_imports
**描述**：重新排序导入。  
**语法**：`rustfmt --config reorder_imports=true <file.rs>`。  
**示例**：  
`rustfmt --config reorder_imports=true main.rs`  
字母排序。  
**高级提示**：默认 true。

## 教程39: 配置 reorder_modules
**描述**：重新排序模块。  
**语法**：`rustfmt --config reorder_modules=true <file.rs>`。  
**示例**：  
`rustfmt --config reorder_modules=true main.rs`  
排序 mod。  
**高级提示**：默认 true。

## 教程40: 配置 reorder_impl_items
**描述**：重新排序 impl 项。  
**语法**：`rustfmt --config reorder_impl_items=true <file.rs>`。  
**示例**：  
`rustfmt --config reorder_impl_items=true main.rs`  
排序 fn/const。  
**高级提示**：默认 false。

## 教程41: 配置 comment_width
**描述**：注释最大宽度。  
**语法**：`rustfmt --config comment_width=80 <file.rs>`。  
**示例**：  
`rustfmt --config comment_width=80 main.rs`  
换行注释。  
**高级提示**：默认 80。

## 教程42: 配置 wrap_comments
**描述**：换行注释。  
**语法**：`rustfmt --config wrap_comments=true <file.rs>`。  
**示例**：  
`rustfmt --config wrap_comments=true main.rs`  
自动换行。  
**高级提示**：默认 false。

## 教程43: 配置 normalize_comments
**描述**：标准化注释。  
**语法**：`rustfmt --config normalize_comments=true <file.rs>`。  
**示例**：  
`rustfmt --config normalize_comments=true main.rs`  
统一 // 和 ///。  
**高级提示**：默认 false。

## 教程44: 配置 normalize_doc_attributes
**描述**：标准化 doc 属性。  
**语法**：`rustfmt --config normalize_doc_attributes=true <file.rs>`。  
**示例**：  
`rustfmt --config normalize_doc_attributes=true main.rs`  
转换为 ///。  
**高级提示**：默认 false。

## 教程45: 配置 format_strings
**描述**：格式化字符串字面量。  
**语法**：`rustfmt --config format_strings=true <file.rs>`。  
**示例**：  
`rustfmt --config format_strings=true main.rs`  
换行字符串。  
**高级提示**：默认 false。

## 教程46: 配置 empty_item_single_line
**描述**：空项单行。  
**语法**：`rustfmt --config empty_item_single_line=true <file.rs>`。  
**示例**：  
`rustfmt --config empty_item_single_line=true main.rs`  
紧凑空块。  
**高级提示**：默认 true。

## 教程47: 配置 struct_lit_multiline_style
**描述**：多行 struct 风格。  
**语法**：`rustfmt --config struct_lit_multiline_style=prefer_single <file.rs>`。  
**示例**：  
`rustfmt --config struct_lit_multiline_style=prefer_single main.rs`  
优先单行。  
**高级提示**：选项：prefer_single/block/force_block。

## 教程48: 配置 fn_call_style
**描述**：函数调用风格。  
**语法**：`rustfmt --config fn_call_style=block <file.rs>`。  
**示例**：  
`rustfmt --config fn_call_style=block main.rs`  
块式调用。  
**高级提示**：选项：block/visual。

## 教程49: 配置 attr_fn_like_style
**描述**：属性函数式风格。  
**语法**：`rustfmt --config attr_fn_like_style=block <file.rs>`。  
**示例**：  
`rustfmt --config attr_fn_like_style=block main.rs`  
块式属性。  
**高级提示**：类似 fn_call_style。

## 教程50: 配置 closure_block_indent_threshold
**描述**：闭包块缩进阈值。  
**语法**：`rustfmt --config closure_block_indent_threshold=1 <file.rs>`。  
**示例**：  
`rustfmt --config closure_block_indent_threshold=1 main.rs`  
缩进闭包。  
**高级提示**：-1 禁用，默认 -1。

## 结语与高级扩展
这些50个教程覆盖了 rustfmt 命令行的核心和高级用法，从基本格式化到详细配置优化。通过实践这些命令，你可以维护一致的 Rust 代码风格。更多细节参考官方文档（rustfmt GitHub）。如果需要集成编辑器，如 VS Code 的 Rust 扩展。遇到问题？运行 `rustfmt --help` 或检查 rustfmt.toml 快速解决！