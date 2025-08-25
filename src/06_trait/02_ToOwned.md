# Trait ToOwned


`ToOwned` trait 来自 `std::borrow` 模块，它的主要目的是从借用数据创建拥有所有权的副本。它是 `Clone` trait 的泛化版本，允许从借用类型（如 `&str`）创建拥有类型（如 `String`），而无需直接实现 `Clone`。 与 `Clone` 不同，`ToOwned` 专注于借用到拥有的转换，尤其在借用类型不是 `Self` 时有用。

## 1. `ToOwned` Trait 简介

### 1.1 定义和目的
`ToOwned` trait 定义在 `std::borrow::ToOwned` 中，自 Rust 1.0.0 起稳定可用。其语法如下：
```rust
pub trait ToOwned {
    type Owned: Borrow<Self>;
    fn to_owned(&self) -> Self::Owned;
    fn clone_into(&self, target: &mut Self::Owned) { /* 默认实现，使用 to_owned */ }
}
```
- **关联类型**：`Owned: Borrow<Self>` - 拥有的类型，必须能借用回原借用类型（确保 round-trip）。
- **方法**：
    - `to_owned(&self) -> Self::Owned` - 从借用创建拥有副本，通常分配新内存。
    - `clone_into(&self, target: &mut Self::Owned)` - 将副本克隆到现有目标中（可选优化，默认使用 `to_owned`）。

**目的**：`ToOwned` 允许从借用数据（如切片、字符串切片）创建拥有所有权的版本，而无需知道具体拥有类型。这在标准库中广泛用于如 `&str` 的 `to_owned()` 返回 `String`，或 `&[T]` 返回 `Vec<T>`。 它促进泛型编程，提供从借用到拥有的标准化方式，尤其在处理集合或借用数据时。

根据官方文档，`ToOwned` 是 `Clone` 的泛化：`Clone` 从 `&Self` 到 `Self`，而 `ToOwned` 从 `&Self` 到 `Owned`（可能不同类型）。 它确保 `Owned` 类型能借用回 `Self`，保持一致性。

- **为什么需要 `ToOwned`？** Rust 强调所有权和借用。`ToOwned` 简化借用到拥有的转换，支持泛型函数边界，并避免手动克隆逻辑。 例如，在处理借用字符串时，使用 `to_owned()` 获取 `String` 而无需 `.clone()` 或 `String::from`。

### 1.2 与相关 Trait 的区别
`ToOwned` 与几个转换 trait 相关，但专注于借用到拥有的转换：

- **与 `Clone`**：
    - `ToOwned` 从 `&Self` 到 `Owned`（可能不同）；`Clone` 从 `&Self` 到 `Self`。
    - `ToOwned` 更泛化；`Clone` 更具体，用于相同类型。
    - 示例：`&str` 的 `to_owned()` 返回 `String`；`String` 的 `clone()` 返回 `String`。
    - 选择：如果借用和拥有类型不同，用 `ToOwned`；否则 `Clone` 足够。

- **与 `Into<Owned>`**：
    - `ToOwned` 从借用（`&Self`）到拥有；`Into` 从拥有（`Self`）到拥有。
    - `ToOwned` 用于借用数据；`Into` 用于所有权转移。
    - 示例：`&str` 无 `Into<String>`（因为借用）；但有 `to_owned()`。

- **与 `Borrow`**：
    - `ToOwned` 从借用到拥有；`Borrow` 从拥有到借用。
    - `ToOwned::Owned: Borrow<Self>` 确保 round-trip（借用到拥有再借用回借用）。
    - 示例：`String: Borrow<str>`，`str: ToOwned<Owned=String>`。

- **与 `ToString`**：
    - `ToOwned` 泛化转换；`ToString` 专用于到 `String`。
    - `ToString` 基于 `Display`；`ToOwned` 基于克隆。
    - 示例：`&str.to_owned()` 返回 `String`；`value.to_string()` 返回格式化 `String`。

**何时选择？** 用 `ToOwned` 处理借用到拥有的泛型转换，尤其在借用类型如切片时；对于相同类型，用 `Clone`；对于字符串输出，用 `ToString`。 最佳实践：实现 `ToOwned` 时，确保 `Owned` 能借用回 `Self` 以保持一致性。

## 2. 手动实现 `ToOwned`

