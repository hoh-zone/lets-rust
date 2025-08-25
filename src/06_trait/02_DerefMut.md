# Rust Trait DerefMut

`DerefMut` trait 来自 `std::ops` 模块，它的主要目的是实现可变解引用操作，如 `*mut` 操作符在可变上下文中的使用。它允许自定义类型像指针一样工作，支持“可变解引用强制转换”（mutable deref coercion），让编译器自动插入 `deref_mut` 调用，使类型更灵活。与 `Deref` 不同，`DerefMut` 专注于可变引用，常用于智能指针如 `Box`、`Rc`、`Arc` 和 `RefCell` 的可变访问。

## 1. `DerefMut` Trait 简介

### 1.1 定义和目的
`DerefMut` trait 定义在 `std::ops::DerefMut` 中，自 Rust 1.0.0 起稳定可用。它继承 `Deref`，其核心是定义可变解引用的方法：
```rust
pub trait DerefMut: Deref {
    fn deref_mut(&mut self) -> &mut Self::Target;
}
```
- **继承**：`DerefMut` 要求实现 `Deref`，所以可变解引用隐含不可变解引用。
- **方法**：`deref_mut(&mut self) -> &mut Self::Target` - 返回目标类型的独占引用。

**目的**：`DerefMut` 使自定义类型支持可变 `*` 操作符，并启用 mutable deref coercion：编译器自动将 `&mut T` 转换为 `&mut U`（如果 `T: DerefMut<Target=U>`），允许类型“继承”目标类型的可变方法。这在智能指针中特别有用，例如 `Box<T>` 可变解引用到 `T`，让用户像修改 `T` 一样操作 `Box<T>`。 它促进抽象，提供零成本可变指针语义，而不牺牲安全。

根据官方文档，`DerefMut` 应仅用于廉价、透明的可变解引用操作，且不应失败。它不提供默认方法，仅要求 `deref_mut`。

- **为什么需要 `DerefMut`？** Rust 的借用系统需要显式可变借用。`DerefMut` 简化智能指针的可变使用，支持方法解析（如在 `Box<Vec<i32>>` 上调用 `Vec` 的可变方法），并避免 boilerplate 代码。 例如，在库设计中，实现 `DerefMut` 让包装类型可变透明。

### 1.2 与相关 Trait 的区别
`DerefMut` 与几个引用 trait 相关，但侧重可变解引用：

- **与 `Deref`**：
    - `DerefMut` 用于可变解引用（`&mut Target`）；`Deref` 用于不可变解引用（`&Target`）。
    - `DerefMut` 继承 `Deref`，所以实现 `DerefMut` 必须也实现 `Deref`。
    - 选择：如果需要修改，用 `DerefMut`；否则 `Deref` 足够。

- **与 `AsMut`**：
    - `DerefMut` 支持 `*mut` 和 mutable coercion；`AsMut` 是显式转换 trait，无 coercion。
    - `DerefMut` 隐式调用；`AsMut` 需手动 `as_mut()`。
    - 许多类型同时实现两者，但 `DerefMut` 更强大（可变方法继承）。
    - 选择：对于智能指针，用 `DerefMut`；对于通用转换，用 `AsMut`。

- **与 `BorrowMut`**：
    - `DerefMut` 不要求哈希等价；`BorrowMut` 要求借用与原值哈希/比较等价，用于集合键。
    - `BorrowMut` 更严格；`DerefMut` 更通用，用于解引用。
    - 示例：`Vec<T>` 实现 `BorrowMut<[T]>` 以用于键借用；`Box<T>` 用 `DerefMut<T>`。

**何时选择？** 实现 `DerefMut` 如果类型是智能指针或透明包装，支持可变访问；否则用 `AsMut` 或 `BorrowMut` 以避免意外 coercion。 最佳实践：仅在解引用廉价且透明时实现。

## 2. 手动实现 `DerefMut`

`DerefMut` 不能自动派生，必须手动实现。但实现简单：定义 `deref_mut`，并实现 `Deref`。

### 2.1 基本示例：自定义智能指针
从官方示例：
```rust
use std::ops::{Deref, DerefMut};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    let mut x = MyBox::new(5);
    *x = 10;  // 使用 *mut
    assert_eq!(10, *x);
}
```
- 这里，`MyBox` 支持可变解引用。

