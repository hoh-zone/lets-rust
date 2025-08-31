# Trait Hash


`Hash` trait 来自 `std::hash` 模块，它的主要目的是为类型定义一种计算哈希值的方式，使得类型可以被哈希（hashed）以用于如 `HashMap` 或 `HashSet` 的键。它要求类型实现 `hash` 方法，将值馈送到一个 `Hasher` 中，以生成一个代表值的整数哈希。 与 `Eq` 结合，`Hash` 确保如果两个值相等，则它们的哈希值也相等，从而避免哈希冲突。 `Hash` 是 `Hasher` trait 的伴侣，用于计算哈希，而 `Hasher` 是状态机，累积哈希数据。

`Hash` 的设计目的是提供一种通用的哈希机制，支持标准库的哈希表实现，并允许自定义类型轻松集成。 它促进一致的哈希语义，支持泛型代码中的键存储，而无需担心哈希冲突或不一致。

- **为什么需要 `Hash`？** Rust 的集合如 `HashMap` 需要高效查找，哈希是关键。`Hash` 允许类型定义自定义哈希计算，确保相等值有相同哈希，支持 DoS 抵抗（如 SipHasher）。 例如，在自定义结构体作为键时，实现 `Hash` 以用于 `HashMap`。

### 1.2 与相关 Trait 的区别
`Hash` 与几个比较和哈希 trait 相关，但侧重哈希计算：

- **与 `Eq`**：
    - `Hash`：哈希计算；`Eq`：总等价。
    - `Hash` 与 `Eq` 结合使用：a == b 必须隐含 hash(a) == hash(b)。
    - `Hash` 无继承 `Eq`，但集合要求 `Eq + Hash` 以一致。
    - 示例：自定义类型实现 `Eq + Hash` 以用作键；违反一致是逻辑错误。
    - 区别：`Hash` 是计算；`Eq` 是比较。

- **与 `PartialEq`**：
    - `Hash`：哈希；`PartialEq`：部分等价。
    - 类似 `Eq`，但 `PartialEq` 允许部分相等（如 NaN），哈希需小心一致。
    - 示例：浮点实现 `PartialEq` 但哈希需处理 NaN。

- **与 `Hasher`**：
    - `Hash`：类型计算哈希；`Hasher`：状态机累积哈希。
    - `Hash` 使用 `Hasher` 的 `write` 方法馈送数据。
    - 示例：`hash` 方法接受 `&mut H: Hasher`。

**何时选择？** 用 `Hash` 当类型需用作哈希键时，与 `Eq` 结合；用 `PartialEq` 单独用于相等。 最佳实践：实现 `Hash` 时，确保与 `Eq` 一致，避免逻辑错误。

## 2. 自动派生 `Hash`（Deriving Hash）

Rust 允许使用 `#[derive(Hash)]` 为结构体、枚举和联合体自动实现 `Hash`，前提是所有字段都实现了 `Hash`。这是最简单的方式，尤其适用于简单类型。

### 2.1 基本示例：结构体
```rust
#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    let p = Point { x: 1, y: 2 };
    let mut hasher = DefaultHasher::new();
    p.hash(&mut hasher);
    println!("Hash: {}", hasher.finish());
}
```
- 派生哈希所有字段。

### 2.2 枚举
```rust
#[derive(Hash, Eq, PartialEq, Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

fn main() {
    let s = Shape::Circle { radius: 5.0 };
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    println!("Hash: {}", hasher.finish());
}
```
- 派生哈希变体和字段。

### 2.3 泛型类型
```rust
#[derive(Hash, Eq, PartialEq, Debug)]
struct Pair<T: Hash + Eq> {
    first: T,
    second: T,
}

fn main() {
    let pair = Pair { first: 1, second: 2 };
    let mut hasher = DefaultHasher::new();
    pair.hash(&mut hasher);
    println!("Hash: {}", hasher.finish());
}
```
- 约束 `T: Hash + Eq` 以派生。

**注意**：派生要求所有字段 `Hash`；浮点字段可派生，但 NaN 哈希需一致（Rust 使用特定 NaN 表示）。

## 3. 手动实现 `Hash`

当需要自定义哈希逻辑时，必须手动实现 `Hash`。

### 3.1 基本手动实现
```rust
use std::hash::{Hash, Hasher};

struct Person {
    id: u32,
    name: String,
    phone: u64,
}

impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.phone.hash(state);
        // 忽略 name，如果不影响 Eq
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.phone == other.phone
    }
}

impl Eq for Person {}

fn main() {
    let p1 = Person { id: 1, name: "Alice".to_string(), phone: 123 };
    let p2 = Person { id: 1, name: "Bob".to_string(), phone: 123 };
    assert_eq!(p1, p2);  // true
    let mut hasher1 = DefaultHasher::new();
    p1.hash(&mut hasher1);
    let mut hasher2 = DefaultHasher::new();
    p2.hash(&mut hasher2);
    assert_eq!(hasher1.finish(), hasher2.finish());  // 相同哈希
}
```
- 手动哈希选定字段，确保与 Eq 一致。

### 3.2 避免前缀冲突
```rust
impl Hash for &str {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_bytes().hash(state);
        0xffu8.hash(state);  // 添加后缀避免前缀冲突
    }
}
```
- 如标准实现，避免 ("ab", "c") 和 ("a", "bc") 冲突。

### 3.3 泛型类型
```rust
struct Wrapper<T> {
    inner: T,
}

impl<T: Hash> Hash for Wrapper<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}
```
- 委托给内部类型。

## 4. 高级主题

### 4.1 与 Eq 一致性
实现 `Hash + Eq` 以用作键：
- 违反一致是逻辑错误，可能导致 UB 在 unsafe 代码中。

### 4.2 前缀冲突避免
始终添加区分符，如 0xFF 对于字符串。

### 4.3 可移植性
哈希不跨平台/版本稳定；勿依赖具体哈希值。

### 4.4 第三方 Crate：derivative
使用 `derivative` 自定义派生，忽略字段。

## 5. 常见用例

- **哈希键**：HashMap 要求 Hash + Eq。
- **自定义哈希**：忽略字段或自定义逻辑。
- **DoS 抵抗**：标准 Hasher 如 SipHasher。
- **测试**：验证哈希一致于 Eq。
- **泛型边界**：T: Hash 以计算哈希。

## 6. 最佳实践

- **优先派生**：用 `#[derive(Hash)]` 简化。
- **与 Eq 一致**：确保相等值相同哈希。
- **避免冲突**：添加后缀避免前缀冲突。
- **文档**：说明哈希语义。
- **测试**：验证哈希与 Eq 一致。
- **忽略字段**：自定义如果不影响 Eq。

## 7. 常见陷阱和错误

- **不一致 Hash/Eq**：导致集合错误；总是匹配。
- **前缀冲突**：忽略导致碰撞；添加区分符。
- **依赖哈希值**：不稳定跨版本；勿硬编码。
- **泛型无边界**：默认无 Hash；添加边界。
- **循环递归**：hash 导致无限循环；用 raw 字段。