# Trait Ord

`Ord` trait 来自 `std::cmp` 模块，它的主要目的是为类型定义一个总序关系（total order）的比较操作。它要求类型实现 `PartialOrd`，并额外保证比较是总序，即对于任何两个值，总有一个明确的顺序关系（小于、等于或大于），适合用于排序或有序集合。与 `PartialOrd` 不同，`Ord` 表示一个总序关系（total order），确保所有值都可比较，且满足反自反性（irreflexivity for <）、传递性和连通性（totality）。`Ord` 是 `PartialOrd` 的子 trait，用于更严格的顺序语义，尤其在标准库集合如 `BTreeMap` 中，作为键要求 `Ord` 以确保有序存储。

## 1. `Ord` Trait 简介

### 1.1 定义和目的
`Ord` trait 定义在 `std::cmp::Ord` 中，自 Rust 1.0.0 起稳定可用。其语法如下：
```rust
pub trait Ord: Eq + PartialOrd<Self> {
    fn cmp(&self, other: &Self) -> Ordering;
    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    { max(self, other) }
    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    { min(self, other) }
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized + PartialOrd,
    { clamp(self, min, max) }
}
```
- **继承**：`Ord` 继承 `Eq` 和 `PartialOrd<Self>`，因此实现 `Ord` 的类型必须也实现 `Eq` 和 `PartialOrd`，确保相等和顺序一致。
- **方法**：
    - `cmp(&self, other: &Self) -> Ordering`：返回 self 和 other 的顺序（Less、Equal 或 Greater），用于 `<`、`>`、`<=`、`>=` 操作符。
    - `max(self, other: Self) -> Self`：返回较大值，默认实现。
    - `min(self, other: Self) -> Self`：返回较小值，默认实现。
    - `clamp(self, min: Self, max: Self) -> Self`：将值限制在 [min, max]，默认实现（自 1.50.0）。

**目的**：`Ord` 提供一种总序关系，允许类型定义严格的比较顺序，确保所有值都可比，且顺序满足反自反、对称（对于 ==）、传递和连通。这在标准库中广泛用于如 `BTreeSet` 或 `sort` 函数，其中顺序必须可靠以避免不一致。根据官方文档，`Ord` 是 `PartialOrd` 的加强版，用于类型不支持部分序的场景（如整数的总序）。 它促进一致的排序语义，支持泛型代码中的顺序检查，而无需担心不可比值。

`Ord` 的设计目的是提供严格的总序，确保比较在数学上是可靠的，尤其在有序集合或排序中。

- **为什么需要 `Ord`？** Rust 的比较系统区分部分和总序。`Ord` 允许类型定义严格总序，支持有序集合和排序，而 `PartialOrd` 允许如浮点数的部分序（NaN 不可比）。 例如，在 `BTreeMap<K, V>` 中，`K: Ord` 确保键有序存储。

### 1.2 与相关 Trait 的区别
`Ord` 与几个比较 trait 相关，但侧重总序：

- **与 `PartialOrd`**：
    - `Ord`：总序，要求所有值可比（无不可比），继承 `PartialOrd`。
    - `PartialOrd`：部分序，可能有不可比值（如浮点 NaN）。
    - `Ord` 是 `PartialOrd` 的子 trait；实现 `Ord` 自动获 `PartialOrd`，但反之不成立。
    - 示例：整数实现 `Ord`（总序）；浮点实现 `PartialOrd` 但不 `Ord`（因 NaN）。
    - 选择：如果类型支持总序，用 `Ord` 以严格；否则用 `PartialOrd` 以灵活。

- **与 `Eq` 和 `PartialEq`**：
    - `Ord`：顺序；`Eq`：总等价，继承 `PartialEq`。
    - `Ord` 要求 `Eq`，以一致相等（a == b 隐含 cmp(a, b) == Equal）。
    - 示例：整数实现 `Ord` 和 `Eq`；浮点实现 `PartialOrd` 和 `PartialEq`。
    - 区别：`Ord` 是顺序；`Eq` 是相等。

- **与 `Hash`**：
    - `Ord`：顺序；`Hash`：哈希计算。
    - 有序集合如 `BTreeMap` 要求键 `Ord`；哈希集合要求 `Eq + Hash`。
    - 示例：自定义类型实现 `Ord` 以用作 BTree 键。

**何时选择？** 用 `Ord` 当需要总序时，尤其在排序或有序集合中；用 `PartialOrd` 当允许部分序，尤其浮点。 最佳实践：为大多数类型派生 `Ord`，除非有如 NaN 的特殊语义。

## 2. 自动派生 `Ord`（Deriving Ord）

