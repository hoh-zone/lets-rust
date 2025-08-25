# Rust Trait FnMut

`FnMut` trait 来自 `std::ops` 模块，它是 Rust 函数 trait（function traits）家族的一部分，用于表示可以像函数一样调用的类型。具体来说，`FnMut` 表示一个可以重复调用并可能修改其捕获状态的闭包或函数指针。它允许类型实现可变接收器的调用操作，支持泛型编程中的函数式风格。与 `Fn` 和 `FnOnce` 不同，`FnMut` 平衡了重复调用和状态修改的能力，适合需要修改捕获变量但可重复调用的场景。

Rust 的函数 trait 家族包括 `Fn`、`FnMut` 和 `FnOnce`，它们是闭包和函数指针的核心抽象，用于描述调用语义。`FnMut` 是中间层：允许修改但不消耗。

## 1. `FnMut` Trait 简介

### 1.1 定义和目的
`FnMut` trait 定义在 `std::ops::FnMut` 中，自 Rust 1.0.0 起稳定可用。其语法如下：
```rust
pub trait FnMut<Args>: FnOnce<Args>
where
    Args: Tuple,
{
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}
```
- **继承**：`FnMut` 继承 `FnOnce`，因此实现 `FnMut` 的类型也必须实现 `FnOnce`。这反映了 trait 的层级：`FnOnce` 是最宽松的，`FnMut` 是中间的，`Fn` 是最严格的。
- **关联类型**：无显式关联类型，但通过继承有 `Output`（从 `FnOnce`）。
- **方法**：`call_mut(&mut self, args: Args) -> Self::Output` - 执行调用操作，接收可变 self 和参数元组。`extern "rust-call"` 指定调用约定，支持可变参数。

**目的**：`FnMut` 提供一种抽象函数调用的方式，允许类型（如闭包或函数指针）被当作函数使用，并允许修改捕获的状态。这在泛型编程中特别有用，可以接受任何可调用的东西作为参数，并重复调用，同时允许内部修改。根据官方文档，`FnMut` 适用于需要重复调用且可能修改状态的场景，例如迭代器适配器或事件循环。

`FnMut` 的设计目的是与闭包语法集成：Rust 闭包自动实现合适的函数 trait，根据捕获方式（可变借用 -> `FnMut`；移动 -> `FnOnce`；不可变 -> `Fn`）。

- **为什么需要 `FnMut`？** Rust 的类型系统需要抽象可调用类型。`FnMut` 允许泛型代码接受闭包或函数指针，而无需知道具体实现，支持函数作为一等公民，同时允许状态修改。 例如，在库中定义接受可变回调的函数，使用 `F: FnMut(Args) -> Output` 边界。

### 1.2 与相关 Trait 的区别
`FnMut` 是函数 trait 家族的一部分，与 `Fn` 和 `FnOnce` 紧密相关，但各有侧重：

- **与 `Fn`**：
  - `FnMut`：调用使用可变 self（`&mut self`），可以修改捕获状态；可以重复调用。
  - `Fn`：调用使用不可变 self（`&self`），不能修改状态；可以重复调用。
  - `Fn` 继承 `FnMut`，所以 `Fn` 类型也可作为 `FnMut` 使用，但反之不成立。
  - 示例：闭包捕获可变引用实现 `FnMut`；不可变引用实现 `Fn`。
  - 选择：如果调用需修改状态，用 `FnMut`；否则用 `Fn` 以更严格安全。

- **与 `FnOnce`**：
  - `FnMut`：可变 self，重复调用，可能修改但不消耗。
  - `FnOnce`：消耗 self（`self`），只能调用一次，可能移动捕获值。
  - `FnMut` 继承 `FnOnce`，所以 `FnMut` 类型也可作为 `FnOnce` 使用。
  - 示例：闭包移动捕获实现 `FnOnce`；可变借用实现 `FnMut`。
  - 选择：如果只需调用一次且可能移动，用 `FnOnce`；否则用 `FnMut` 以重复。

- **与 `fn` 类型**：
  - `FnMut` 是 trait；`fn` 是函数指针类型（如 `fn(i32) -> i32`）。
  - 函数指针实现 `FnMut`（如果安全），但闭包可能只实现 `FnOnce`。
  - 示例：`let f: fn(i32) -> i32 = add_one;` 实现 `FnMut`；闭包可能不。

