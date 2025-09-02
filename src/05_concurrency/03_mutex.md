# std::sync::Mutex 模块教程

Rust 的 `std::sync::Mutex` 类型是标准库 `std::sync` 模块中实现互斥锁（Mutual Exclusion Lock）的核心组成部分，提供 `Mutex<T>` 和 `MutexGuard<'a, T>` 等类型，用于保护共享数据在多线程环境中的安全访问。它抽象了底层 OS 同步原语（如 Unix 的 pthread_mutex 和 Windows 的 SRWLock 或 CriticalSection），确保跨平台兼容性，并通过 `std::sync::Mutex::lock` 返回 `Result<MutexGuard<'a, T>, PoisonError<MutexGuard<'a, T>>>` 显式处理错误如锁中毒（poisoning，当持有锁的线程 panic 时）。`std::sync::Mutex` 强调 Rust 的并发安全原则：通过 RAII（Resource Acquisition Is Initialization）模式和借用检查器确保锁的自动释放，防止死锁和数据竞争；支持泛型 T 的保护，T 无需 Send/Sync（Mutex 自身是 Sync）；提供 try_lock 以非阻塞尝试获取锁。模块的设计优先简单性和低开销，适用于同步线程共享状态（异步用 tokio::sync::Mutex 或 parking_lot::Mutex），并支持锁中毒恢复机制以允许继续使用中毒锁。`std::sync::Mutex` 与 `std::sync::Arc`（共享引用计数，用于多线程所有权）、`std::thread`（线程创建和加入）、`std::sync::Condvar`（条件变量结合实现监视器模式）、`std::panic`（毒锁传播 panic 信息）和 `std::os`（OS 特定锁扩展如 pthread_mutexattr）深度集成，支持高级并发模式如读者-写者锁模拟（用 RwLock 替代）和错误恢复。


## 1. std::sync::Mutex 简介

- **导入和高级结构**：除了基本导入 `use std::sync::{Mutex, MutexGuard, PoisonError};`，高级用法可包括 `use std::sync::TryLockError;` 以处理 try_lock 错误，以及 `use std::sync::LockResult;` 以别名 Result<Guard, PoisonError>。模块的内部结构包括 Mutex 的 Arc<OsMutex> 实现（OS 依赖，如 Unix pthread_mutex_t、Windows SRWLOCK）、MutexGuard 的 RAII Drop（自动解锁）和 PoisonError 的 Box<Any + Send + 'static> payload（panic 信息）。
    - **类型详解**：
        - `Mutex<T>`：泛型互斥锁，支持 new(T)、lock() 返回 LockResult<MutexGuard<'a, T>>、try_lock() 返回 TryLockResult<MutexGuard<'a, T>>、is_poisoned() 检查中毒状态、get_mut(&mut self) 以独占访问内部 T（非线程安全时用）。
        - `MutexGuard<'a, T>`：锁守卫，实现 Deref/DerefMut 以透明访问 &T/&mut T；Drop 时解锁；支持 map/unmap (1.49+) 以子字段锁定。
        - `PoisonError<G>`：中毒错误，支持 into_inner(G) 忽略毒继续、get_ref(&G) 访问守卫。
        - `TryLockError<G>`/`TryLockResult<G>`：try_lock 错误，分类 Poisoned/TryLockError::WouldBlock。
        - `LockResult<G>`：lock 的 Result<G, PoisonError<G>>。
    - **函数和方法扩展**：`Mutex::new` 创建；高级如 Mutex::clear_poison (future) 手动清除毒。
    - **宏**：无，但相关如 std::sync 在宏扩展用于 lock! (future proposal)。
- **设计哲学扩展**：`std::sync::Mutex` 遵循 "pay for what you use"，锁开销低但争用高；毒机制允许恢复而非禁用锁；Guard RAII 防忘记解锁；无内置公平锁（用 parking_lot 替代）。Mutex 是 Sync 但 T 无需，允许非 Send T（如 Rc）在单线程用。
- **跨平台详解**：Windows SRWLock (轻量，1.15+) vs CriticalSection (fallback)；Unix pthread_mutex (recursive 默认 no，errorcheck 用 cfg) vs futex (Linux 优化)；测试差异用 CI，焦点锁公平性和优先级继承。
- **性能详析**：lock/unlock ~10-50ns 无争用；争用下 ~1us-10ms (自旋+等待)；Guard deref 零开销；大 T 锁复制开销，用 &mut T。基准用 criterion，profile 用 perf (Linux)/VTune (Windows)。
- **常见用例扩展**：共享计数器（Web 请求统计）、配置缓存（热重载）、数据库连接池（互斥借用）、游戏状态同步（多线程更新）。
- **超级扩展概念**：与 std::sync::RwLock 对比读者多模式；与 std::panic::AssertUnwindSafe 安全毒恢复；错误链用 thiserror 自定义 Poison；与 parking_lot::Mutex 高性能无毒替代；高吞吐用 spin::Mutex 自旋；与 tracing::instrument 装饰锁获取日志；历史：从 1.0 Mutex 到 1.38 try_lock 优化以及 future 的 fair mutex proposal。

