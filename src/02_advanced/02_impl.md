# Rust impl

在 Rust 中，`impl` 关键字用于为类型实现方法和关联函数。它是 Rust 面向对象编程风格的核心，允许你为结构体（struct）、枚举（enum）或 trait 定义行为。`impl` 块可以将方法附加到类型上，实现封装和多态。

本教程从基础开始，逐步深入，包含代码示例和解释。假设你已熟悉 Rust 的基本语法（如 struct 和 enum）。每个示例后，我会解释关键点。如果你有 Rust 环境，可以复制代码运行测试。教程基于 Rust 1.80+（截至 2025 年，核心语法未变）。

## 1. impl 简介

- **impl 的作用**：为现有类型添加方法（methods）和关联函数（associated functions）。方法可以访问实例数据（self），关联函数类似于静态方法。
- **语法**：
  ```rust
  impl TypeName {
      // 方法和关联函数
  }
  ```
- **关键概念**：
    - **self**：表示实例引用。&self（不可变借用）、&mut self（可变借用）、self（所有权转移）。
    - **关联函数**：不带 self 的函数，常用于构造函数（如 new）。
    - impl 可以多次定义（在不同模块中），Rust 会合并它们。
- **与 trait 的关系**：impl 可以实现 trait（接口），提供多态。

## 2. 为结构体实现方法

### 示例：基本方法
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 关联函数：构造函数
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    // 方法：计算面积
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 可变方法：缩放
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    // 消耗 self 的方法
    fn into_square(self) -> Rectangle {
        let side = self.width.max(self.height);
        Rectangle { width: side, height: side }
    }
}

fn main() {
    let mut rect = Rectangle::new(5, 10);
    println!("面积: {}", rect.area());  // 输出: 面积: 50

    rect.scale(2);
    println!("新矩形: {:?}", rect);  // 输出: 新矩形: Rectangle { width: 10, height: 20 }

    let square = rect.into_square();
    println!("正方形: {:?}", square);  // 输出: 正方形: Rectangle { width: 20, height: 20 }
}
```

- **解释**：
    - `fn new` 是关联函数，通过 `Type::function` 调用。
    - `&self` 方法借用实例，不修改它。
    - `&mut self` 方法允许修改实例。
    - `self` 方法消耗实例的所有权，适合转换操作。
    - `Self` 是当前类型的别名，便于泛型中使用。

### 多 impl 块
你可以拆分 impl：
```rust
impl Rectangle {
    fn area(&self) -> u32 { /* ... */ }
}

impl Rectangle {
    fn scale(&mut self, factor: u32) { /* ... */ }
}
```
- **用处**：在大型项目中，按功能分组方法。

## 3. 为枚举实现方法

枚举也可以有方法，常用于模式匹配。

### 示例：枚举 impl
```rust
#[derive(Debug)]
enum Shape {
    Circle(f64),  // 半径
    Square(f64),  // 边长
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Square(s) => s * s,
        }
    }
}

fn main() {
    let circle = Shape::Circle(5.0);
    println!("圆面积: {}", circle.area());  // 输出: 圆面积: 78.53981633974483
}
```

- **解释**：方法使用 `match` 处理不同变体。枚举方法增强了类型的安全性和表达力。

## 4. 实现 trait

trait 是 Rust 的接口。`impl Trait for Type` 为类型实现 trait 定义的行为。

### 示例：简单 trait
```rust
trait Drawable {
    fn draw(&self);
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("绘制矩形: {} x {}", self.width, self.height);
    }
}

fn main() {
    let rect = Rectangle::new(3, 4);
    rect.draw();  // 输出: 绘制矩形: 3 x 4
}
```

- **解释**：trait 定义签名，impl 提供实现。类型可以实现多个 trait。

### 默认实现和 trait bound
trait 可以有默认方法。
```rust
trait Summary {
    fn summarize(&self) -> String {
        String::from("（默认摘要）")
    }
}

impl Summary for Rectangle {}  // 使用默认

fn print_summary<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

fn main() {
    let rect = Rectangle::new(1, 2);
    print_summary(&rect);  // 输出: （默认摘要）
}
```

- **解释**：
    - 默认方法允许可选实现。
    - `T: Summary` 是 trait bound，确保泛型参数实现了 trait。

## 5. 泛型 impl

impl 可以是泛型的，支持 trait bound。

### 示例：泛型类型
```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T: std::fmt::Display> Point<T> {  // 只为 Display 类型实现
    fn display(&self) {
        println!("({}, {})", self.x, self.y);
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("x: {}", p.x());  // 输出: x: 5
    p.display();  // 输出: (5, 10)
}
```

- **解释**：
    - `impl<T>` 为所有 T 实现。
    - bound 如 `T: Display` 限制实现范围，编译时检查。

## 6. 高级主题：impl trait 和 dyn

- **impl Trait**：作为返回类型，表示“某个实现 Trait 的类型”（不指定具体类型）。
  ```rust
  fn returns_drawable() -> impl Drawable {
      Rectangle::new(1, 1)
  }
  ```
- **dyn Trait**： trait 对象，用于运行时多态（有虚表开销）。
  ```rust
  fn draw_it(d: &dyn Drawable) {
      d.draw();
  }
  ```

- **用处**：在需要异构集合时，如 `Vec<Box<dyn Drawable>>`。

## 7. 最佳实践和常见陷阱

- **方法 vs 关联函数**：用 self 的叫方法，不用 self 的叫关联函数。
- **私有性**：用 `pub` 暴露方法/函数。
- **避免过度 impl**：保持类型内聚，方法应与类型数据相关。
- **trait 继承**：trait 可以继承其他 trait，如 `trait Super: Sub {}`。
- **orphan rule**：不能为外部类型实现外部 trait（防止冲突）。
- **常见错误**：
    - 借用规则违反：如在 &self 中修改字段（用 &mut self）。
    - 未实现 trait：编译错误，强制实现所有方法。
    - 泛型 bound 不足：添加 where 子句，如 `impl<T> where T: Clone`。
- **性能**：方法调用是静态分发的（零开销），除非用 dyn。

## 练习建议
1. 为一个自定义 struct 实现多个方法，包括构造函数和计算方法。
2. 定义一个 trait，并为两个不同类型实现它，使用泛型函数调用。
3. 探索标准库 impl，如 Vec 的方法，理解 Rust 如何使用 impl。
