# Rust Trait Borrow

`Borrow` trait 来自 `std::borrow` 模块，它的主要目的是允许类型借用为另一种类型，同时确保借用值与原值在比较、哈希和相等性上等价。它常用于集合如 `HashMap` 的键借用，允许使用 `&String` 查找 `HashMap<&str>` 中的值。 与 `AsRef` 或 `Deref` 不同，`Borrow` 强调借用的语义一致性，而不是通用引用转换。

## 1. `Borrow` Trait 简介

### 1.1 定义和目的
`Borrow` trait 定义在 `std::borrow::Borrow` 中，自 Rust 1.0.0 起稳定可用。其语法如下：
```rust
pub trait Borrow<Borrowed: ?Sized> {
    fn borrow(&self) -> &Borrowed;
}
```
- **目的**：提供一种借用机制，确保借用值（`&Borrowed`）与原值在 `Eq`、`Ord` 和 `Hash` 上等价。这允许类型在集合中作为键时，使用借用形式查找，而无需克隆或转换。 根据官方文档，`Borrow` 应返回一个廉价借用，且借用值应与原值哈希等价（hash equivalently）。 这在标准库中广泛用于如 `String` 实现 `Borrow<str>`，允许 `&String` 借用为 `&str`。

`Borrow` 的设计目的是支持高效的借用语义，尤其在泛型集合中：如 `HashMap<K, V>` 的查找方法接受 `Q: ?Sized + Hash + Eq` where `K: Borrow<Q>`，允许混合键类型。 它促进类型安全，提供零成本借用抽象。

- **为什么需要 `Borrow`？** 在 Rust 中，集合键需要一致的哈希和比较。`Borrow` 允许类型借用为更通用的形式（如 `String` 到 `str`），简化 API 并避免不必要的分配。 例如，在 `HashMap<&str, V>` 中，使用 `String` 作为查询键，而无需 `.as_str()`。

### 1.2 与相关 Trait 的区别
`Borrow` 与几个引用 trait 相关，但强调语义等价：

- **与 `AsRef`**：
    - `Borrow` 要求借用值与原值哈希/比较等价；`AsRef` 无此要求，仅转换引用。
    - `Borrow` 用于键借用（如集合查找）；`AsRef` 用于通用引用转换（如路径处理）。
    - 示例：`String` 实现 `AsRef<str>` 和 `Borrow<str>`；但自定义类型可能仅需 `AsRef`。
    - 选择：如果需要哈希等价，用 `Borrow`；否则 `AsRef` 更灵活。

- **与 `Deref`**：
    - `Borrow` 是借用 trait；`Deref` 是解引用 trait，支持 `*` 和 coercion。
    - `Deref` 支持方法继承；`Borrow` 无 coercion，仅借用。
    - `Borrow` 更严格（等价要求）；`Deref` 更强大但可能不安全。
    - 示例：`String` 实现 `Deref<Target=str>` 但不用于键借用；用 `Borrow`。

- **与 `ToOwned`**：
    - `Borrow` 从自有到借用；`ToOwned` 从借用到自有（克隆）。
    - 常结合：`Borrowed: ToOwned<Owned = Self>`。
    - 示例：`str` 实现 `ToOwned<Owned=String>`。

**何时选择？** 用 `Borrow` 在集合键或需要等价借用的场景；对于通用引用，用 `AsRef`；对于智能指针，用 `Deref`。 最佳实践：如果类型实现 `Deref`，考虑添加 `Borrow` 以支持集合。

## 2. 手动实现 `Borrow`

`Borrow` 不能自动派生，必须手动实现。但实现简单：返回借用。

### 2.1 基本示例：结构体
```rust
use std::borrow::Borrow;

struct MyString(String);

impl Borrow<str> for MyString {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl Borrow<String> for MyString {
    fn borrow(&self) -> &String {
        &self.0
    }
}

fn main() {
    let s = MyString("hello".to_string());
    let borrowed: &str = s.borrow();
    println!("{}", borrowed);  // hello
}
```
- 支持借用为 `str` 或 `String`。

### 2.2 用于集合键
```rust
use std::collections::HashMap;
use std::hash::Hash;

let mut map: HashMap<&str, i32> = HashMap::new();
map.insert("key", 42);

let query = String::from("key");
println!("{}", map.get(&*query).unwrap());  // 42, 通过 Borrow
```
- `String: Borrow<str>` 允许 `&String` 借用为 `&str`。

### 2.3 泛型类型
```rust
struct Wrapper<T>(T);

impl<T, U: ?Sized> Borrow<U> for Wrapper<T>
where T: Borrow<U> {
    fn borrow(&self) -> &U {
        self.0.borrow()
    }
}
```
- 委托借用。

### 2.4 自定义类型确保等价
实现时，确保 `Eq`、`Hash` 等价：
```rust
impl PartialEq for MyString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for MyString {}
impl Hash for MyString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
```
- 匹配借用值的哈希。

## 3. Blanket Implementations

标准库提供 blanket impl：
- 对于任何 `T`：`impl<T: ?Sized> Borrow<T> for T { fn borrow(&self) -> &T { self } }`。 这允许自借用。

- 对于 `&T`：支持引用借用。

## 4. 高级主题

### 4.1 对于 Trait 对象
```rust
trait MyTrait {}

impl Borrow<dyn MyTrait> for Box<dyn MyTrait> {
    fn borrow(&self) -> &dyn MyTrait {
        &**self
    }
}
```
- 支持动态借用。

### 4.2 与 BorrowMut 结合
实现两者以支持可变/不可变借用。

### 4.3 第三方类型
用新类型包装实现 `Borrow`。

## 5. 常见用例

- **集合键**：混合键类型查找。
- **API 设计**：泛型借用参数。
- **包装类型**：借用内部。
- **性能**：避免克隆键。
- **库集成**：与标准集合兼容。

## 6. 最佳实践

- **确保等价**：借用值必须哈希/比较同原值。
- **与 AsRef/Deref 结合**：多 trait 支持。
- **文档**：说明借用语义。
- **测试**：验证哈希等价。
- **避免复杂借用**：保持廉价。

## 7. 常见陷阱和错误

- **无等价**：导致集合行为不一致。
- **孤儿规则**：不能为外部实现。
- **与 Deref 混淆**：`Borrow` 无 coercion。
- **性能**：复杂借用开销。
