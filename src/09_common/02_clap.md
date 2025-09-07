### Clap

`clap` 是 Rust 中一个功能全面、高效的命令行参数解析器（Command Line Argument Parser），它支持声明式（derive API）和过程式（builder API）两种方式来定义 CLI，支持子命令、验证、帮助生成、shell 补全等功能。clap 旨在提供开箱即用的精致 CLI 体验，包括彩色输出、建议修复和常见参数行为。它特别适合构建 CLI 工具，如 `cargo` 或 `git` 风格的应用程序。

#### 1. 安装 clap
在你的 `Cargo.toml` 文件中添加依赖。最新版本为 4.5.46（发布于 2025 年 8 月 26 日）。推荐启用 `derive` 特性以使用声明式 API。

```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }  # 启用 derive API
```

对于其他特性，如 `color`（彩色输出，默认启用）、`env`（环境变量支持）、`wrap_help`（帮助文本换行），可以添加相应 feature。运行 `cargo build` 安装。clap 支持 MSRV（Minimum Supported Rust Version）为 1.74，并遵循 semver，每 6-9 个月一个重大版本。

#### 2. 基本用法
clap 提供两种主要 API：
- **Derive API**：通过结构体和属性宏声明 CLI（推荐用于简单应用）。
- **Builder API**：过程式构建 `Command` 和 `Arg`（更灵活，用于复杂场景）。

基本语法（Derive API）：

```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]  // 命令属性
struct Args {
    #[arg(short, long)]  // 参数属性
    name: String,
}

fn main() {
    let args = Args::parse();
    println!("Name: {}", args.name);
}
```

运行 `./app --name Alice` 将解析 `--name` 参数。clap 自动生成 `--help` 和 `--version`。

Builder API 示例：

```rust
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("app")
        .arg(Arg::new("name").short('n').long("name"))
        .get_matches();
    let name = matches.get_one::<String>("name").unwrap();
    println!("Name: {}", name);
}
```

#### 3. 语义和实现细节
- **参数类型**：支持标志（bool）、选项（带值）、位置参数、子命令。
- **验证**：内置验证如 `required`、`default_value`、`value_parser`（自定义解析）。
- **帮助生成**：自动 `--help`，支持自定义模板和彩色输出。
- **错误处理**：使用 `clap::Error`，支持上下文和建议修复。
- **性能**：高效解析，基准测试显示优于许多替代品；二进制大小合理，通过 feature flags 控制。
- **链式错误**：参数冲突通过 `ArgGroup` 处理。
- **回溯和调试**：集成 `RUST_BACKTRACE` 支持详细错误。
- **最近变化**：4.x 版本引入更多 derive 支持、更好的 env 集成和性能优化；从 3.x 迁移需注意 builder API 变化。

#### 4. 高级用法
- **子命令**：使用 `Subcommand` trait。
- **环境变量**：启用 `env` feature，使用 `env` 属性。
- **shell 补全**：使用 `clap_complete` crate 生成 bash/zsh 等补全脚本。
- **自定义解析**：实现 `ValueParser` 或使用 `value_parser!`。
- **多线程**：clap 的 `ArgMatches` 是线程安全的。
- **国际化**：结合 `clap-i18n-richformatter`。
- **响应文件**：结合 `argfile` crate 支持 `@file` 加载参数。
- **Windows 支持**：结合 `wild` 处理通配符。

#### 5. 注意事项
- Derive API 更简洁，但 builder API 更灵活；对于库，推荐 builder 以避免宏依赖。
- 避免在 derive 中使用复杂类型；自定义类型需实现 `FromStr`。
- 性能开销低，但大型 CLI 可能增加二进制大小；使用 feature flags 优化。
- 文档有时被视为信息过多；从教程章节开始学习。
- 不支持互联网访问或动态加载；所有配置在编译时。
- 与 `anyhow` 集成处理错误。

#### 6. 替代方案
- **structopt**：已弃用，被集成到 clap 的 derive API。
- **argh**：轻量 derive-based 解析器，适合简单 CLI。
- **pico-args**：极简、无依赖解析器，性能高但功能少。
- **docopt**：基于使用字符串，简单但不灵活。
- **gumdrop**：另一个 derive 风格，但 clap 更全面。
  clap 被视为 Rust CLI 的标准选择。

#### 7. 20 个例子
以下是 20 个例子，从简单到复杂，覆盖标志、选项、子命令等。每个例子包括代码、预期输出（如果适用）和解释。假设已导入 `use clap::{Parser, Subcommand, Arg, Command};`。为简洁，使用 derive API 为主。

##### 示例 1: 基本问候程序
```rust
#[derive(Parser, Debug)]
#[command(version, about = "Greet a person")]
struct Args {
    #[arg(short, long)]
    name: String,
}

fn main() {
    let args = Args::parse();
    println!("Hello, {}!", args.name);
}
```
运行：`./app --name Alice` 输出：`Hello, Alice!`  
解释：简单选项参数。

##### 示例 2: 默认值和计数
```rust
#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();
    for _ in 0..args.count {
        println!("Hello!");
    }
}
```
运行：`./app --count 3` 输出：三行 `Hello!`  
解释：默认值和数字解析。

##### 示例 3: 布尔标志
```rust
#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    if args.verbose {
        println!("Verbose mode on");
    }
}
```
运行：`./app --verbose` 输出：`Verbose mode on`  
解释：布尔标志，默认 false。

