# Rust std::process 模块教程

Rust 的 `std::process` 模块是标准库中处理子进程管理和执行外部命令的核心组成部分，提供 `Command`、`Child`、`ExitStatus` 等类型，用于启动、控制和等待进程。它抽象了底层 OS 进程 API（如 Unix fork/exec 和 Windows CreateProcess），确保跨平台兼容性，并通过 `std::io::Result` 显式处理错误如命令不存在或权限不足。`std::process` 强调安全性：防止命令注入（用 arg 而非字符串拼接），集成 stdin/stdout/stderr 重定向，支持环境变量和当前目录自定义。模块常与 `std::io`（I/O 流）、`std::thread`（并发等待）和 `std::env`（环境变量）结合使用，支持同步阻塞操作（异步用 tokio::process）。

## 1. std::process 简介

- **导入和基本结构**：通常用 `use std::process::{Command, Stdio};` 或指定如 `use std::process::ExitStatus;`。模块分为命令构建、进程执行和状态检查三大类。
    - **类型概述**：
        - `Command`：构建器，用于设置命令、参数、环境、目录、stdio 重定向和 OS 特定标志。
        - `Child`：运行进程句柄，支持 stdin/stdout/stderr 访问、kill、wait。
        - `ExitStatus`：进程退出状态，支持 success()、code()（退出码）。
        - `ExitCode`：进程退出码枚举（SUCCESS/FAILURE）。
        - `Output`：output() 返回的结构体（status、stdout、stderr）。
        - `Stdio`：stdio 配置（Piped、Null、Inherit）。
    - **函数**：`exit`（当前进程退出）、`id`（当前 PID）、`abort`（异常退出）。
    - **Trait**：`CommandExt`（OS 扩展，如 unix::CommandExt::uid）。
- **设计哲学**：`std::process` 是阻塞同步的（wait 阻塞调用者）；错误通过 io::ErrorKind 分类（如 NotFound、PermissionDenied）；支持管道（piped stdio）。进程所有权：Child drop 时不自动 kill，用 try_wait 检查。
- **跨平台注意**：Windows 用 cmd.exe 处理 bat，Unix 用 sh；路径分隔用 Path 以兼容；测试多 OS 用 CI 或 VM。
- **性能基础**：启动进程开销大（fork/exec），最小化调用；wait O(1) 但阻塞；多进程用 thread 池管理。
- **常见用例**：运行 shell 命令、管道数据、守护进程、并行任务、测试外部工具。
- **扩展概念**：与 std::os 集成 OS 标志（如 Windows detached）；与 std::io 读写 Child 流；错误重试机制；与 rayon 结合并行 spawn；资源限制如 ulimit（Unix 扩展）。

## 2. Command 类型：构建和配置命令

`Command` 是链式构建器，用于安全配置命令。

### 示例：基本 Command 执行（status 示例）
```rust
use std::process::Command;

fn main() -> std::io::Result<()> {
    let status = Command::new("ls")
        .arg("-l")
        .status()?;
    println!("退出码: {:?}", status.code());  // Some(0) if success
    println!("成功？{}", status.success());
    Ok(())
}
```

- **解释**：`new` 创建从命令路径。`arg` 添加参数（防注入）。`status` 执行并等待，返回 ExitStatus。性能：阻塞直到结束。

### 示例：捕获输出（output 扩展）
```rust
use std::process::Command;

fn main() -> std::io::Result<()> {
    let output = Command::new("echo")
        .arg("hello")
        .output()?;
    println!("状态: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));  // "hello\n"
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    Ok(())
}
```

- **解释**：`output` 返回 Output（status + Vec<u8> stdout/stderr）。`from_utf8_lossy` 处理输出。陷阱：大输出 OOM，用 spawn + read 流式。

### 示例：环境和目录配置（扩展自定义）
```rust
use std::process::Command;
use std::env;

fn main() -> std::io::Result<()> {
    let mut cmd = Command::new("printenv");
    cmd.env("MY_VAR", "value");  // 设置变量
    cmd.env_clear();  // 清空所有（小心）
    cmd.env("PATH", env::var("PATH")?);  // 恢复 PATH

    cmd.current_dir("/tmp");  // 设置工作目录

    let output = cmd.output()?;
    println!("输出: {}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}
```

- **解释**：`env` 设置单个；`env_clear` 清空继承；`current_dir` 设置 cwd。性能：环境复制 O(n) 于变量数。扩展：用 envs 批量设置 Iterator<(K, V)>。