## 2. 创建和获取锁：Mutex::new 和 lock

`Mutex::new` 创建，`lock` 获取守卫。

### 示例：基本 Mutex 使用（共享计数器扩展）
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("结果: {}", *counter.lock().unwrap());  // 10
}
```

- **解释**：`Mutex::new` 初始化 T。`lock` 返回 MutexGuard。`unwrap` 忽略毒。性能：Arc 计数 ~10ns。

### 示例：Try Lock 非阻塞（try_lock 扩展）
```rust
use std::sync::Mutex;

fn main() {
    let mutex = Mutex::new(42);
    match mutex.try_lock() {
        Ok(guard) => println!("获取: {}", *guard),
        Err(e) => println!("失败: {:?}", e.kind()),  // WouldBlock 或 Poisoned
    }
}
```

- **解释**：`try_lock` 返回 TryLockResult。`kind` 分类。扩展：轮询 try_lock + backoff 退避避免忙等。

### 示例：Poison 处理和恢复（毒锁扩展）
```rust
use std::sync::Mutex;
use std::panic;

fn main() {
    let mutex = Mutex::new(0);
    let _ = panic::catch_unwind(|| {
        let _guard = mutex.lock().unwrap();
        panic!("毒");
    });

    let guard = mutex.lock();
    if guard.is_err() {
        let poisoned = guard.unwrap_err();
        println!("毒: {}", mutex.is_poisoned());  // true
        let inner = poisoned.into_inner();
        println!("恢复: {}", *inner);  // 0，忽略毒
    }
}
```

- **解释**：panic 毒锁。`is_poisoned` 检查。`into_inner` 恢复守卫。性能：毒标志原子检查。

### 示例：MutexGuard Map（子字段锁扩展）
```rust
use std::sync::Mutex;

struct Data {
    a: i32,
    b: String,
}

fn main() {
    let mutex = Mutex::new(Data { a: 1, b: "hello".to_string() });
    let mut guard = mutex.lock().unwrap();
    let a_mut = MutexGuard::map(guard, |d| &mut d.a);
    *a_mut += 10;
    drop(a_mut);  // 解锁 a，但整体锁仍持
    // guard.b 仍可访问
}
```

- **解释**：`MutexGuard::map` 映射子字段。扩展：unmap 恢复全守卫。

## 3. Mutex 在多线程：Arc 和 集成

Mutex 常与 Arc 共享。

### 示例：高级计数器（争用分析扩展）
```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let start = Instant::now();
    let mut handles = vec![];

    for _ in 0..100 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..10000 {
                *counter.lock().unwrap() += 1;
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("计: {}", *counter.lock().unwrap());
    println!("时间: {:?}", start.elapsed());  // 分析争用
}
```

- **解释**：高争用慢。性能：锁时间主导，优化用 atomic。

### 示例：与 Condvar 集成（监视器模式扩展）
```rust
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }
    println!("启动");
}
```

- **解释**：Mutex + Condvar 等待条件。扩展：wait_timeout 限时。

### 示例：与 Thread 集成（后台任务扩展）
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let state = Arc::new(Mutex::new("初始".to_string()));
    let state_clone = Arc::clone(&state);

    let handle = thread::spawn(move || {
        let mut guard = state_clone.lock().unwrap();
        *guard = "更新".to_string();
    });

    handle.join().unwrap();
    println!("状态: {}", *state.lock().unwrap());
}
```

- **解释**：线程更新共享状态。扩展：用 channel 通知更新。

## 4. 错误和毒锁：PoisonError

毒锁是 panic 保护。

