# Rust Trait Unpin

`Unpin` trait 来自 `std::marker` 模块，它是一个自动标记 trait（auto marker trait），表示类型可以安全地移动，即使它被固定（pinned）。它与 Rust 的 Pinning 系统密切相关，主要用于异步编程、futures 和 self-referential 类型，确保某些类型在被 Pin 后不会被意外移动，从而避免内存不安全。 与 `Pin` 类型结合，`Unpin` 允许开发者在需要固定内存的位置（如 futures 中的 self-referential pointers）时，灵活处理类型，而不会限制所有类型的移动。

## 1. `Unpin` Trait 简介

### 1.1 定义和目的
`Unpin` trait 定义在 `std::marker::Unpin` 中，自 Rust 1.33.0 起稳定可用。其语法如下：
```rust
pub auto trait Unpin { }
```
- **关键点**：
    - `auto`：编译器自动为符合条件的类型实现 `Unpin`，无需手动实现（除非自定义）。
    - 无方法：作为标记 trait，仅标记类型在被 `Pin` 固定后，可以安全移动（unpinned）。
    - **目的**：`Unpin` 表示类型是“pinning-agnostic”的，即它不依赖任何 pinning 保证，可以在 `Pin` 包装下自由移动，而不会引入内存不安全。根据官方文档，`Unpin` 缓解了需要 `Pin` 的 API 的 ergonomic 问题：对于不关心 pinning 的类型（如大多数简单类型），实现 `Unpin` 允许绕过 pinning 限制，而对于需要 pinning 的类型（如某些 futures），不实现 `Unpin` 确保它们在 pinned 时不可移动。 这在异步编程中特别有用，因为许多 `Future` 类型需要 self-referential pointers，这些指针在移动时会失效；`Pin` 固定内存地址，`Unpin` 标记哪些类型不需要这种固定。

`Unpin` 的设计目的是平衡安全和易用性：在 Pinning 系统下，类型默认是 `!Unpin`（不可移动，当 pinned 时），但大多数类型自动实现 `Unpin`，因为它们不依赖固定地址。只有包含如 `PhantomPinned` 的类型才是 `!Unpin`。

- **为什么需要 `Unpin`？** Rust 的所有权系统允许值移动，但对于 self-referential 类型（如 futures 中的指针指向自身字段），移动会使指针失效，导致 UB（undefined behavior）。`Pin` 防止移动，`Unpin` 标记哪些类型不受此限制，可以安全移动，从而简化 API（如 `Future::poll`）。 例如，在 async/await 中，许多 futures 不需要 pinning，因此实现 `Unpin` 以提高 ergonomics。

### 1.2 与相关 Trait 的区别
`Unpin` 与几个 marker trait 相关，但专注于 Pinning 的 opt-out：

- **与 `Pin`**：
    - `Unpin` 是 trait；`Pin` 是 wrapper 类型，用于固定值在内存中不可移动。
    - `Unpin` 表示类型在 `Pin` 下可移动（无 pinning 依赖）；`Pin` 强制不可移动，除非类型 `Unpin`。
    - 示例：对于 `T: Unpin`，`Pin<&mut T>` 行为如 `&mut T`（可移动）；对于 `!Unpin`，`Pin` 防止移动。
    - 关系：`Unpin` 允许在 `Pin` 下忽略 pinning 保证。

- **与 `Send` 和 `Sync`**：
    - `Unpin` 与 Pinning 相关；`Send`/`Sync` 与线程安全相关。
    - `Unpin` 不影响线程安全；但 futures 常需 `Unpin + Send` 以跨 await 发送。
    - 示例：`dyn Future + Unpin + Send` 支持异步线程安全。
    - 区别：`Unpin` 是 opt-out pinning；`Send`/`Sync` 是 opt-in 线程安全。

- **与 `PhantomPinned`**：
    - `Unpin` 是 auto trait；`PhantomPinned` 是 marker，用于使类型 `!Unpin`（不可 Unpin）。
    - `PhantomPinned` 用于需要 pinning 的类型（如 self-referential structs）；包含它使类型 `!Unpin`。
    - 示例：添加 `PhantomPinned` 以禁用自动 `Unpin`。

**何时选择？** 实现 `Unpin` （通常自动）当类型不依赖固定地址时；用 `!Unpin`（通过 `PhantomPinned`）当类型有 self-referential 指针需要 pinning 保护。 最佳实践：大多数类型自动 `Unpin`，仅在需要 pinning 时手动禁用。

## 2. 自动实现 `Unpin`（Auto-Implemented）