`ToOwned` 不能自动派生（derive），必须手动实现。但实现简单：定义 `Owned` 和 `to_owned`，可选优化 `clone_into`。

### 2.1 基本示例：借用类型
标准库示例：`str` 实现 `ToOwned`：
```rust
impl ToOwned for str {
    type Owned = String;
    fn to_owned(&self) -> String {
        self.to_string()
    }
    fn clone_into(&self, target: &mut String) {
        target.clear();
        target.push_str(self);
    }
}
```
- 从 `&str` 创建 `String`，`clone_into` 优化重用现有 `String`。

### 2.2 自定义类型
从 Stack Overflow 示例：
```rust
use std::borrow::{Borrow, ToOwned};

#[derive(Clone)]
struct Foo {
    data: String,
}

impl ToOwned for Foo {
    type Owned = Foo;

    fn to_owned(&self) -> Foo {
        self.clone()
    }
}
```
- 对于相同类型，使用 `Clone` 实现。

### 2.3 不同类型实现
```rust
struct MySlice<'a>(&'a [i32]);

impl<'a> ToOwned for MySlice<'a> {
    type Owned = Vec<i32>;

    fn to_owned(&self) -> Vec<i32> {
        self.0.to_vec()
    }
}
```
- 从自定义切片到 `Vec`。

### 2.4 优化 clone_into
```rust
impl ToOwned for [u8] {
    type Owned = Vec<u8>;

    fn to_owned(&self) -> Vec<u8> {
        self.to_vec()
    }

    fn clone_into(&self, target: &mut Vec<u8>) {
        target.clear();
        target.extend_from_slice(self);
    }
}
```
- 优化重用目标内存。

## 3. 与 Borrow 的关系

`ToOwned::Owned: Borrow<Self>` 确保拥有的类型能借用回借用类型：
```rust
let borrowed: &str = "hello";
let owned: String = borrowed.to_owned();
let reborrowed: &str = owned.borrow();  // 通过 Borrow
assert_eq!(borrowed, reborrowed);
```
- 支持 round-trip 转换。

## 4. 高级主题

### 4.1 Blanket Implementations
标准库提供 blanket impl：对于实现 `Clone` 的类型，`impl<T: Clone> ToOwned for T { type Owned = T; fn to_owned(&self) -> T { self.clone() } }`。 这允许任何 `Clone` 类型自动获 `ToOwned`。

自定义 blanket 需小心孤儿规则。

### 4.2 对于 Trait 对象
`ToOwned` 可用于 trait 对象，如果 `Owned` 是 `Box<dyn Trait>`：
```rust
trait MyTrait: Clone {}

impl ToOwned for dyn MyTrait {
    type Owned = Box<dyn MyTrait>;

    fn to_owned(&self) -> Box<dyn MyTrait> {
        Box::new(self.clone())
    }
}
```
- 需要 trait 支持 `Clone`。

### 4.3 与 Cow 结合
`ToOwned` 常与 `Cow`（Clone on Write）结合：
```rust
use std::borrow::Cow;

fn process<'a>(s: Cow<'a, str>) -> String {
    if s.is_borrowed() {
        s.to_owned()
    } else {
        s.into_owned()
    }
}
```
- `Cow<'a, str>: ToOwned<Owned=String>`。

## 5. 常见用例

- **借用到拥有**：从 `&str` 到 `String`。
- **泛型函数**：边界 `T: ToOwned<Owned=U>` 接受借用并转换为拥有。
- **集合处理**：克隆借用键到拥有。
- **性能优化**：使用 `clone_into` 重用内存。
- **库设计**：支持借用数据的拥有转换。

## 6. 最佳实践

- **实现 Borrow**：确保 `Owned: Borrow<Self>`。
- **优化 clone_into**：减少分配。
- **边界用 ToOwned**：泛型借用到拥有。
- **文档**：说明转换语义。
- **测试**：验证 round-trip 和等价。
- **避免 panic**：转换不应失败。

## 7. 常见陷阱和错误

- **失败时 panic**：违反约定；用 Err。
- **失败克隆**：失败时 panic；用 Err。
- **失败时 panic**：违反约定；用 Err。
- **方向混淆**：`TryInto<T> for U` 是从 `U` 到 `T`。
- **与 TryFrom 冲突**：优先 TryFrom。
- **性能**：转换可能有检查开销。