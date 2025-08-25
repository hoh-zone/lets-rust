## 1. Rust 异步编程简介

### 1.1 定义和目的
Rust 的异步编程模型允许代码在等待 I/O 或其他操作时不阻塞线程，而是通过 Future 来表示将来完成的值。核心是 `async` 关键字，用于定义异步函数或块，返回一个 `Future`。 目的：实现高效的并发 I/O，避免线程阻塞，提高吞吐量，尤其在服务器或网络应用中。 与同步代码不同，异步代码不立即执行，而是生成一个可轮询（poll）的 Future。

Rust 异步的核心组件：
- **Future trait**：表示异步计算，返回 Poll::Pending 或 Poll::Ready。
- **async/await**：语法糖，使异步代码像同步一样书写。
- **运行时**：如 Tokio，提供 executor 执行 Future。

### 1.2 与同步编程的区别
- **同步**：代码顺序执行，I/O 阻塞线程。
- **异步**：代码非阻塞，等待时切换任务，提高效率。
- **线程模型**：同步用多线程；异步用单线程或少线程 + event loop。
- **错误处理**：异步用 Result<Future> 或 anyhow；同步用 ?。

**何时选择异步？** 当程序有大量 I/O 操作时，如 web server；同步适合 CPU 密集任务。

## 2. 基础语法和概念

### 2.1 Future Trait
Future 是异步计算的核心：
```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct MyFuture {
    value: i32,
}

impl Future for MyFuture {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.value == 0 {
            Poll::Pending
        } else {
            self.value -= 1;
            if self.value == 0 {
                Poll::Ready(42)
            } else {
                Poll::Pending
            }
        }
    }
}
```
- `poll` 方法检查 Future 是否就绪。

### 2.2 async/await 语法
async 函数返回 Future：
```rust
async fn fetch_data() -> Result<String, Box<dyn std::error::Error>> {
    // 模拟 I/O
    Ok("data".to_string())
}

#[tokio::main]
async fn main() {
    let data = fetch_data().await.unwrap();
    println!("{}", data);
}
```
- `await` 等待 Future 完成。

### 2.3 运行时：Tokio 示例
安装 Tokio：`cargo add tokio --features full`
```rust
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            // 处理 socket
        });
    }
}
```
- Tokio 提供 executor 和 I/O 原语。

## 3. Pinning 和 Unpin

异步代码可能生成 self-referential Future，需要 Pin 固定内存。

### 3.1 Pin 示例
```rust
use std::pin::Pin;
use std::task::{Context, Poll};
use std::future::Future;

struct Delay {
    remaining: u32,
}

impl Future for Delay {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.remaining == 0 {
            Poll::Ready(())
        } else {
            self.remaining -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
```
- `Pin<&mut Self>` 确保不移动。

### 3.2 Unpin 类型
大多数类型自动 Unpin，不需 Pin。

## 4. 并发原语

### 4.1 Channels
```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    tokio::spawn(async move {
        tx.send("hello").await.unwrap();
    });
    println!("{}", rx.recv().await.unwrap());
}
```
- 多生产者单消费者。

### 4.2 Mutex 和 Arc
```rust
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));
    let counter_clone = counter.clone();
    tokio::spawn(async move {
        *counter_clone.lock().await += 1;
    });
    *counter.lock().await += 1;
    println!("{}", *counter.lock().await);
}
```
- 线程安全共享。

## 5. 错误处理

使用 anyhow 或 thiserror 处理异步错误：
```rust
use anyhow::{Result, anyhow};

async fn fetch() -> Result<String> {
    Err(anyhow!("Error"))
}

#[tokio::main]
async fn main() -> Result<()> {
    fetch().await?;
    Ok(())
}
```
- `?` 在 async 中传播错误。

## 6. Streams 和 Sinks

使用 futures 或 tokio_stream 处理流：
```rust
use tokio_stream::StreamExt;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    tx.send(1).await.unwrap();
    tx.send(2).await.unwrap();
    drop(tx);
    while let Some(item) = rx.recv().await {
        println!("{}", item);
    }
}
```
- 处理异步流。

## 7. 高级主题

### 7.1 Async Traits
在 trait 中定义 async fn：
```rust
trait AsyncService {
    async fn handle(&self, req: String) -> String;
}

struct Service;

impl AsyncService for Service {
    async fn handle(&self, req: String) -> String {
        format!("Handled: {}", req)
    }
}

#[tokio::main]
async fn main() {
    let s = Service;
    println!("{}", s.handle("request".to_string()).await);  // Handled: request
}
```
- 自 Rust 1.75，支持 async fn in traits。

### 7.2 Select 和 Join
使用 tokio::select! 处理多个 Future：
```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = sleep(Duration::from_secs(1)) => println!("Timer 1"),
        _ = sleep(Duration::from_secs(2)) => println!("Timer 2"),
    };
}
```
- 等待第一个完成。

## 8. 用例

- **Web 服务器**：处理并发请求。
- **I/O 操作**：文件、网络非阻塞。
- **数据库查询**：异步连接。
- **GUI 事件**：非阻塞 UI。
- **微服务**：高吞吐 API。

## 9. 最佳实践

- **选择运行时**：Tokio 适合生产；async-std 简单。
- **处理错误**：用 anyhow 简化。
- **避免阻塞**：用 async 原语替换 sync。
- **Pinning 处理**：了解 Unpin 类型。
- **测试**：用 tokio::test 测试 async。
- **文档**：说明 lifetime 和 Send/Sync。

## 10. 常见陷阱和错误

- **阻塞代码**：sync I/O 在 async 中阻塞运行时；用 async 版本。
- **Lifetime 错误**：async 借用需 'static 或 scoped。
- **Pinning 遗忘**：!Unpin Future 需 Pin；处理或用 Unpin 类型。
- **运行时缺失**：async fn 需 executor 如 tokio::main。
- **取消安全**：async 代码需考虑取消；用 drop 处理。