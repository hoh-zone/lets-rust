# Rust std::thread 模块教程

Rust 的 `std::thread` 模块是标准库中实现多线程并发编程的根本支柱，提供 `Thread`、`JoinHandle`、`Builder`、`Scope` 等类型和函数，用于线程的创建、配置、管理、同步和错误处理。它抽象了底层操作系统线程机制（如 POSIX pthread、Windows CreateThread 和其他平台的对应实现），确保跨平台兼容性，并通过 `std::io::Result`、`std::thread::Result` 或专用 panic 传播机制显式处理错误如线程资源耗尽、栈溢出或 OS 级失败。`std::thread` 强调 Rust 的核心安全原则：通过 move 闭包和借用检查器防止数据竞争；panic 在线程边界隔离，但可通过 JoinHandle 捕获和传播；支持线程本地存储（TLS）和作用域线程以简化借用。模块的设计优先简单性和可靠性，适用于同步阻塞场景（异步用 tokio 或 async-std），并提供 Builder 以自定义线程属性如栈大小、名称和优先级（OS 扩展）。`std::thread` 与 `std::sync`（共享状态如 Mutex/Arc）、`std::panic`（钩子和捕获）、`std::time`（延时和超时）、`std::os`（OS 特定线程扩展如 pthread_attr）和 `std::env`（环境继承）深度集成，支持高级并发模式如线程池模拟和条件等待。

## 1. std::thread 简介

- **导入和高级结构**：除了基本导入 `use std::thread::{self, Builder, JoinHandle, Scope, ScopedJoinHandle, Thread, ThreadId};`，高级用法可包括 `use std::thread::AvailableParallelism;` 以查询系统并发度，以及 `use std::thread::panicking;` 以检查线程 panic 状态。模块的内部结构包括线程句柄的原子引用计数、OS 依赖的线程创建（通过 sys 内部模块）和 panic 传播的 Box<dyn Any + Send> 机制。
    - **类型详解**：
        - `Thread`：线程元数据容器，支持 name()（Option<&str>）、id()（ThreadId）、park_timeout_ms (Unix 特定，1.72+)、is_finished()（检查 join 状态，1.63+）。
        - `JoinHandle<T>`：泛型结果句柄，支持 thread() 返回 Thread、join() 以阻塞等待并返回 Result<T, Box<dyn Any + Send + 'static>>。
        - `Builder`：链式配置器，支持 stack_size(usize)、name(String)、affinity (OS 特定，future)、priority (OS 扩展)。
        - `ThreadId`：不透明 u64 ID，支持 Debug/Eq/Hash 以用于映射键。
        - `Scope<'env>`/`ScopedJoinHandle<'scope, T>`：环境借用作用域（'env 生命周期）和句柄，支持 spawn 在 scope 内借用非 'static 数据。
        - `AvailableParallelism`：系统 CPU 信息，支持 get() 返回 NonZeroUsize（至少 1）。
        - `Panicking`：panicking() 函数返回 bool 检查当前线程是否 unwinding。
    - **函数详解**：`spawn`（简单创建，返回 JoinHandle）、`Builder::spawn`（配置创建）、`sleep`（阻塞延时）、`yield_now`（调度让出）、`park`/`unpark`（低级等待/唤醒）、`park_timeout`（有时限 park）、`current`（当前 Thread）、`panicking`（检查 unwind）、`scope`（创建 Scope）、`available_parallelism`（CPU 数）。
    - **宏扩展**：`thread_local!` 创建 TLS，支持 with_borrow/with_borrow_mut (1.78+) 以借用访问。
- **设计哲学扩展**：`std::thread` 遵循 Rust 的 "fearless concurrency"，通过编译时检查防止 race；move 闭包强制转移，避免共享 mut；panic 隔离但可恢复；scope 解决 'static 限制，提升表达力。无内置池以保持最小，但易扩展。
- **跨平台详解**：Windows 线程用 HANDLE/ID，Unix 用 pthread_t/tid；栈大小 Windows 默认 1MB，Unix 8MB，用 Builder 统一；优先级 Windows 用 SetThreadPriority，Unix 用 sched_setparam；测试 leap 用 VM 模拟 OS 差异。
- **性能详析**：spawn ~10-50us (OS 调用)；join <1us (轮询)；park/unpark ~100ns；多线程上下文切换 ~1-5us；大栈分配慢，用小栈优化。
- **常见用例扩展**：Web 服务器请求处理、数据并行计算、后台下载、GUI 事件循环、测试并发 bug。
- **超扩展概念**：与 std::sync::atomic 集成原子同步；与 std::panic::AssertUnwindSafe 安全捕获；错误链用 thiserror 自定义；与 rayon::ThreadPoolBuilder 扩展池；高性能用 cpu_affinity (crate) 绑定核心；与 tracing::instrument 装饰线程日志；历史：从 1.0 spawn 到 1.63 scope 的借用革命。