Rust 允许使用 `#[derive(Ord)]` 为结构体、枚举和联合体自动实现 `Ord`，前提是所有字段都实现了 `Ord`、`Eq` 和 `PartialOrd`。这是最简单的方式，尤其适用于简单类型。

### 2.1 基本示例：结构体
```rust
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 2, y: 1 };
    assert!(p1 < p2);  // true，基于字段顺序比较
}
```
- 派生比较字段，从左到右。

### 2.2 枚举
```rust
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

fn main() {
    let c = Shape::Circle { radius: 5.0 };
    let r = Shape::Rectangle { width: 4.0, height: 3.0 };
    assert!(c > r);  // true，如果 Circle 变体序号大于 Rectangle
}
```
- 派生先比较变体序号（定义顺序），然后字段。

### 2.3 泛型类型
```rust
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Pair<T: Ord> {
    first: T,
    second: T,
}

fn main() {
    let pair1 = Pair { first: 1, second: 2 };
    let pair2 = Pair { first: 2, second: 1 };
    assert!(pair1 < pair2);  // true
}
```
- 约束 `T: Ord` 以派生。

**注意**：派生要求所有字段 `Ord`；浮点字段不能派生 `Ord`（因 NaN），需手动实现 `PartialOrd`。

## 3. 手动实现 `Ord`

当需要自定义顺序逻辑时，必须手动实现 `Ord`（和 `PartialOrd`、`Eq`、`PartialEq`）。

### 3.1 基本手动实现
```rust
use std::cmp::{Ord, PartialOrd, Eq, PartialEq, Ordering};

struct Person {
    age: u32,
    name: String,
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.age == other.age && self.name == other.name
    }
}

impl Eq for Person {}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Person {
    fn cmp(&self, other: &Self) -> Ordering {
        self.age.cmp(&other.age).then(self.name.cmp(&other.name))
    }
}

fn main() {
    let p1 = Person { age: 30, name: "Alice".to_string() };
    let p2 = Person { age: 25, name: "Bob".to_string() };
    assert!(p1 > p2);  // true，先 age 然后 name
}
```
- 手动实现 `cmp`，链式比较字段。

### 3.2 浮点类型手动实现
浮点默认 `PartialOrd`：
```rust
let a = f64::NAN;
let b = 1.0;
assert_eq!(a.partial_cmp(&b), None);  // None，不可比
```
- NaN 与任何值不可比。

自定义总序浮点：
```rust
struct TotalFloat(f64);

impl PartialEq for TotalFloat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for TotalFloat {}

impl PartialOrd for TotalFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for TotalFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        // 自定义处理 NaN，如 NaN > all 或 < all
        if self.0.is_nan() && other.0.is_nan() {
            Ordering::Equal
        } else if self.0.is_nan() {
            Ordering::Greater
        } else if other.0.is_nan() {
            Ordering::Less
        } else {
            self.0.partial_cmp(&other.0).unwrap()
        }
    }
}
```
- 手动处理 NaN 以总序。

### 3.3 泛型类型
```rust
struct Wrapper<T> {
    inner: T,
}

impl<T: PartialEq> PartialEq for Wrapper<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T: Eq> Eq for Wrapper<T> {}

impl<T: PartialOrd> PartialOrd for Wrapper<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<T: Ord> Ord for Wrapper<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}
```
- 委托给内部类型。

## 4. 高级主题

### 4.1 与 Hash 结合
实现 `Ord + Hash` 以用作有序哈希：
```rust
use std::hash::{Hash, Hasher};

impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.age.hash(state);
        self.name.hash(state);
    }
}
```
- 确保 cmp(a, b) == Equal 隐含 hash(a) == hash(b)。

### 4.2 第三方 Crate：ord_subset
使用 crate 如 `ord_subset` 处理子集序。

## 5. 常见用例

- **排序**：vec.sort() 要求 Ord。
- **有序集合**：BTreeMap 要求键 Ord。
- **最大/最小**：max/min 方法。
- **夹紧**：clamp 值。
- **泛型边界**：T: Ord 以总序。

## 6. 最佳实践

- **优先派生**：用 `#[derive(Ord, PartialOrd, Eq, PartialEq)]` 简化。
- **与 Eq 配对**：Ord 要求 Eq。
- **浮点小心**：避免 Ord，用 PartialOrd。
- **链式 cmp**：用 then 比较多字段。
- **文档**：说明顺序语义。
- **测试**：验证反自反、传递、连通。

## 7. 常见陷阱和错误

- **浮点 Ord**：NaN 违反连通；勿派生。
- **无 Eq**：Ord 要求继承 Eq。
- **不一致 cmp**：违反总序导致排序错误。
- **泛型无边界**：默认无 Ord；添加边界。
- **循环递归**：cmp 导致无限循环；用 raw 字段。