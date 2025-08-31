# std::env 模块教程

Rust 的 `std::env` 模块提供了访问和操作进程环境的工具，包括命令行参数、环境变量和当前工作目录等。它是标准库的一部分，用于编写可移植的 CLI 工具或需要环境交互的程序。`std::env` 的函数多返回 `Result` 以处理错误，如变量不存在或权限问题，确保安全性和显式错误处理。


## 1. std::env 简介

- **导入**：`use std::env;`
- **主要功能**：
    - 命令行参数：`args()` 和 `args_os()`。
    - 环境变量：`var()`、`set_var()`、`vars()`。
    - 目录操作：`current_dir()`、`set_current_dir()`、`home_dir()`、`temp_dir()`。
    - 其他：`consts` 子模块（OS 常量如 `ARCH`、`OS`）。
- **优势**：跨平台（Windows/Unix），处理 Unicode 和 OS 特定字符串（OsString）。
- **注意**：环境变量是进程级的，修改仅影响当前进程及其子进程。

## 2. 命令行参数

`args()` 返回命令行参数的迭代器，包括程序名作为第一个元素。

### 示例：解析参数
```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("所有参数: {:?}", args);

    if args.len() > 1 {
        println!("第一个参数: {}", args[1]);
    } else {
        println!("无额外参数");
    }
}
```

- **解释**：运行 `cargo run -- hello world` 输出：所有参数: ["target/debug/myapp", "hello", "world"]。`collect()` 转为 Vec。参数是 String，但如果包含无效 UTF-8，用 `args_os()` 返回 OsString。

### 示例：OsString 参数
```rust
use std::env;
use std::ffi::OsString;

fn main() {
    let args_os: Vec<OsString> = env::args_os().collect();
    if let Some(first) = args_os.get(1) {
        println!("第一个参数: {:?}", first);
    }
}
```

- **解释**：`args_os` 处理 OS 特定字符串（如 Windows 非 UTF-8）。用 `to_string_lossy()` 转为 Cow<str> 以打印。

## 3. 环境变量

环境变量是键值对，用于配置（如 PATH）。

### 示例：获取和设置变量
```rust
use std::env;

fn main() {
    match env::var("PATH") {
        Ok(val) => println!("PATH: {}", val),
        Err(e) => println!("错误: {}", e),  // 如 "environment variable not found"
    }

    env::set_var("MY_VAR", "hello");
    println!("MY_VAR: {}", env::var("MY_VAR").unwrap());

    env::remove_var("MY_VAR");
    println!("移除后: {:?}", env::var("MY_VAR"));  // Err(NotFound)
}
```

- **解释**：`var` 返回 Result<String, VarError>。`set_var` 设置（覆盖现有）。`remove_var` 删除。变量是大小写敏感的（Unix 区分，Windows 不完全）。

### 示例：迭代所有变量
```rust
use std::env;

fn main() {
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
}
```

- **解释**：`vars()` 返回 (String, String) 迭代器。用于调试或导出环境。

## 4. 工作目录操作

管理当前目录和特殊目录。

### 示例：当前目录和切换
```rust
use std::env;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let current = env::current_dir()?;
    println!("当前目录: {}", current.display());

    env::set_current_dir(Path::new("/tmp"))?;
    println!("新目录: {}", env::current_dir()?.display());

    Ok(())
}
```

- **解释**：`current_dir` 返回 PathBuf。`set_current_dir` 更改目录，返回 Result。处理错误如目录不存在。

### 示例：家目录和临时目录
```rust
use std::env;

fn main() {
    if let Some(home) = env::home_dir() {
        println!("家目录: {}", home.display());
    } else {
        println!("无家目录");
    }

    println!("临时目录: {}", env::temp_dir().display());
}
```

- **解释**：`home_dir` 返回 Option<PathBuf>（基于 HOME 变量）。`temp_dir` 返回系统临时目录（如 /tmp）。

## 5. 常量和 OS 信息

`env::consts` 提供编译时常量。

### 示例：OS 常量
```rust
use std::env::consts;

fn main() {
    println!("OS: {}", consts::OS);        // 如 "linux"
    println!("Arch: {}", consts::ARCH);     // 如 "x86_64"
    println!("Family: {}", consts::FAMILY); // 如 "unix"
}
```

- **解释**：这些是静态字符串，用于条件编译或日志。其他：DLL_PREFIX、EXE_EXTENSION。

## 6. 高级主题：OsStr 和 跨平台

- `OsStr` 和 `OsString`：处理非 UTF-8 字符串。
- 条件编译：用 `#[cfg(target_os = "windows")]` 处理平台差异。

### 示例：OsStr 使用
```rust
use std::env;
use std::ffi::OsStr;

fn main() {
    let key = OsStr::new("PATH");
    match env::var_os(key) {
        Some(val) => println!("PATH: {:?}", val),
        None => println!("未找到"),
    }
}
```

- **解释**：`var_os` 返回 Option<OsString>，避免 UTF-8 转换错误。

## 7. 最佳实践和常见陷阱

- **错误处理**：总是处理 Result/Option，如用 ? 或 match，避免 unwrap（生产代码）。
- **安全性**：环境变量可被外部修改，验证输入（如路径）。避免设置敏感变量。
- **跨平台**：用 Path/PathBuf 处理路径分隔符（/ vs \）。测试多 OS。
- **性能**：`vars()` 迭代所有变量可能慢（大环境），优先 `var` 单查。
- **常见错误**：
    - 变量不存在：VarError::NotPresent – 用 unwrap_or("") 处理。
    - 无效 Unicode：用 var_os 代替 var。
    - 权限：set_current_dir 可能失败（用 Result）。
- **与 clap/structopt**：复杂 CLI 用外部 crate 解析参数，而非手动 args。
- **环境变量线程安全**：全局，但 Rust 确保安全访问。

## 练习建议
1. 编写 CLI 工具：用 args() 读取文件名，var("DEBUG") 控制日志。
2. 创建备份脚本：用 current_dir() 和 temp_dir() 复制文件。
3. 检查 OS：用 consts::OS 打印平台特定消息。

如果需要更多示例、与其他模块的集成（如 std::process），或特定函数的深入解释，请提供细节！