# Rustc

Rustc 是 Rust 编程语言的核心编译器，用于将 Rust 源代码编译成可执行文件或库。它支持丰富的命令行选项，允许开发者精细控制编译过程，包括输出格式、优化级别、目标平台等。本教程基于官方文档，提供超级扩展的指导，分为30个独立教程部分，每个部分聚焦一个关键命令行选项或组合使用场景。每个教程包括：

- **描述**：选项的功能说明。
- **语法**：基本命令格式。
- **示例**：实际命令和预期效果（假设有一个简单的 `hello.rs` 文件：`fn main() { println!("Hello, World!"); }`）。
- **高级提示**：扩展用法或注意事项。

这些教程从基础开始，逐步深入，适合初学者到高级用户。安装 Rust 后，直接在终端运行 `rustc` 即可开始实验。注意：Rustc 通常通过 Cargo 使用，但本教程聚焦纯 rustc 命令行。

## 教程1: 获取帮助信息 (-h / --help)
**描述**：显示 rustc 的所有命令行选项和简要说明，帮助快速上手。  
**语法**：`rustc -h` 或 `rustc --help`。  
**示例**：  
`rustc -h`  
输出：列出所有选项，如 `-o`、`--emit` 等。  
**高级提示**：结合 `--help-hidden` 查看隐藏（不稳定）选项。

## 教程2: 查看版本信息 (-V / --version)
**描述**：打印当前 rustc 版本，便于检查兼容性。  
**语法**：`rustc -V` 或 `rustc --version`。  
**示例**：  
`rustc -V`  
输出：`rustc 1.81.0 (eeb90cda0 2024-09-04)`（版本依安装而定）。  
**高级提示**：使用 `rustc --version --verbose` 获取更多细节，如 Git 提交哈希。

## 教程3: 基本编译单个文件
**描述**：默认模式下，rustc 编译源文件生成可执行文件。  
**语法**：`rustc <source_file.rs>`。  
**示例**：  
`rustc hello.rs`  
生成 `hello`（或 `hello.exe` on Windows）可执行文件，运行 `./hello` 输出 "Hello, World!"。  
**高级提示**：如果有多个文件，使用 `--extern` 链接 crate。

## 教程4: 指定输出文件名 (-o)
**描述**：自定义编译输出的可执行文件名。  
**语法**：`rustc -o <output> <source_file.rs>`。  
**示例**：  
`rustc -o myapp hello.rs`  
生成 `myapp`，运行 `./myapp`。  
**高级提示**：与 `--out-dir` 结合使用：`rustc -o myapp --out-dir bin hello.rs`。

## 教程5: 设置输出目录 (--out-dir)
**描述**：将输出文件放置到指定目录。  
**语法**：`rustc --out-dir <dir> <source_file.rs>`。  
**示例**：  
`rustc --out-dir target hello.rs`  
在 `target/` 下生成 `hello`。  
**高级提示**：如果 `-o` 已指定，此选项被忽略；适合多文件项目。

## 教程6: 控制输出类型 (--emit)
**描述**：指定生成的文件类型，如可执行文件、LLVM IR 或依赖信息。  
**语法**：`rustc --emit=<type1,type2> <source_file.rs>`（类型：obj, mir, llvm-ir 等）。  
**示例**：  
`rustc --emit=llvm-ir hello.rs`  
生成 `hello.ll`（LLVM IR 文件）。  
**高级提示**：`--emit=dep-info=deps.d` 生成 Makefile 兼容的依赖文件。

## 教程7: 条件编译配置 (--cfg)
**描述**：启用或禁用 `#[cfg]` 条件编译标志。  
**语法**：`rustc --cfg <cfg_expr> <source_file.rs>`。  
**示例**：（假设代码有 `#[cfg(debug)] println!("Debug!");`）  
`rustc --cfg debug hello.rs`  
编译时启用 debug 分支。  
**高级提示**：`--cfg 'feature="nightly"'` 模拟不稳定特性。

## 教程8: 指定 Rust 版本 (--edition)
**描述**：选择 Rust 版本（如 2018、2021），影响语法和特性。  
**语法**：`rustc --edition=<year> <source_file.rs>`。  
**示例**：  
`rustc --edition=2021 hello.rs`  
使用 2021 版编译，支持 async/await 等。  
**高级提示**：默认 2015；检查兼容性用 `cargo check --edition 2021`。

