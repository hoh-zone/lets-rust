# Trait Sync

`Sync` trait 来自 `std::marker` 模块，它是一个标记 trait（marker trait），表示类型可以安全地在多个线程间共享引用（即 `&T` 是 `Send` 的）。它确保类型在并发环境中不会导致数据竞争或内存不安全，常与 `std::sync::Arc` 等结合使用。与 `Send` 不同，`Sync` 专注于共享引用的线程安全，而不是所有权转移的安全。

## 1. `Sync` Trait 简介

### 1.1 定义和目的
`Sync` trait 定义在 `std::marker::Sync` 中，自 Rust 1.0.0 起稳定可用。其语法如下：
```rust
pub unsafe auto trait Sync { }
```
- **关键点**：
    - `unsafe`：表示实现 `Sync` 可能不安全，需要开发者保证正确性。
    - `auto`：编译器自动为符合条件的类型实现 `Sync`，无需手动实现（除非自定义）。
    - 无方法：作为标记 trait，仅标记类型安全共享引用。

**目的**：`Sync` 标记类型可以安全地在多个线程间共享不可变引用，而不会引入数据竞争。这意味着如果 `T: Sync`，则 `&T: Send`，允许多个线程同时持有 `&T`。根据官方文档，`Sync` 是多线程安全的核心，确保类型如 `i32`、`Mutex<T>`（如果 `T: Send`）可以共享，而如 `RefCell<T>`（内部可变性，非线程安全）不能实现 `Sync`。

`Sync` 的设计目的是支持并发共享：Rust 的线程模型要求共享的数据必须 `Sync`，以防止竞争。 它促进线程安全，提供零成本抽象，因为它是编译时检查。

- **为什么需要 `Sync`？** Rust 强调无数据竞争。`Sync` 允许编译器静态检查共享引用的安全性，避免运行时错误。 例如，在 `Arc<T>` 中，`T` 必须 `Sync` 以允许多线程访问。

### 1.2 与相关 Trait 的区别
`Sync` 与几个并发 trait 相关，但专注于共享引用的安全性：

- **与 `Send`**：
    - `Sync`：类型可以安全共享引用（`&T: Send`）；`Send`：类型可以安全转移所有权到另一个线程。
    - `Sync` 关注共享；`Send` 关注转移。
    - 示例：`Mutex<T>` 实现 `Sync`（如果 `T: Send`），允许多线程共享 `&Mutex`；`RefCell<T>` 实现 `Send`（如果 `T: Send`），但不 `Sync`。
    - 关系：`T: Sync` 隐含 `&T: Send`；`T: Send` 不隐含 `Sync`。
    - 选择：用 `Sync` 对于共享数据；用 `Send` 对于转移数据。

- **与 `Unpin`**：
    - `Sync` 与线程安全相关；`Unpin` 与 Pinning 和移动相关。
    - `Unpin` 影响异步；`Sync` 影响并发。
    - 示例：`dyn Future + Sync` 支持线程安全异步。
    - 区别：`Sync` 是 opt-in 线程共享；`Unpin` 是 opt-out pinning。

- **与 `Copy`**：
    - `Sync` 是 marker；`Copy` 是拷贝 marker。
    - `Copy` 类型常自动 `Sync`（如 primitives）；但 `Sync` 不要求拷贝。
    - 示例：`i32: Copy + Sync`；自定义类型需检查。

**何时选择？** 用 `Sync` 在多线程共享场景中，确保引用安全；与 `Send` 结合实现全线程安全。 最佳实践：为自定义类型自动派生 `Sync`，除非有非 `Sync` 字段（如 `Cell<T>`）。

## 2. 自动派生 `Sync`（Auto-Implemented）

Rust 编译器自动为符合条件的类型实现 `Sync`：如果所有字段实现 `Sync`，则结构体/枚举自动 `Sync`。无需手动实现，除非使用 `unsafe` 覆盖。