Rust 编译器自动为符合条件的类型实现 `Unpin`：如果类型的所有字段实现 `Unpin`，则类型自动 `Unpin`。无需手动实现，除非使用 `PhantomPinned` 禁用。

### 2.1 基本示例：Unpin 类型
```rust
use std::pin::Pin;

struct Simple(i32);  // 自动 Unpin

fn main() {
    let mut s = Simple(42);
    let pinned = Pin::new(&mut s);  // 因为 Unpin，可安全移动
    let moved = *pinned;  // OK
}
```
- 简单类型自动 `Unpin`，`Pin` 不限制移动。

### 2.2 !Unpin 类型示例
```rust
use std::marker::PhantomPinned;
use std::pin::Pin;

struct SelfRef {
    data: String,
    ptr: *const String,
    _pin: PhantomPinned,  // 使 !Unpin
}

impl SelfRef {
    fn new(data: String) -> Self {
        Self { data, ptr: std::ptr::null(), _pin: PhantomPinned }
    }

    fn init(self: Pin<&mut Self>) {
        let this = unsafe { self.get_unchecked_mut() };
        this.ptr = &this.data;
    }
}

fn main() {
    let mut sr = SelfRef::new("hello".to_string());
    let mut pinned = Pin::new(&mut sr);
    pinned.as_mut().init();  // 安全初始化 self-ref
    // sr = SelfRef::new("world".to_string());  // 不能移动，因为 !Unpin
}
```
- `PhantomPinned` 禁用自动 `Unpin`，确保 pinned 时不可移动。

### 2.3 泛型类型
```rust
struct Container<T> {
    item: T,
}

// 如果 T: Unpin，Container<T>: Unpin
fn move_pinned<T: Unpin>(mut pinned: Pin<&mut T>) {
    let unpinned = unsafe { Pin::into_inner_unchecked(pinned) };  // OK 因为 Unpin
    // 使用 unpinned
}
```
- 泛型依赖 `T: Unpin` 以安全 unpin。

## 3. `Unpin` 在 Pinning 系统中的作用

Pinning 系统防止 self-referential 类型移动：
- `Pin<P>` 包装指针 `P`，保证指向的值不移动，除非 `Unpin`。
- 对于 `T: Unpin`，`Pin<&mut T>` 行为如 `&mut T`（可移动）。
- 对于 `!Unpin`，`Pin` 限制 API，仅允许不可移动操作。

示例：Futures 需要 pinning 以安全存储 self-referential pointers。

## 4. 高级主题

### 4.1 与 Futures 和 Async 结合
在 async Rust 中，许多 futures 是 `!Unpin`，因为它们使用 generators 生成 self-referential 代码：
```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct MyFuture;

impl Future for MyFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}

fn main() {
    let mut fut = MyFuture;
    // fut 需 Pin 以 poll，因为 !Unpin (假设)
    let mut pinned = Box::pin(fut);
    // poll pinned
}
```
- `Future` trait 方法使用 `Pin<&mut Self>` 以支持 !Unpin futures。

### 4.2 手动禁用 Unpin
使用 `PhantomPinned`：
```rust
use std::marker::PhantomPinned;

struct PinnedStruct {
    _pin: PhantomPinned,
}

// PinnedStruct: !Unpin
```
- 防止类型自动 Unpin。

### 4.3 Unsafe Pinned
RFC 3467 引入 `unsafe_pinned` 用于自定义 pinned 字段。

## 5. 常见用例

- **Async/Await**：许多 futures 自动 Unpin，简化使用。
- **Self-Referential Structs**：使用 !Unpin 保护。
- **Generators**：Rust generators 是 !Unpin。
- **库设计**：trait 方法用 Pin<Self> 以支持 !Unpin。
- **性能优化**：Unpin 类型无 pinning 开销。

## 6. 最佳实践

- **依赖自动实现**：让编译器处理 Unpin。
- **用 !Unpin 仅必要**：仅 self-referential 时禁用。
- **Pin 类型**：用 Box::pin 或 stack pin 以固定。
- **文档**：说明类型是否 Unpin 和原因。
- **测试**：验证 pinning 行为（用 unsafe unpin 检查）。
- **与 Send/Sync 结合**：异步需 Unpin + Send。

## 7. 常见陷阱和错误

- **意外移动**：!Unpin 类型 pinned 后移动导致 UB；用 Pin API。
- **Trait 默认 Sized**：trait 方法 Self Sized；加 ?Sized 以支持 unsized !Unpin。
- **Futures 不 Unpin**：需 Pin 以 poll；用 Box::pin。
- **PhantomPinned 滥用**：仅需时使用；否则保持 Unpin。
- **Unsafe 错误**：手动 unpin !Unpin 类型导致 UB。