## 2. 创建线程：spawn、Builder 和 Scope

`spawn` 是入口，`Builder` 配置，`Scope` 借用。

### 示例：高级 spawn（返回复杂类型扩展）
```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| -> (i32, String) {
        (42, "answer".to_string())
    });

    let (num, text) = handle.join().unwrap();
    println!("num: {}, text: {}", num, text);
}
```

- **解释**：闭包返回元组。性能：小返回复制，大用 Arc。

### 示例：Move 闭包高级（捕获和计算扩展）
```rust
use std::thread;

fn main() {
    let data = (1..1000).collect::<Vec<i32>>();
    let handle = thread::spawn(move || data.iter().sum::<i32>());
    let sum = handle.join().unwrap();
    println!("sum: {}", sum);
}
```

- **解释**：move 转移 Vec。陷阱：大 move 复制开销，用 Arc<Vec> 共享。

### 示例：Builder 高级配置（栈/名/亲和扩展）
```rust
use std::thread::Builder;
use std::os::unix::thread::BuilderExt;  // Unix 亲和

fn main() -> std::io::Result<()> {
    let builder = Builder::new()
        .name("compute".to_string())
        .stack_size(4 * 1024 * 1024);  // 4MB

    #[cfg(unix)]
    let builder = builder.cpu_affinity(vec![0]);  // 绑定 CPU 0

    let handle = builder.spawn(|| {
        // 计算
    })?;

    handle.join().unwrap();
    Ok(())
}
```

- **解释**：`cpu_affinity` Unix 特定。性能：亲和减缓存失效。陷阱：无效 CPU Err。

### 示例：Scope 高级借用（多线程访问扩展）
```rust
use std::thread;
use std::sync::Arc;

fn main() {
    let data = Arc::new(vec![1, 2, 3]);
    thread::scope(|s| {
        for i in 0..3 {
            s.spawn(move || println!("项 {}: {}", i, data[i]));
        }
    });
}
```

- **解释**：scope 借用 Arc。扩展：用 ScopedJoinHandle::join 顺序等待。

### 示例：AvailableParallelism 高级（动态调整扩展）
```rust
use std::thread::AvailableParallelism;

fn main() {
    let cores = AvailableParallelism::new().unwrap_or(NonZeroUsize::new(1).unwrap()).get();
    (0..cores).map(|_| thread::spawn(|| {})).collect::<Vec<_>>().into_iter().for_each(|h| h.join().unwrap());
}
```

- **解释**：`get` 返回核心数。扩展：用 env var 覆盖。

## 3. 管理线程：Join、Park、Sleep、Yield

控制执行流。

### 示例：高级 Join（panic 恢复扩展）
```rust
use std::thread;
use std::any::Any;

fn main() {
    let handle = thread::spawn(|| panic!("err"));
    let res = handle.join();
    if let Err(e) = res {
        if let Some(s) = e.downcast_ref::<String>() {
            println!("str err: {}", s);
        }
    }
}
```

- **解释**：downcast 恢复。性能：join 低开销。

### 示例：Park/Unpark 高级（自定义信号扩展）
```rust
use std::thread;
use std::sync::Arc;

fn main() {
    let flag = Arc::new(());
    let flag2 = flag.clone();

    let handle = thread::spawn(move || {
        thread::park_timeout(Duration::from_secs(1));  //有时限
        println!("超时或 unpark");
    });

    flag.unpark();  // 唤醒
    handle.join().unwrap();
}
```

- **解释**：`park_timeout` 限时。扩展：用 with park/unpark 实现 semaphore。

### 示例：Sleep 高级（精确延时扩展）
```rust
use std::thread;
use std::time::{Duration, Instant};

fn precise_sleep(dur: Duration) {
    let start = Instant::now();
    while start.elapsed() < dur {
        thread::yield_now();
    }
}
```

- **解释**：忙等精确 sleep。性能：高 CPU，用于 ns 级。

### 示例：Yield_now 高级（协作调度扩展）
```rust
use std::thread;

fn main() {
    for _ in 0..1000 {
        // 计算
        thread::yield_now();  // 让出给其他
    }
}
```