## 教程9: 交叉编译目标 (--target)
**描述**：编译针对特定平台（如 ARM）。  
**语法**：`rustc --target=<triple> <source_file.rs>`。  
**示例**：  
`rustc --target=x86_64-unknown-linux-gnu hello.rs`  
生成 Linux x64 可执行。  
**高级提示**：列出可用目标：`rustup target list`；需先安装目标。

## 教程10: 构建测试套件 (--test)
**描述**：编译测试代码，忽略 main 函数，生成测试运行器。  
**语法**：`rustc --test <source_file.rs>`。  
**示例**：（假设有 `#[test] fn it_works() { ... }`）  
`rustc --test hello.rs`  
生成测试二进制，运行 `./hello --test` 执行测试。  
**高级提示**：结合 `-g` 生成调试信息：`rustc --test -g hello.rs`。

## 教程11: 添加库搜索路径 (-L)
**描述**：指定外部库或依赖的搜索目录。  
**语法**：`rustc -L <kind>=<path> <source_file.rs>`（kind: dependency, native 等）。  
**示例**：  
`rustc -L dependency=/path/to/libs hello.rs`  
从指定路径链接依赖。  
**高级提示**：`-L native=/usr/lib` 用于系统库如 pthread。

## 教程12: 链接本地库 (-l)
**描述**：链接指定的本地库（如 C 库）。  
**语法**：`rustc -l <kind>=<name> <source_file.rs>`。  
**示例**：  
`rustc -l dylib=sqlite3 hello.rs`  
链接动态 SQLite 库。  
**高级提示**：静态链接：`-l static=mylib`；重命名：`-l renamed=original`。

## 教程13: 外部 crate 指定 (--extern)
**描述**：手动指定外部 crate 的位置和名称。  
**语法**：`rustc --extern <crate>=<path> <source_file.rs>`。  
**示例**：  
`rustc --extern serde=/path/to/serde.rlib hello.rs`  
使用自定义 serde 库。  
**高级提示**：省略路径时搜索标准位置；用于 no_std 环境。

## 教程14: 代码生成选项 (-C)
**描述**：控制代码生成，如优化级别（opt-level）、目标 CPU。  
**语法**：`rustc -C <option>=<value> <source_file.rs>`。  
**示例**：  
`rustc -C opt-level=3 hello.rs`  
启用最高优化，减小二进制大小。  
**高级提示**：`-C target-cpu=native` 使用主机 CPU 特性；`-C lto` 启用链接时优化。

## 教程15: 调试信息 (-g)
**描述**：生成调试符号，便于使用 gdb 或 lldb 调试。  
**语法**：`rustc -g <source_file.rs>`（或 `-g <level>`）。  
**示例**：  
`rustc -g hello.rs`  
生成带调试信息的可执行，运行 `gdb ./hello`。  
**高级提示**：`-g pub-names` 仅生成公共符号，减小文件大小。

## 教程16: 详细输出 (-v / --verbose)
**描述**：启用详细日志，显示编译过程细节。  
**语法**：`rustc -v <source_file.rs>`。  
**示例**：  
`rustc -v hello.rs`  
输出：显示 crate 类型、链接步骤等。  
**高级提示**：结合 `--timings`：`rustc -v --timings hello.rs` 获取性能计时。

## 教程17: 解释错误代码 (--explain)
**描述**：详细解释特定错误代码的原因和修复建议。  
**语法**：`rustc --explain <error_code>`。  
**示例**：  
`rustc --explain E0308`  
输出：解释类型不匹配错误的细节。  
**高级提示**：在编译失败后，使用错误消息中的代码查询。

## 教程18: 路径前缀重映射 (--remap-path-prefix)
**描述**：在诊断或调试信息中重映射源路径（隐私或构建优化）。  
**语法**：`rustc --remap-path-prefix <from>=<to> <source_file.rs>`。  
**示例**：  
`rustc --remap-path-prefix /home/user=~/src hello.rs`  
调试信息中路径显示为 `~/src` 而非绝对路径。  
**高级提示**：多次使用：`--remap-path-prefix from1=to1 --remap-path-prefix from2=to2`。

## 教程19: Crate 类型指定 (--crate-type)
**描述**：控制生成的 crate 类型（如 bin、lib、rlib）。  
**语法**：`rustc --crate-type=<type> <source_file.rs>`。  
**示例**：  
`rustc --crate-type=lib hello.rs`  
生成静态库 `libhello.rlib`（需无 main 函数）。  
**高级提示**：多类型：`--crate-type=lib,cdylib` 生成库和动态库。