### 2.2 Mutable Deref Coercion 示例
```rust
fn modify(s: &mut str) {
    s.make_ascii_uppercase();
}

fn main() {
    let mut m = MyBox::new(String::from("hello"));
    modify(&mut m);  // Coercion: &mut MyBox<String> -> &mut String -> &mut str
    assert_eq!(*m, "HELLO");
}
```
- 编译器自动插入 `deref_mut` 调用。

### 2.3 新类型（Newtype）实现
```rust
struct NonNegative(i32);

impl Deref for NonNegative {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NonNegative {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    let mut num = NonNegative(42);
    *num = 100;  // 可变访问
    println!("{}", *num);  // 100
}
```
- 但小心：对于新类型，实现 `DerefMut` 可能导致混淆，因为它允许绕过类型检查。

### 2.4 泛型类型
```rust
struct Wrapper<T>(T);

impl<T> Deref for Wrapper<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Wrapper<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

fn main() {
    let mut w = Wrapper(vec![1, 2, 3]);
    w.push(4);  // 通过 coercion 调用 Vec::push
    assert_eq!(w.len(), 4);
}
```
- 泛型 impl，支持任何 `T`。

## 3. Mutable Deref Coercion 详解

Mutable deref coercion 是 `DerefMut` 的关键特性：编译器在类型不匹配时自动应用 `deref_mut`。

- **规则**：如果 `T: DerefMut<Target=U>`，则 `&mut T` 可强制为 `&mut U`。 支持多级：`&mut MyBox<MyBox<T>>` -> `&mut T`。
- **应用**：函数参数、可变方法调用、字段修改。
- **限制**：仅在可变引用上下文中；不影响所有权。

示例：多级 coercion。
```rust
let mut inner = String::from("hello");
let mut outer = MyBox::new(MyBox::new(inner));
modify(&mut outer);  // &mut MyBox<MyBox<String>> -> &mut MyBox<String> -> &mut String -> &mut str
assert_eq!(*outer, "HELLO");
```
- 自动可变解包。

## 4. 高级主题

### 4.1 Deref Polymorphism（反模式）
使用 `DerefMut` 来模拟继承或多态，常被视为反模式。 示例：
```rust
struct Child {
    parent: Parent,
}

impl Deref for Child {
    type Target = Parent;
    fn deref(&self) -> &Parent { &self.parent }
}

impl DerefMut for Child {
    fn deref_mut(&mut self) -> &mut Parent { &mut self.parent }
}
```
- 这允许 `Child` “继承” `Parent` 的可变方法，但可能导致方法冲突或意外行为。 替代：使用组合和显式委托。

### 4.2 对于 Trait 对象
```rust
trait Drawable {}

struct MyDrawable;

impl Drawable for MyDrawable {}

struct Pointer(Box<dyn Drawable>);

impl Deref for Pointer {
    type Target = dyn Drawable;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl DerefMut for Pointer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}
```
- 支持动态可变分发。

### 4.3 与 RefCell 结合
`DerefMut` 常用于内部可变性：
```rust
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

let cell = RefCell::new(vec![1, 2, 3]);
let mut guard = cell.borrow_mut();  // RefMut<Vec<i32>>
guard.push(4);  // 通过 DerefMut 修改
assert_eq!(*cell.borrow(), vec![1, 2, 3, 4]);
```
- `RefMut` 实现 `DerefMut` 支持运行时借用。

## 5. 常见用例

- **智能指针**：`Box`、`Rc` 等可变访问。
- **包装类型**：如 `RefCell`、`Mutex` 的守卫。
- **可变方法继承**：让包装类型使用内部可变方法。
- **API 设计**：透明可变包装外部类型。
- **性能优化**：零成本可变抽象。

## 6. 最佳实践

- **配对 Deref**：始终实现 `Deref` 以匹配。
- **仅透明时实现**：可变解引用应像访问内部一样。
- **避免失败**：`deref_mut` 不应 panic 或失败。
- **文档**：说明 mutable coercion 行为。
- **测试**：验证 `*mut` 和可变方法调用。
- **避免新类型 DerefMut**：使用显式方法以防混淆。

## 7. 常见陷阱和错误

- **方法冲突**：包装类型方法覆盖内部可变方法。
- **意外 Coercion**：导致类型推断问题。
- **借用规则违反**：多级可变解引用可能导致别名问题。
- **Deref Polymorphism**：模拟继承导致维护难题。
- **Unsized 类型**：需小心 `?Sized` 约束。
