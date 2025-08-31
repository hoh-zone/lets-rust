# Trait TryInto

`TryInto` trait 来自 `std::convert` 模块，它的主要目的是定义如何将一个类型的值尝试转换为另一个类型的值，同时消耗输入值，并处理可能的失败。它是 `TryFrom` trait 的互补，通常用于泛型上下文中的可能失败转换，尤其在不需要指定目标类型时非常有用。与 `Into` 不同，`TryInto` 返回一个 `Result`，允许处理转换错误。

## 1. `TryInto` Trait 简介

### 1.1 定义和目的
`TryInto` trait 定义在 `std::convert::TryInto` 中，自 Rust 1.34.0 起稳定可用。其语法如下：
```rust
pub trait TryInto<T>: Sized {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}
```
- **目的**：提供一种可能失败的值到值转换机制。它允许类型定义如何尝试转换为其他类型，提供一个“尝试转换”方法，通常用于可能丢失数据或无效输入的场景。这在泛型编程中特别有用，可以让函数接受多种输入类型，并尝试转换为目标类型。

根据官方文档，`TryInto` 应仅用于可能失败的转换；如果转换总是成功，使用 `Into`。它特别有用在 API 设计中：允许用户以多种方式提供输入，并处理潜在失败，而无需 panic。

- **为什么需要 `TryInto`？** Rust 强调安全和显式错误处理。`TryInto` 使转换标准化，支持边界约束，并简化代码，如在函数参数中使用 `T: TryInto<U>` 以接受可尝试转换为 `U` 的类型，并处理错误。

### 1.2 与相关 Trait 的区别
`TryInto` 与几个转换 trait 相关，但各有侧重：

- **与 `Into`**：
    - `TryInto<T> for U` 用于可能失败的转换，返回 `Result<T, Error>`；`Into<T> for U` 用于总是成功的转换。
    - 如果转换无损且无失败，使用 `Into`；否则用 `TryInto` 以避免 panic。
    - 示例：`let s: String = "hello".into()` 总是成功；`let i: i32 = i64::MAX.try_into()?` 可能失败。

- **与 `TryFrom`**：
    - `TryInto<T> for U` 意味着从 `U` 尝试转换为 `T`；`TryFrom<U> for T` 是其互补。
    - 实现 `TryFrom` 自动提供 `TryInto` 的实现（通过 blanket impl）。
    - 优先实现 `TryFrom`，因为它更直接；用 `TryInto` 在泛型边界中以更宽松。
    - 示例：`t.try_into()` 等价于 `T::try_from(t)`，但前者更适合链式调用。

- **与 `FromStr`**：
    - `FromStr` 专用于从 `&str` 解析，类似于 `TryInto<Self> for &str` 但更特定。
    - `FromStr` 先于 `TryInto` 存在，更适合字符串解析（如 `str::parse`）；`TryInto` 更通用。
    - 选择：对于字符串输入，优先 `FromStr` 以兼容标准方法；否则用 `TryInto`。

**何时选择？** 在泛型函数边界中使用 `TryInto` 以更宽松接受输入；对于类型定义，优先 `TryFrom`。最佳实践：仅用于有潜在失败的转换，避免在 `try_into` 中 panic。

## 2. 手动实现 `TryInto`

`TryInto` 不能自动派生，必须手动实现。但由于 `TryFrom` 的 blanket impl，通常无需直接实现 `TryInto`。

### 2.1 基本示例：结构体
```rust
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryInto<i32> for EvenNumber {
    type Error = ();

    fn try_into(self) -> Result<i32, Self::Error> {
        if self.0 % 2 == 0 {
            Ok(self.0)
        } else {
            Err(())
        }
    }
}

fn main() {
    let even = EvenNumber(8);
    let num: Result<i32, ()> = even.try_into();
    assert_eq!(num, Ok(8));

    let odd = EvenNumber(5);
    assert_eq!(odd.try_into(), Err(()));
}
```
- 从 `EvenNumber` 尝试转换为 `i32`，仅偶数成功。

### 2.2 通过 `TryFrom` 间接实现
优先这样：
```rust
impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

// 现在可使用 try_into()
fn main() {
    let num: i32 = 8;
    let even: Result<EvenNumber, ()> = num.try_into();
    assert_eq!(even, Ok(EvenNumber(8)));
}
```
- 自动获益于 blanket impl。

### 2.3 泛型类型
```rust
#[derive(Debug)]
struct Bounded<T: PartialOrd + Copy>(T); // value <= max

impl<T: PartialOrd + Copy> TryInto<T> for Bounded<T> {
    type Error = &'static str;

    fn try_into(self) -> Result<T, Self::Error> {
        let max = T::default(); // 假设默认是 max，这里简化
        if self.0 <= max {
            Ok(self.0)
        } else {
            Err("Value exceeds bound")
        }
    }
}
```
- 委托转换，检查边界。

### 2.4 在错误处理中
```rust
use std::num::TryFromIntError;

fn process_large(num: i64) -> Result<i32, TryFromIntError> {
    num.try_into()
}
```
- 标准库数值转换使用 `TryInto` 处理溢出。

## 3. 与 `TryFrom` 的关系

实现 `TryFrom<U> for T` 自动提供 `TryInto<T> for U`：
- 这使 API 更灵活，用户可选择 `.try_into()` 或 `T::try_from(u)`。

## 4. 高级主题

### 4.1 Blanket Implementations
标准库提供：`impl<T, U> TryInto<U> for T where U: TryFrom<T>`；自反 impl `TryInto<T> for T` 总是成功，Error 为 `Infallible`。

自定义 blanket：
- 小心孤儿规则，避免冲突。

### 4.2 与 `FromStr` 结合
对于字符串解析：
- 实现 `FromStr` 后，可用 `s.parse::<T>()`，内部类似 `TryInto`。

### 4.3 第三方类型
用新类型包装外部类型实现 `TryInto`。

## 5. 常见用例

- **泛型函数**：`fn foo<T: TryInto<i32>>(n: T) -> Result<i32, T::Error>` 接受多种数值，尝试转换。
- **数值转换**：处理溢出。
- **解析输入**：从原始到自定义类型。
- **API 设计**：提供灵活输入，处理失败。
- **数组/切片**：长度检查转换。

## 6. 最佳实践

- **优先 `TryFrom`**：自动获 `TryInto`。
- **自定义 Error**：提供有意义错误。
- **边界用 `TryInto`**：更宽松。
- **文档**：说明失败条件。
- **测试**：覆盖成功/失败。
- **避免 panic**：始终返回 Err。

## 7. 常见陷阱和错误

- **失败时 panic**：违反约定；用 Err。
- **孤儿规则**：不能为外部实现。
- **方向混淆**：`TryInto<T> for U` 是从 `U` 到 `T`。
- **与 TryFrom 冲突**：优先 TryFrom。
- **性能**：检查开销在热路径评估。