## 教程20: 不稳定特性启用 (-Z)
**描述**：访问实验性（nightly）特性，需小心使用。  
**语法**：`rustc -Z <unstable-option> <source_file.rs>`。  
**示例**：  
`rustc -Z unstable-options hello.rs`  
启用不稳定选项；或 `-Znext-solver` 使用新求解器。  
**高级提示**：仅在 nightly 通道可用：`rustup default nightly`；查看所有：`rustc -Z help`。结合 `#![feature(...)]` 在代码中使用。

## 教程21: 打印信息 (--print)
**描述**：打印编译器内部信息，如 crate 类型或 sysroot 路径，而不进行编译。  
**语法**：`rustc --print <kind>`。  
**示例**：  
`rustc --print cfg`  
输出：当前配置的 cfg 值列表。  
**高级提示**：种类包括 `crate-name`、`file-names`、`sysroot` 等；用于脚本自动化。

## 教程22: 设置警告级别 (-W / --warn)
**描述**：控制特定 lint 的警告级别，启用额外检查。  
**语法**：`rustc -W <lint>`。  
**示例**：  
`rustc -W unused-variables hello.rs`  
警告未使用变量。  
**高级提示**：多次使用：`-W unused -W dead_code`；覆盖默认 lint 级别。

## 教程23: 允许 lint (-A / --allow)
**描述**：允许特定 lint，通过编译而不警告。  
**语法**：`rustc -A <lint>`。  
**示例**：  
`rustc -A unused-variables hello.rs`  
忽略未使用变量警告。  
**高级提示**：用于临时抑制 lint；在代码中用 `#[allow(...)]` 更精确。

## 教程24: 拒绝 lint (-D / --deny)
**描述**：将特定 lint 视为错误，阻止编译。  
**语法**：`rustc -D <lint>`。  
**示例**：  
`rustc -D unused-variables hello.rs`  
如果有未使用变量，则编译失败。  
**高级提示**：加强代码质量；结合 Clippy 使用更多 lint。

## 教程25: 禁止 lint (-F / --forbid)
**描述**：类似于 --deny，但不能被代码属性覆盖。  
**语法**：`rustc -F <lint>`。  
**示例**：  
`rustc -F unused-variables hello.rs`  
强制拒绝未使用变量，无法用 #[allow] 覆盖。  
**高级提示**：用于严格的 CI/CD 管道。

## 教程26: 限制 lint 级别 (--cap-lints)
**描述**：设置 lint 级别的上限，防止更高优先级覆盖。  
**语法**：`rustc --cap-lints <level>`（level: allow, warn, deny, forbid）。  
**示例**：  
`rustc --cap-lints warn hello.rs`  
所有 lint 最多为 warn 级别。  
**高级提示**：用于子 crate 继承父设置。

## 教程27: 输出颜色控制 (--color)
**描述**：控制诊断输出的颜色使用。  
**语法**：`rustc --color <mode>`（mode: auto, always, never）。  
**示例**：  
`rustc --color always hello.rs`  
始终使用颜色，即使非 tty。  
**高级提示**：默认 auto；在脚本中用 never 避免 ANSI 码。

## 教程28: 错误格式 (--error-format)
**描述**：自定义错误和警告的输出格式。  
**语法**：`rustc --error-format <format>`（format: human, json, short）。  
**示例**：  
`rustc --error-format json hello.rs`  
输出 JSON 格式的诊断，便于工具解析。  
**高级提示**：结合 --json 用于 IDE 集成。

## 教程29: JSON 输出 (--json)
**描述**：启用 JSON 格式的编译器输出，用于机器可读。  
**语法**：`rustc --json <kinds>`（kinds: diagnostic-rendered-ansi 等）。  
**示例**：  
`rustc --json diagnostic-rendered-ansi hello.rs`  
输出带 ANSI 颜色的 JSON 诊断。  
**高级提示**：多种类：`--json artifacts,diagnostic-short`。

## 教程30: 系统根目录 (--sysroot)
**描述**：指定 Rust 系统根目录，用于自定义安装。  
**语法**：`rustc --sysroot <path>`。  
**示例**：  
`rustc --sysroot /custom/rust hello.rs`  
使用自定义 sysroot 编译。  
**高级提示**：覆盖默认 sysroot；在嵌入式开发中有用。