- **解释**：yield 提示切换。扩展：用在 spinlock 减忙等。

## 4. ThreadLocal：本地存储

TLS 用于 per-thread 数据。

### 示例：高级 TLS（借用和 mut 扩展）
```rust
use std::cell::RefCell;
use std::thread;

thread_local! {
    static LOCAL: RefCell<Vec<i32>> = RefCell::new(vec![]);
}

fn main() {
    LOCAL.with_borrow(|v| println!("借用: {:?}", v));

    thread::spawn(|| {
        LOCAL.with_borrow_mut(|v| v.push(1));
        LOCAL.with_borrow(|v| println!("线程: {:?}", v));
    }).join().unwrap();

    LOCAL.with_borrow(|v| println!("主: {:?}", v));  // 空
}
```

- **解释**：`with_borrow`/`mut` 安全访问。扩展：用 OnceCell 懒惰初始化。

## 5. OS 扩展：ThreadExt 和 Raw

平台特定。

### 示例：Unix ThreadExt 高级（优先级扩展）
```rust
#[cfg(unix)]
use std::os::unix::thread::{JoinHandleExt, BuilderExt};
#[cfg(unix)]
use std::thread::Builder;

#[cfg(unix)]
fn main() -> std::io::Result<()> {
    let builder = Builder::new().name("prio");
    let handle = builder.spawn(|| {})?;
    let pthread = handle.as_pthread_t();
    // pthread_setschedprio(pthread, prio); unsafe
    handle.join().unwrap();
    Ok(())
}

#[cfg(not(unix))]
fn main() {}
```

- **解释**：`as_pthread_t` 获取 raw。扩展：用 sched 库设置。

### 示例：Windows ThreadExt 高级（优先级扩展）
```rust
#[cfg(windows)]
use std::os::windows::thread::JoinHandleExt;
#[cfg(windows)]
use std::thread;

#[cfg(windows)]
fn main() {
    let handle = thread::spawn(|| {});
    let whandle = handle.as_handle();
    // SetThreadPriority(whandle, THREAD_PRIORITY_HIGH); unsafe
    handle.join().unwrap();
}
```

- **解释**：`as_handle` 获取 HANDLE。扩展：用 WinAPI crate 调用。

## 6. 高级主题：Scoped、TLS、Panic 和 集成

- Scoped：借用。
- TLS：本地。
- Panic：捕获。

### 示例：Scoped 高级（错误传播扩展）
```rust
use std::thread;

fn main() {
    thread::scope(|s| {
        let h1 = s.spawn(|| panic!("err1"));
        let h2 = s.spawn(|| 42);

        if let Err(e) = h1.join() {
            println!("panic: {:?}", e);
        }
        println!("h2: {:?}", h2.join().unwrap());
    });
}
```

- **解释**：ScopedJoinHandle join 处理 panic。

### 示例：Panic 钩子（全局捕获扩展）
```rust
use std::panic;
use std::thread;

fn main() {
    panic::set_hook(Box::new(|info| {
        println!("全局 panic: {}", info);
    }));

    thread::spawn(|| panic!("捕获"));
}
```

- **解释**：`set_hook` 设置钩子。扩展：用 update_hook 动态。

## 7. 最佳实践和常见陷阱

- **线程最佳**：scope 优先借用；Builder 自定义大任务；TLS 最小全局。
- **性能**：池复用 spawn；yield 协作；affinity 绑核减迁移。
- **错误**：join 总检查；os 错误 raw_os_error 分类。
- **安全**：Arc 无 race；TLS 防共享；panic 捕获不传播。
- **跨平台**：cfg 标志；测试 pthread vs WinThread。
- **测试**：loom race；mock spawn 测试逻辑。
- **资源**：join 回收；kill 不安全，用 channel 信号。
- **常见扩展**：
    - Borrow Err：scope/move 解决。
    - Overflow：大栈 OOM，用 guard。
    - Deadlock：park 配 unpark。
    - Panic 丢失：总 join。

## 8. 练习建议

1. 编写池：Builder 创建，channel 任务，join 管理。
2. 实现 TLS 缓存：thread_local 用 OnceCell 懒惰。
3. 创建 scoped 并行：scope 多 spawn 借用处理数据。
4. 处理 panic 恢复：catch_unwind + downcast 线程内。
5. 基准：比较 spawn vs pool 时间，用 Instant。
6. 与 sync：用 TLS + Mutex 混合存储。
7. 错误框架：mock os Err 测试 spawn 重试。
8. 高级 app：实现游戏多线程：render/input/physics。
