# Rust 闭包教程

Rust 中的闭包（closures）是一种匿名函数，可以捕获其环境中的变量。闭包类似于其他语言中的 lambda 表达式，但 Rust 的闭包系统与所有权和借用紧密集成，确保内存安全。闭包可以作为函数参数、返回值，或存储在变量中，常用于迭代器、线程和回调。Rust 闭包实现了 Fn trait 家族（Fn、FnMut、FnOnce），根据捕获方式决定其行为。

本教程从基础开始，逐步深入，包含代码示例和解释。假设你已熟悉 Rust 的函数、所有权和借用（如 & 和 move）。每个示例后，我会解释关键点。如果你有 Rust 环境，可以复制代码运行测试。教程基于 Rust 1.80+（截至 2025 年，闭包核心未变，但有异步闭包的优化）。

## 1. 闭包简介

- **什么是闭包？**：闭包是可调用（callable）的匿名函数，能捕获周围作用域的变量。语法：`|params| expression` 或 `{ body }`。
- **优势**：简洁、捕获上下文（无需显式传递变量）、与迭代器/线程集成。
- **捕获方式**：
    - 不可变借用（&）：默认，读访问。
    - 可变借用（&mut）：修改捕获变量。
    - 所有权转移（move）：拥有变量。
- **Fn trait**：
    - `FnOnce`：调用一次，消耗闭包（可能移动捕获）。
    - `FnMut`：可多次调用，可修改捕获。
    - `Fn`：可多次调用，只读捕获。
- **自动推断**：Rust 根据使用推断 trait。

### 示例：基本闭包
```rust
fn main() {
    let add_one = |x: i32| x + 1;  // 简单闭包
    println!("结果: {}", add_one(5));  // 输出: 结果: 6
}
```

- **解释**：`|x: i32|` 是参数，`x + 1` 是体。类型可省略（推断）。闭包存储在变量中，像函数调用。

## 2. 捕获变量

闭包可以捕获外部变量。

### 示例：捕获借用
```rust
fn main() {
    let x = 4;
    let equal_to_x = |z| z == x;  // 借用 x (&x)

    println!("相等？{}", equal_to_x(4));  // 输出: 相等？true
    println!("x 仍有效: {}", x);  // x 未移动
}
```

- **解释**：闭包借用 x（&），所以 x 后仍可用。如果修改 x，需要 &mut。

### 示例：可变捕获
```rust
fn main() {
    let mut x = 4;
    let mut increment = || { x += 1; };  // &mut x

    increment();
    println!("x: {}", x);  // 输出: x: 5
}
```

- **解释**：闭包捕获 &mut x，因为修改它。闭包本身需 mut 如果多次调用。

## 3. Move 闭包

用 `move` 关键字转移所有权到闭包。

### 示例：Move 闭包
```rust
fn main() {
    let x = vec![1, 2, 3];
    let contains = move |z| x.contains(&z);  // 移动 x 到闭包

    println!("包含 2？{}", contains(2));  // 输出: 包含 2？true
    // println!("{:?}", x);  // 错误！x 已移动
}
```

- **解释**：`move` 强制转移所有权，常用于线程（std::thread::spawn 需要 'static 生命周期）。即使不需 move，如果捕获非 Copy 类型并消耗，编译器会要求。

## 4. 闭包作为参数和返回值

闭包可传给函数，使用 trait bound。

### 示例：闭包参数
```rust
fn apply<F>(f: F, x: i32) -> i32
where
    F: FnOnce(i32) -> i32,  // bound FnOnce
{
    f(x)
}

fn main() {
    let double = |n| n * 2;
    println!("结果: {}", apply(double, 5));  // 输出: 结果: 10
}
```

- **解释**：用 FnOnce（最宽松），因为闭包可能消耗。FnMut 或 Fn 更严格。where 子句提高可读性。

### 示例：返回闭包
```rust
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn main() {
    let closure = returns_closure();
    println!("{}", closure(5));  // 6
}
```

- **解释**：`impl Fn` 表示返回实现了 Fn 的类型。不暴露具体闭包类型。

## 5. 闭包与迭代器

闭包常用于 map、filter 等。

### 示例：迭代器闭包
```rust
fn main() {
    let v = vec![1, 2, 3];
    let doubled: Vec<_> = v.iter().map(|&x| x * 2).collect();
    println!("{:?}", doubled);  // [2, 4, 6]
}
```

- **解释**：`|&x| x * 2` 借用元素。iter() 借用，into_iter() 消耗。

## 6. 高级主题：Cacher 和 生命周期

- **Cacher 示例**：用闭包实现简单缓存。
  ```rust
  use std::collections::HashMap;

  struct Cacher<T> where T: Fn(u32) -> u32 {
      calculation: T,
      value: HashMap<u32, u32>,
  }

  impl<T> Cacher<T> where T: Fn(u32) -> u32 {
      fn new(calculation: T) -> Cacher<T> {
          Cacher { calculation, value: HashMap::new() }
      }

      fn value(&mut self, arg: u32) -> u32 {
          match self.value.get(&arg) {
              Some(&v) => v,
              None => {
                  let v = (self.calculation)(arg);
                  self.value.insert(arg, v);
                  v
              }
          }
      }
  }
  ```
- **解释**：泛型 T bound Fn。存储闭包并调用。

- **生命周期**：闭包捕获引用时，需确保生命周期匹配（如 'a）。

## 7. 最佳实践和常见陷阱

- **选择正确 Fn trait**：从 FnOnce 开始，如果需多次调用，用 FnMut 或 Fn。
- **避免不必要 move**：让编译器推断，除非跨线程。
- **闭包大小**：闭包有大小（捕获变量决定），用 Box<Fn()> 如果需动态大小。
- **常见错误**：
    - 借用冲突：闭包捕获 &mut 时，确保无其他借用。
    - 生命周期不足：返回捕获引用的闭包需 'a（如 impl Fn(&'a str) -> &'a str）。
    - 非 'static 线程：spawn 要求 move 和 'static（无外部引用）。
    - 类型推断失败：显式注解参数类型。
- **性能**：闭包零开销，编译为函数。
- **异步闭包**：在 async 块中使用，需 async move。

## 练习建议
1. 编写闭包，捕获变量并在线程中使用（用 move）。
2. 创建返回闭包的函数，实现计数器。
3. 用闭包过滤 Vec，只保留偶数。