##### 示例 4: 位置参数
```rust
#[derive(Parser, Debug)]
struct Args {
    input: String,
}

fn main() {
    let args = Args::parse();
    println!("Input: {}", args.input);
}
```
运行：`./app file.txt` 输出：`Input: file.txt`  
解释：无标志的位置参数。

##### 示例 5: 子命令
```rust
#[derive(Parser, Debug)]
#[command(subcommand_required = true)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add { x: i32, y: i32 },
}

fn main() {
    let args = Args::parse();
    match args.command {
        Commands::Add { x, y } => println!("Sum: {}", x + y),
    }
}
```
运行：`./app add --x 1 --y 2` 输出：`Sum: 3`  
解释：基本子命令。

##### 示例 6: 环境变量支持
```rust
#[derive(Parser, Debug)]
struct Args {
    #[arg(long, env = "API_KEY")]
    api_key: String,
}

fn main() {
    let args = Args::parse();
    println!("API Key: {}", args.api_key);
}
```
解释：从环境变量读取（需启用 `env` feature）。

##### 示例 7: 值验证
```rust
#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..=10))]
    level: u32,
}

fn main() {
    let args = Args::parse();
    println!("Level: {}", args.level);
}
```
解释：范围验证。

##### 示例 8: 多个值
```rust
#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, num_args = 1..)]
    files: Vec<String>,
}

fn main() {
    let args = Args::parse();
    println!("Files: {:?}", args.files);
}
```
运行：`./app --files a.txt b.txt` 输出：`Files: ["a.txt", "b.txt"]`  
解释：可变数量参数。

##### 示例 9: 自定义枚举
```rust
use clap::ValueEnum;

#[derive(Parser, Debug)]
struct Args {
    #[arg(value_enum)]
    mode: Mode,
}

#[derive(ValueEnum, Debug, Clone)]
enum Mode { Fast, Slow }

fn main() {
    let args = Args::parse();
    println!("Mode: {:?}", args.mode);
}
```
解释：枚举值解析。

##### 示例 10: Builder API 基本
```rust
fn main() {
    let matches = Command::new("app")
        .arg(Arg::new("debug").short('d').action(clap::ArgAction::SetTrue))
        .get_matches();
    if matches.get_flag("debug") {
        println!("Debug on");
    }
}
```
解释：过程式构建。

##### 示例 11: ArgGroup（互斥参数）
```rust
#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, group = "mode")]
    fast: bool,
    #[arg(short, long, group = "mode")]
    slow: bool,
}

fn main() {
    let args = Args::parse();
    println!("Fast: {}, Slow: {}", args.fast, args.slow);
}
```
解释：参数组确保互斥。

##### 示例 12: 自定义解析
```rust
use std::str::FromStr;

#[derive(Parser, Debug)]
struct Args {
    #[arg(value_parser = |s: &str| s.parse::<Custom>())]
    custom: Custom,
}

struct Custom(i32);
impl FromStr for Custom {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Custom(s.parse::<i32>().map_err(|e| e.to_string())?))
    }
}

fn main() {
    let args = Args::parse();
    println!("Custom: {}", args.custom.0);
}
```
解释：自定义类型解析。

##### 示例 13: 隐藏参数
```rust
#[derive(Parser, Debug)]
struct Args {
    #[arg(long, hide = true)]
    secret: Option<String>,
}

fn main() {
    let args = Args::parse();
    if let Some(s) = args.secret {
        println!("Secret: {}", s);
    }
}
```
解释：不在帮助中显示。

##### 示例 14: 默认动作
```rust
#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() {
    let args = Args::parse();
    println!("Verbose level: {}", args.verbose);
}
```
运行：`./app -vvv` 输出：`Verbose level: 3`  
解释：计数动作。

##### 示例 15: 嵌套子命令
```rust
#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Config {
        #[command(subcommand)]
        sub: ConfigSub,
    },
}

#[derive(Subcommand, Debug)]
enum ConfigSub {
    Set { key: String },
}

fn main() {
    let args = Args::parse();
    // 处理...
}
```
解释：多级子命令。

##### 示例 16: 帮助模板自定义
```rust
#[derive(Parser, Debug)]
#[command(help_template = "Usage: {usage}\n{about}")]
struct Args {}

fn main() {
    Args::parse();
}
```
解释：自定义帮助输出。

##### 示例 17: 从字符串解析（测试用）
```rust
fn main() {
    let args: Vec<String> = vec!["app".to_string(), "--name".to_string(), "Alice".to_string()];
    let matches = Command::new("app")
        .arg(Arg::new("name").long("name"))
        .try_get_matches_from(args).unwrap();
    println!("Name: {}", matches.get_one::<String>("name").unwrap());
}
```
解释：从字符串解析，用于测试。

##### 示例 18: 必需参数
```rust
#[derive(Parser, Debug)]
struct Args {
    #[arg(required = true)]
    input: String,
}

fn main() {
    let args = Args::parse();
    println!("Input: {}", args.input);
}
```
解释：强制要求参数。

##### 示例 19: 版本信息
```rust
#[derive(Parser, Debug)]
#[command(version = "1.0", author = "You <you@email.com>")]
struct Args {}

fn main() {
    Args::parse();
    // ./app --version 显示 1.0
}
```
解释：自定义版本和作者。

##### 示例 20: 全局参数
```rust
#[derive(Parser, Debug)]
#[command(propagate_version = true)]
struct Args {
    #[arg(global = true, long)]
    config: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Run,
}

fn main() {
    let args = Args::parse();
    if let Some(c) = args.config {
        println!("Config: {}", c);
    }
}
```
运行：`./app run --config file.toml`  
解释：全局参数适用于子命令。