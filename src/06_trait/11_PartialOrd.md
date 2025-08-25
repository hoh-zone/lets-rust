# Rust Trait PartialOrd

`PartialOrd` trait 来自 `std::cmp` 模块，它的主要目的是为类型定义一个部分预序关系（partial order）的比较操作。它允许类型实现 `<`、`>`、`<=`、`>=` 操作符，支持部分比较语义，例如浮点数中的 NaN 不可与任何值比较。与 `Ord` 不同，`PartialOrd` 表示一个部分预序关系（partial preorder），允许某些值不可比，而不要求所有值都有明确的顺序关系。`PartialOrd` 是 `Ord` 的超 trait，用于更灵活的顺序语义，尤其在标准库集合如 `BinaryHeap` 中，可以作为元素要求 `PartialOrd` 以支持部分有序。

## 1. `PartialOrd` Trait 简介

### 1.1 定义和目的
`PartialOrd` trait 定义在 `std::cmp::PartialOrd` 中，自 Rust 1.0.0 起稳定可用。其语法如下：
```rust
pub trait PartialOrd<Rhs: ?Sized = Self>: PartialEq<Rhs> {
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;
    fn lt(&self, other: &Rhs) -> bool { ... }  // 默认实现基于 partial_cmp
    fn le(&self, other: &Rhs) -> bool { ... }
    fn gt(&self, other: &Rhs) -> bool { ... }
    fn ge(&self, other: &Rhs) -> bool { ... }
}
```
- **泛型参数**：`Rhs: ?Sized = Self` - 允许比较不同类型（cross-type comparison），默认 Self 以支持同类型比较。
- **继承**：`PartialOrd` 继承 `PartialEq<Rhs>`，因此实现 `PartialOrd` 的类型必须也实现 `PartialEq`，确保顺序与相等一致（a < b 隐含 a != b）。
- **方法**：
    - `partial_cmp(&self, other: &Rhs) -> Option<Ordering>`：返回 self 和 other 的可能顺序（Some(Less/Equal/Greater) 或 None 如果不可比），用于 `<` 等操作符。
    - `lt/ le/ gt/ ge`：默认实现基于 `partial_cmp`，返回 bool，但如果不可比 panic（在 debug 中）或 false（release 中）；不应覆盖以保持一致性。

**目的**：`PartialOrd` 提供一种灵活的部分预序关系，允许类型定义顺序比较，而不要求所有值都可比。这在标准库中广泛用于如浮点数（f32、f64）的比较，其中 NaN 与任何值不可比。根据官方文档，`PartialOrd` 对应部分预序关系（partial preorder），支持反自反（irreflexivity for <）、传递性和部分连通性（partial totality），但允许不可比值，用于类型如浮点数。 它促进一致的顺序语义，支持泛型代码中的比较检查，而无需担心总序的要求。

`PartialOrd` 的设计目的是提供灵活的顺序，支持 cross-type 比较（如 `i32` 和 `f64`），并允许实现部分比较，但不强制所有值可比。

- **为什么需要 `PartialOrd`？** Rust 的比较系统区分部分和总序。`PartialOrd` 允许类型定义灵活顺序，支持如浮点数的特殊语义，而 `Ord` 要求严格总序。 例如，在排序算法中，`PartialOrd` 允许处理不可比值，而 `Ord` 保证所有可比。

### 1.2 与相关 Trait 的区别
`PartialOrd` 与几个比较 trait 相关，但侧重部分序：

- **与 `Ord`**：
    - `PartialOrd`：部分序，可能有不可比值（如 NaN）；不要求总序。
    - `Ord`：总序，要求所有值可比（无不可比），继承 `PartialOrd`。
    - `Ord` 是 `PartialOrd` 的子 trait；实现 `Ord` 自动获 `PartialOrd`，但反之不成立。
    - 示例：整数实现 `Ord`（总序）；浮点实现 `PartialOrd` 但不 `Ord`（因 NaN）。
    - 选择：如果类型支持部分序，用 `PartialOrd` 以灵活；否则用 `Ord` 以严格。

- **与 `PartialEq` 和 `Eq`**：
    - `PartialOrd`：顺序；`PartialEq`：部分等价。
    - `PartialOrd` 继承 `PartialEq`，以一致顺序（a < b 隐含 a != b）。
    - 示例：浮点实现 `PartialOrd` 和 `PartialEq`；整数实现 `Ord` 和 `Eq`。
    - 区别：`PartialOrd` 是顺序；`PartialEq` 是相等。

