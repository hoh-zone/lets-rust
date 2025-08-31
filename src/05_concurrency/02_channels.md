# std::sync::mpsc 模块教程

Rust 的 `std::sync::mpsc` 模块是标准库中实现多生产者单消费者（Multi-Producer Single-Consumer）通道的核心组成部分，提供 `Sender`、`Receiver`、`channel` 等类型和函数，用于线程间安全通信和数据传输。它抽象了底层同步机制（如 Mutex 和 Condvar），确保跨平台兼容性，并通过 `std::sync::mpsc::RecvError`、`std::sync::mpsc::SendError` 或 `std::sync::mpsc::TryRecvError` 显式处理错误如通道断开、超时或阻塞失败。`std::sync::mpsc` 强调 Rust 的并发安全：通道使用所有权转移发送数据，避免共享 mutable 状态；接收端独占消费，防止竞争；支持 bounded/unbounded 通道以控制背压。模块的设计优先简单性和可靠性，适用于同步线程通信（异步用 tokio::sync::mpsc 或 futures::channel），并提供 try_send/try_recv 以非阻塞操作。`std::sync::mpsc` 与 `std::thread`（线程创建）、`std::sync`（其他同步如 Arc）、`std::time`（超时接收）、`std::panic`（断开通道）和 `std::os`（OS 特定集成如 select）深度集成，支持高级模式如广播通道模拟和错误恢复。


## 1. std::sync::mpsc 简介

- **导入和高级结构**：除了基本导入 `use std::sync::mpsc::{self, channel, Sender, Receiver, TryRecvError, RecvTimeoutError};`，高级用法可包括 `use std::sync::mpsc::{SyncSender, TrySendError};` 以访问同步变体，以及 `use std::sync::mpsc::Select;` 以多通道选择（deprecated，但扩展用 select crate 替代）。模块的内部结构包括通道的 Arc<Mutex<Queue>> 实现（unbounded 用 VecDeque，bounded 用 Condvar 等待）、错误类型层次（SendError<T> 包含数据以回收）和 Select 的多路复用（内部 poll）。
    - **类型详解**：
        - `Sender<T>`：异步发送端，支持 clone 以多生产者；send() 阻塞于 bounded。
        - `Receiver<T>`：独占接收端，支持 recv()/try_recv()/recv_timeout()。
        - `SyncSender<T>`：同步发送端（bounded channel），try_send() 非阻塞。
        - `Select`：多 Receiver 选择（deprecated，用 crossbeam-select）。
        - `RecvError`/`SendError<T>`/`TryRecvError`/`RecvTimeoutError`/`TrySendError<T>`：错误类型，支持 into_inner() 回收数据。
    - **函数详解**：`channel::<T>()`（unbounded 返回 (Sender, Receiver)）、`sync_channel::<T>(bound: usize)`（bounded）、`Sender::clone`（多 Sender）。
    - **宏**：无，但相关如 std::sync::mpsc 在宏扩展用于 Select。
- **设计哲学扩展**：`std::sync::mpsc` 遵循 "one receiver" 以简化所有权；bounded 提供背压防 OOM；错误回收 SendError<T> 中的 T 避免丢失；无内置 broadcast（用 broadcast-channel crate）。通道是 MPSC，但 clone Sender 支持 MP。
- **跨平台详解**：Windows 用 SRWLock/ConditionVariable，Unix 用 pthread_mutex/cond；bounded 等待 Unix futex-like，Windows WaitForSingleObject；测试差异用 CI，焦点 Condvar 唤醒顺序。
- **性能详析**：send/recv ~100-500ns (锁争用)；unbounded VecDeque 分配，bounded Condvar 等待 ~1us；多 Sender clone Arc 计数；大消息复制开销，用 Arc<T> 共享。
- **常见用例扩展**：任务队列（Web worker）、生产者消费者、GUI 事件、测试通道 mock、分布式 actor 模拟。
- **超级扩展概念**：与 std::sync::Arc 集成共享发送；与 std::panic::catch_unwind 捕获发送 panic；错误链用 thiserror 自定义；与 flume::bounded 高性能替代；高吞吐用 crossbeam::channel 无锁；与 tracing::instrument 装饰通道日志；历史：从 1.0 channel 到 1.5 TrySendError 的回收支持。