### 示例：Stdio 重定向（管道扩展）
```rust
use std::process::{Command, Stdio};
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut child = Command::new("grep")
        .arg("hello")
        .stdin(Stdio::piped())  // 管道输入
        .stdout(Stdio::piped()) // 管道输出
        .stderr(Stdio::null())  // 丢弃错误
        .spawn()?;

    {
        let mut stdin = child.stdin.take().unwrap();
        stdin.write_all(b"hello world\n")?;
    }  // drop stdin 关闭

    let output = child.wait_with_output()?;
    println!("过滤: {}", String::from_utf8_lossy(&output.stdout));  // "hello world\n"
    Ok(())
}
```

- **解释**：`Stdio::piped` 创建管道；`null` 丢弃；`inherit` 继承父进程。`spawn` 返回 Child；`take` 移动 stdin。`wait_with_output` 等待并捕获。陷阱：未关闭 stdin 可能死锁。扩展：用 Stdio::from(File) 重定向文件。

## 3. Child 类型：管理运行进程

`Child` 是进程句柄，支持 I/O 和控制。

### 示例：等待和杀死进程（基本 Child）
```rust
use std::process::Command;

fn main() -> std::io::Result<()> {
    let mut child = Command::new("sleep").arg("5").spawn()?;
    println!("PID: {}", child.id());

    child.kill()?;  // 发送 SIGKILL (Unix) 或 TerminateProcess (Windows)
    let status = child.wait()?;
    println!("状态: {}", status);
    Ok(())
}
```

- **解释**：`spawn` 启动返回 Child。`id` 返回 PID。`kill` 终止。`wait` 阻塞等待。性能：wait 轮询 OS。

### 示例：try_wait 和 非阻塞检查（扩展监控）
```rust
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let mut child = Command::new("sleep").arg("3").spawn()?;

    loop {
        match child.try_wait()? {
            Some(status) => {
                println!("退出: {}", status);
                break;
            }
            None => {
                println!("仍在运行");
                sleep(Duration::from_secs(1));
            }
        }
    }
    Ok(())
}
```

- **解释**：`try_wait` 非阻塞检查退出，返回 Option<ExitStatus>。用于轮询。陷阱：频繁 try_wait 开销，用 notify/waitpid（OS 扩展）优化。

### 示例：I/O 流交互（扩展管道）
```rust
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Write};

fn main() -> std::io::Result<()> {
    let mut child = Command::new("bc")  // 交互计算器
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(b"2 + 3\n")?;
    drop(stdin);  // 关闭输入

    let stdout = child.stdout.take().unwrap();
    let reader = BufReader::new(stdout);
    for line in reader.lines() {
        println!("结果: {}", line?);
    }

    child.wait()?;
    Ok(())
}
```

- **解释**：`take` 移动流所有权。BufReader 高效读行。扩展：用 thread 并发读写流避免死锁。

## 4. ExitStatus 和 当前进程

`ExitStatus` 检查退出；当前进程函数。

### 示例：ExitStatus 详细检查（扩展代码信号）
```rust
use std::process::Command;

fn main() -> std::io::Result<()> {
    let status = Command::new("false").status()?;
    println!("成功？{}", status.success());  // false
    println!("代码: {:?}", status.code());   // Some(1)

    #[cfg(unix)]
    println!("信号: {:?}", status.signal());  // None 或 Some(sig)

    Ok(())
}
```

- **解释**：`success` 检查 code == 0。`signal` Unix 特定。扩展：用 os::unix::process::ExitStatusExt::from_raw 自定义。

### 示例：当前进程退出和 abort（扩展控制）
```rust
use std::process;

fn main() {
    if some_condition() {
        process::exit(1);  // 立即退出，code 1
    }

    // 扩展：abort 异常退出
    if panic_condition() {
        process::abort();  // 无 unwind，核心转储
    }
}
```

- **解释**：`exit` 运行 atexit 但不 unwind。`abort` 用于调试崩溃。陷阱：exit 不运行 drop，资源泄漏。

## 5. OS 扩展：CommandExt 和 ChildExt

用 std::os 扩展平台标志。

### 示例：Unix CommandExt（进程组扩展）
```rust
#[cfg(unix)]
use std::os::unix::process::CommandExt;
#[cfg(unix)]
use std::process::Command;

#[cfg(unix)]
fn main() -> std::io::Result<()> {
    let mut cmd = Command::new("sleep");
    cmd.arg("10");
    cmd.process_group(0);  // 新进程组
    cmd.spawn()?;
    Ok(())
}

#[cfg(not(unix))]
fn main() {}
```

