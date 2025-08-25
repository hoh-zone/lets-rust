# Rust Trait Future

`Future` trait 来自 `std::future` 模块，它是 Rust 异步编程的核心，用于表示一个异步计算的值或操作。它定义了一个 `poll` 方法，用于检查异步任务是否完成，并返回结果或继续等待。 与 `async/await` 语法结合，`Future` trait 是 Rust 非阻塞 I/O 和并发的基础。 `Future` 是 poll-based 的模型，允许运行时（如 Tokio）高效调度任务，而不阻塞线程。

`Future` 的设计目的是提供一个统一的异步抽象，支持从简单延迟到复杂网络操作的一切。它与 Pinning 系统集成，以处理 self-referential futures。

- **为什么需要 `Future`？** Rust 的异步模型避免线程阻塞，提高效率。`Future` 允许定义可轮询的异步任务，支持运行时调度。 例如，在处理网络请求时，`Future` 表示请求的完成，而不阻塞调用线程。

### 1.2 与相关 Trait 的区别
`Future` 是异步 trait 的核心，与几个相关 trait 和概念有区别：

- **与 `Iterator`**：
    - `Future`：异步 poll，返回 Poll::Pending/Ready；单值。
    - `Iterator`：同步 next，返回 Option；多值。
    - `Future` 如异步 Iterator；Stream 是异步 Iterator。
    - 区别：`Future` 非阻塞；`Iterator` 阻塞。

- **与 `Unpin`**：
    - `Future`：可能 !Unpin（self-referential）。
    - `Unpin`：标记 Future 可移动，即使 pinned。
    - `Unpin` 是 opt-out；大多数 Future Unpin。
    - 选择：Unpin Future 无 Pin 需求。

- **与 `Send` / `Sync`**：
    - `Future`：可 + Send/Sync 以线程安全。
    - Send/Sync 与并发相关；Future 与异步相关。
    - 示例：dyn Future + Send 支持跨线程。

**何时选择 `Future`？** 用 `Future` 定义异步操作；用 async/await 简化实现。

## 2. 手动实现 `Future`

`Future` 不能自动派生，必须手动实现。但实现简单：定义 `Output` 和 `poll`。

### 2.1 例子1: 简单立即就绪 Future
```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct SimpleFuture {
    value: i32,
}

impl Future for SimpleFuture {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(self.value)
    }
}

#[tokio::main]
async fn main() {
    let fut = SimpleFuture { value: 42 };
    println!("{}", fut.await);  // 42
}
```
- 立即返回 Ready。

### 2.2 例子2: 延迟 Future
```rust
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

#[tokio::main]
async fn main() {
    Delay { remaining: 3 }.await;
    println!("Done");
}
```
- poll 多次 Pending。

### 2.3 例子3: 泛型 Future
```rust
struct GenericFuture<T> {
    value: T,
}

impl<T: Copy> Future for GenericFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(self.value)
    }
}

#[tokio::main]
async fn main() {
    let fut = GenericFuture { value: "hello" };
    println!("{}", fut.await);
}
```
- 泛型 Output。

### 2.4 例子4: 错误处理 Future
```rust
use std::io::{Error, ErrorKind};

struct IoFuture;

impl Future for IoFuture {
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(Err(Error::new(ErrorKind::Other, "IO error")))
    }
}

#[tokio::main]
async fn main() {
    if let Err(e) = IoFuture.await {
        println!("Error: {}", e);
    }
}
```
- 返回 Result。

### 2.5 例子5: Self-Referential Future (需 Pin)
```rust
use std::marker::PhantomPinned;

struct SelfRefFuture {
    data: String,
    ptr: *const String,
    _pin: PhantomPinned,
}

impl SelfRefFuture {
    fn new(data: String) -> Self {
        Self { data, ptr: std::ptr::null(), _pin: PhantomPinned }
    }

    fn init(self: Pin<&mut Self>) {
        let this = unsafe { self.get_unchecked_mut() };
        this.ptr = &this.data;
    }
}

impl Future for SelfRefFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { &*self.ptr };
        Poll::Ready(this.clone())
    }
}

#[tokio::main]
async fn main() {
    let mut fut = SelfRefFuture::new("hello".to_string());
    let mut pinned = Pin::new(&mut fut);
    pinned.as_mut().init();
    println!("{}", pinned.await);  // hello
}
```
- 处理 self-ref，需要 Pin。