### 示例：毒锁恢复高级（get_mut 扩展）
```rust
use std::sync::Mutex;

fn main() {
    let mutex = Mutex::new(vec![1, 2]);
    {
        let mut guard = mutex.lock().unwrap();
        guard.push(3);
        if true { panic!("模拟毒"); }
    }  // panic 毒锁

    if mutex.is_poisoned() {
        let mut data = mutex.get_mut().unwrap();  // &mut 内数据（非线程时）
        data.clear();  // 恢复
    }

    let guard = mutex.lock().expect("毒");
    println!("恢复数据: {:?}", *guard);
}
```

- **解释**：`get_mut` 独占访问修复。性能：is_poisoned 原子。

### 示例：TryLockError 分类（非阻塞扩展）
```rust
use std::sync::Mutex;

fn main() {
    let mutex = Mutex::new(0);
    match mutex.try_lock() {
        Ok(g) => println!("获取: {}", *g),
        Err(TryLockError::Poisoned(e)) => println!("毒: {:?}", e.into_inner()),
        Err(TryLockError::WouldBlock) => println!("阻塞"),
    }
}
```

- **解释**：`Poisoned` 子错误；`WouldBlock` 忙。扩展：轮询 + exp backoff。

## 5. OS 扩展：MutexExt 和 Raw

平台特定锁。

### 示例：Unix MutexAttr（属性扩展）
```rust
#[cfg(unix)]
use std::os::unix::sync::MutexExt;
#[cfg(unix)]
use std::sync::Mutex;

#[cfg(unix)]
fn main() {
    let mutex = Mutex::new(0);
    // pthread_mutexattr_settype 等 unsafe
}
```

- **解释**：扩展 raw pthread_mutex。扩展：用 priority inheritance 防优先级反转。

### 示例：Windows MutexAttr（扩展）
```rust
#[cfg(windows)]
use std::os::windows::sync::MutexExt;
#[cfg(windows)]
use std::sync::Mutex;

#[cfg(windows)]
fn main() {
    let mutex = Mutex::new(0);
    // InitializeCriticalSectionAndSpinCount unsafe
}
```

- **解释**：扩展 spin count 优化忙锁。

## 6. 高级主题：Guard Map、Poison 恢复和 集成

- Map：子锁。
- Poison：恢复。

### 示例：Guard Map 高级（嵌套扩展）
```rust
use std::sync::Mutex;

struct Nested {
    inner: Mutex<i32>,
}

fn main() {
    let outer = Mutex::new(Nested { inner: Mutex::new(42) });
    let guard = outer.lock().unwrap();
    let inner_guard = MutexGuard::map(guard, |n| &n.inner);
    println!("内: {}", *inner_guard.lock().unwrap());
}
```

- **解释**：map 嵌套锁。扩展：unmap 回外守卫。

### 示例：与 Arc/Thread 集成（池扩展）
```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::VecDeque;

struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    sender: mpsc::Sender<Box<dyn FnOnce() + Send>>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let receiver = Arc::clone(&receiver);
            workers.push(thread::spawn(move || loop {
                let task = receiver.lock().unwrap().recv().unwrap();
                task();
            }));
        }

        ThreadPool { workers, sender }
    }
}

fn main() {
    let pool = ThreadPool::new(4);
    pool.sender.send(Box::new(|| println!("任务"))).unwrap();
}
```

- **解释**：Mutex 保护 receiver。扩展：用 Condvar 唤醒。

## 7. 最佳实践和常见陷阱

- **锁最佳**：短持守卫；try_lock 非阻塞；map 子字段减粒度。
- **性能**：parking_lot 快 2x；try_lock 退避 exp。
- **错误**：毒恢复 into_inner；WouldBlock 重试。
- **安全**：Guard RAII 防泄漏；毒标志防坏数据。
- **跨平台**：SRWLock Windows 轻；pthread Unix  robust。
- **测试**：loom 锁 race；mock Mutex 测试逻辑。
- **资源**：Guard drop 解锁；毒不影响。
- **常见扩展**：
    - Deadlock：循环锁顺序一致。
    - Poison：panic 后恢复数据。
    - Contention：用 RwLock 读多。
    - Overflow：大 T 锁用 &T。

## 8. 练习建议

1. 编写锁池：Mutex<Vec<T>> 借用/返回。
2. 实现监视器：Mutex + Condvar 队列。
3. 创建子锁：用 map 嵌套 Mutex。
4. 处理毒恢复：catch panic，into_inner 清数据。
5. 基准：比较 Mutex vs parking_lot 争用时间，用 criterion。
6. 与 thread：用 Mutex 共享，scope 借用锁。
7. 错误框架：mock Poison 测试恢复逻辑。
8. 高级 app：实现 DB 连接池：Mutex<HashMap> 管理连接。