- **解释**：`process_group` 设置 pgid。扩展：用 before_exec 自定义 fork 后 exec 前。

### 示例：Windows CommandExt（标志扩展）
```rust
#[cfg(windows)]
use std::os::windows::process::CommandExt;
#[cfg(windows)]
use std::process::Command;

#[cfg(windows)]
fn main() -> std::io::Result<()> {
    let mut cmd = Command::new("cmd.exe");
    cmd.creation_flags(0x8000000);  // 高优先级
    cmd.spawn()?;
    Ok(())
}

#[cfg(not(windows))]
fn main() {}
```

- **解释**：`creation_flags` 设置 CreateProcess 标志。扩展：用 raw_arg 原始参数字符串。

## 6. 高级主题：管道链、错误处理和集成

- 管道：多 Command 链。
- 集成：与 thread/io。

### 示例：管道链（多进程扩展）
```rust
use std::process::{Command, Stdio};

fn main() -> std::io::Result<()> {
    let ls = Command::new("ls")
        .stdout(Stdio::piped())
        .spawn()?;

    let grep = Command::new("grep")
        .arg("Cargo")
        .stdin(Stdio::from(ls.stdout.unwrap()))
        .output()?;

    println!("过滤: {}", String::from_utf8_lossy(&grep.stdout));
    Ok(())
}
```

- **解释**：`Stdio::from` 移动 stdout 到 stdin。扩展：用 thread 并发读多管道。

### 示例：详细错误处理（重试扩展）
```rust
use std::process::Command;
use std::io;
use std::time::Duration;
use std::thread::sleep;

fn run_with_retry(cmd: &str, retries: u32) -> io::Result<std::process::Output> {
    let mut last_err = None;
    for _ in 0..retries {
        match Command::new(cmd).output() {
            Ok(out) if out.status.success() => return Ok(out),
            Ok(out) => last_err = Some(io::Error::new(io::ErrorKind::Other, format!("失败 code {}", out.status.code().unwrap_or(-1)))),
            Err(e) => {
                last_err = Some(e);
                if e.kind() == io::ErrorKind::NotFound {
                    return Err(e);  // 不可恢复
                }
                sleep(Duration::from_secs(1));
            }
        }
    }
    Err(last_err.unwrap_or_else(|| io::Error::new(io::ErrorKind::Other, "未知")))
}

fn main() {
    match run_with_retry("nonexistent", 3) {
        Ok(out) => println!("输出: {:?}", out),
        Err(e) => println!("最终错误: {} ({:?})", e, e.kind()),
    }
}
```

- **解释**：重试失败。检查 status.success。扩展：日志 stderr 于错误。

## 7. 最佳实践和常见陷阱

- **命令最佳实践**：用 arg 防注入；显式 env 以隔离；piped 时及时关闭 stdin。
- **性能陷阱**：频繁 spawn 开销大，批量命令；wait 阻塞，用 try_wait 轮询。
- **错误最佳实践**：分类 kind()；重试 NotFound（路径问题）；日志 output.stderr。
- **安全性**：sanitize 用户输入 arg；避免 shell=true，用 sh -c 显式。
- **跨平台扩展**：用 cfg 设置标志；测试 bat/sh 差异。
- **测试扩展**：用 mock Command 测试 spawn 无实际执行；集成 test crate。
- **资源管理**：Child drop 不 kill，用 kill 显式终止；limit 子进程用 rlimit（Unix）。
- **常见错误扩展**：
    - NotFound：检查 PATH，用 which crate 验证。
    - PermissionDenied：检查可执行位，用 os 扩展 chmod。
    - InvalidInput：arg 非 UTF-8，用 OsString。
    - BrokenPipe：子进程早关闭，检查 status。

## 8. 练习建议

1. 编写管道工具：链 ls | grep | wc，用 piped 和 output。
2. 实现守护进程：用 unix fork/setsid，windows detached 创建。
3. 创建并行 runner：用 thread spawn 多 Command，join 等待。
4. 处理交互 shell：用 piped 读写 bc/REPL 命令。
5. 基准测试：比较 spawn vs exec 时间，用 Instant。
6. 与 io 集成：用 BufReader 读 Child stdout 行，写 stdin。
7. 错误模拟：用 mock 失败 Command 测试重试逻辑。
