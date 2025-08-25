# Trait Default

`Default` trait 来自 `std::default` 模块，它的主要目的是为类型提供一个有用的默认值。它允许你使用 `Default::default()` 来获取类型的默认实例，通常用于初始化结构体、集合或泛型参数。 与其他初始化方式不同，`Default` 强调一个“合理”的默认值，而非零初始化。

## 1. `Default` Trait 简介

### 1.1 定义和目的
`Default` trait 定义在 `std::default::Default` 中，自 Rust 1.0.0 起稳定可用。其语法如下：
```rust
pub trait Default: Sized {
    fn default() -> Self;
}
```
- **目的**：为类型定义一个默认值工厂方法。它允许类型实现 `default()`，返回一个合理的初始实例。这在标准库中广泛用于如 `Vec::new()`（内部用 `Default`）或泛型中提供默认值。 根据官方文档，Rust 为许多基本类型实现 `Default`，如数值（0）、布尔（false）、Option（None）、Vec（空向量）等。

`Default` 的设计目的是提供一个“零成本”默认初始化，尤其在泛型中：函数可以接受 `T: Default` 以使用 `T::default()` 而无需知道具体类型。 它也支持派生（derive），使自定义类型轻松获得默认值。

- **为什么需要 `Default`？** 在 Rust 中，许多 API（如 `HashMap::new()`）使用 `Default` 来初始化。实现 `Default` 使你的类型与生态集成更好，支持泛型和便利初始化。 例如，在构建器模式中，使用 `Default` 作为起始点。

### 1.2 与相关 Trait 的区别
`Default` 与其他 trait 相关，但专注于默认值：

- **与 `Clone`**：
    - `Default` 创建新实例（从无到有）；`Clone` 复制现有实例。
    - `Default` 不需现有值；`Clone` 需要。
    - 示例：`let v: Vec<i32> = Default::default();`（空）；`let copy = v.clone();`（复制）。
    - 选择：用 `Default` 初始化；用 `Clone` 复制。

- **与 `Copy`**：
    - `Default` 是 trait 方法；`Copy` 是标记 trait，确保类型可按位复制。
    - 许多 `Copy` 类型也实现 `Default`（如 primitives），但非必需。
    - 区别：`Copy` 影响语义（move vs copy）；`Default` 仅提供值。

- **与 `new()` 构造函数**：
    - `Default` 是 trait，泛型友好；`new()` 是自定义方法。
    - `Default` 支持派生；`new()` 手动实现。
    - 最佳：为类型提供 `new()` 调用 `Default::default()`。

**何时选择？** 用 `Default` 在泛型或需要默认初始化的场景；对于自定义初始化，用构造函数。

## 2. 自动派生 `Default`（Deriving Default）

Rust 允许使用 `#[derive(Default)]` 为结构体、枚举和联合体自动实现 `Default`，前提是所有字段/变体都实现了 `Default`。这是最简单的方式，尤其适用于简单类型。

### 2.1 基本示例：结构体
```rust
#[derive(Default, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p: Point = Default::default();
    println!("{:?}", p);  // Point { x: 0, y: 0 }
}
```
- 字段使用各自默认值（i32: 0）。

### 2.2 枚举
```rust
#[derive(Default, Debug)]
enum Status {
    #[default]
    Idle,
    Active,
    Error,
}

fn main() {
    let s: Status = Default::default();
    println!("{:?}", s);  // Idle
}
```
- 使用 `#[default]` 指定默认变体（Rust 1.62+）。 无 `#[default]` 时，第一个变体为默认。

### 2.3 泛型类型
```rust
#[derive(Default, Debug)]
struct Container<T: Default> {
    item: T,
}

fn main() {
    let c: Container<i32> = Default::default();
    println!("{:?}", c);  // Container { item: 0 }
}
```
- 约束 `T: Default` 以派生。

**注意**：派生要求所有字段实现 `Default`；否则编译错误。

## 3. 手动实现 `Default`

当需要自定义默认值时，必须手动实现 `Default`。

### 3.1 基本手动实现
```rust
use std::default::Default;

struct Config {
    port: u16,
    debug: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config { port: 8080, debug: false }
    }
}

fn main() {
    let cfg = Config::default();
    assert_eq!(cfg.port, 8080);
}
```
- 自定义默认值。

### 3.2 部分默认（使用宏）
使用第三方 crate 如 `smart_default` 为复杂结构体提供部分默认。
```rust
use smart_default::SmartDefault;

#[derive(SmartDefault, Debug)]
struct Settings {
    #[default = 80]
    port: u16,
    #[default(_code = "String::from(\"prod\")")]
    env: String,
}
```
- 允许字段级自定义默认。

### 3.3 对于 Trait 对象
`Default` 要求 `Sized`，不能直接为 dyn Trait 实现。但可为 Box<dyn Trait> 实现。

## 4. 美化打印（不直接相关，但结合使用）

`Default` 常与 `Debug` 结合打印默认值，使用 `{:#?}` 美化。

## 5. 高级主题

### 5.1 泛型和约束
在泛型中添加 `T: Default`：
```rust
fn create<T: Default>() -> T {
    T::default()
}
```
- 支持泛型默认初始化。

### 5.2 第三方类型实现 `Default`
你可以为外部类型实现 `Default`，但需遵守孤儿规则（用新类型包装）。

### 5.3 与其他 Trait 结合
- 与 `Builder`：默认作为构建器起点。
- 与 `serde`：默认值在反序列化中使用。

## 6. 常见用例

- **初始化集合**：`let mut map: HashMap<K, V> = Default::default();`。
- **泛型函数**：提供默认参数。
- **配置结构体**：默认设置。
- **测试**：默认实例作为 baseline。
- **CLI 工具**：默认选项。

## 7. 最佳实践

- **优先派生**：为简单类型用 `#[derive(Default)]`。
- **自定义时合理**：默认值应“安全”且有意义。
- **结合 new()**：`impl MyType { fn new() -> Self { Self::default() } }`。
- **泛型边界**：用 `T: Default` 简化 API。
- **文档**：说明默认值语义。
- **使用宏**：如 `smart_default` 处理复杂默认。

## 8. 常见陷阱和错误

- **无 Default 字段**：派生失败；手动实现或添加约束。
- **Sized 要求**：不能为 unsized 类型（如 [T]）实现。
- **默认不安全**：如默认端口暴露风险；文档警告。
- **与 Clone 混淆**：默认不是复制。
- **枚举无 #[default]**：编译错误（新版）。
