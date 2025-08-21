### 1. 变量（Variables）
Rust 中的变量默认是不可变的（immutable），这有助于避免意外修改数据，提高代码安全性。变量使用 `let` 关键字声明。

- **声明和不可变性**：默认情况下，变量一旦赋值就不能改变。如果尝试修改，会在编译时出错。
  示例：
  ```rust
  fn main() {
      let x = 5;
      // x = 6; // 错误：cannot assign twice to immutable variable
      println!("x 的值为: {}", x);
  }
  ```

- **可变性（Mutability）**：使用 `mut` 关键字使变量可变，可以多次赋值。
  示例：
  ```rust
  fn main() {
      let mut x = 5;
      println!("x 的值为: {}", x);
      x = 6;
      println!("x 的值为: {}", x);
  }
  ```
  注意：可变性是可选的，用于表示变量可能在未来变化。

- **常量（Constants）**：使用 `const` 声明，常量总是不可变的，必须指定类型，且值必须是常量表达式（编译时计算）。常量可以全局声明。
  示例：
  ```rust
  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

  fn main() {
      println!("三小时的秒数: {}", THREE_HOURS_IN_SECONDS);
  }
  ```

- **遮蔽（Shadowing）**：可以使用相同的变量名重新声明（用 `let`），新变量会“遮蔽”旧的。这允许类型转换，而不需使用 `mut`，且防止意外修改。
  示例：
  ```rust
  fn main() {
      let x = 5;
      let x = x + 1; // 遮蔽，x 现在是 6

      {
          let x = x * 2; // 内作用域遮蔽，x 是 12
          println!("内层 x 的值为: {}", x);
      }

      println!("外层 x 的值为: {}", x); // 打印 6
  }
  ```
  注意：遮蔽不同于可变性，它允许改变类型（例如从字符串到数字）。

变量的作用域是块级（用 `{}` 包围），超出作用域后变量被销毁。

### 2. 基本类型（Basic Types）
Rust 是静态类型语言，编译时必须知道所有变量的类型，但支持类型推断（type inference）。基本类型分为标量类型（scalar）和复合类型（compound）。

#### 标量类型（Scalar Types）
这些类型表示单个值。

- **整数（Integers）**：无小数部分，分有符号（`i`）和无符号（`u`），不同位宽。默认是 `i32`。
  | 类型     | 有符号范围示例（i8） | 无符号范围示例（u8） | 示例 |
  |----------|----------------------|----------------------|------|
  | 8-bit   | -128 到 127         | 0 到 255            | `let x: i8 = -10;` |
  | 16-bit  | -32768 到 32767     | 0 到 65535          | `let y: u16 = 1000;` |
  | 32-bit  | -2^31 到 2^31-1     | 0 到 2^32-1         | `let z = 42; // i32 默认` |
  | 64-bit  | -2^63 到 2^63-1     | 0 到 2^64-1         | `let a: i64 = 999999999999;` |
  | 128-bit | -2^127 到 2^127-1   | 0 到 2^128-1        | `let b: u128 = 1;` |
  | isize/usize | 依赖架构（64-bit 系统为 64 位） | 同左 | 用于索引 |

  字面量示例：十进制 `98_222`、十六进制 `0xff`、八进制 `0o77`、二进制 `0b1111_0000`、字节 `b'A'`（仅 `u8`）。
  注意：整数溢出在调试模式会 panic，在发布模式会环绕（two’s complement）。使用 `wrapping_add` 等方法处理溢出。

- **浮点数（Floating-Point Numbers）**：有小数部分，默认 `f64`（更高精度）。
  示例：
  ```rust
  fn main() {
      let x = 2.0; // f64
      let y: f32 = 3.0; // f32
      println!("x: {}, y: {}", x, y);
  }
  ```
  注意：符合 IEEE-754 标准。

- **布尔（Booleans）**：`true` 或 `false`，大小 1 字节，用于条件判断。
  示例：
  ```rust
  fn main() {
      let t = true;
      let f: bool = false;
      if t {
          println!("真");
      }
  }
  ```

- **字符（Characters）**：用单引号表示，4 字节，支持 Unicode（包括表情符号）。
  示例：
  ```rust
  fn main() {
      let c = 'z';
      let z: char = 'ℤ';
      let heart_eyed_cat = '😻';
      println!("字符: {}", heart_eyed_cat);
  }
  ```
  注意：不同于字符串（双引号），`char` 是 Unicode 标量值。

#### 复合类型（Compound Types）
这些类型组合多个值。

- **元组（Tuples）**：固定长度，允许不同类型。访问用点号或解构。
  示例：
  ```rust
  fn main() {
      let tup: (i32, f64, u8) = (500, 6.4, 1);
      let (x, y, z) = tup; // 解构
      println!("y 的值为: {}", y);
      let five_hundred = tup.0; // 索引访问
  }
  ```
  注意：空元组 `()` 表示无值，常用于无返回值的函数。

- **数组（Arrays）**：固定长度，所有元素同类型，栈上分配。
  示例：
  ```rust
  fn main() {
      let a: [i32; 5] = [1, 2, 3, 4, 5];
      let b = [3; 5]; // [3, 3, 3, 3, 3]
      println!("第一个元素: {}", a[0]);
  }
  ```
  注意：长度固定，不能增长。越界访问会 panic。

### 3. 函数（Functions）
Rust 函数使用 `fn` 关键字定义，使用 snake_case 命名（小写下划线分隔）。函数可以有参数、返回值的声明，且顺序不影响调用（只要在作用域内）。

- **定义和调用**：函数体用 `{}` 包围，调用用函数名加 `()`。
  示例：
  ```rust
  fn main() {
      println!("Hello, world!");
      another_function();
  }

  fn another_function() {
      println!("另一个函数。");
  }
  ```

- **参数（Parameters）**：必须声明类型，多参数用逗号分隔。
  示例：
  ```rust
  fn main() {
      print_labeled_measurement(5, 'h');
  }

  fn print_labeled_measurement(value: i32, unit_label: char) {
      println!("测量值为: {value}{unit_label}");
  }
  ```

- **语句 vs 表达式**：语句不返回值（以 `;` 结束），表达式返回值（无 `;`）。函数体可由语句组成，最后表达式作为返回值。
  注意：块 `{}` 也是表达式。

- **返回值（Return Values）**：用 `->` 指定类型，最后表达式（无 `;`）即返回值，也可用 `return` 提前返回。
  示例：
  ```rust
  fn main() {
      let x = plus_one(5);
      println!("x 的值为: {}", x);
  }

  fn plus_one(x: i32) -> i32 {
      x + 1  // 无分号，返回值
  }
  ```

更多细节可参考 Rust 官方书籍：https://doc.rust-lang.org/book/。如果需要高级主题如所有权或模式匹配，请进一步询问！