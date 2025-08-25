# Trait Sized

`Sized` trait 来自 `std::marker` 模块，它是一个标记 trait（marker trait），表示类型的尺寸在编译时已知。它允许编译器在泛型和 trait 对象中处理大小未知的类型（如切片或 trait 对象），并通过边界约束要求类型必须有固定大小。与 `?Sized` 不同，`Sized` 是默认的，而 `?Sized` 放松了大小要求，常用于 trait 对象或切片。

## 1. `Sized` Trait 简介

### 1.1 定义和目的
`Sized` trait 定义在 `std::marker::Sized` 中，自 Rust 1.0.0 起稳定可用。其语法如下：
```rust
pub trait Sized { }
```
- **关键点**：
    - 无方法：作为标记 trait，仅标记类型在编译时有固定大小（known size at compile time）。
    - 自动实现：编译器为所有固定大小的类型自动实现 `Sized`，如原始类型、固定大小数组、没有 unsized 字段的结构体。
    - **目的**：`Sized` 确保类型可以用于需要知道大小的操作，如栈分配、泛型 monomorphization 或作为 trait 对象时的 boxed。 根据官方文档，`Sized` 是泛型默认边界：`fn foo<T>(t: T)` 隐含 `T: Sized`，因为函数参数需固定大小。 它促进编译时优化，提供对 unsized 类型的支持，通过 `?Sized` 边界处理动态大小类型（如 `[T]`、`str`、`dyn Trait`）。

`Sized` 的设计目的是区分 sized 和 unsized 类型：在 Rust 中，大多数类型是 sized 的，但如切片（`[T]`）是 unsized（fat pointer）。`Sized` 允许编译器静态分配内存，并防止 unsized 类型用于不兼容上下文。

- **为什么需要 `Sized`？** Rust 强调编译时安全和零开销抽象。`Sized` 允许泛型代码处理大小未知类型（如 trait 对象），并通过边界约束避免运行时错误。 例如，在 trait 中，使用 `Self: ?Sized` 以支持 trait 对象。

### 1.2 与相关 Trait 的区别
`Sized` 与几个 marker trait 相关，但专注于编译时大小：

- **与 `Unpin`**：
    - `Sized`：类型大小已知；`Unpin`：类型可以安全移动，即使 pinned。
    - `Unpin` 与 Pinning 相关（futures）；`Sized` 与分配相关。
    - 示例：大多数 sized 类型自动 `Unpin`；unsized 类型如 `dyn Future` 需 Pin。
    - 区别：`Sized` 是默认；`Unpin` 是 opt-out。

- **与 `Send` 和 `Sync`**：
    - `Sized` 与大小相关；`Send`/`Sync` 与线程安全相关。
    - Unsized 类型如 `dyn Trait + Send` 可以是 trait 对象。
    - 示例：`Box<dyn Send>` 是 sized，但内部 unsized。
    - 结合：泛型边界如 `T: ?Sized + Send` 支持 unsized 发送。

- **与 `Clone`**：
    - `Sized` 是 marker；`Clone` 是行为 trait。
    - `Clone` 继承 `Sized`（因为克隆需大小已知）。
    - 示例：unsized 类型如 `[T]` 不能 `Clone`，因为无大小。

**何时选择？** 用 `Sized` 边界要求固定大小（如栈分配）；用 `?Sized` 放松以支持 unsized 类型（如 trait 对象）。 最佳实践：默认使用 `Sized`，仅在需要 unsized 时用 `?Sized`。

## 2. 自动实现 `Sized`（Auto-Implemented）

Rust 编译器自动为符合条件的类型实现 `Sized`：如果类型及其所有字段有固定大小，则自动 `Sized`。无需手动实现或派生。

### 2.1 基本示例：Sized 类型
```rust
struct Point {
    x: i32,  // fixed size
    y: i32,
}

fn take_sized<T: Sized>(t: T) {
    // T 必须 sized
}

fn main() {
    let p = Point { x: 1, y: 2 };
    take_sized(p);  // OK, Point: Sized
}
```
- `i32` 是 sized，结构体继承。

### 2.2 Unsized 类型示例
```rust
fn take_unsized<T: ?Sized>(t: &T) {
    // &T 是 sized (thin pointer)，但 T 可 unsized
}

fn main() {
    let slice: &[i32] = &[1, 2, 3];  // [i32] unsized
    take_unsized(slice);  // OK with ?Sized
    // take_sized(slice);  // 错误：[i32] 非 Sized
}
```
- Unsized 类型需引用或 Box。

### 2.3 泛型边界
```rust
fn process<T: ?Sized + std::fmt::Debug>(t: &T) {
    println!("{:?}", t);
}

fn main() {
    let s: &str = "hello";  // str unsized
    process(s);  // OK
}
```
- `?Sized` 允许 unsized 参数（通过引用）。

## 3. 手动实现 `Sized`（不推荐）

`Sized` 是 auto trait，不能手动实现。它由编译器决定；尝试手动会导致错误。

### 3.1 Unsized 自定义类型
使用 `CoerceUnsized` trait 自定义 unsized 类型转换，但罕见。

## 4. 高级主题

### 4.1 Trait 对象和 ?Sized
Trait 对象是 unsized：
```rust
trait MyTrait {}

fn use_trait_object(t: &dyn MyTrait) {
    // dyn MyTrait unsized
}

impl MyTrait for i32 {}

fn main() {
    let i: i32 = 42;
    use_trait_object(&i as &dyn MyTrait);  // OK
}
```
- Trait 方法需 `Self: ?Sized` 以支持对象。

### 4.2 与 Box 和 Pointers
`Box<T>` 是 sized，即使 `T` unsized：
```rust
let boxed: Box<[i32]> = Box::new([1, 2, 3]);  // [i32] unsized，但 Box sized
```
- Fat pointer 存储大小/元数据。

### 4.3 自定义 Unsized 类型
使用 struct with unsized field：
```rust
struct MyUnsized {
    data: [i32],  // last field unsized
}

fn main() {
    let m: &MyUnsized = &MyUnsized { data: [1, 2, 3] };
    // 直接 MyUnsized 不能实例化，因为 unsized
}
```
- 最后字段 unsized，使结构体 unsized。

## 5. 常见用例

- **泛型函数**：默认 `Sized` 以栈分配。
- **Trait 对象**：用 `?Sized` 支持 dyn Trait。
- **切片处理**：`&[T]` 函数用 `?Sized`。
- **性能优化**：Sized 类型高效分配。
- **库设计**：放松边界以支持 unsized。

## 6. 最佳实践

- **默认 Sized**：泛型保持默认以优化。
- **用 ?Sized**：仅在需要 unsized 时。
- **引用包装**：unsized 类型用 & 或 Box。
- **文档**：说明边界原因。
- **测试**：编译检查 unsized 用法。
- **避免手动**：依赖自动实现。

## 7. 常见陷阱和错误

- **Unsized 参数**：直接 T: unsized 错误；用 &T 或 Box<T>。
- **Trait 默认 Sized**：trait 方法 Self Sized；加 ?Sized 放松。
- **泛型意外 Sized**：隐含边界导致错误；显式 ?Sized。
- **Copy/Clone 要求 Sized**：unsized 不能 Copy/Clone。
- **性能**：unsized 需 heap 分配。