## 2. 创建通道：channel 和 sync_channel

`channel` 是 unbounded，`sync_channel` 是 bounded。

### 示例：基本 channel（MPSC 扩展）
```rust
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (tx, rx) = channel::<i32>();

    thread::spawn(move || tx.send(42).unwrap());

    let val = rx.recv().unwrap();
    println!("接收: {}", val);
}
```

- **解释**：`channel` 返回 (Sender, Receiver)。`send` 转移所有权。性能：unbounded 无阻塞。

### 示例：Multi Producer（clone Sender 扩展）
```rust
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (tx, rx) = channel::<String>();
    let tx2 = tx.clone();

    thread::spawn(move || tx.send("from 1".to_string()).unwrap());
    thread::spawn(move || tx2.send("from 2".to_string()).unwrap());

    println!("1: {}", rx.recv().unwrap());
    println!("2: {}", rx.recv().unwrap());
}
```

- **解释**：`clone` 创建新 Sender（Arc 内部）。顺序不定。陷阱：drop 所有 Sender 断开 rx。

### 示例：Bounded sync_channel（背压扩展）
```rust
use std::sync::mpsc::sync_channel;
use std::thread;

fn main() {
    let (tx, rx) = sync_channel::<i32>(2);  // 缓冲 2

    tx.send(1).unwrap();
    tx.send(2).unwrap();
    // tx.send(3).unwrap();  // 阻塞直到 recv

    thread::spawn(move || {
        println!("接收: {}", rx.recv().unwrap());
    });

    tx.send(3).unwrap();  // 现在发送
}
```

- **解释**：`sync_channel` 限缓冲。send 满时阻塞。性能：Condvar 等待。

### 示例：TrySend/TryRecv 非阻塞（扩展变体）
```rust
use std::sync::mpsc::sync_channel;

fn main() {
    let (tx, rx) = sync_channel(1);
    tx.try_send(1).unwrap();  // Ok
    if let Err(e) = tx.try_send(2) {
        println!("满: {:?}", e.kind());  // WouldBlock
        println!("回收: {:?}", e.into_inner());  // 2
    }

    println!("try_recv: {:?}", rx.try_recv());  // Ok(1)
    println!("try_recv 空: {:?}", rx.try_recv());  // Err(Empty)
}
```

- **解释**：`try_send` 返回 TrySendError（WouldBlock/Disconnected）。`into_inner` 回收数据。扩展：轮询 try_recv 用于事件循环。

### 示例：RecvTimeout（有时限扩展）
```rust
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    let (tx, rx) = channel();
    match rx.recv_timeout(Duration::from_secs(1)) {
        Ok(v) => println!("值: {}", v),
        Err(RecvTimeoutError::Timeout) => println!("超时"),
        Err(RecvTimeoutError::Disconnected) => println!("断开"),
    }
}
```

- **解释**：`recv_timeout` 限时阻塞。错误分类 Timeout/Disconnected。

## 3. 通道错误和断开

通道错误支持回收和分类。

### 示例：SendError 回收（发送失败扩展）
```rust
use std::sync::mpsc::channel;

fn main() {
    let (tx, rx) = channel::<Vec<i32>>();
    drop(rx);  // 断开

    if let Err(e) = tx.send(vec![1, 2]) {
        let data = e.into_inner();
        println!("回收: {:?}", data);  // [1, 2]
    }
}
```

- **解释**：`SendError::into_inner` 返回 T。性能：无额外分配。

### 示例：RecvError 和 Disconnected（接收失败扩展）
```rust
use std::sync::mpsc::channel;

fn main() {
    let (tx, rx) = channel::<()>();
    drop(tx);

    match rx.recv() {
        Ok(_) => {},
        Err(RecvError) => println!("所有 Sender 掉"),
    }
}
```

