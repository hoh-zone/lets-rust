# Rust Trait BorrowMut

`BorrowMut` trait 来自 `std::borrow` 模块，它的主要目的是允许类型互借为另一种类型，同时确保借用值与原值在比较、哈希和相等性上等价，并支持可变借用。它常用于集合如 `HashMap` 的键借用，允许修改借用值，同时保持语义一致。 与 `Borrow` 不同，`BorrowMut` 专注于可变借用，常与内部可变性模式结合使用，如在 `RefCell` 中。

## 1. `BorrowMut` Trait 简介

### 1.1 定义和目的
`BorrowMut` trait 定义在 `std::borrow::BorrowMut` 中，自 Rust 1.0.0 起稳定可用。其语法如下：
```rust
pub trait BorrowMut<Borrowed: ?Sized>: Borrow<Borrowed> {
    fn borrow_mut(&mut self) -> &mut Borrowed;
}
```
- **目的**：提供一种可变借用机制，确保借用值（`&mut Borrowed`）与原值在 `Eq`、`Ord` 和 `Hash` 上等价。这允许类型在集合中作为键时，使用可变借用形式操作，而无需克隆或转换。 根据官方文档，`BorrowMut` 是 `Borrow` 的伴侣 trait，用于可变借用数据。 它促进高效的借用语义，尤其在泛型集合中：如 `HashMap<K, V>` 的可变操作允许借用键。

`BorrowMut` 的设计目的是支持内部可变性，并在借用时保持一致的哈希和比较。它在标准库中广泛用于如 `Vec<T>` 实现 `BorrowMut<[T]>`，允许可变借用为切片。

- **为什么需要 `BorrowMut`？** 在 Rust 中，可变借用需要严格遵守借用规则。`BorrowMut` 允许类型可变借用为更通用的形式（如 `Vec<T>` 到 `[T]`），简化 API 并避免不必要的分配。 例如，在处理可变集合时，使用 `BorrowMut` 确保借用安全且高效。

### 1.2 与相关 Trait 的区别
`BorrowMut` 与几个引用 trait 相关，但强调可变借用和语义等价：

- **与 `Borrow`**：
    - `BorrowMut` 用于可变借用（`&mut Borrowed`）；`Borrow` 用于不可变借用（`&Borrowed`）。
    - `BorrowMut` 继承 `Borrow`，所以实现 `BorrowMut` 必须也实现 `Borrow`。
    - 选择：如果需要修改借用值，用 `BorrowMut`；否则用 `Borrow`。

- **与 `AsMut`**：
    - `BorrowMut` 要求借用值与原值哈希/比较等价；`AsMut` 无此要求，仅转换可变引用。
    - `BorrowMut` 用于键借用（如可变集合）；`AsMut` 用于通用可变引用转换。
    - 示例：`Vec<T>` 实现 `AsMut<[T]>` 和 `BorrowMut<[T]>`；但 `BorrowMut` 确保切片哈希同向量。
    - 选择：如果需要哈希等价，用 `BorrowMut`；否则 `AsMut` 更灵活。

- **与 `DerefMut`**：
    - `BorrowMut` 是借用 trait；`DerefMut` 是解引用 trait，支持 `*mut` 和 coercion。
    - `DerefMut` 支持方法继承；`BorrowMut` 无 coercion，仅借用。
    - `BorrowMut` 更严格（等价要求）；`DerefMut` 更强大但可能不安全。
    - 示例：`RefCell<T>` 使用 `DerefMut` 但结合 `BorrowMut` 支持内部可变性。

**何时选择？** 用 `BorrowMut` 在可变集合键或需要等价可变借用的场景；对于通用可变引用，用 `AsMut`；对于智能指针，用 `DerefMut`。 最佳实践：如果类型实现 `Borrow`，考虑添加 `BorrowMut` 以支持可变借用。

## 2. 手动实现 `BorrowMut`

`BorrowMut` 不能自动派生（derive），必须手动实现。但实现简单：返回可变借用，并确保继承 `Borrow`。

