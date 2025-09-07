### once_cell

`once_cell` 是 Rust 中一个高效的 crate，用于实现单赋值单元（single-assignment cells）和延迟初始化值。它提供了一种现代、安全的方式来处理全局状态、懒加载和线程安全的初始化，常作为 `lazy_static` 的替代品。`once_cell` 的核心类型包括 `OnceCell`（单线程写一次单元）、`Lazy`（延迟初始化）和 `OnceLock`（线程安全的 `OnceCell`，从 Rust 1.70 起标准化）。它支持 no-std 环境、低开销，并避免了宏的复杂性。

#### 1. 安装 once_cell
在你的 `Cargo.toml` 文件中添加依赖。最新版本为 1.20.2（发布于 2025 年 7 月 31 日）。 默认支持 std；对于 no-std，禁用默认特性。

```toml
[dependencies]
once_cell = "1.20"  # 基本版本
```

启用特性示例（如 critical-section 用于嵌入式）：

```toml
once_cell = { version = "1.20", features = ["std", "critical-section"] }
```

运行 `cargo build` 安装。`once_cell` 支持 MSRV 1.36，并可与标准库 `OnceLock` 互操作。

#### 2. 基本用法
`once_cell` 提供 `OnceCell<T>`（写一次，读多次）和 `Lazy<T>`（延迟初始化）。语法简单，无需宏。

基本语法：

```rust
use once_cell::sync::{OnceCell, Lazy};

static CELL: OnceCell<String> = OnceCell::new();
static LAZY: Lazy<u32> = Lazy::new(|| 42);

fn main() {
    // OnceCell: 设置值（只一次）
    let _ = CELL.set("Hello".to_string());
    println!("{}", CELL.get().unwrap());

    // Lazy: 自动初始化
    println!("{}", *LAZY);
}
```

`OnceCell` 返回 `Result` 以处理重复设置；`Lazy` 在首次访问时初始化。

#### 3. 语义和实现细节
- **OnceCell**：单赋值单元，设置后不可变。`set` 或 `try_insert` 只允许一次成功。支持 `get_or_init` 延迟设置。
- **Lazy**：延迟值，使用闭包初始化，只执行一次。线程安全版本使用自旋锁。
- **OnceLock**：线程安全的 `OnceCell`，从 Rust 1.70 标准化。使用原子操作确保安全。
- **Unsafecell**：低级不安全版本，用于自定义同步。
- **错误处理**：`set` 返回 `Result`，重复设置返回 `Err`（带旧值）。
- **性能**：低开销，初始化使用原子检查；基准显示优于 `lazy_static`（无宏开销）。

#### 4. 高级用法
- **线程安全**：使用 `sync::OnceCell` 或 `OnceLock`。结合 `std::sync::Mutex` 处理可变状态。
- **延迟初始化**：`get_or_init` 或 `Lazy::force` 强制初始化。
- **no-std 支持**：启用 "alloc" 特性，使用 `race::OnceCell` 等变体。
- **与标准库集成**：在 Rust 1.70+，可迁移到 `std::sync::OnceLock`。
- **自定义同步**：使用 `critical-section` 特性在嵌入式中自定义锁。
- **多线程**：`OnceCell` 非线程安全；使用 `sync` 变体传播值。
- **集成**：与 `serde`（序列化延迟值）、`tokio`（异步初始化）、`tracing`（日志初始化）。

#### 5. 注意事项
- `OnceCell` 非 `Sync`，不可跨线程共享；使用 `sync` 版本。
- 避免循环依赖：初始化闭包中不要递归访问同一单元。
- 性能开销：自旋锁在高争用下可能慢；基准显示在大多数情况下优于 `Mutex`。
- no-std：需 "alloc" 特性；嵌入式中可能需自定义 critical section。
- 与 `lazy_static` 比较：`once_cell` 无宏，更安全，但需手动处理类型。
- 弃用：旧 API 如 `unsync::Lazy` 已标准化。

#### 6. 替代方案
- **lazy_static**：宏-based 延迟静态，但有宏开销；`once_cell` 更现代。
- **std::sync::OnceLock**：标准库版本（1.70+），无需额外 crate。
- **std::sync::Once**：手动初始化，但繁琐。
- **singleton**：简单单例，但不灵活。
  `once_cell` 被视为延迟初始化的首选。

#### 7. 20 个例子
以下是 20 个例子，从简单到复杂，覆盖 OnceCell、Lazy 等。每个例子包括代码、输出（如果适用）和解释。假设已导入 `use once_cell::{sync::{Lazy, OnceCell}, unsync::OnceCell as UnsyncOnceCell};`。

##### 示例 1: 基本 OnceCell
```rust
fn main() {
    static CELL: OnceCell<u32> = OnceCell::new();
    let _ = CELL.set(42);
    println!("{}", CELL.get().unwrap());
}
```
输出：`42`  
解释：简单设置和获取。

##### 示例 2: Lazy 初始化
```rust
static LAZY: Lazy<u32> = Lazy::new(|| 42);

fn main() {
    println!("{}", *LAZY);
}
```
输出：`42`  
解释：延迟计算。

##### 示例 3: get_or_init
```rust
fn main() {
    static CELL: OnceCell<u32> = OnceCell::new();
    let value = CELL.get_or_init(|| 42);
    println!("{}", value);
}
```
输出：`42`  
解释：延迟设置。

