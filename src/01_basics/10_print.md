以下是关于 Rust 编程语言中 print 函数（实际为宏）和 debug 宏的教程。
Rust 不提供内置的 print 函数，而是使用宏（如 print! 和 println!）来处理输出。这些宏是标准库的一部分，用于控制台打印。调试宏如 dbg! 用于开发时快速检查值，而 Debug trait 则用于结构化数据的打印。

### 1. print! 和 println! 宏：基本输出
Rust 使用宏来处理格式化输出，因为宏允许在编译时扩展代码，提供灵活性。print! 用于打印不换行，println! 用于打印并换行。它们类似于 C 的 printf，但使用 Rust 的格式化语法。

- **语法**：
    - `print!("格式字符串", 参数...);`：打印到标准输出（stdout），不添加换行。
    - `println!("格式字符串", 参数...);`：打印并添加换行。
    - 格式字符串使用 `{}` 作为占位符，参数会自动推断类型。
    - 支持命名参数 `{name}` 和位置参数 `{0}`。

- **基本示例**：
  ```rust
  fn main() {
      print!("Hello, ");
      println!("world!");  // 输出: Hello, world!（换行）
      
      let x = 42;
      println!("x 的值为: {}", x);  // 输出: x 的值为: 42
      
      println!("{} + {} = {}", 1, 2, 1 + 2);  // 输出: 1 + 2 = 3
  }
  ```

- **格式化选项**：
    - `{:?}`：调试格式（用于 Debug trait）。
    - `{:#?}`：美化调试格式（多行缩进）。
    - `{:b}`：二进制，`{:x}`：十六进制，`{:o}`：八进制。
    - `{:.2}`：浮点数精度（小数点后两位）。
      示例：
  ```rust
  fn main() {
      let pi = 3.141592;
      println!("Pi 约为 {:.2}", pi);  // 输出: Pi 约为 3.14
      
      println!("二进制: {:b}", 10);  // 输出: 二进制: 1010
  }
  ```

- **命名参数**：
  示例：
  ```rust
  fn main() {
      println!("{subject} {verb} {object}",
               object="懒狗",
               subject="快速的棕色狐狸",
               verb="跳过");  // 输出: 快速的棕色狐狸 跳过 懒狗
  }
  ```

- **注意**：这些宏会 panic 如果格式字符串无效（如参数不匹配）。输出到 stderr 使用 eprint! 和 eprintln!。

### 2. debug 宏：dbg!
dbg! 宏用于调试，它打印表达式的值和源代码位置，然后返回该值。适合插入代码中快速检查，而不中断流程。

- **语法**：
    - `dbg!(表达式);`：打印表达式的文件名、行号、列号和值，返回表达式本身。
    - 支持借用（&），避免所有权转移。

- **示例**：
  ```rust
  fn main() {
      let x = 5;
      let y = dbg!(x * 2);  // 打印: [src/main.rs:3:13] x * 2 = 10，返回 10
      
      dbg!(y + 1);  // 打印: [src/main.rs:4:5] y + 1 = 11
  }
  ```

- **与结构体结合**：
  dbg! 使用 Debug trait，如果结构体未实现 Debug，会编译错误。

- **注意**：dbg! 只在调试构建中有效，在发布模式下可能被优化掉。输出到 stderr，便于区分正常输出。

### 3. Debug trait：结构化调试打印
Debug trait 用于自定义类型的调试输出，常与 `{:?}` 或 `{:#?}` 结合。标准库类型（如 i32、String）已实现 Debug，自定义类型需派生或手动实现。

- **派生 Debug**：
  使用 `#[derive(Debug)]` 注解结构体或枚举，自动生成 Debug 实现。
  示例：
  ```rust
  #[derive(Debug)]
  struct Rectangle {
      width: u32,
      height: u32,
  }

  fn main() {
      let rect = Rectangle { width: 30, height: 50 };
      println!("rect 是 {:?}", rect);  // 输出: rect 是 Rectangle { width: 30, height: 50 }
      println!("rect 是 {:#?}", rect);  // 美化输出，多行缩进
  }
  ```

- **手动实现 Debug**：
  如果需要自定义格式，实现 `std::fmt::Debug` trait。
  示例：
  ```rust
  use std::fmt;

  struct Point {
      x: i32,
      y: i32,
  }

  impl fmt::Debug for Point {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          f.debug_struct("Point")
           .field("x", &self.x)
           .field("y", &self.y)
           .finish()
      }
  }

  fn main() {
      let p = Point { x: 1, y: 2 };
      println!("{:?}", p);  // 输出: Point { x: 1, y: 2 }
  }
  ```

- **Debug vs Display**：
    - Debug：用于开发者，格式如 `{ x: 1, y: 2 }`，通过 `{:?}`。
    - Display：用于用户友好输出，通过 `{}`。需手动实现 `std::fmt::Display`。
      示例（Display）：
  ```rust
  impl fmt::Display for Point {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          write!(f, "({}, {})", self.x, self.y)
      }
  }

  println!("{}", p);  // 输出: (1, 2)
  ```

### 注意事项
- **性能**：打印宏在发布模式下高效，但 dbg! 适合开发阶段。
- **所有权**：dbg! 借用值，避免移动；println! 等宏不消耗所有权。
- **错误处理**：打印到 stdout/stderr 是阻塞的，如果 IO 失败会 panic。
- **替代**：对于复杂日志，使用 log 或 tracing crate。
- **实践**：在 RustRover 或命令行中使用 `cargo run` 测试这些示例，观察输出差异。