- **解释**：`RecvError` 表示断开。扩展：用 try_recv 循环检查。

### 示例：TryRecvError 分类（非阻塞扩展）
```rust
use std::sync::mpsc::sync_channel;

fn main() {
    let (_, rx) = sync_channel::<i32>(0);
    match rx.try_recv() {
        Ok(v) => println!("值: {}", v),
        Err(TryRecvError::Empty) => println!("空"),
        Err(TryRecvError::Disconnected) => println!("断开"),
    }
}
```

- **解释**：`Empty` 表示无消息但连接；`Disconnected` 结束。

## 4. 高级通道：Select、Clone 和 集成

- Select：多通道选择（deprecated）。

### 示例：Select 多路（替代扩展）
```rust
use std::sync::mpsc::channel;

fn main() {
    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();

    tx1.send("chan1").unwrap();
    tx2.send("chan2").unwrap();

    let sel = std::sync::mpsc::Select::new();
    let oper1 = sel.recv(&rx1);
    let oper2 = sel.recv(&rx2);

    let ready = sel.ready();
    match ready {
        id if id == oper1 => println!("rx1: {}", rx1.recv().unwrap()),
        id if id == oper2 => println!("rx2: {}", rx2.recv().unwrap()),
        _ => {},
    }
}
```

- **解释**：`Select` 添加 recv 操作，ready() 返回 ID。deprecated，用 crossbeam::Select。

### 示例：与 thread 集成（生产消费扩展）
```rust
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (tx, rx) = channel();

    thread::spawn(move || {
        for i in 0..5 {
            tx.send(i).unwrap();
        }
    });

    for val in rx {
        println!("消费: {}", val);
    }
}
```

- **解释**：rx 作为 Iterator。drop tx 结束循环。

## 5. OS 扩展和 Raw

通道无直接 OS，但线程集成。

（类似 thread，扩展通道在 OS 线程通信）。

## 6. 高级主题：Bounded 背压、Error 恢复和 集成

- 背压：bounded 限生产。

### 示例：背压控制（速率限扩展）
```rust
use std::sync::mpsc::sync_channel;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = sync_channel(3);

    thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(100));  // 生产慢
        }
    });

    for val in rx {
        println!("消费: {}", val);
        thread::sleep(Duration::from_millis(500));  // 消费慢，背压生产
    }
}
```

- **解释**：缓冲满 tx 阻塞。扩展：用 try_send 丢弃旧消息。

## 7. 最佳实践和常见陷阱

- **通道最佳**：unbounded 简单，bounded 防 OOM；clone Sender 限数；try_* 非阻塞。
- **性能**：unbounded Vec 分配，bounded Condvar 等待；大 T 用 Arc<T>。
- **错误**：Disconnected 用；回收 into_inner 避免丢失。
- **安全**：通道 Send/Sync T 要求；panic 断开通道。
- **跨平台**：Condvar 行为一致。
- **测试**：loom channel race；mock send 测试逻辑。
- **资源**：drop 通道关闭资源。
- **常见扩展**：
    - Disconnected：所有 tx drop。
    - WouldBlock：try_send 满。
    - Timeout：recv_timeout 用 Instant 精确。

## 8. 练习建议

1. 编写 MPSC 队列：bounded channel，multi thread send，single recv。
2. 实现 broadcast：用 channel<Vec<Sender>> 克隆。
3. 创建 rate limiter：用 sync_channel(1) + sleep 限速。
4. 处理 error 恢复：send Err 用 into_inner 重发。
5. 基准：比较 mpsc vs crossbeam channel 吞吐，用 Instant。
6. 与 thread：用 channel 线程通信，scope 借用通道。
7. 错误框架：mock Disconnected 测试消费重试。
8. 高级 app：实现 actor 系统：channel 消息，thread 处理。
