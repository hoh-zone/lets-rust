# Rust 泛型教程

Rust 的泛型（generics）允许你编写抽象、可重用的代码，而不牺牲性能。它类似于其他语言的模板或泛型，但 Rust 的泛型是零成本抽象：在编译时单态化（monomorphization），生成具体类型的代码。这确保了类型安全，同时避免运行时开销。泛型常用于函数、结构体、枚举和 trait 中，帮助创建如 Vec<T> 或 HashMap<K, V> 这样的标准库类型。

## 1. 泛型简介

- **什么是泛型？**：使用类型参数（如 <T>）定义代码，允许在不同类型上重用。T 是占位符，在使用时替换为具体类型。
- **优势**：代码复用、类型安全、性能高（编译时展开）。
- **语法**：在函数、struct 等后用 <参数>，如 fn foo<T>(arg: T)。
- **与 trait 的关系**：泛型常结合 trait bound（如 T: Clone）限制类型。

### 示例：简单泛型函数
```rust
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {  // 错误！T 可能不支持 >
            largest = item;
        }
    }
    largest
}
```

- **解释**：这个会编译错误，因为 T 未指定支持比较。需要 trait bound（见下文）。

## 2. 泛型函数

函数可以有泛型参数。

### 示例：泛型函数
```rust
fn print_value<T>(value: T) {
    println!("值: {:?}", value);  // 错误！T 需实现 Debug
}

fn main() {
    print_value(5);       // T = i32
    print_value("hello"); // T = &str
}
```

- **解释**：编译错误，因为 println! 需要 Debug。添加 bound 修复（见第 4 节）。

## 3. 泛型结构体和枚举

结构体和枚举可以泛型化。

### 示例：泛型结构体
```rust
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    
    println!("整数点: {:?}", integer_point);
    println!("浮点: {:?}", float_point.x());
}
```

- **解释**：Point<T> 为 T 生成具体类型。impl<T> 为所有 T 实现方法。多参数如 <T, U> 允许不同类型，如 Point { x: 5, y: 3.14 }。

### 示例：泛型枚举
```rust
enum Option<T> {  // 标准库中的简化版
    Some(T),
    None,
}

fn main() {
    let some_number = Option::Some(5);
    let absent: Option<i32> = Option::None;
}
```

- **解释**：枚举变体持泛型值。标准库 Option<T> 和 Result<T, E> 是泛型枚举。

## 4. Trait Bound

Bound 限制泛型参数必须实现某些 trait。

- **语法**：fn foo<T: Trait1 + Trait2>(arg: T)
- **常见 bound**：Copy、Clone、Debug、PartialEq、PartialOrd 等。

### 示例：带 bound 的函数
```rust
use std::fmt::Debug;

fn print_value<T: Debug>(value: T) {
    println!("值: {:?}", value);
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("最大: {}", largest(&numbers));  // 输出: 最大: 100
}
```

- **解释**：T: PartialOrd 确保 > 操作符可用。多 bound 用 +，如 T: Debug + Clone。

## 5. Where 子句

对于复杂 bound，用 where 子句提高可读性。

### 示例：Where 子句
```rust
fn some_function<T, U>(t: T, u: U) -> U
where
    T: Debug + Clone,
    U: Clone + PartialEq,
{
    if t.clone() == u {  // 错误！T 和 U 类型不同，不能比较
        // ...
    }
    u
}
```

- **解释**：where 在签名后。适用于函数、impl、trait。

## 6. 泛型 impl 和 trait

impl 可以泛型，trait 可以定义泛型方法。

### 示例：泛型 impl
```rust
impl<T: Debug> Point<T> {  // 只为 Debug 类型实现
    fn debug_print(&self) {
        println!("{:?}", self);
    }
}
```

- **解释**：bound 限制 impl 范围。

### 示例：泛型 trait
```rust
trait Summary<T> {
    fn summarize(&self) -> T;
}
```

- **解释**：trait 本身可以泛型，但常见是方法内用泛型。

## 7. 高级主题：关联类型和生命周期

- **关联类型**：在 trait 中定义类型占位符，避免过多泛型。
  ```rust
  trait Iterator {
      type Item;  // 关联类型
      fn next(&mut self) -> Option<Self::Item>;
  }
  ```
- **泛型与生命周期**：结合 'a，如 fn longest<'a, T>(x: &'a T, y: &'a T) -> &'a T。
- **性能**：单态化生成具体代码，可能增加二进制大小，但运行时零开销。

## 8. 最佳实践和常见陷阱

- **使用 bound 最小化**：只添加必要 trait，避免过度限制。
- **优先具体类型**：泛型用于真正需要重用时。
- ** turbofish 语法**：指定类型如 Vec::<i32>::new()，当推断失败时用。
- **常见错误**：
    - 未 bound：操作如 + 时错误（添加 T: Add）。
    - 类型不匹配：如混合 T 和 U 时，确保兼容。
    - 过度泛型：导致代码复杂，考虑 trait 对象（dyn Trait）用于运行时多态（有开销）。
    - 编译时间长：过多泛型展开，优化 bound 或用 Box<dyn Trait>。
- **标准库示例**：Vec<T>、HashMap<K, V> – 研究它们的 impl。

## 练习建议
1. 编写泛型函数，接收 &[T] 并返回反转切片（需 T: Clone）。
2. 创建泛型 struct Pair<T>，实现方法如 swap()（需 T: Copy）。
3. 定义 trait Printable<T>，为不同类型实现，并用泛型函数调用。
