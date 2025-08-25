# Trait Deref

`Deref` trait 来自 `std::ops` 模块，它的主要目的是实现不可变解引用操作，如 `*` 操作符在不可变上下文中的使用。它允许自定义类型像指针一样工作，支持“解引用强制转换”（deref coercion），让编译器自动插入 `deref` 调用，使类型更灵活。 与 `DerefMut` 不同，`Deref` 专注于不可变引用，常用于智能指针如 `Box`、`Rc`、`Arc` 和 `Cow`。

## 1. `Deref` Trait 简介

### 1.1 定义和目的
`Deref` trait 定义在 `std::ops::Deref` 中，自 Rust 1.0.0 起稳定可用。其核心是定义解引用的目标类型和方法：
```rust
pub trait Deref {
    type Target: ?Sized;
    fn deref(&self) -> &Self::Target;
}
```
- **关联类型**：`Target: ?Sized` - 解引用后的类型，可能为 unsized 类型（如切片或 trait 对象）。
- **方法**：`deref(&self) -> &Self::Target` - 返回目标类型的共享引用。

**目的**：`Deref` 使自定义类型像指针一样支持 `*` 操作符，并启用 deref coercion：编译器自动将 `&T` 转换为 `&U`（如果 `T: Deref<Target=U>`），允许类型“继承”目标类型的方法。 这在智能指针中特别有用，例如 `Box<T>` 解引用到 `T`，让用户像使用 `T` 一样操作 `Box<T>`。 它促进抽象，提供零成本指针语义，而不牺牲安全。

根据官方文档，`Deref` 应仅用于廉价、透明的解引用操作，且不应失败。 它不提供默认方法，仅要求 `deref`。

- **为什么需要 `Deref`？** Rust 的所有权系统需要显式借用。`Deref` 简化智能指针的使用，支持方法解析（如在 `Box<Vec<i32>>` 上调用 `Vec` 方法），并避免 boilerplate 代码。 例如，在库设计中，实现 `Deref` 让包装类型透明。

### 1.2 与相关 Trait 的区别
`Deref` 与几个引用 trait 相关，但侧重解引用：

- **与 `DerefMut`**：
    - `Deref` 用于不可变解引用（`&Target`）；`DerefMut` 用于可变解引用（`&mut Target`）。
    - `DerefMut` 继承 `Deref`，用于可变上下文。
    - 选择：如果需要修改，用 `DerefMut`；否则 `Deref` 足够。

- **与 `AsRef`**：
    - `Deref` 支持 `*` 和 coercion；`AsRef` 是显式转换 trait，无 coercion。
    - `Deref` 隐式调用；`AsRef` 需手动 `as_ref()`。
    - 许多类型同时实现两者，但 `Deref` 更强大（方法继承）。
    - 选择：对于智能指针，用 `Deref`；对于通用转换，用 `AsRef`。

- **与 `Borrow`**：
    - `Deref` 不要求哈希等价；`Borrow` 要求借用与原值哈希/比较等价，用于集合键。
    - `Borrow` 更严格；`Deref` 更通用。
    - 示例：`String` 实现 `Borrow<str>` 以用于 `HashMap<&str>` 键。

**何时选择？** 实现 `Deref` 如果类型是智能指针或透明包装；否则用 `AsRef` 或 `Borrow` 以避免意外 coercion。 最佳实践：仅在解引用廉价且透明时实现。

## 2. 手动实现 `Deref`

`Deref` 不能自动派生，必须手动实现。但实现简单：定义 `Target` 和 `deref`。

### 2.1 基本示例：自定义智能指针
从官方示例：
```rust
use std::ops::Deref;

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

fn main() {
    let x = MyBox::new(5);
    assert_eq!(5, *x);  // 使用 *
}
```
- 这里，`MyBox` 像 `Box` 一样解引用到内部值。

### 2.2 Deref Coercion 示例
```rust
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);  // Coercion: &MyBox<String> -> &String -> &str
}
```
- 编译器自动插入 `deref` 调用。 这允许方法继承：`m.len()` 调用 `String::len()`。

### 2.3 新类型（Newtype）实现
```rust
struct NonNegative(i32);

impl Deref for NonNegative {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let num = NonNegative(42);
    println!("{}", *num);  // 42
}
```
- 但小心：对于新类型，实现 `Deref` 可能导致混淆，因为它允许绕过类型检查。 许多开发者认为这是坏实践，除非新类型是透明的。

### 2.4 泛型类型
```rust
struct Wrapper<T>(T);

impl<T> Deref for Wrapper<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let w = Wrapper(vec![1, 2, 3]);
    println!("{}", w.len());  // 3, 通过 coercion 调用 Vec::len
}
```
- 泛型 impl，支持任何 `T`。

## 3. Deref Coercion 详解

Deref coercion 是 `Deref` 的关键特性：编译器在类型不匹配时自动应用 `deref`。

- **规则**：如果 `T: Deref<Target=U>`，则 `&T` 可强制为 `&U`。 支持多级：`&MyBox<MyBox<T>>` -> `&T`。
- **应用**：函数参数、方法调用、字段访问。
- **限制**：仅在引用上下文中；不影响所有权。

示例：多级 coercion。
```rust
let inner = String::from("hello");
let outer = MyBox::new(MyBox::new(inner));
hello(&outer);  // &MyBox<MyBox<String>> -> &MyBox<String> -> &String -> &str
```
- 自动解包。

## 4. 高级主题

### 4.1 Deref Polymorphism（反模式）
使用 `Deref` 来模拟继承或多态，常被视为反模式。 示例：
```rust
struct Child {
    parent: Parent,
}

impl Deref for Child {
    type Target = Parent;
    fn deref(&self) -> &Parent { &self.parent }
}
```
- 这允许 `Child` “继承” `Parent` 方法，但可能导致方法冲突或意外行为。 替代：使用组合和显式委托。

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
```
- 支持动态分发。

### 4.3 与 DerefMut 结合
实现两者以支持可变/不可变解引用。 示例：标准库 `Box` 实现两者。

## 5. 常见用例

- **智能指针**：`Box`、`Rc` 等。
- **包装类型**：如 `Cow`、`RefCell` 的守卫。
- **方法继承**：让包装类型使用内部方法。
- **API 设计**：透明包装外部类型。
- **性能优化**：零成本抽象。

## 6. 最佳实践

- **仅透明时实现**：解引用应像访问内部一样。
- **避免失败**：`deref` 不应 panic 或失败。
- **与 DerefMut 配对**：如果适用。
- **文档**：说明 coercion 行为。
- **测试**：验证 `*` 和方法调用。
- **避免新类型 Deref**：使用显式方法以防混淆。

## 7. 常见陷阱和错误

- **方法冲突**：包装类型方法覆盖内部方法。
- **意外 Coercion**：导致类型推断问题。
- **别名问题**：多级解引用可能违反借用规则。
- **Deref Polymorphism**：模拟继承导致维护难题。
- **Unsized 类型**：需小心 `?Sized` 约束。
