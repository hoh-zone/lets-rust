# Trait FnOnce

`FnOnce` trait 来自 `std::ops` 模块，它是 Rust 函数 trait（function traits）家族的一部分，用于表示可以像函数一样调用的类型。具体来说，`FnOnce` 表示一个可以调用一次并可能消耗其捕获状态的闭包或函数指针。它允许类型实现消耗接收器的调用操作，支持泛型编程中的函数式风格。与 `FnMut` 和 `Fn` 不同，`FnOnce` 是最宽松的，允许调用一次并可能移动捕获的值，因此适合只需调用一次的场景。 Rust 的函数 trait 家族包括 `Fn`、`FnMut` 和 `FnOnce`，它们是闭包和函数指针的核心抽象，用于描述调用语义。`FnOnce` 是基础层：允许消耗 self。

## 1. `FnOnce` Trait 简介

### 1.1 定义和目的
`FnOnce` trait 定义在 `std::ops::FnOnce` 中，自 Rust 1.0.0 起稳定可用。其语法如下：
```rust
pub trait FnOnce<Args>
where
    Args: Tuple,
{
    type Output;
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}
```
- **关联类型**：`Output` - 调用返回的类型。
- **方法**：`call_once(self, args: Args) -> Self::Output` - 执行调用操作，接收 self（消耗）和参数元组。`extern "rust-call"` 指定调用约定，支持可变参数。

**目的**：`FnOnce` 提供一种抽象函数调用的方式，允许类型（如闭包或函数指针）被当作函数使用，并允许消耗捕获的状态。这在泛型编程中特别有用，可以接受任何可调用的东西作为参数，并调用一次，可能移动值。根据官方文档，`FnOnce` 适用于只需调用一次的场景，例如高阶函数或事件处理。 它促进函数式编程，支持高阶函数和闭包的泛型使用，尤其在可能消耗资源的闭包中。

`FnOnce` 的设计目的是与闭包语法集成：Rust 闭包自动实现合适的函数 trait，根据捕获方式（移动捕获 -> `FnOnce`；可变借用 -> `FnMut`；不可变 -> `Fn`）。

- **为什么需要 `FnOnce`？** Rust 的类型系统需要抽象可调用类型。`FnOnce` 允许泛型代码接受闭包或函数指针，而无需知道具体实现，支持函数作为一等公民，并允许消耗捕获的值。 例如，在库中定义接受一次性回调的函数，使用 `F: FnOnce(Args) -> Output` 边界。

### 1.2 与相关 Trait 的区别
`FnOnce` 是函数 trait 家族的基础，与 `FnMut` 和 `Fn` 紧密相关，但各有侧重：

- **与 `FnMut`**：
  - `FnOnce`：调用使用 self（消耗），只能调用一次，可能移动捕获状态。
  - `FnMut`：调用使用可变 self（`&mut self`），可以重复调用，可能修改状态。
  - `FnMut` 继承 `FnOnce`，所以 `FnMut` 类型也可作为 `FnOnce` 使用，但反之不成立。
  - 示例：闭包移动捕获实现 `FnOnce`；可变借用实现 `FnMut`。
  - 选择：如果只需调用一次且可能移动，用 `FnOnce`；否则用 `FnMut` 以重复。

- **与 `Fn`**：
  - `FnOnce`：消耗 self，只能一次。
  - `Fn`：不可变 self（`&self`），重复调用，不修改状态。
  - `Fn` 继承 `FnMut` 继承 `FnOnce`，所以 `Fn` 类型也可作为 `FnOnce` 使用。
  - 示例：闭包不可变借用实现 `Fn`；移动实现 `FnOnce`。
  - 选择：如果需要重复且不修改，用 `Fn`；否则用 `FnOnce`。

- **与 `fn` 类型**：
  - `FnOnce` 是 trait；`fn` 是函数指针类型（如 `fn(i32) -> i32`）。
  - 函数指针实现 `FnOnce`（如果安全），但闭包可能只实现 `FnOnce`。
  - 示例：`let f: fn(i32) -> i32 = add_one;` 实现 `FnOnce`；闭包可能不。

