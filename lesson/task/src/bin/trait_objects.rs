// 教学示例 - 允许未使用的代码
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// 第10章：trait 对象
// trait 对象允许在运行时进行动态分发，实现多态

use std::fmt::{Debug, Display};

// 1. 基本的 trait 定义
trait Draw {
    fn draw(&self);
}

// 2. 实现 Draw trait 的不同类型
struct Circle {
    radius: f64,
}

impl Draw for Circle {
    fn draw(&self) {
        println!("绘制半径为 {} 的圆形", self.radius);
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Draw for Rectangle {
    fn draw(&self) {
        println!("绘制 {} x {} 的矩形", self.width, self.height);
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Draw for Triangle {
    fn draw(&self) {
        println!("绘制底边 {} 高 {} 的三角形", self.base, self.height);
    }
}

// 3. 使用 trait 对象的结构体
struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn new() -> Self {
        Screen {
            components: Vec::new(),
        }
    }
    
    fn add_component(&mut self, component: Box<dyn Draw>) {
        self.components.push(component);
    }
    
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// 4. 对象安全的 trait
trait Animal {
    fn name(&self) -> &str;
    fn sound(&self) -> &str;
    
    // 默认实现
    fn make_sound(&self) {
        println!("{} 发出 {} 的声音", self.name(), self.sound());
    }
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn sound(&self) -> &str {
        "汪汪"
    }
}

struct Cat {
    name: String,
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn sound(&self) -> &str {
        "喵喵"
    }
}

// 5. 不是对象安全的 trait 示例
trait Clone2 {
    fn clone(&self) -> Self; // 返回 Self，不是对象安全的
}

trait Iterator2 {
    type Item;
    fn next(&mut self) -> Option<Self::Item>; // 关联类型，不是对象安全的
}

// 6. 对象安全的 trait（修改版本）
trait Cloneable {
    fn clone_box(&self) -> Box<dyn Cloneable>;
}

impl Cloneable for Circle {
    fn clone_box(&self) -> Box<dyn Cloneable> {
        Box::new(Circle { radius: self.radius })
    }
}

impl Cloneable for Rectangle {
    fn clone_box(&self) -> Box<dyn Cloneable> {
        Box::new(Rectangle {
            width: self.width,
            height: self.height,
        })
    }
}

// 7. 复杂的 trait 对象使用
trait Processor {
    fn process(&self, data: &str) -> String;
    fn name(&self) -> &str;
}

struct UpperCaseProcessor;

impl Processor for UpperCaseProcessor {
    fn process(&self, data: &str) -> String {
        data.to_uppercase()
    }
    
    fn name(&self) -> &str {
        "大写处理器"
    }
}

struct ReverseProcessor;

impl Processor for ReverseProcessor {
    fn process(&self, data: &str) -> String {
        data.chars().rev().collect()
    }
    
    fn name(&self) -> &str {
        "反转处理器"
    }
}

struct LengthProcessor;

impl Processor for LengthProcessor {
    fn process(&self, data: &str) -> String {
        format!("长度: {}", data.len())
    }
    
    fn name(&self) -> &str {
        "长度处理器"
    }
}

// 8. 处理器链
struct ProcessorChain {
    processors: Vec<Box<dyn Processor>>,
}

impl ProcessorChain {
    fn new() -> Self {
        ProcessorChain {
            processors: Vec::new(),
        }
    }
    
    fn add_processor(&mut self, processor: Box<dyn Processor>) {
        self.processors.push(processor);
    }
    
    fn process(&self, mut data: String) -> String {
        for processor in &self.processors {
            println!("使用 {} 处理", processor.name());
            data = processor.process(&data);
            println!("结果: {}", data);
        }
        data
    }
}

// 9. 带生命周期的 trait 对象
trait Display2<'a> {
    fn display(&self) -> &'a str;
}

struct Message<'a> {
    content: &'a str,
}

impl<'a> Display2<'a> for Message<'a> {
    fn display(&self) -> &'a str {
        self.content
    }
}

// 10. 智能指针与 trait 对象
use std::rc::Rc;
use std::sync::Arc;

trait Shareable {
    fn share_info(&self) -> String;
}

impl Shareable for String {
    fn share_info(&self) -> String {
        format!("共享字符串: {}", self)
    }
}

impl Shareable for i32 {
    fn share_info(&self) -> String {
        format!("共享数字: {}", self)
    }
}

// 11. 错误处理与 trait 对象
trait CustomError: std::error::Error + Send + Sync {
    fn error_code(&self) -> i32;
}

#[derive(Debug)]
struct NetworkError {
    message: String,
}

