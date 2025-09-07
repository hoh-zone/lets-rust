### lazy_static

`lazy_static` 是 Rust 中一个广泛使用的 crate，用于实现静态变量的延迟初始化（lazy initialization）。它允许在运行时执行代码来初始化静态变量，这在标准 `static` 无法处理的情况下非常有用，例如涉及堆分配（如 `Vec` 或 `HashMap`）、函数调用或环境变量读取的场景。`lazy_static` 通过宏确保初始化只发生一次，并支持线程安全。

#### 1. 安装 lazy_static
在你的 `Cargo.toml` 文件中添加依赖：

```toml
[dependencies]
lazy_static = "1.5.0"  # 使用最新版本，检查 crates.io 以获取更新
```

然后运行 `cargo build` 来安装。注意：`lazy_static` 支持 `no_std` 环境，通过启用 `spin_no_std` 特性。

#### 2. 基本用法
使用 `lazy_static!` 宏声明静态变量。语法如下：

```rust
use lazy_static::lazy_static;

lazy_static! {
    [pub] static ref NAME: TYPE = EXPR;
}
```

- `pub` 是可选的，用于公开可见性。
- 支持属性，如文档注释。
- 类型必须实现 `Sync` 以确保线程安全。
- 初始化表达式 `EXPR` 在首次访问时执行，只执行一次。

访问时使用 `*NAME` 或通过 `Deref` 隐式解引用。宏内部生成一个实现 `Deref<TYPE>` 的独特类型，并使用原子操作确保线程安全。

#### 3. 语义和实现细节
- **延迟初始化**：首次解引用时评估 `EXPR`，后续访问返回相同对象。
- **线程安全**：使用隐藏的静态变量和原子检查（`std::sync::Once` 或类似）。
- **死锁风险**：如果多个 lazy static 相互依赖初始化，可能导致死锁。
- **属性**：与标准 `static` 类似，但不支持析构函数在进程退出时运行。
- **性能**：每次访问有轻微原子开销，但基准测试显示在大多数情况下与手动初始化相当（约 26-27 ns/iter）。

#### 4. 高级用法
- **手动初始化**：使用 `initialize(&NAME)` 强制初始化。
- **内部可变性**：结合 `Mutex` 或 `RwLock` 实现全局可变状态。
- **No-std 支持**：启用 `spin_no_std` 特性，使用 `spin` crate。
- **LazyStatic trait**：支持额外操作，如初始化检查。
- **多声明**：宏中可声明多个静态变量。

#### 5. 注意事项
- 类型必须是 `Sync`。
- 避免循环依赖以防死锁。
- 性能开销小，但在高频循环中可能累积；基准测试显示与 `once_cell` 类似。
- 不要在函数内部使用 `lazy_static!`，因为它无法捕获动态环境；改为模块级别。
- 全局变量可能增加代码耦合；优先考虑局部变量或参数传递。
- 如果未使用，lazy static 不会初始化。

#### 6. 替代方案
- Rust 1.70+ 的 `std::sync::LazyLock`：标准库替代，无需额外 crate。
- `once_cell`：更灵活，支持非线程安全版本，可能更快。
- `std::sync::Once`：手动实现简单 lazy。
- 对于常量，使用 `const`；对于可变，使用 `Mutex` 包装。

`lazy_static` 已被社区视为遗留；考虑迁移到标准库。

#### 7. 20 个例子
以下是 20 个例子，从简单到复杂，覆盖配置、缓存、多线程、数学计算等实际场景。每个例子包括代码、输出（如果适用）和解释。假设已导入 `use lazy_static::lazy_static;`。

##### 示例 1: 简单 HashMap 初始化
```rust
use std::collections::HashMap;

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m
    };
}

fn main() {
    println!("Value for 0: {}", HASHMAP.get(&0).unwrap());
}
```
输出：`Value for 0: foo`  
解释：延迟构建 HashMap，只在首次访问时初始化。

##### 示例 2: 从 HashMap 计算长度
```rust
use std::collections::HashMap;

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = { /* 如上 */ };
    static ref COUNT: usize = HASHMAP.len();
}

fn main() {
    println!("Map has {} entries.", *COUNT);
}
```
输出：`Map has 2 entries.`  
解释：依赖另一个 lazy static 计算值。

##### 示例 3: 函数调用初始化
```rust
lazy_static! {
    static ref NUMBER: u32 = times_two(21);
}

fn times_two(n: u32) -> u32 { n * 2 }

fn main() {
    println!("Result: {}", *NUMBER);
}
```
输出：`Result: 42`  
解释：运行时函数调用。

##### 示例 4: 带文档注释
```rust
lazy_static! {
    /// 示例文档
    static ref EXAMPLE: u8 = 42;
}

fn main() {
    println!("Value: {}", *EXAMPLE);
}
```
输出：`Value: 42`  
解释：支持属性和文档。

