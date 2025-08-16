// 教学示例 - 允许未使用的代码
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// 第5章：结构体示例
// 使用命令：cargo run --bin structs

fn main() {
    println!("🦀 Rust 基础教程 - 第5章：结构体");
    println!("============================\n");
    
    // 5.1 定义与使用
    println!("📍 5.1 定义与使用");
    println!("---------------");
    struct_definition_and_usage();
    println!();
    
    // 5.2 关联方法与函数
    println!("📍 5.2 关联方法与函数");
    println!("-------------------");
    associated_methods();
    println!();
    
    // 5.3 内存布局
    println!("📍 5.3 内存布局");
    println!("---------------");
    memory_layout();
    
    println!("\n✅ 第5章示例运行完成！");
}

// 定义结构体
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 5.1 定义与使用
fn struct_definition_and_usage() {
    println!("基本结构体定义与使用:");
    
    // 创建结构体实例
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    // 访问字段
    println!("  用户名: {}", user1.username);
    println!("  邮箱: {}", user1.email);
    
    println!("\n可变结构体:");
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
    
    println!("  新邮箱: {}", user.email);
    println!("  登录次数: {}", user.sign_in_count);
    
    println!("\n字段初始化简写:");
    let user2 = build_user(
        String::from("test@example.com"),
        String::from("testuser")
    );
    println!("  {} 已创建", user2.username);
    
    println!("\n结构体更新语法:");
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1  // 其余字段从 user1 获取
    };
    
    // 注意：user1 的 username 被移动了
    println!("  user3: {}", user3.username);
    println!("  user1.active 仍可用: {}", user1.active);  // bool 是 Copy
    
    println!("\n元组结构体:");
    tuple_structs();
    
    println!("\n类单元结构体:");
    unit_like_structs();
    
    println!("\n结构体的打印:");
    struct_printing();
}

fn build_user(email: String, username: String) -> User {
    User {
        email,      // 字段初始化简写
        username,   // 当变量名与字段名相同时
        active: true,
        sign_in_count: 1,
    }
}

fn tuple_structs() {
    // 定义元组结构体
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    // 访问字段
    println!("  黑色 RGB: ({}, {}, {})", black.0, black.1, black.2);
    println!("  原点坐标: ({}, {}, {})", origin.0, origin.1, origin.2);
    
    // 即使值相同，Color 和 Point 是不同的类型
}

fn unit_like_structs() {
    // 类单元结构体
    struct AlwaysEqual;
    
    let _subject = AlwaysEqual;
    println!("  类单元结构体创建成功（不占用内存空间）");
}

fn struct_printing() {
    // 让结构体可以打印
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    
    // 使用 {:?} 打印
    println!("  rect is {:?}", rect);
    
    // 使用 {:#?} 美化打印
    println!("  rect is {:#?}", rect);
    
    // 使用 dbg! 宏
    println!("  使用 dbg! 宏:");
    dbg!(&rect);
    
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),  // dbg! 返回表达式的值
        height: 50,
    };
    println!("  rect2: {:?}", rect2);
}

// 5.2 关联方法与函数
fn associated_methods() {
    println!("方法定义:");
    methods_demo();
    
    println!("\n关联函数:");
    associated_functions_demo();
    
    println!("\n方法链:");
    method_chaining();
}

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
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
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

fn methods_demo() {
    let mut rect = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("  面积: {}", rect.area());
    
    rect.double_size();
    println!("  双倍大小后: {:?}", rect);
    
    let sum = rect.consume();
    println!("  宽高之和: {}", sum);
    // println!("{:?}", rect);  // 错误！rect 已被消耗
}

fn associated_functions_demo() {
    // 使用 :: 调用关联函数
    let rect = Rectangle::new(30, 50);
    let sq = Rectangle::square(20);
    
    println!("  矩形: {:?}", rect);
    println!("  正方形: {:?}", sq);
    
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);
    let rect3 = Rectangle::new(60, 45);
    
    println!("  rect1 能容纳 rect2 吗？{}", rect1.can_hold(&rect2));
    println!("  rect1 能容纳 rect3 吗？{}", rect1.can_hold(&rect3));
}

fn method_chaining() {
    let mut rect = Rectangle::new(10, 20);
    
    // 方法链式调用
    rect.set_width(30)
        .set_height(40);
    
    println!("  新尺寸: {}x{}", rect.width(), rect.width);
}

// 5.3 内存布局
fn memory_layout() {
    use std::mem;
    
    println!("结构体的内存布局:");
    memory_layout_demo();
    
    println!("\n内存对齐:");
    memory_alignment();
    
    println!("\n泛型结构体:");
    generic_structs();
    
    println!("\n结构体的模式匹配:");
    pattern_matching();
}

fn memory_layout_demo() {
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
    
    println!("  Point 大小: {} 字节", mem::size_of::<Point>());
    println!("  Color 大小: {} 字节", mem::size_of::<Color>());
    
    // 字段偏移
    let p = Point { x: 10, y: 20 };
    let p_addr = &p as *const Point as usize;
    let x_addr = &p.x as *const i32 as usize;
    let y_addr = &p.y as *const i32 as usize;
    
    println!("  Point 地址: 0x{:x}", p_addr);
    println!("  x 偏移: {} 字节", x_addr - p_addr);
    println!("  y 偏移: {} 字节", y_addr - p_addr);
}

fn memory_alignment() {
    use std::mem;
    
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
    
    println!("  Unoptimized 大小: {} 字节", mem::size_of::<Unoptimized>());
    println!("  Optimized 大小: {} 字节", mem::size_of::<Optimized>());
    
    // 零大小类型（ZST）
    #[derive(Copy, Clone)]
    struct Empty;
    
    println!("  Empty 大小: {} 字节", mem::size_of::<Empty>());
    
    // ZST 不占用内存
    let array: [Empty; 1000] = [Empty; 1000];
    println!("  1000 个 Empty 的数组大小: {} 字节", 
             mem::size_of_val(&array));
}

fn generic_structs() {
    struct Point<T> {
        x: T,
        y: T,
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
    
    let integer_point = Point::new(5, 10);
    let float_point = Point::new(1.0, 4.0);
    
    println!("  整数点 x: {}", integer_point.x());
    println!("  浮点数点 x: {}", float_point.x());
}

fn pattern_matching() {
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p = Point { x: 0, y: 7 };
    
    // 解构结构体
    let Point { x, y } = p;
    println!("  解构: x: {}, y: {}", x, y);
    
    // 部分解构
    let Point { x, .. } = p;
    println!("  只要 x: {}", x);
    
    // 重命名
    let Point { x: a, y: b } = p;
    println!("  重命名: a: {}, b: {}", a, b);
    
    // match 中使用
    match p {
        Point { x: 0, y } => println!("  在 y 轴上，y = {}", y),
        Point { x, y: 0 } => println!("  在 x 轴上，x = {}", x),
        Point { x, y } => println!("  在 ({}, {})", x, y),
    }
} 