impl std::fmt::Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "网络错误: {}", self.message)
    }
}

impl std::error::Error for NetworkError {}

impl CustomError for NetworkError {
    fn error_code(&self) -> i32 {
        1001
    }
}

#[derive(Debug)]
struct DatabaseError {
    message: String,
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "数据库错误: {}", self.message)
    }
}

impl std::error::Error for DatabaseError {}

impl CustomError for DatabaseError {
    fn error_code(&self) -> i32 {
        2001
    }
}

// 12. 回调函数与 trait 对象
trait EventHandler {
    fn handle(&self, event: &str);
}

struct Logger;

impl EventHandler for Logger {
    fn handle(&self, event: &str) {
        println!("[LOG] {}", event);
    }
}

struct EmailNotifier {
    email: String,
}

impl EventHandler for EmailNotifier {
    fn handle(&self, event: &str) {
        println!("[EMAIL to {}] {}", self.email, event);
    }
}

struct EventSystem {
    handlers: Vec<Box<dyn EventHandler>>,
}

impl EventSystem {
    fn new() -> Self {
        EventSystem {
            handlers: Vec::new(),
        }
    }
    
    fn add_handler(&mut self, handler: Box<dyn EventHandler>) {
        self.handlers.push(handler);
    }
    
    fn trigger_event(&self, event: &str) {
        for handler in &self.handlers {
            handler.handle(event);
        }
    }
}