### 2.1 基本示例：结构体
```rust
use std::borrow::{Borrow, BorrowMut};

struct MyVec<T>(Vec<T>);

impl<T> Borrow<[T]> for MyVec<T> {
    fn borrow(&self) -> &[T] {
        &self.0
    }
}

impl<T> BorrowMut<[T]> for MyVec<T> {
    fn borrow_mut(&mut self) -> &mut [T] {
        &mut self.0
    }
}

fn main() {
    let mut v = MyVec(vec![1, 2, 3]);
    let borrowed: &mut [i32] = v.borrow_mut();
    borrowed[0] = 10;
    assert_eq!(v.0, vec![10, 2, 3]);
}
```
- 支持可变借用为切片，并继承 `Borrow`。

### 2.2 枚举
```rust
use std::borrow::{Borrow, BorrowMut};

enum Container<T> {
    Vec(Vec<T>),
    Slice(&'static [T]),
}

impl<T> Borrow<[T]> for Container<T> {
    fn borrow(&self) -> &[T] {
        match self {
            Container::Vec(v) => v.as_slice(),
            Container::Slice(s) => s,
        }
    }
}

impl<T> BorrowMut<[T]> for Container<T> {
    fn borrow_mut(&mut self) -> &mut [T] {
        match self {
            Container::Vec(v) => v.as_mut_slice(),
            Container::Slice(_) => panic!("Cannot mutably borrow slice"),  // 或返回 Err，但 trait 不允许失败
        }
    }
}
```
- 支持变体借用，但需小心不可变变体。

### 2.3 泛型类型
```rust
struct Wrapper<T>(T);

impl<T, U: ?Sized> Borrow<U> for Wrapper<T>
where T: Borrow<U> {
    fn borrow(&self) -> &U {
        self.0.borrow()
    }
}

impl<T, U: ?Sized> BorrowMut<U> for Wrapper<T>
where T: BorrowMut<U> {
    fn borrow_mut(&mut self) -> &mut U {
        self.0.borrow_mut()
    }
}
```
- 委托给内部类型。

### 2.4 与内部可变性结合
从 Rust Book 示例：
```rust
use std::cell::RefCell;
use std::borrow::{Borrow, BorrowMut};

let cell = RefCell::new(5);
let mut borrowed = cell.borrow_mut();  // 通过 BorrowMut
*borrowed = 10;
assert_eq!(*cell.borrow(), 10);
```
- `RefCell` 使用 `BorrowMut` 支持运行时借用检查。

## 3. 标准库实现

- `Vec<T>` 实现 `BorrowMut<[T]>`。
- `String` 实现 `BorrowMut<str>`（自 1.36.0）。
- `&mut T` 和 `T` 有 blanket impl。
- `Box<T>` 实现 `BorrowMut<T>`。

## 4. 高级主题

### 4.1 Blanket Implementations
标准库提供 blanket impl：
- 对于 `T`：`impl<T: ?Sized> BorrowMut<T> for T { fn borrow_mut(&mut self) -> &mut T { self } }`。
- 对于 `&mut T`：支持引用借用。

### 4.2 对于 Trait 对象
```rust
trait MyTrait {}

impl BorrowMut<dyn MyTrait> for Box<dyn MyTrait> {
    fn borrow_mut(&mut self) -> &mut dyn MyTrait {
        &mut **self
    }
}
```
- 支持动态可变借用。

### 4.3 与 Cow 结合
`BorrowMut` 常与 `Cow`（Clone on Write）结合，用于可变借用：
```rust
use std::borrow::Cow;

fn modify<T: BorrowMut<str> + Into<Cow<'static, str>>>(data: &mut T) {
    let mut s: &mut str = data.borrow_mut();
    s.make_ascii_uppercase();
}
```
- 允许借用或拥有。

## 5. 常见用例

- **可变集合键**：混合键类型可变操作。
- **内部可变性**：如 `RefCell` 支持运行时借用。
- **泛型函数**：可变借用参数。
- **包装类型**：可变借用内部。
- **性能优化**：避免克隆可变键。

## 6. 最佳实践

- **继承 Borrow**：始终实现 `Borrow` 以匹配。
- **确保等价**：借用值必须哈希/比较同原值。
- **与 AsMut 结合**：多 trait 支持。
- **文档**：说明借用语义和等价。
- **测试**：验证哈希等价和借用安全。
- **避免多重借用**：使用模式避免借用 checker 错误。

## 7. 常见陷阱和错误

- **无等价**：导致集合行为不一致。
- **借用 checker 冲突**：多次可变借用导致错误。
- **孤儿规则**：不能为外部类型实现。
- **与 DerefMut 混淆**：`BorrowMut` 无 coercion。
- **性能**：复杂借用可能有开销。

