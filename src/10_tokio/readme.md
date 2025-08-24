# Rust Tokio 教程

Tokio 是 Rust 生态中领先的异步运行时（runtime），专为构建高效、可靠的异步应用而设计。它提供多线程调度器、异步 I/O、定时器、同步原语等工具，广泛用于网络服务、数据库客户端和并发任务处理。Tokio 构建在 Rust 的 async/await 语法之上，与标准库无缝集成，支持从嵌入式设备到大型服务器的各种场景。截至 2025 年 8 月，Tokio 的最新版本是 1.47.0（于 2025 年 7 月 25 日发布），引入了合作调度（cooperative scheduling）的 poll_proceed 和 cooperative 模块，以及 sync 模块中的 SetOnce。 本教程从基础到高级逐步讲解 Tokio 的使用，包括安装、任务管理、通道、I/O 操作、深入异步机制、选择分支、流处理、帧处理和优雅关闭。假设你已安装 Rust（通过 `rustup`），并使用 `cargo` 创建项目（如 `cargo new tokio-app`）。所有示例基于 Tokio 1.47.x，可复制到 `src/main.rs` 中，使用 `cargo run` 执行。教程将包含详细解释、多个代码示例、最佳实践和练习，以帮助你构建生产级异步应用。

## 1. 安装与依赖

在 `Cargo.toml` 中添加核心依赖：

```toml
[dependencies]
tokio = { version = "1.47", features = ["full"] }
```

- **Tokio**：核心运行时。启用 `"full"` 特性以包含所有模块（如 rt-multi-thread、io-util、sync、time 等），简化开发。
- **可选特性**：如果项目针对特定场景，可自定义特性，如 `"rt"`（仅运行时）、`"net"`（网络 I/O）、`"fs"`（文件系统）。避免 `"full"` 以减少二进制大小。
- **LTS 支持**：1.36.x 至 2025 年 3 月，1.38.x 至 2025 年 7 月。 对于生产环境，推荐使用 LTS 版本以确保稳定性。

运行 `cargo build` 安装。Tokio 1.47 优化了调度器性能，减少了上下文切换开销。 如果你使用其他运行时如 async-std，Tokio 在 2025 年仍是首选，因其生态更丰富和社区支持更强。

## 2. 基础：Hello Tokio 与运行时启动

从一个简单异步程序开始，了解 Tokio 的核心：async/await 和运行时。

### 示例代码：基本 Hello Tokio
```rust
use tokio::main;

#[main]
async fn main() {
    println!("Hello, Tokio!");
    let result = async_operation().await;
    println!("结果: {}", result);
}

async fn async_operation() -> i32 {
    // 模拟异步任务
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    42
}
```

- **解释**：`#[tokio::main]` 宏将 main 函数转换为异步运行时入口，使用多线程调度器。`async fn` 定义异步函数，`.await` 等待 Future 完成而不阻塞线程。Tokio 处理调度，确保高效并发。
- **运行**：`cargo run`，输出 "Hello, Tokio!" 后延迟 1 秒打印结果。
- **扩展**：自定义运行时：
  ```rust
  use tokio::runtime::Runtime;

  fn main() {
      let rt = Runtime::new().unwrap();
      rt.block_on(async {
          println!("自定义运行时");
      });
  }
  ```
  这适用于嵌入 Tokio 到同步代码中，或配置单线程运行时（`Runtime::builder().enable_all().threaded_scheduler(false).build()`）。

### 高级启动：配置运行时
Tokio 允许细粒度配置：
```rust
use tokio::runtime::Builder;

let rt = Builder::new_multi_thread()
    .worker_threads(4)  // 设置工作线程数
    .enable_all()       // 启用所有 I/O 和时间驱动
    .build()
    .unwrap();

rt.block_on(async {
    // 异步代码
});
```
- **注意**：多线程默认使用工作窃取（work-stealing）调度器，提高负载均衡。1.47 版本的 cooperative 模块允许任务主动让出 CPU。

## 3. 生成任务（Spawning Tasks）

Tokio 使用 `tokio::spawn` 生成独立任务，允许并发执行。

### 基本语法与示例
```rust
use tokio::{main, spawn, time::sleep};
use std::time::Duration;

#[main]
async fn main() {
    let handle1 = spawn(async {
        sleep(Duration::from_secs(2)).await;
        println!("任务1 完成");
    });

    let handle2 = spawn(async {
        sleep(Duration::from_secs(1)).await;
        println!("任务2 完成");
    });

    handle1.await.unwrap();
    handle2.await.unwrap();
    println!("所有任务完成");
}
```

- **解释**：`spawn` 返回 JoinHandle，可通过 `.await` 等待结果或处理错误。任务在运行时线程池中并发执行。
- **输出**：任务2 先完成，然后任务1，主线程不阻塞。

### 高级任务：带返回值的任务
```rust
let handle = spawn(async {
    // 计算
    42
});

let result = handle.await.unwrap();
println!("结果: {}", result);
```

- **错误处理**：如果任务 panic，`.await` 返回 Err(JoinError)。
- **最佳实践**：使用 `tokio::task::spawn_blocking` 处理阻塞操作（如 CPU 密集任务），避免阻塞异步线程。

## 4. 共享状态（Shared State）

在异步环境中安全共享数据，使用 Arc 和 Mutex。

### 示例：计数器共享
```rust
use std::sync::{Arc, Mutex};
use tokio::{main, spawn};

#[main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];
    for _ in 0..10 {
        let counter_clone = counter.clone();
        handles.push(spawn(async move {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    println!("计数: {}", *counter.lock().unwrap());
}
```

