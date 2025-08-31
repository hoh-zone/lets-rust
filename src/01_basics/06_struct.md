以下是关于 Rust 编程语言中结构体（struct）的教程。内容基于 Rust 官方文档（The Rust Book）的相关章节，提供简明解释、代码示例和关键注意事项。结构体是 Rust 中用于创建自定义数据类型的核心机制，它允许将多个相关值组合成一个有意义的数据单元。Rust 的结构体强调所有权、借用和可变性，以确保内存安全。

### 1. 定义结构体（Defining Structs）
结构体使用 `struct` 关键字定义，后面跟随结构体名称和用大括号 `{}` 包围的字段列表。每个字段包括名称和类型。

- **语法**：
  ```rust
  struct User {
      active: bool,
      username: String,
      email: String,
      sign_in_count: u64,
  }
  ```

- **关键点**：
    - 结构体通常拥有其数据，使用如 `String` 的拥有类型，以确保数据在结构体存在期间有效。
    - 如果使用引用（如 `&str`），需要指定生命周期（lifetime），以避免悬垂引用（dangling references）。
    - 字段不能单独标记为可变，整个结构体实例必须是可变的才能修改字段。

- **元组结构体（Tuple Structs）**：无命名字段，仅用类型定义，类似于命名元组。访问字段用索引（如 `.0`）。
  示例：
  ```rust
  struct Color(i32, i32, i32);
  let black = Color(0, 0, 0);
  println!("R: {}", black.0);
  ```

- **单元结构体（Unit-Like Structs）**：无字段，用于不需要数据的类型。
  示例：
  ```rust
  struct AlwaysEqual;
  let subject = AlwaysEqual;
  ```

### 2. 实例化结构体（Instantiating Structs）
创建结构体实例时，使用大括号指定每个字段的值。字段顺序不需匹配定义顺序。

- **基本实例化**：
  示例：
  ```rust
  let user1 = User {
      active: true,
      username: String::from("someusername123"),
      email: String::from("someone@example.com"),
      sign_in_count: 1,
  };
  ```

- **字段初始化简写（Field Init Shorthand）**：当参数名与字段名相同时，可省略字段名。
  示例（在函数中）：
  ```rust
  fn build_user(email: String, username: String) -> User {
      User {
          active: true,
          username,
          email,
          sign_in_count: 1,
      }
  }
  ```

- **结构体更新语法（Struct Update Syntax）**：使用 `..` 从另一个实例复制剩余字段。
  示例：
  ```rust
  let user2 = User {
      email: String::from("another@example.com"),
      ..user1
  };
  ```
  注意：这会移动 `user1` 的拥有值（如 `String`），除非那些字段实现了 Copy trait。

- **访问和更新字段**：使用点号 `.` 访问字段。要更新，需要可变实例（`mut`）。
  示例：
  ```rust
  let mut user1 = User { /* ... */ };
  user1.email = String::from("newemail@example.com");
  ```

### 3. 示例程序：使用结构体计算矩形面积
这是一个经典示例，展示如何使用结构体组织数据、借用和计算。

- **定义和实例化**：
  ```rust
  #[derive(Debug)]  // 启用调试打印
  struct Rectangle {
      width: u32,
      height: u32,
  }

  fn main() {
      let rect1 = Rectangle {
          width: 30,
          height: 50,
      };
      println!("rect1 是 {:?}", rect1);  // 使用 {:?} 打印调试信息
  }
  ```

- **更新字段和调试**：使用 `dbg!` 宏调试表达式值。
  示例：
  ```rust
  let scale = 2;
  let rect1 = Rectangle {
      width: dbg!(30 * scale),  // 打印并赋值 60
      height: 50,
  };
  dbg!(&rect1);  // 调试引用，避免移动所有权
  ```

- **借用处理**：函数借用结构体以避免所有权转移。
  示例（计算面积函数）：
  ```rust
  fn area(rectangle: &Rectangle) -> u32 {
      rectangle.width * rectangle.height
  }

  fn main() {
      let rect1 = Rectangle { width: 30, height: 50 };
      println!("面积: {} 平方像素", area(&rect1));
  }
  ```
  注意：使用 `&` 借用，确保 `main` 保留所有权。

### 4. 方法语法（Method Syntax）
方法是与结构体关联的函数，使用 `impl` 块定义。方法总是以 `self` 为第一个参数，表示调用实例。

- **定义方法**：
  使用 `impl StructName { }` 块。`&self` 表示不可变借用。
  示例：
  ```rust
  impl Rectangle {
      fn area(&self) -> u32 {
          self.width * self.height
      }
  }

  fn main() {
      let rect1 = Rectangle { width: 30, height: 50 };
      println!("面积: {}", rect1.area());  // 调用方法
  }
  ```

- **可变性和额外参数**：
    - `&mut self`：允许修改实例。
    - 示例（额外参数）：
      ```rust
      impl Rectangle {
          fn can_hold(&self, other: &Rectangle) -> bool {
              self.width > other.width && self.height > other.height
          }
      }
      ```

- **关联函数（Associated Functions）**：不带 `self`，常用于构造函数。使用 `::` 调用。
  示例：
  ```rust
  impl Rectangle {
      fn square(size: u32) -> Self {
          Self { width: size, height: size }
      }
  }

  let sq = Rectangle::square(3);
  ```

- **多个 impl 块**：允许将方法分散定义，但等价于单个块。

- **关键差异与 OOP**：
    - Rust 无自动 getter/setter，需要手动定义。
    - 方法名可与字段名相同（基于语法区分）。
    - 强调借用规则，与 OOP 的封装不同。
    - Rust 自动处理引用/解引用，无需 `->` 操作符。

### 注意事项
- **所有权**：结构体字段若为拥有类型（如 `String`），实例移动时会转移所有权。
- **借用**：优先使用借用（`&`）以避免不必要的移动。
- **调试**：使用 `#[derive(Debug)]` 注解结构体，以启用 `{:?}` 打印。
- **实践**：通过 `cargo new` 创建项目，在 `main.rs` 中测试这些示例。

