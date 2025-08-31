# Trait PartialEq

`PartialEq` trait 来自 `std::cmp` 模块，它的主要目的是为类型定义一个部分等价关系（partial equivalence relation）的相等比较。它允许类型实现 `==` 和 `!=` 操作符，支持部分相等语义，例如浮点数中的 NaN 不等于自身。 与 `Eq` 不同，`PartialEq` 不要求自反性（reflexivity），因此适合如浮点数的场景，其中 NaN != NaN。 `PartialEq` 是 `Eq` 的超 trait，用于更灵活的相等语义，尤其在标准库集合如 `HashMap` 中，可以作为键要求 `PartialEq` 以支持部分相等。

## 1. `PartialEq` Trait 简介

### 1.1 定义和目的
`PartialEq` trait 定义在 `std::cmp::PartialEq` 中，自 Rust 1.0.0 起稳定可用。其语法如下：
```rust
pub trait PartialEq<Rhs: ?Sized = Self> {
    fn eq(&self, other: &Rhs) -> bool;
    fn ne(&self, other: &Rhs) -> bool { !self.eq(other) }
}
```
- **泛型参数**：`Rhs: ?Sized = Self` - 允许比较不同类型（cross-type comparison），默认 Self 以支持同类型比较。
- **方法**：
    - `eq(&self, other: &Rhs) -> bool`：测试 self 和 other 是否相等，用于 `==` 操作符。
    - `ne(&self, other: &Rhs) -> bool`：测试不等，用于 `!=`，默认实现为 `!eq`，不应覆盖以保持一致性。

**目的**：`PartialEq` 提供一种部分等价关系，允许类型定义相等比较，而不要求所有值都可比较或自反。这在标准库中广泛用于如浮点数（f32、f64）的比较，其中 NaN != NaN。根据官方文档，`PartialEq` 对应部分等价关系（partial equivalence relation），支持对称（symmetry）和传递（transitivity），但不要求自反，用于类型如浮点数。 它促进一致的比较语义，支持泛型代码中的相等检查，而无需担心总等价的要求。

`PartialEq` 的设计目的是提供灵活的相等，支持 cross-type 比较（如 `Book` 和 `BookFormat`），并允许实现对称和传递，但不强制自反。 它不定义新语义，而是依赖实现者的保证。

- **为什么需要 `PartialEq`？** Rust 的比较系统区分部分和总相等。`PartialEq` 允许类型定义灵活相等，支持如浮点数的特殊语义，而 `Eq` 要求严格总等价。 例如，在集合中，键可能只需 `PartialEq`，但哈希表要求 `Eq` 以确保一致。

### 1.2 与相关 Trait 的区别
`PartialEq` 与几个比较 trait 相关，但侧重部分相等：

- **与 `Eq`**：
    - `PartialEq`：部分等价，可能不自反（如 NaN != NaN）；不要求总等价。
    - `Eq`：总等价，要求自反、对称、传递；继承 `PartialEq`。
    - `Eq` 是 `PartialEq` 的子 trait；实现 `Eq` 自动获 `PartialEq`，但反之不成立。
    - 示例：整数实现 `Eq`（总等价）；浮点实现 `PartialEq` 但不 `Eq`（因 NaN）。
    - 选择：如果类型支持部分相等，用 `PartialEq` 以灵活；否则用 `Eq` 以严格。

- **与 `PartialOrd` 和 `Ord`**：
    - `PartialEq`：相等；`PartialOrd`：部分序，可能不总比较（如浮点 NaN）。
    - `Ord`：总序，继承 `Eq` 和 `PartialOrd`。
    - `PartialOrd` 要求与 `PartialEq` 一致（a < b 隐含 a != b）。
    - 示例：整数实现 `Ord` 和 `Eq`；浮点实现 `PartialOrd` 和 `PartialEq`。
    - 区别：`PartialEq` 是相等；`PartialOrd` 是顺序。

- **与 `Hash`**：
    - `PartialEq`：相等；`Hash`：哈希计算。
    - 集合如 `HashMap` 要求键 `PartialEq + Hash`，但 `Eq` 用于总等价。
    - 确保 a == b 隐含 hash(a) == hash(b)，即使部分相等。