**何时选择？** 用 `FnOnce` 当只需调用一次并可能消耗时；用 `FnMut` 当重复修改；用 `Fn` 当重复不修改。 最佳实践：函数边界用 `FnOnce` 以最大兼容性。

## 2. 自动实现 `FnOnce`（Auto-Implemented）

Rust 编译器自动为闭包和函数指针实现合适的函数 trait，根据捕获方式：
- 移动捕获：实现 `FnOnce`。
- 无需手动实现 `FnOnce`；闭包语法自动处理。

### 2.1 基本示例：闭包
```rust
fn main() {
    let s = String::from("hello");
    let consume = move || { drop(s); };  // 实现 FnOnce，因为 move
    consume();  // 调用一次
    // consume();  // 错误：已消耗
}
```
- 移动捕获闭包实现 `FnOnce`。

### 2.2 函数指针
```rust
fn square(x: i32) -> i32 { x * x }

fn main() {
    let f: fn(i32) -> i32 = square;  // fn pointer 实现 FnOnce
    println!("{}", f(5));  // 25
}
```
- 函数指针实现 `FnOnce`。

### 2.3 泛型函数边界
```rust
fn call_once<F: FnOnce(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

fn main() {
    let add_one = |y| y + 1;
    println!("{}", call_once(add_one, 5));  // 6
}
```
- `F: FnOnce` 允许消耗调用。

## 3. 手动实现 `FnOnce`

手动实现 `FnOnce` 罕见，通常用于自定义可调用类型。需实现 `call_once`。

### 3.1 手动示例
```rust
use std::ops::FnOnce;

struct Consumer(String);

impl FnOnce<()> for Consumer {
    type Output = String;

    extern "rust-call" fn call_once(self, _args: ()) -> String {
        self.0  // 消耗 self
    }
}

fn main() {
    let cons = Consumer("hello".to_string());
    let result = cons();  // 调用一次
    println!("{}", result);  // hello
}
```
- 自定义类型实现 `FnOnce`。

## 4. 高级主题

### 4.1 函数 trait 层级
- `FnOnce`：最宽松，只能一次，可能消耗。
- `FnMut`：继承 `FnOnce`，可重复，可修改。
- `Fn`：继承 `FnMut`，可重复，不可修改。
- 使用最宽边界以最大兼容性。

### 4.2 与 Trait 对象
```rust
trait MyFnOnce: FnOnce(i32) -> i32 {}

impl MyFnOnce for fn(i32) -> i32 {}

fn main() {
    let f: Box<dyn MyFnOnce> = Box::new(|x| x + 1);
    let result = f(5);  // 调用一次
    println!("{}", result);  // 6
}
```
- 支持动态一次性函数。

### 4.3 Crate fn_traits
使用 crate 如 `fn_traits` 在稳定 Rust 中手动实现函数 trait。

## 5. 常见用例

- **高阶函数**：接受一次性闭包。
- **事件处理**：消耗回调。
- **资源消耗**：调用后释放。
- **并发**：一次性闭包在线程。
- **泛型库**：接受消耗函数。

## 6. 最佳实践

- **选择合适 trait**：用 `FnOnce` 以允许消耗。
- **边界最宽**：函数参数用 `FnOnce` 以兼容。
- **闭包语法**：依赖自动实现。
- **文档**：说明边界原因。
- **测试**：验证只调用一次。
- **性能**：`FnOnce` 无开销。

## 7. 常见陷阱和错误

- **捕获方式**：非 move 捕获导致错 trait；用 move。
- **边界太严**：`FnMut` 拒绝 `FnOnce` 闭包；用 `FnOnce`。
- **函数指针 vs 闭包**：函数指针实现 `Fn`，但闭包可能 `FnOnce`。
- **Trait 对象大小**：dyn FnOnce 需要 Box 或 &。
- **稳定限制**：手动实现需 nightly 或 crate。