## 3. async/await 与 Future

async 是 Future 的语法糖，返回匿名 Future。

### 3.1 例子6: 简单 async fn
```rust
async fn hello() -> String {
    "hello".to_string()
}

#[tokio::main]
async fn main() {
    println!("{}", hello().await);
}
```
- async fn 返回 impl Future。

### 3.2 例子7: 异步 I/O
```rust
use tokio::fs::File;
use tokio::io::AsyncReadExt;

async fn read_file(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

#[tokio::main]
async fn main() {
    let contents = read_file("file.txt").await.unwrap();
    println!("{}", contents);
}
```
- 非阻塞文件读。

### 3.3 例子8: 组合 Futures
```rust
use futures::join;

async fn task1() -> i32 { 1 }

async fn task2() -> i32 { 2 }

#[tokio::main]
async fn main() {
    let (r1, r2) = join!(task1(), task2());
    println!("{}", r1 + r2);  // 3
}
```
- join 等待多个。

## 4. Pinning 和 Unpin

Future 可能 self-referential，需要 Pin 固定。

### 4.1 例子9: Unpin Future
```rust
use std::marker::Unpin;

struct UnpinFut(i32);

impl Future for UnpinFut {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(self.0)
    }
}

impl Unpin for UnpinFut {}
```
- 标记 Unpin，可无 Pin poll。

### 4.2 例子10: !Unpin Future
使用 PhantomPinned 禁用 Unpin。

### 4.3 例子11: Pin 使用
```rust
let mut fut = Delay { remaining: 3 };
let pinned = Pin::new(&mut fut);
pinned.poll(&mut cx)
```
- Pin &mut Future 以 poll。

## 5. Futures 组合

### 5.1 例子12: and_then
```rust
async fn task() -> i32 { 5 }

let chained = task().and_then(|x| async move { x + 1 });
println!("{}", chained.await);  // 6
```
- 链式 Future。

### 5.2 例子13: select
```rust
use futures::select;

select! {
    a = task1().fuse() => println!("Task1"),
    b = task2().fuse() => println!("Task2"),
};
```
- 等待第一个完成。

## 6. 运行时和 Executor

运行时执行 Future。

### 6.1 例子14: Tokio Executor
```rust
use tokio::runtime::Runtime;

let rt = Runtime::new().unwrap();
rt.block_on(async {
    println!("Hello from Tokio");
});
```
- 自定义 runtime。

### 6.2 例子15: Custom Executor
简单 executor：
```rust
use std::collections::VecDeque;

struct MiniTokio {
    tasks: VecDeque<Pin<Box<dyn Future<Output = ()> + Send>>>,
}

impl MiniTokio {
    fn new() -> Self { MiniTokio { tasks: VecDeque::new() } }

    fn spawn(&mut self, fut: impl Future<Output = ()> + Send + 'static) {
        self.tasks.push_back(Box::pin(fut));
    }

    fn run(&mut self) {
        let mut cx = Context::from_waker(&nop_waker());
        while let Some(mut task) = self.tasks.pop_front() {
            if task.as_mut().poll(&mut cx) == Poll::Pending {
                self.tasks.push_back(task);
            }
        }
    }
}
```
- 自定义 poll 循环。

## 7. 高级主题

### 7.1 自定义 Future 组合
实现 join 或 race。

### 7.2 Stream 和 Sink
异步 Iterator。

## 8. 常见用例

- **网络请求**：http client Future。
- **延迟操作**：timer Future。
- **任务链**：and_then 处理结果。
- **并发**：join all Futures。
- **自定义 async**：poll-based I/O。

## 9. 最佳实践

- **用 async/await**：简化 Future 实现。
- **处理 Pin**：!Unpin 用 Pin。
- **运行时选择**：Tokio 生产。
- **文档**：说明 poll 语义。
- **测试**：用 futures-test 测试 poll。
- **性能**：避免不必要 Pin。

## 10. 常见陷阱和错误

- **无 Pin poll**：self-ref 导致 UB；用 Pin。
- **Pending 遗忘 waker**：不 wake 导致挂起；wake_by_ref。
- **运行时缺失**：Future 需 executor。
- **Lifetime 错误**：Future 借用需 'static。
- **!Unpin 移动**：意外移动导致 UB；Pin 保护。