**何时选择？** 用 `FnMut` 当需要重复调用并修改状态时；用 `Fn` 当不可修改；用 `FnOnce` 当只需一次。 最佳实践：函数边界用最宽松的（如 `FnMut`），以支持更多闭包类型。

## 2. 自动实现 `FnMut`（Auto-Implemented）

Rust 编译器自动为闭包和函数指针实现合适的函数 trait，根据捕获方式：
- 可变借用捕获：实现 `FnMut` 和 `FnOnce`。
- 无需手动实现 `FnMut`；闭包语法自动处理。

### 2.1 基本示例：闭包
```rust
fn main() {
    let mut count = 0;
    let mut increment = || { count += 1; count };  // 实现 FnMut，因为 mut 捕获
    println!("{}", increment());  // 1
    println!("{}", increment());  // 2
}
```
- 可变捕获闭包实现 `FnMut`。

### 2.2 函数指针
```rust
fn square(mut x: i32) -> i32 { x *= x; x }  // 函数可修改参数

fn main() {
    let f: fn(i32) -> i32 = square;  // fn pointer 实现 FnMut? No, fn 是 Fn
    // 函数指针实现 Fn，如果不修改 self（无 self）
}
```
- 函数指针通常实现 `Fn`，非 `FnMut`（无状态）。

### 2.3 泛型函数边界
```rust
fn call_mut_twice<F: FnMut(i32) -> i32>(mut f: F, x: i32) -> i32 {
    f(x) + f(x)
}

fn main() {
    let mut factor = 2;
    let multiply = |y| { factor *= y; factor };
    println!("{}", call_mut_twice(multiply, 3));  // 6 + 18 = 24? Wait, modified
}
```
- `F: FnMut` 允许修改状态。

## 3. 手动实现 `FnMut`

手动实现 `FnMut` 罕见，通常用于自定义可调用类型。需实现 `call_mut`，并继承 `FnOnce`。

### 3.1 手动示例
```rust
use std::ops::{FnMut, FnOnce};

struct Counter {
    count: i32,
}

impl FnOnce<(i32,)> for Counter {
    type Output = i32;
    extern "rust-call" fn call_once(mut self, args: (i32,)) -> i32 {
        self.call_mut(args)
    }
}

impl FnMut<(i32,)> for Counter {
    extern "rust-call" fn call_mut(&mut self, args: (i32,)) -> i32 {
        self.count += args.0;
        self.count
    }
}

fn main() {
    let mut counter = Counter { count: 0 };
    println!("{}", counter(5));  // 5
    println!("{}", counter(3));  // 8
}
```
- 自定义类型实现 `FnMut`。

## 4. 高级主题

### 4.1 函数 trait 层级
- `FnOnce`：最宽松，只需调用一次。
- `FnMut`：继承 `FnOnce`，可重复，可修改。
- `Fn`：继承 `FnMut`，可重复，不可修改。
- 使用最宽边界以最大兼容性。

### 4.2 与 Trait 对象
```rust
trait MyFnMut: FnMut(i32) -> i32 {}

impl MyFnMut for fn(i32) -> i32 {}

fn main() {
    let f: Box<dyn MyFnMut> = Box::new(|mut x| { x += 1; x });
    println!("{}", f(5));  // 6
}
```
- 支持动态可变函数。

### 4.3 Crate fn_traits
使用 crate 如 `fn_traits` 在稳定 Rust 中手动实现函数 trait。

## 5. 常见用例

- **迭代器**：map 使用 FnMut。
- **事件循环**：可变回调。
- **状态机**：修改状态闭包。
- **并发**：可变闭包在线程。
- **泛型库**：接受可变函数。

## 6. 最佳实践

- **选择合适 trait**：用 `FnMut` 以允许修改。
- **边界最宽**：函数参数用 `FnMut` 以兼容。
- **闭包语法**：依赖自动实现。
- **文档**：说明边界原因。
- **测试**：验证状态修改。
- **性能**：`FnMut` 无开销。

## 7. 常见陷阱和错误

- **捕获方式**：非 mut 捕获导致错 trait；用 &mut。
- **边界太严**：`Fn` 拒绝 `FnMut` 闭包；用 `FnMut`。
- **函数指针 vs 闭包**：函数指针实现 `Fn`，但闭包可能 `FnMut`。
- **Trait 对象大小**：dyn FnMut 需要 Box 或 &。
- **稳定限制**：手动实现需 nightly 或 crate。