- **与 `Hash`**：
    - `PartialOrd`：顺序；`Hash`：哈希计算。
    - 无直接关系，但有序集合要求 `Ord`；哈希集合要求 `Eq + Hash`。
    - 示例：自定义类型实现 `PartialOrd` 以用作部分排序键。

**何时选择？** 用 `PartialOrd` 当允许部分序时，尤其浮点；用 `Ord` 当需要总序，尤其排序。 最佳实践：为大多数类型派生 `PartialOrd`，除非有如 NaN 的特殊语义。

## 2. 自动派生 `PartialOrd`（Deriving PartialOrd）

Rust 允许使用 `#[derive(PartialOrd)]` 为结构体、枚举和联合体自动实现 `PartialOrd`，前提是所有字段都实现了 `PartialOrd` 和 `PartialEq`。这是最简单的方式，尤其适用于简单类型。

### 2.1 基本示例：结构体
```rust
#[derive(PartialOrd, PartialEq, Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = Point { x: 2.0, y: 1.0 };
    assert!(p1 < p2);  // true，基于字段顺序比较
}
```
- 派生比较字段，从左到右；返回 None 如果任何字段 None。

### 2.2 枚举
```rust
#[derive(PartialOrd, PartialEq, Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

fn main() {
    let c = Shape::Circle { radius: 5.0 };
    let r = Shape::Rectangle { width: 4.0, height: 3.0 };
    assert!(c.partial_cmp(&r).is_some());  // 基于变体序号
}
```
- 派生先比较变体序号，然后字段。

### 2.3 泛型类型
```rust
#[derive(PartialOrd, PartialEq, Debug)]
struct Pair<T: PartialOrd> {
    first: T,
    second: T,
}

fn main() {
    let pair1 = Pair { first: 1.0, second: 2.0 };
    let pair2 = Pair { first: 2.0, second: 1.0 };
    assert!(pair1 < pair2);  // true
}
```
- 约束 `T: PartialOrd` 以派生。

**注意**：派生要求所有字段 `PartialOrd`；浮点字段可派生，但 NaN 返回 None。

## 3. 手动实现 `PartialOrd`

当需要自定义顺序逻辑时，必须手动实现 `PartialOrd`（和 `PartialEq`）。

### 3.1 基本手动实现
```rust
use std::cmp::{PartialOrd, PartialEq, Ordering};

struct Person {
    age: u32,
    name: String,
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.age == other.age && self.name == other.name
    }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.age.partial_cmp(&other.age) {
            Some(Ordering::Equal) => self.name.partial_cmp(&other.name),
            ord => ord,
        }
    }
}

fn main() {
    let p1 = Person { age: 30, name: "Alice".to_string() };
    let p2 = Person { age: 25, name: "Bob".to_string() };
    assert_eq!(p1.partial_cmp(&p2), Some(Ordering::Greater));
}
```
- 手动实现 `partial_cmp`，链式比较字段。

### 3.2 Cross-Type 比较
```rust
impl PartialOrd<BookFormat> for Person {
    fn partial_cmp(&self, other: &BookFormat) -> Option<Ordering> {
        // 自定义
        None
    }
}
```
- 支持不同类型顺序。

### 3.3 浮点类型手动实现
浮点默认 `PartialOrd`：
```rust
let a = f64::NAN;
let b = 1.0;
assert_eq!(a.partial_cmp(&b), None);  // None
```
- NaN 与任何值不可比。

## 4. 高级主题

### 4.1 与 PartialEq 一致
实现确保 a < b 隐含 a != b。

### 4.2 第三方 Crate：approx
使用 crate 如 `approx` 处理浮点近似比较。

## 5. 常见用例

- **部分排序**：处理浮点或不可比值。
- **集合元素**：BinaryHeap 要求 PartialOrd。
- **最大/最小**：partial_max/partial_min。
- **测试**：assert! (a < b) 使用 PartialOrd。
- **泛型边界**：T: PartialOrd 以灵活顺序。

## 6. 最佳实践

- **优先派生**：用 `#[derive(PartialOrd, PartialEq)]` 简化。
- **与 Ord 配对**：如果总序，实现 Ord。
- **浮点小心**：处理 NaN 返回 None。
- **链式 partial_cmp**：用 match 处理 None。
- **文档**：说明顺序语义。
- **测试**：验证不可比和顺序。

## 7. 常见陷阱和错误

- **浮点 NaN**：NaN 不可比导致 None；处理或用 Ord。
- **无 PartialEq**：PartialOrd 要求继承。
- **不一致 partial_cmp**：违反部分序导致逻辑错误。
- **泛型无边界**：默认无 PartialOrd；添加边界。
- **循环递归**：partial_cmp 导致无限循环；逆序字段。