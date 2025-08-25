# Rust async/await

Rust 的 async/await 是异步编程的语法糖，它使异步代码像同步代码一样易读和编写。async/await 构建在 Future trait 之上，允许开发者定义异步函数，并使用 await 来暂停执行直到 Future 就绪。 与同步代码不同，async/await 非阻塞线程，而是通过运行时调度任务，提高 I/O 密集应用的效率。 async/await 自 Rust 1.39 起稳定，是 Rust 异步生态的核心。

## 1. Rust async/await 简介

### 1.1 定义和目的
async/await 是 Rust 异步编程的语法糖：
- **async fn**：定义异步函数，返回一个 Future。
- **await**：暂停当前任务，等待 Future 完成，返回其 Output。
- 目的：使异步代码更易读，像同步一样书写，而无需手动 poll Future。 它解决异步回调地狱，提供线性代码流。 async/await 基于 generator 实现，每个 await 点是潜在暂停点。

### 1.2 与同步编程的区别
- **同步**：顺序执行，I/O 阻塞。
- **异步**：非阻塞，await 时切换任务。
- **运行时**：async 需要 executor 如 Tokio 执行。
- **错误**：async 用 ? 传播 Result<Future>。

**何时选择 async/await？** I/O 密集任务，如 web 服务；同步适合 CPU 密集。

## 2. 基础语法

### 2.1 async fn 和 await
async fn 返回 impl Future：
- 例子1: 简单 async fn
```rust
async fn hello() -> String {
    "hello".to_string()
}

#[tokio::main]
async fn main() {
    println!("{}", hello().await);
}
```
- await 等待完成。

- 例子2: async 块
```rust
#[tokio::main]
async fn main() {
    let result = async { 42 }.await;
    println!("{}", result);
}
```
- 匿名 async。

### 2.2 与 Future 的关系
async 是 Future 语法糖：
- 例子3: 手动 poll vs await
```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

struct ManualFut;

impl Future for ManualFut {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(42)
    }
}

#[tokio::main]
async fn main() {
    let fut = ManualFut;
    println!("{}", fut.await);
}
```
- await 内部 poll。

## 3. 运行时集成

async 需要运行时执行。

### 3.1 Tokio 运行时
- 例子4: Tokio main
```rust
#[tokio::main]
async fn main() {
    println!("Hello Tokio");
}
```
- 属性启动 runtime。

- 例子5: 自定义 runtime
```rust
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        println!("Hello");
    });
}
```
- 手动 runtime。

### 3.2 async-std 运行时
- 例子6: async-std main
```rust
use async_std::task;

fn main() {
    task::block_on(async {
        println!("Hello async-std");
    });
}
```
- 替代 runtime。

## 4. 并发和同步原语

### 4.1 Spawn 任务
- 例子7: Spawn
```rust
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        "spawned"
    });
    println!("{}", handle.await.unwrap());
}
```
- 并发任务。

### 4.2 Channels
- 例子8: mpsc
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
- 通信。

### 4.3 Mutex
- 例子9: Mutex
```rust
use tokio::sync::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let m = Arc::new(Mutex::new(0));
    let m2 = m.clone();
    tokio::spawn(async move {
        *m2.lock().await += 1;
    });
    *m.lock().await += 1;
    println!("{}", *m.lock().await);
}
```
- 共享状态。

## 5. 错误处理

### 5.1 Result in async
- 例子10: ? in async
```rust
async fn fetch() -> Result<String, anyhow::Error> {
    Ok("data".to_string())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let data = fetch().await?;
    println!("{}", data);
    Ok(())
}
```
- 传播错误。

### 5.2 anyhow
- 例子11: anyhow
```rust
use anyhow::{Result, anyhow};

async fn error_task() -> Result<()> {
    Err(anyhow!("error"))
}

#[tokio::main]
async fn main() {
    if let Err(e) = error_task().await {
        println!("Error: {}", e);
    }
}
```
- 简单错误。

## 6. Streams 和 Sinks

### 6.1 Stream
- 例子12: iter stream
```rust
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut stream = tokio_stream::iter(vec![1, 2, 3]);
    while let Some(item) = stream.next().await {
        println!("{}", item);
    }
}
```
- 异步 iter。

### 6.2 Sink
- 例子13: sink send
```rust
use futures::sink::SinkExt;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (mut tx, rx) = mpsc::channel(32);
    tx.send("hello").await.unwrap();
    drop(tx);
    // rx 消费
}
```
- 发送到 sink。

## 7. 高级主题

### 7.1 Async Traits
- 例子14: async trait
```rust
trait AsyncService {
    async fn handle(&self) -> String;
}

struct Service;

impl AsyncService for Service {
    async fn handle(&self) -> String {
        "handled".to_string()
    }
}

#[tokio::main]
async fn main() {
    let s = Service;
    println!("{}", s.handle().await);
}
```
- 异步 trait。

### 7.2 Custom Await
- 例子15: await chain
```rust
async fn chain() {
    let data = fetch().await.unwrap();
    process(data).await;
}
```
- 链式 await。

## 8. 常见用例

- **Web 客户端**：fetch URL。
- **服务器**：处理请求。
- **数据库**：异步查询。
- **定时任务**：delay。
- **并发 I/O**：多 fetch。

## 9. 最佳实践

- **用 Tokio**：生产运行时。
- **Pinning 处理**：!Unpin 用 Box::pin。
- **错误统一**：anyhow/thiserror。
- **测试**：tokio::test。
- **文档**：lifetime 和 Send。
- **性能**：避免阻塞操作。

## 10. 常见陷阱和错误

- **阻塞**：sync in async 阻塞 runtime；用 async。
- **Lifetime**：借用需 'static。
- **Pinning**：!Unpin 需 Pin。
- **无 runtime**：async 不执行；用 main。
- **取消**：drop Future 取消；处理 cleanup。