**何时选择？** 用 `PartialEq` 当允许部分相等时，尤其浮点；用 `Eq` 当需要总等价，尤其哈希。

## 2. 自动派生 `PartialEq`（Deriving PartialEq）

Rust 允许使用 `#[derive(PartialEq)]` 为结构体、枚举和联合体自动实现 `PartialEq`，前提是所有字段都实现了 `PartialEq`。这是最简单的方式，尤其适用于简单类型。

### 2.1 基本示例：结构体
```rust
#[derive(PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    assert_eq!(p1, p2);  // true
}
```
- 派生比较所有字段。

### 2.2 枚举
```rust
#[derive(PartialEq, Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

fn main() {
    let s1 = Shape::Circle { radius: 5.0 };
    let s2 = Shape::Circle { radius: 5.0 };
    assert_eq!(s1, s2);  // true
}
```
- 派生比较变体和字段。

### 2.3 泛型类型
```rust
#[derive(PartialEq, Debug)]
struct Pair<T: PartialEq> {
    first: T,
    second: T,
}

fn main() {
    let pair1 = Pair { first: 1.0, second: 2.0 };
    let pair2 = Pair { first: 1.0, second: 2.0 };
    assert_eq!(pair1, pair2);  // true
}
```
- 约束 `T: PartialEq` 以派生。

**注意**：派生要求所有字段 `PartialEq`；浮点字段可派生，但 NaN != NaN。

## 3. 手动实现 `PartialEq`

当需要自定义比较逻辑时，必须手动实现 `PartialEq`。

### 3.1 基本手动实现
```rust
use std::cmp::PartialEq;

struct Book {
    isbn: i32,
    format: BookFormat,
}

enum BookFormat {
    Paperback,
    Hardback,
    Ebook,
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.isbn == other.isbn  // 忽略 format
    }
}

fn main() {
    let b1 = Book { isbn: 3, format: BookFormat::Paperback };
    let b2 = Book { isbn: 3, format: BookFormat::Ebook };
    assert_eq!(b1, b2);  // true
}
```
- 自定义相等基于 ISBN。

### 3.2 Cross-Type 比较
```rust
impl PartialEq<BookFormat> for Book {
    fn eq(&self, other: &BookFormat) -> bool {
        self.format == *other
    }
}

impl PartialEq<Book> for BookFormat {
    fn eq(&self, other: &Book) -> bool {
        *self == other.format
    }
}

fn main() {
    let book = Book { isbn: 1, format: BookFormat::Paperback };
    assert_eq!(book, BookFormat::Paperback);  // true
}
```
- 支持不同类型比较，确保对称。

### 3.3 浮点类型手动实现
浮点默认 `PartialEq`：
```rust
let a = f64::NAN;
let b = f64::NAN;
assert!(a != b);  // true
```
- NaN != NaN，符合 IEEE 754。

## 4. 高级主题

### 4.1 与 PartialOrd 结合
实现一致：
```rust
use std::cmp::{PartialOrd, Ordering};

impl PartialOrd for Complex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // 自定义序
        None  // 或实现
    }
}
```
- 确保 a < b 隐含 a != b。

### 4.2 第三方 Crate：float_eq
使用 crate 如 `float_eq` 处理浮点相等（容差）。

## 5. 常见用例

- **相等检查**：自定义类型 ==。
- **集合键**：HashMap 可能用 PartialEq，但通常 Eq。
- **测试**：assert_eq! 使用 PartialEq。
- **排序**：PartialOrd 结合 PartialEq。
- **泛型边界**：T: PartialEq 以灵活比较。

## 6. 最佳实践

- **优先派生**：用 `#[derive(PartialEq)]` 简化。
- **与 Eq 配对**：如果总等价，实现 Eq。
- **浮点小心**：了解 NaN 行为。
- **对称确保**：cross-type 实现双向。
- **文档**：说明比较语义。
- **测试**：验证对称、传递（即使部分）。

## 7. 常见陷阱和错误

- **浮点 NaN**：NaN != NaN 导致意外；用 is_nan。
- **无对称**：cross-type 仅单向导致逻辑错误。
- **Hash 不一致**：a == b 但 hash(a) != hash(b) 导致集合错误。
- **泛型无边界**：默认无 PartialEq；添加边界。
- **循环递归**：比较导致无限循环；用 raw 字段。