### 2.1 基本示例：结构体
```rust
struct Point {
    x: i32,
    y: i32,
}

use std::sync::Arc;

fn main() {
    let p = Point { x: 1, y: 2 };
    let arc = Arc::new(p);  // Point 自动 Sync
    let arc_clone = arc.clone();
    std::thread::spawn(move || {
        println!("Shared: {} {}", arc_clone.x, arc_clone.y);
    }).join().unwrap();
}
```
- `i32: Sync`，所以 `Point` 自动 `Sync`，允许 `Arc` 共享。

### 2.2 枚举
```rust
enum Status {
    Ok,
    Error(String),  // String: Sync
}

fn main() {
    let s = Status::Error("oops".to_string());
    let arc = Arc::new(s);  // Status 自动 Sync
    // 多线程共享 arc
}
```
- 变体字段 `Sync`，枚举自动 `Sync`。

### 2.3 泛型类型
```rust
struct Container<T> {
    item: T,
}

// 如果 T: Sync，Container<T>: Sync
fn main() {
    let c = Container { item: 42 };
    let arc = Arc::new(c);
    // 共享 arc
}
```
- 泛型依赖 `T: Sync`。

**注意**：如果类型有非 `Sync` 字段（如 `RefCell<T>`），不自动实现 `Sync`；编译器拒绝 `Arc` 等。

## 3. 手动实现 `Sync`（Unsafe）

手动实现 `Sync` 是 `unsafe`，因为开发者必须保证安全。通常避免，除非自定义原始类型或指针。

### 3.1 手动示例
```rust
use std::marker::Sync;

struct RawMutex(*mut i32);

unsafe impl Sync for RawMutex {}  // 开发者保证安全

// 但不推荐；使用 std::sync::Mutex 代替
```
- `unsafe` 因为 raw mutex 非线程安全；手动实现需小心。

### 3.2 非 Sync 类型
如 `RefCell<T>` 不实现 `Sync`，因为内部可变性非原子：
```rust
use std::cell::RefCell;
use std::sync::Arc;

fn main() {
    let rc = RefCell::new(5);
    // let arc = Arc::new(rc);  // 编译错误：RefCell 非 Sync
}
```
- 用 `Mutex` 代替以 Sync。

## 4. 高级主题

### 4.1 泛型和约束
在泛型中添加 `T: Sync`：
```rust
fn share<T: Sync + 'static>(value: Arc<T>) -> Arc<T> {
    value.clone()
}
```
- 确保值可共享；`'static` 确保无借用。

### 4.2 与 Send 结合
`Sync + Send` 表示可共享且可转移：
- `Arc<T: Sync + Send>`：线程安全共享。
- `Mutex<T: Send>`：可转移互斥锁，但若 `T: Sync`，Mutex `Sync`。

### 4.3 自定义非 Sync 类型
使用 `Cell` 或 raw pointers 标记非 Sync：
```rust
use std::cell::Cell;

struct NonSync(Cell<i32>);  // Cell 非 Sync
```
- 防止类型自动 Sync。

## 5. 常见用例

- **共享数据**：Arc 包装 Sync 类型多线程共享。
- **全局静态**：静态变量需 Sync 以多线程访问。
- **并行计算**：Rayon 等库要求 Sync 数据。
- **异步**：Tokio 等需 Sync 以跨任务共享。
- **泛型边界**：确保类型线程共享安全。

## 6. 最佳实践

- **依赖自动派生**：让编译器处理 Sync。
- **避免手动 unsafe**：使用标准类型。
- **结合 Send**：全线程安全。
- **文档**：说明类型是否 Sync。
- **测试**：编译检查 Sync 用法。
- **性能**：Sync 不加开销；仅标记。

## 7. 常见陷阱和错误

- **非 Sync 类型共享**：编译错误；用 Mutex 包装。
- **生命周期**：需 `'static` 或 scoped threads。
- **内部可变性**：用 Mutex/Arc 而非 RefCell。
- **泛型无边界**：意外非 Sync；添加 Sync 边界。
- **Rc vs Arc**：用 Arc 以 Sync。