##### 示例 5: Regex 缓存
```rust
use regex::Regex;

lazy_static! {
    static ref DATE_REGEX: Regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
}

fn main() {
    println!("Is match: {}", DATE_REGEX.is_match("2025-09-03"));
}
```
输出：`Is match: true`  
解释：避免重复编译 Regex，提高性能。

##### 示例 6: 全局 Mutex 可变向量
```rust
use std::sync::Mutex;

lazy_static! {
    static ref NAMES: Mutex<Vec<String>> = Mutex::new(vec!["Alice".to_string()]);
}

fn main() {
    NAMES.lock().unwrap().push("Bob".to_string());
    println!("Names: {:?}", *NAMES.lock().unwrap());
}
```
输出：`Names: ["Alice", "Bob"]`  
解释：线程安全全局可变状态。

##### 示例 7: 环境变量读取
```rust
use std::env;

lazy_static! {
    static ref LANG: String = env::var("LANG").unwrap_or("en_US".to_string());
}

fn main() {
    println!("Language: {}", *LANG);
}
```
解释：运行时读取环境变量。

##### 示例 8: 数学计算
```rust
lazy_static! {
    static ref ROOT_OF_SEVEN: f64 = 7_f64.sqrt();
}

fn main() {
    println!("√7: {:.2}", *ROOT_OF_SEVEN);
}
```
输出：`√7: 2.65`  
解释：非常量数学操作。

##### 示例 9: 全局计数器
```rust
use std::sync::Mutex;

lazy_static! {
    static ref COUNTER: Mutex<u32> = Mutex::new(0);
}

fn increment() {
    *COUNTER.lock().unwrap() += 1;
}

fn main() {
    increment();
    println!("Counter: {}", *COUNTER.lock().unwrap());
}
```
输出：`Counter: 1`  
解释：线程安全计数器。

##### 示例 10: 配置加载
```rust
lazy_static! {
    static ref CONFIG: String = "config_value".to_string();  // 模拟从文件加载
}

fn main() {
    println!("Config: {}", *CONFIG);
}
```
解释：延迟加载配置。

##### 示例 11: 模拟数据库连接
```rust
use std::sync::Mutex;

struct DbPool;  // 模拟

lazy_static! {
    static ref DB_POOL: Mutex<DbPool> = Mutex::new(DbPool);
}

fn main() {
    let _ = DB_POOL.lock().unwrap();
    println!("DB connected.");
}
```
解释：全局连接池。

##### 示例 12: 与 Arc 结合
```rust
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref SHARED: Arc<Mutex<u32>> = Arc::new(Mutex::new(42));
}

fn main() {
    println!("Shared: {}", *SHARED.lock().unwrap());
}
```
解释：共享所有权。

##### 示例 13: 手动初始化
```rust
lazy_static! {
    static ref DATA: String = "Initialized".to_string();
}

fn main() {
    lazy_static::initialize(&DATA);
    println!("{}", *DATA);
}
```
解释：强制提前初始化。

##### 示例 14: No-std 环境（需启用 spin_no_std）
```rust
// 在 no_std 项目中
lazy_static! {
    static ref VALUE: u32 = 42;
}
```
解释：无标准库支持。

##### 示例 15: 多声明
```rust
lazy_static! {
    static ref A: u32 = 1;
    static ref B: u32 = 2;
}

fn main() {
    println!("A + B: {}", *A + *B);
}
```
输出：`A + B: 3`  
解释：宏中多个静态。

##### 示例 16: 公共静态
```rust
lazy_static! {
    pub static ref PUBLIC: u32 = 42;
}
```
解释：模块间可见。

##### 示例 17: 自定义结构体
```rust
#[derive(Debug)]
struct MyStruct { value: u32 }

lazy_static! {
    static ref STRUCT: MyStruct = MyStruct { value: 42 };
}

fn main() {
    println!("{:?}", *STRUCT);
}
```
解释：自定义类型。

##### 示例 18: 缓存昂贵计算
```rust
fn expensive() -> u32 { /* 模拟耗时 */ 42 }

lazy_static! {
    static ref CACHE: u32 = expensive();
}

fn main() {
    println!("Cached: {}", *CACHE);
}
```
解释：避免重复计算。

##### 示例 19: 单例模式
```rust
struct Singleton;

lazy_static! {
    static ref INSTANCE: Singleton = Singleton;
}

fn main() {
    let _ = &*INSTANCE;
    println!("Singleton accessed.");
}
```
解释：实现单例。

##### 示例 20: 全局日志器
```rust
use std::sync::Mutex;

#[derive(Debug)]
struct Logger { logs: Mutex<Vec<String>> }

lazy_static! {
    static ref LOGGER: Logger = Logger { logs: Mutex::new(vec![]) };
}

fn log(msg: &str) {
    LOGGER.logs.lock().unwrap().push(msg.to_string());
}

fn main() {
    log("Hello");
    println!("Logs: {:?}", *LOGGER.logs.lock().unwrap());
}
```
输出：`Logs: ["Hello"]`  
解释：线程安全日志。