fn main() {
    println!("=== 第10章：trait 对象 ===\n");
    
    // 1. 基本的 trait 对象使用
    println!("1. 基本的 trait 对象使用：");
    let mut screen = Screen::new();
    
    screen.add_component(Box::new(Circle { radius: 5.0 }));
    screen.add_component(Box::new(Rectangle { width: 10.0, height: 20.0 }));
    screen.add_component(Box::new(Triangle { base: 8.0, height: 6.0 }));
    
    screen.run();
    println!();
    
    // 2. 动物 trait 对象
    println!("2. 动物 trait 对象：");
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog { name: "旺财".to_string() }),
        Box::new(Cat { name: "小咪".to_string() }),
    ];
    
    for animal in animals {
        animal.make_sound();
    }
    println!();
    
    // 3. 处理器链示例
    println!("3. 处理器链示例：");
    let mut chain = ProcessorChain::new();
    
    chain.add_processor(Box::new(UpperCaseProcessor));
    chain.add_processor(Box::new(ReverseProcessor));
    chain.add_processor(Box::new(LengthProcessor));
    
    let input = "Hello, Rust!".to_string();
    println!("输入: {}", input);
    let result = chain.process(input);
    println!("最终结果: {}", result);
    println!();
    
    // 4. 函数参数中的 trait 对象
    println!("4. 函数参数中的 trait 对象：");
    
    fn draw_shape(shape: &dyn Draw) {
        shape.draw();
    }
    
    let circle = Circle { radius: 3.0 };
    let rectangle = Rectangle { width: 5.0, height: 7.0 };
    
    draw_shape(&circle);
    draw_shape(&rectangle);
    println!();
    
    // 5. 返回 trait 对象
    println!("5. 返回 trait 对象：");
    
    fn create_shape(shape_type: &str) -> Box<dyn Draw> {
        match shape_type {
            "circle" => Box::new(Circle { radius: 1.0 }),
            "rectangle" => Box::new(Rectangle { width: 2.0, height: 3.0 }),
            "triangle" => Box::new(Triangle { base: 4.0, height: 5.0 }),
            _ => Box::new(Circle { radius: 0.0 }),
        }
    }
    
    let shapes = vec!["circle", "rectangle", "triangle"];
    for shape_type in shapes {
        let shape = create_shape(shape_type);
        println!("创建的形状: ");
        shape.draw();
    }
    println!();
    
    // 6. 智能指针与 trait 对象
    println!("6. 智能指针与 trait 对象：");
    
    let shared_items: Vec<Rc<dyn Shareable>> = vec![
        Rc::new(String::from("Hello")),
        Rc::new(42),
    ];
    
    for item in shared_items {
        println!("{}", item.share_info());
        let cloned = Rc::clone(&item);
        println!("引用计数: {}", Rc::strong_count(&cloned));
    }
    println!();
    
    // 7. 错误处理与 trait 对象
    println!("7. 错误处理与 trait 对象：");
    
    fn simulate_operations() -> Vec<Box<dyn CustomError>> {
        vec![
            Box::new(NetworkError {
                message: "连接超时".to_string(),
            }),
            Box::new(DatabaseError {
                message: "查询失败".to_string(),
            }),
        ]
    }
    
    let errors = simulate_operations();
    for error in errors {
        println!("错误: {}", error);
        println!("错误代码: {}", error.error_code());
        println!("调试信息: {:?}", error);
    }
    println!();
    
    // 8. 事件系统示例
    println!("8. 事件系统示例：");
    let mut event_system = EventSystem::new();
    
    event_system.add_handler(Box::new(Logger));
    event_system.add_handler(Box::new(EmailNotifier {
        email: "admin@example.com".to_string(),
    }));
    
    event_system.trigger_event("用户登录");
    event_system.trigger_event("系统启动");
    println!();
    
    // 9. trait 对象的性能考虑
    println!("9. trait 对象的性能考虑：");
    
    // 静态分发（编译时确定）
    fn static_dispatch<T: Draw>(shape: &T) {
        shape.draw();
    }
    
    // 动态分发（运行时确定）
    fn dynamic_dispatch(shape: &dyn Draw) {
        shape.draw();
    }
    
    let circle = Circle { radius: 2.0 };
    
    println!("静态分发:");
    static_dispatch(&circle);
    
    println!("动态分发:");
    dynamic_dispatch(&circle);
    println!();
    
    // 10. trait 对象的限制
    println!("10. trait 对象的限制：");
    
    // 不能使用关联类型的 trait
    // let iter: Box<dyn Iterator2> = ...; // 编译错误
    
    // 不能使用返回 Self 的方法
    // let cloneable: Box<dyn Clone2> = ...; // 编译错误
    
    // 但可以使用对象安全的替代方案
    let shapes: Vec<Box<dyn Cloneable>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Rectangle { width: 2.0, height: 3.0 }),
    ];
    
    let cloned_shapes: Vec<Box<dyn Cloneable>> = shapes
        .iter()
        .map(|shape| shape.clone_box())
        .collect();
    
    println!("克隆了 {} 个形状", cloned_shapes.len());
    println!();
    
    // 11. 复杂的 trait 对象组合
    println!("11. 复杂的 trait 对象组合：");
    
    trait Drawable: Draw + Debug {
        fn area(&self) -> f64;
    }
    
    #[derive(Debug)]
    struct Square {
        side: f64,
    }
    
    impl Draw for Square {
        fn draw(&self) {
            println!("绘制边长为 {} 的正方形", self.side);
        }
    }
    
    impl Drawable for Square {
        fn area(&self) -> f64 {
            self.side * self.side
        }
    }
    
    let square = Square { side: 4.0 };
    let drawable: &dyn Drawable = &square;
    
    drawable.draw();
    println!("面积: {}", drawable.area());
    println!("调试信息: {:?}", drawable);
    println!();
    
    // 12. trait 对象与闭包
    println!("12. trait 对象与闭包：");
    
    let processors: Vec<Box<dyn Fn(&str) -> String>> = vec![
        Box::new(|s| s.to_uppercase()),
        Box::new(|s| s.chars().rev().collect()),
        Box::new(|s| format!("处理后: {}", s)),
    ];
    
    let mut data = "hello".to_string();
    for (i, processor) in processors.iter().enumerate() {
        data = processor(&data);
        println!("步骤 {}: {}", i + 1, data);
    }
    
    println!("\n=== 第10章完成 ===");
}

// 13. trait 对象的高级用法
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trait_object_equality() {
        // trait 对象不能直接比较相等性
        // 但可以通过其他方式实现
        
        trait Identifiable {
            fn id(&self) -> u32;
        }
        
        struct Item {
            id: u32,
            name: String,
        }
        
        impl Identifiable for Item {
            fn id(&self) -> u32 {
                self.id
            }
        }
        
        let item1 = Item { id: 1, name: "Item1".to_string() };
        let item2 = Item { id: 2, name: "Item2".to_string() };
        
        let objects: Vec<&dyn Identifiable> = vec![&item1, &item2];
        
        // 通过 ID 比较
        assert_eq!(objects[0].id(), 1);
        assert_eq!(objects[1].id(), 2);
    }
    
    #[test]
    fn test_trait_object_downcasting() {
        use std::any::Any;
        
        trait MyTrait: Any {
            fn as_any(&self) -> &dyn Any;
        }
        
        struct MyStruct {
            value: i32,
        }
        
        impl MyTrait for MyStruct {
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
        
        let my_struct = MyStruct { value: 42 };
        let trait_object: &dyn MyTrait = &my_struct;
        
        // 向下转型
        if let Some(concrete) = trait_object.as_any().downcast_ref::<MyStruct>() {
            assert_eq!(concrete.value, 42);
        }
    }
} 