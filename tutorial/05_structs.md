# 第5章：结构体

结构体（struct）是 Rust 中创建自定义数据类型的主要方式，它允许你将相关的数据组合在一起。

## 5.1 定义与使用

### 定义结构体

```rust
// 定义一个结构体
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // 创建结构体实例
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    // 访问字段
    println!("用户名: {}", user1.username);
    println!("邮箱: {}", user1.email);
}
```

### 可变结构体

```rust
fn main() {
    // 整个实例必须是可变的
    let mut user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    // 修改字段
    user.email = String::from("anotheremail@example.com");
    user.sign_in_count += 1;
    
    println!("新邮箱: {}", user.email);
    println!("登录次数: {}", user.sign_in_count);
}
```

### 字段初始化简写

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,      // 字段初始化简写
        username,   // 当变量名与字段名相同时
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user = build_user(
        String::from("test@example.com"),
        String::from("testuser")
    );
    
    println!("{} 已创建", user.username);
}
```

### 结构体更新语法

```rust
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    // 基于 user1 创建 user2
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1  // 其余字段从 user1 获取
    };
    
    // 注意：user1 的 username 被移动了
    println!("user2: {}", user2.username);
    // println!("{}", user1.username);  // 错误！已被移动
    println!("{}", user1.active);        // 正常，bool 是 Copy
}
```

### 元组结构体

元组结构体有结构体名称，但没有具体的字段名：

```rust
// 定义元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    // 访问字段
    println!("黑色 RGB: ({}, {}, {})", black.0, black.1, black.2);
    
    // 即使值相同，Color 和 Point 是不同的类型
    // let point = black;  // 错误！类型不匹配
}
```

### 类单元结构体

没有任何字段的结构体：

```rust
// 类单元结构体
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
    
    // 常用于实现 trait
    // 不占用内存空间
}
```

### 结构体的打印

```rust
// 让结构体可以打印
#[derive(Debug)]  // 派生 Debug trait
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    
    // 使用 {:?} 打印
    println!("rect is {:?}", rect);
    
    // 使用 {:#?} 美化打印
    println!("rect is {:#?}", rect);
    
    // 使用 dbg! 宏
    dbg!(&rect);
    
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),  // dbg! 返回表达式的值
        height: 50,
    };
}
```

## 5.2 关联方法与函数

### 方法定义

方法是在结构体的上下文中定义的函数：

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl 块定义方法
impl Rectangle {
    // 方法的第一个参数总是 self
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // 可变方法
    fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
    
    // 获取所有权的方法（较少见）
    fn consume(self) -> u32 {
        self.width + self.height
    }
}

fn main() {
    let mut rect = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("面积: {}", rect.area());
    
    rect.double_size();
    println!("双倍大小后: {:?}", rect);
    
    let sum = rect.consume();
    println!("宽高之和: {}", sum);
    // println!("{:?}", rect);  // 错误！rect 已被消耗
}
```

### 关联函数

不以 `self` 作为参数的函数称为关联函数：

```rust
impl Rectangle {
    // 关联函数（静态方法）
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    
    // 另一个关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // 使用 :: 调用关联函数
    let rect = Rectangle::new(30, 50);
    let sq = Rectangle::square(20);
    
    println!("矩形: {:?}", rect);
    println!("正方形: {:?}", sq);
}
```

### 多个 impl 块

可以为同一个结构体定义多个 `impl` 块：

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);
    let rect3 = Rectangle::new(60, 45);
    
    println!("rect1 能容纳 rect2 吗？{}", rect1.can_hold(&rect2));
    println!("rect1 能容纳 rect3 吗？{}", rect1.can_hold(&rect3));
}
```

### 方法链

```rust
impl Rectangle {
    fn width(&self) -> u32 {
        self.width
    }
    
    fn set_width(&mut self, width: u32) -> &mut Self {
        self.width = width;
        self  // 返回自身的可变引用
    }
    
    fn set_height(&mut self, height: u32) -> &mut Self {
        self.height = height;
        self
    }
}

fn main() {
    let mut rect = Rectangle::new(10, 20);
    
    // 方法链式调用
    rect.set_width(30)
        .set_height(40);
    
    println!("新尺寸: {}x{}", rect.width(), rect.height);
}
```

## 5.3 内存布局

### 结构体的内存布局

```rust
use std::mem;

struct Point {
    x: i32,  // 4 字节
    y: i32,  // 4 字节
}

