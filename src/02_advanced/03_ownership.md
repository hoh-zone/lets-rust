# Rust 所有权

Rust 的所有权（ownership）系统是其核心特性之一，它确保了内存安全、线程安全和无垃圾回收的性能。通过所有权，Rust 在编译时防止数据竞争、悬垂引用和内存泄漏等问题，而无需运行时开销。这使得 Rust 成为系统编程的强大工具，同时避免了 C++ 中的常见错误。

## 1. 所有权简介

- **什么是所有权？**：每个值都有一个“所有者”（owner），负责在值超出作用域时释放它。Rust 使用所有权来管理堆内存，而不依赖垃圾回收器。
- **三大规则**：
    1. 每个值都有一个所有者变量。
    2. 值在任一时刻只有一个所有者。
    3. 当所有者超出作用域时，值会被丢弃（drop）。
- **栈 vs 堆**：栈数据（如 i32）固定大小，堆数据（如 String）动态大小。所有权主要管理堆数据。
- **为什么重要？**：防止双重释放（double free）、使用后释放（use after free）和数据竞争。

### 示例：基本所有权
```rust
fn main() {
    let s = String::from("hello");  // s 是 "hello" 的所有者
    // 这里可以使用 s
    println!("{}", s);
}  // s 超出作用域，String 被 drop，内存释放
```

- **解释**：String 是堆分配的，当 main 结束时，Rust 调用 `drop` 方法释放内存。固定大小类型（如 i32）直接在栈上，不涉及所有权复杂性。

## 2. 移动（Move）

当你将值赋给另一个变量时，所有权会“移动”，原变量失效。这防止了多个所有者。

### 示例：移动所有权
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // 所有权从 s1 移动到 s2，s1 失效

    // println!("{}", s1);  // 错误！s1 已失效
    println!("{}", s2);  // 有效
}
```

- **解释**：移动后，s1 不能再用（编译错误：use of moved value）。这适用于堆数据；栈数据（如 i32）会复制。
- **函数中的移动**：
  ```rust
  fn takes_ownership(s: String) {  // s 获得所有权
      println!("{}", s);
  }  // s 超出作用域，被 drop

  fn main() {
      let s = String::from("hello");
      takes_ownership(s);  // 所有权移动到函数
      // println!("{}", s);  // 错误！s 已移动
  }
  ```

## 3. 复制（Copy）

某些类型实现了 `Copy` trait，不会移动而是复制（如基本类型：i32、bool、f64、char，以及只含 Copy 类型的元组）。

### 示例：Copy vs Move
```rust
fn main() {
    let x: i32 = 5;  // 栈数据，Copy
    let y = x;       // 复制 x 的值
    println!("x: {}, y: {}", x, y);  // 两者都有效

    let s1 = String::from("hello");  // 堆数据，非 Copy
    let s2 = s1;                      // 移动
    // println!("{}", s1);           // 错误
}
```

- **解释**：`Copy` 类型是廉价复制的。String 不实现 Copy，因为复制堆数据昂贵且不安全。你可以用 `#[derive(Copy, Clone)]` 为自定义类型添加（仅限栈数据）。
- **Clone**：显式复制。非 Copy 类型可以用 `clone()`：
  ```rust
  let s2 = s1.clone();  // 深拷贝，s1 仍有效
  ```

## 4. 借用（Borrowing）

借用允许临时访问值而不转移所有权，使用引用（&）。借用规则：
- 任何时候，只能有一个可变借用（&mut），或多个不可变借用（&），但不能同时。
- 引用必须有效（无悬垂引用）。

### 示例：不可变借用
```rust
fn calculate_length(s: &String) -> usize {  // 借用 &String
    s.len()  // 不修改 s
}  // 借用结束

fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);  // 传递引用
    println!("长度: {}, 值: {}", len, s);  // s 仍有效
}
```

- **解释**：`&` 创建引用。函数借用而不拥有。

### 示例：可变借用
```rust
fn change(s: &mut String) {
    s.push_str(", world!");
}

fn main() {
    let mut s = String::from("hello");  // mut 变量
    change(&mut s);                     // 可变借用
    println!("{}", s);                  // 输出: hello, world!
}
```

- **解释**：`&mut` 允许修改。借用期间，不能有其他借用：
  ```rust
  let mut s = String::from("hello");
  let r1 = &s;     // 不可变借用
  let r2 = &s;     // 另一个不可变借用 OK
  // let r3 = &mut s;  // 错误！不能在有不可变借用时可变借用
  ```

## 5. 切片（Slices）

切片是借用的一部分数据，如字符串切片 `&str`。

### 示例：字符串切片
```rust
fn first_word(s: &str) -> &str {  // 接受 &str（String 的借用）
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("第一个词: {}", word);  // 输出: hello
}
```

- **解释**：`&str` 是字符串的借用视图。切片如 `&s[2..5]`。防止修改底层数据以避免失效引用。

## 6. 所有权与函数返回

函数可以返回所有权。

### 示例：返回所有权
```rust
fn gives_ownership() -> String {
    String::from("yours")  // 返回新值，所有权转移
}

fn takes_and_gives_back(s: String) -> String {
    s  // 接收所有权，然后返回
}

fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);  // s2 移动，然后 s3 获得
}
```

- **解释**：返回时，所有权转移给调用者。

## 7. 高级主题：Drop 和 RAII

- **Drop trait**：类型超出作用域时自动调用 `drop` 方法。
- **RAII**（Resource Acquisition Is Initialization）：资源在创建时获取，销毁时释放。
- 自定义 Drop：
  ```rust
  struct Custom {
      data: String,
  }

  impl Drop for Custom {
      fn drop(&mut self) {
          println!("Dropping: {}", self.data);
      }
  }

  fn main() {
      let c = Custom { data: String::from("hello") };
  }  // 输出: Dropping: hello
  ```

## 8. 最佳实践和常见陷阱

- **避免不必要的 clone**：优先借用，clone 只在必要时。
- **mut 的使用**：只在需要修改时用 mut。
- **作用域控制**：用 {} 显式限制作用域，早释放资源。
- **常见错误**：
    - 使用已移动值：编译错误，确保不重复使用。
    - 同时借用冲突：如在循环中借用 vector 同时 push（用索引代替）。
    - 悬垂引用：Rust 编译器防止，如返回局部变量的引用（错误）。
- **与生命周期结合**：借用涉及生命周期（'a），详见生命周期教程。
- **性能**：所有权是零成本抽象，编译时检查。

## 练习建议
1. 编写一个函数，接收 String，返回其长度和修改后的版本（用借用）。
2. 创建一个 struct，实现 Drop，并观察释放顺序。
3. 尝试切片数组和向量，处理边界情况。

如果需要更多示例、与借用规则的深入结合，或与其他概念（如 trait）的集成，请提供细节！