- **解释**：Arc 允许多线程引用，Mutex 确保互斥访问。Tokio 的 sync 模块提供异步友好版本如 Mutex（支持 .await 锁）。
- **高级**：使用 Tokio 的 OnceCell 或 1.47 新增的 SetOnce 用于一次性初始化。

### 替代：原子类型
对于简单数据，使用 std::sync::atomic：
```rust
use std::sync::atomic::{AtomicU32, Ordering};

let counter = Arc::new(AtomicU32::new(0));
counter.fetch_add(1, Ordering::Relaxed);
```

## 5. 通道（Channels）

Tokio 提供 mpsc（多生产者单消费者）、oneshot 等通道用于任务间通信。

### 示例：mpsc 通道
```rust
use tokio::{main, spawn, sync::mpsc};

#[main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    spawn(async move {
        for i in 0..5 {
            tx.send(i).await.unwrap();
        }
    });

    while let Some(msg) = rx.recv().await {
        println!("接收: {}", msg);
    }
}
```

- **解释**：通道容量 32，发送方异步发送，接收方 .recv().await。
- **类型**：oneshot 用于单次通信；broadcast 用于多消费者。

### 高级：watch 通道
用于观察值变化：
```rust
use tokio::sync::watch;

let (tx, mut rx) = watch::channel("初始值");
tx.send("新值").unwrap();
println!("变化: {}", rx.changed().await.unwrap());
```

## 6. I/O 操作

Tokio 提供异步 I/O API，如 TcpStream、File 等。

### 示例：异步 TCP 服务器
```rust
use tokio::{main, net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}};

#[main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        spawn(async move {
            let mut buf = [0; 1024];
            let n = socket.read(&mut buf).await.unwrap();
            socket.write_all(&buf[0..n]).await.unwrap();
        });
    }
}
```

- **解释**：异步绑定、接受连接、读写。适用于网络应用。
- **文件 I/O**：使用 tokio::fs::File 的 async_read/write。

### 高级：UDP 和 Unix 套接字
Tokio 支持 UdpSocket 和 UnixStream，类似 TCP。

## 7. 异步深入（Async in Depth）

理解 Future、Pin 和上下文。

- **Future**：异步计算的 trait。Tokio 运行时轮询 Future。
- **示例**：自定义 Future（很少需要，但理解有用）。
- **合作调度**：1.47 新增 poll_proceed 允许长任务让出。

## 8. Select! 分支

`tokio::select!` 用于并发等待多个异步操作。

### 示例
```rust
use tokio::{main, select, time::{sleep, Duration}};

#[main]
async fn main() {
    select! {
        _ = sleep(Duration::from_secs(1)) => println!("超时1"),
        _ = sleep(Duration::from_secs(2)) => println!("超时2"),
    }
}
```

- **解释**：第一个完成的分支执行，其他取消。

## 9. 流处理（Streams）

Tokio 支持 Stream trait 用于异步迭代。

### 示例：TCP 流
```rust
use tokio::net::TcpStream;
use tokio_stream::StreamExt;

async fn process_stream(mut stream: TcpStream) {
    while let Some(item) = stream.next().await {
        // 处理
    }
}
```

- **实用**：与 channels 结合处理数据流。

## 10. 帧处理（Framing）

使用 codecs 处理协议帧，如 lines codec。

### 示例
```rust
use tokio_util::codec::{Framed, LinesCodec};
use tokio::net::TcpStream;

let framed = Framed::new(TcpStream::connect("...").await?, LinesCodec::new());
```

- **解释**：自动分割输入为帧。

## 11. 优雅关闭（Graceful Shutdown）

使用信号和 JoinSet 管理关闭。

### 示例
```rust
use tokio::{main, signal, spawn, task::JoinSet};

#[main]
async fn main() {
    let mut set = JoinSet::new();
    set.spawn(async { /* 任务 */ });

    signal::ctrl_c().await.unwrap();
    while let Some(_) = set.join_next().await {}
}
```

- **最佳**：广播关闭信号到所有任务。

## 12. 常见错误与最佳实践

扩展表格：

| 问题 | 原因 | 解决方案 |
|------|------|----------|
| 阻塞运行时 | 使用 sync 操作 | 用 spawn_blocking 包装 |
| Future 未 Pin | 移动 self-referential | 使用 pin! 宏 |
| 通道满 | 未接收 | 增加容量或使用 unbounded |
| 性能低 | 过多任务 | 限制并发，使用 semaphore |
| Panic 传播 | 未处理 JoinError | 用 .await.unwrap_or_else 处理 |
| I/O 错误 | 未处理 Result | 总是检查异步 API 的 Result |
| 调度不均 | 默认配置 | 调整 worker_threads |

- **最佳实践**：使用 tracing 集成日志；测试使用 tokio::test 宏；优先多线程运行时；定期更新到最新版本以获优化。 避免与 async-std 混用，因 API 不兼容。

## 13. 练习与高级主题

1. 构建异步 HTTP 客户端（使用 reqwest + Tokio）。
2. 实现聊天服务器，使用 mpsc 广播消息。
3. 添加超时到 I/O 操作，使用 select!。
4. 集成数据库（如 sqlx 的异步支持）。
5. 探索 Mio（底层 I/O）与 Tokio 结合。

高级主题：Tokio 与 Hyper/Tonic 集成构建 web/gRPC 服务；使用 metrics 监控性能；嵌入式应用配置当前线程运行时。

通过这个全面教程，你能掌握 Tokio 的核心。参考官方文档或 GitHub 以获取更多细节。 如需调试，提供代码反馈！