struct Color {
    r: u8,   // 1 字节
    g: u8,   // 1 字节
    b: u8,   // 1 字节
    a: u8,   // 1 字节
}

fn main() {
    println!("Point 大小: {} 字节", mem::size_of::<Point>());
    println!("Color 大小: {} 字节", mem::size_of::<Color>());
    
    // 字段偏移
    let p = Point { x: 10, y: 20 };
    let p_addr = &p as *const Point as usize;
    let x_addr = &p.x as *const i32 as usize;
    let y_addr = &p.y as *const i32 as usize;
    
    println!("Point 地址: 0x{:x}", p_addr);
    println!("x 偏移: {} 字节", x_addr - p_addr);
    println!("y 偏移: {} 字节", y_addr - p_addr);
}
```

### 内存对齐

Rust 会自动进行内存对齐以提高性能：

```rust
// 未优化的布局
struct Unoptimized {
    a: u8,    // 1 字节
    b: u32,   // 4 字节
    c: u8,    // 1 字节
}

// 手动优化的布局
struct Optimized {
    b: u32,   // 4 字节
    a: u8,    // 1 字节
    c: u8,    // 1 字节
}

fn main() {
    println!("Unoptimized 大小: {} 字节", mem::size_of::<Unoptimized>());
    println!("Optimized 大小: {} 字节", mem::size_of::<Optimized>());
}
```

### repr 属性

使用 `#[repr]` 属性控制内存布局：

```rust
// C 兼容的布局
#[repr(C)]
struct CCompatible {
    a: u8,
    b: u32,
    c: u8,
}

// 紧凑布局
#[repr(packed)]
struct Packed {
    a: u8,
    b: u32,
    c: u8,
}

// 指定对齐
#[repr(align(16))]
struct Aligned {
    data: [u8; 10],
}

fn main() {
    println!("CCompatible 大小: {} 字节", mem::size_of::<CCompatible>());
    println!("Packed 大小: {} 字节", mem::size_of::<Packed>());
    println!("Aligned 大小: {} 字节", mem::size_of::<Aligned>());
    println!("Aligned 对齐: {} 字节", mem::align_of::<Aligned>());
}
```

### 零大小类型（ZST）

```rust
struct Empty;
struct Marker;

fn main() {
    println!("Empty 大小: {} 字节", mem::size_of::<Empty>());
    
    // ZST 不占用内存
    let array: [Empty; 1000] = [Empty; 1000];
    println!("1000 个 Empty 的数组大小: {} 字节", 
             mem::size_of_val(&array));
}
```

### 泛型结构体

```rust
struct Point<T> {
    x: T,
    y: T,
}

struct Point3D<T, U> {
    x: T,
    y: T,
    z: U,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

impl<T: Copy> Point<T> {
    fn x(&self) -> T {
        self.x
    }
}

fn main() {
    let integer_point = Point::new(5, 10);
    let float_point = Point::new(1.0, 4.0);
    
    let point3d = Point3D {
        x: 5,
        y: 10,
        z: 2.0,
    };
    
    println!("整数点 x: {}", integer_point.x());
    println!("浮点数点 x: {}", float_point.x());
}
```

### 生命周期标注的结构体

```rust
// 包含引用的结构体需要生命周期标注
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("重要级别: {}", i.level());
    let part = i.announce_and_return_part("这是重要内容");
    println!("内容: {}", part);
}
```

### 结构体的模式匹配

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };
    
    // 解构结构体
    let Point { x, y } = p;
    println!("x: {}, y: {}", x, y);
    
    // 部分解构
    let Point { x, .. } = p;
    println!("只要 x: {}", x);
    
    // 重命名
    let Point { x: a, y: b } = p;
    println!("a: {}, b: {}", a, b);
    
    // match 中使用
    match p {
        Point { x: 0, y } => println!("在 y 轴上，y = {}", y),
        Point { x, y: 0 } => println!("在 x 轴上，x = {}", x),
        Point { x, y } => println!("在 ({}, {})", x, y),
    }
}
```

## 小结

本章学习了 Rust 中的结构体：

1. **定义与使用**：创建自定义类型，组织相关数据
2. **方法与关联函数**：为结构体添加行为
3. **内存布局**：理解结构体在内存中的表示

结构体是 Rust 中构建复杂数据类型的基础，配合所有权系统和借用规则，可以创建安全高效的数据结构。下一章我们将学习常用的集合类型。 