##### 示例 4: 重复设置错误
```rust
fn main() {
    static CELL: OnceCell<u32> = OnceCell::new();
    CELL.set(42).unwrap();
    if let Err(e) = CELL.set(43) {
        println!("Error: {}", e);
    }
}
```
输出：`Error: 42`  
解释：处理重复。

##### 示例 5: 线程安全 OnceLock
```rust
use std::sync::OnceLock;

static LOCK: OnceLock<u32> = OnceLock::new();

fn main() {
    println!("{}", LOCK.get_or_init(|| 42));
}
```
解释：标准库版本。

##### 示例 6: Unsync OnceCell
```rust
fn main() {
    let cell = UnsyncOnceCell::new();
    cell.set(42).unwrap();
    println!("{}", cell.get().unwrap());
}
```
解释：非线程安全版本。

##### 示例 7: Lazy 复杂计算
```rust
static LAZY: Lazy<u32> = Lazy::new(|| (1..=10).sum());

fn main() {
    println!("Sum: {}", *LAZY);
}
```
输出：`Sum: 55`  
解释：昂贵计算。

##### 示例 8: 全局配置
```rust
static CONFIG: OnceCell<String> = OnceCell::new();

fn main() {
    CONFIG.set("config".to_string()).unwrap();
    println!("{}", CONFIG.get().unwrap());
}
```
解释：全局值。

##### 示例 9: 多线程 Lazy
```rust
use std::thread;

static LAZY: Lazy<u32> = Lazy::new(|| {
    println!("Initializing");
    42
});

fn main() {
    let t1 = thread::spawn(|| println!("{}", *LAZY));
    let t2 = thread::spawn(|| println!("{}", *LAZY));
    t1.join().unwrap();
    t2.join().unwrap();
}
```
输出：初始化只一次。  
解释：线程安全。

##### 示例 10: get_or_try_init
```rust
use std::io::{self, Error};

fn main() -> Result<(), Error> {
    static CELL: OnceCell<u32> = OnceCell::new();
    let value = CELL.get_or_try_init(|| Ok(42))?;
    println!("{}", value);
    Ok(())
}
```
解释：错误传播。

##### 示例 11: 缓存结果
```rust
static CACHE: OnceCell<u32> = OnceCell::new();

fn compute() -> u32 {
    CACHE.get_or_init(|| (1..=100).sum())
}

fn main() {
    println!("{}", compute());
    println!("{}", compute());  // 使用缓存
}
```
解释：函数缓存。

##### 示例 12: no-std 支持
```rust
#![no_std]
use once_cell::race::OnceCell;

static CELL: OnceCell<u32> = OnceCell::new();

fn main() {
    CELL.set(42).unwrap();
}
```
解释：无标准库。

##### 示例 13: 结合 Mutex
```rust
use std::sync::Mutex;

static DATA: Lazy<Mutex<Vec<u32>>> = Lazy::new(|| Mutex::new(vec![]));

fn main() {
    DATA.lock().unwrap().push(1);
    println!("{:?}", *DATA.lock().unwrap());
}
```
解释：可变全局。

##### 示例 14: 强制初始化
```rust
static LAZY: Lazy<u32> = Lazy::new(|| 42);

fn main() {
    Lazy::force(&LAZY);
    println!("{}", *LAZY);
}
```
解释：提前初始化。

##### 示例 15: 自定义类型
```rust
#[derive(Debug)]
struct Custom(u32);

static CELL: OnceCell<Custom> = OnceCell::new();

fn main() {
    CELL.set(Custom(42)).unwrap();
    println!("{:?}", CELL.get().unwrap());
}
```
解释：自定义结构体。

##### 示例 16: 环境变量加载
```rust
use std::env;

static LANG: Lazy<String> = Lazy::new(|| env::var("LANG").unwrap_or("en".to_string()));

fn main() {
    println!("{}", *LANG);
}
```
解释：运行时加载。

##### 示例 17: 单例模式
```rust
struct Singleton;

static INSTANCE: OnceCell<Singleton> = OnceCell::new();

fn get_instance() -> &'static Singleton {
    INSTANCE.get_or_init(|| Singleton)
}

fn main() {
    let _ = get_instance();
}
```
解释：实现单例。

##### 示例 18: 异步初始化（模拟）
```rust
use tokio::runtime::Runtime;

static LAZY: Lazy<u32> = Lazy::new(|| {
    let rt = Runtime::new().unwrap();
    rt.block_on(async { 42 })
});

fn main() {
    println!("{}", *LAZY);
}
```
解释：结合异步。

##### 示例 19: 性能测试
```rust
static LAZY: Lazy<u32> = Lazy::new(|| {
    // 模拟耗时
    std::thread::sleep(std::time::Duration::from_millis(100));
    42
});

fn main() {
    let start = std::time::Instant::now();
    let _ = *LAZY;
    println!("First: {:?}", start.elapsed());
    let start = std::time::Instant::now();
    let _ = *LAZY;
    println!("Second: {:?}", start.elapsed());
}
```
解释：初始化开销。

##### 示例 20: 与 Serde 集成
```rust
use serde::Serialize;

#[derive(Serialize)]
struct Data(u32);

static LAZY: Lazy<Data> = Lazy::new(|| Data(42));

fn main() {
    let json = serde_json::to_string(&*LAZY).unwrap();
    println!("{}", json);
}
```
输出：`{"0":42}`  
解释：序列化延迟值。
