// 教学示例 - 允许未使用的代码
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// 第8章：泛型与 trait
// 泛型允许我们编写灵活、可重用的代码，trait 定义共享的行为

use std::fmt::{Debug, Display};
use std::ops::Add;

// 1. 泛型函数
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

// 多个泛型参数
fn compare_and_display<T, U>(x: T, y: U) 
where 
    T: Display + PartialOrd<U>,
    U: Display,
{
    if x > y {
        println!("{} 大于 {}", x, y);
    } else {
        println!("{} 小于等于 {}", x, y);
    }
}

// 2. 泛型结构体
#[derive(Debug, Clone)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    
    fn x(&self) -> &T {
        &self.x
    }
    
    fn y(&self) -> &T {
        &self.y
    }
}

// 为特定类型实现方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 多个泛型参数的结构体
#[derive(Debug)]
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> MixedPoint<T, U> {
    fn mixup<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

// 3. 泛型枚举
#[derive(Debug)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 4. trait 定义
trait Summary {
    // 必须实现的方法
    fn summarize(&self) -> String;
    
    // 默认实现
    fn summarize_author(&self) -> String {
        String::from("(作者未知)")
    }
    
    // 使用默认实现的方法
    fn full_summary(&self) -> String {
        format!("{}，作者：{}", self.summarize(), self.summarize_author())
    }
}

// 实现 trait 的结构体
#[derive(Debug)]
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

#[derive(Debug)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// 5. trait 作为参数
fn notify(item: &impl Summary) {
    println!("突发新闻！{}", item.summarize());
}

// trait bound 语法
fn notify_bound<T: Summary>(item: &T) {
    println!("突发新闻！{}", item.summarize());
}

// 多个 trait bound
fn notify_multiple<T: Summary + Display>(item: &T) {
    println!("突发新闻！{}", item.summarize());
    println!("详细信息：{}", item);
}

// where 子句
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("T: {}", t);
    println!("U: {:?}", u);
    42
}

// 6. 返回实现了 trait 的类型
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("当然，正如你可能已经知道的，人们"),
        reply: false,
        retweet: false,
    }
}

// 7. 有条件地实现方法
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("最大的成员是 x = {}", self.x);
        } else {
            println!("最大的成员是 y = {}", self.y);
        }
    }
}

// 8. 运算符重载
#[derive(Debug, Clone, Copy, PartialEq)]
struct Vector2D {
    x: f64,
    y: f64,
}

impl Add for Vector2D {
    type Output = Vector2D;
    
    fn add(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Display for Vector2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// 9. 关联类型
trait Iterator {
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: usize,
    max: usize,
}

impl Counter {
    fn new(max: usize) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            let current = self.count;
            self.count += 1;
            Some(current)
        } else {
            None
        }
    }
}

// 10. 默认泛型类型参数
trait Add2<Rhs = Self> {
    type Output;
    
    fn add(self, rhs: Rhs) -> Self::Output;
}

// 11. 完全限定语法
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("这里是你的机长在广播。");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("飞起来！");
    }
}

impl Human {
    fn fly(&self) {
        println!("*疯狂挥舞手臂*");
    }
}

// 12. 超 trait
trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point<i32> {}

impl Display for Point<i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// 13. newtype 模式
struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    println!("=== 第8章：泛型与 trait ===\n");
    
    // 1. 泛型函数使用
    println!("1. 泛型函数使用：");
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("最大的数字是 {}", result);
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("最大的字符是 {}", result);
    println!();
    
    // 2. 泛型结构体使用
    println!("2. 泛型结构体使用：");
    let integer_point = Point::new(5, 10);
    let float_point = Point::new(1.0, 4.0);
    
    println!("整数点: {:?}", integer_point);
    println!("浮点数点: {:?}", float_point);
    println!("浮点数点到原点的距离: {}", float_point.distance_from_origin());
    
    let mixed1 = MixedPoint { x: 5, y: 10.4 };
    let mixed2 = MixedPoint { x: "Hello", y: 'c' };
    let mixed3 = mixed1.mixup(mixed2);
    println!("混合点: {:?}", mixed3);
    println!();
    
    // 3. trait 使用
    println!("3. trait 使用：");
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("当然，正如你可能已经知道的，人们"),
        reply: false,
        retweet: false,
    };
    
    let article = NewsArticle {
        headline: String::from("企鹅队赢得了斯坦利杯冠军！"),
        location: String::from("匹兹堡，PA，美国"),
        author: String::from("Iceburgh"),
        content: String::from("匹兹堡企鹅队再次成为了 NHL 斯坦利杯的冠军..."),
    };
    
    println!("1 条新推文：{}", tweet.summarize());
    println!("新文章可用！{}", article.summarize());
    println!("完整摘要：{}", tweet.full_summary());
    println!();
    
    // 4. trait 作为参数
    println!("4. trait 作为参数：");
    notify(&tweet);
    notify_bound(&article);
    println!();
    
    // 5. 返回 trait
    println!("5. 返回 trait：");
    let summarizable = returns_summarizable();
    println!("返回的摘要：{}", summarizable.summarize());
    println!();
    
    // 6. 有条件的方法实现
    println!("6. 有条件的方法实现：");
    let pair = Pair::new(10, 20);
    pair.cmp_display();
    println!();
    
    // 7. 运算符重载
    println!("7. 运算符重载：");
    let v1 = Vector2D { x: 1.0, y: 2.0 };
    let v2 = Vector2D { x: 3.0, y: 4.0 };
    let v3 = v1 + v2;
    println!("{} + {} = {}", v1, v2, v3);
    println!();
    
    // 8. 关联类型
    println!("8. 关联类型：");
    let mut counter = Counter::new(5);
    while let Some(value) = counter.next() {
        println!("计数器值: {}", value);
    }
    println!();
    
    // 9. 完全限定语法
    println!("9. 完全限定语法：");
    let person = Human;
    
    person.fly();                    // 调用 Human 的方法
    Pilot::fly(&person);            // 调用 Pilot trait 的方法
    Wizard::fly(&person);           // 调用 Wizard trait 的方法
    println!();
    
    // 10. 超 trait
    println!("10. 超 trait：");
    let point = Point::new(3, 5);
    point.outline_print();
    println!();
    
    // 11. newtype 模式
    println!("11. newtype 模式：");
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("包装器: {}", w);
    println!();
    
    // 12. 泛型性能测试
    println!("12. 泛型性能测试：");
    
    // 单态化：编译器为每种具体类型生成专门的代码
    fn generic_function<T: Add<Output = T> + Copy>(x: T, y: T) -> T {
        x + y
    }
    
    let int_result = generic_function(5, 10);
    let float_result = generic_function(5.5, 10.5);
    
    println!("整数相加: {}", int_result);
    println!("浮点数相加: {}", float_result);
    println!();
    
    // 13. trait 对象
    println!("13. trait 对象：");
    let items: Vec<Box<dyn Summary>> = vec![
        Box::new(tweet),
        Box::new(article),
    ];
    
    for item in items {
        println!("动态分发：{}", item.summarize());
    }
    println!();
    
    // 14. 生命周期与泛型
    println!("14. 生命周期与泛型：");
    
    fn longest<'a, T>(x: &'a T, y: &'a T) -> &'a T 
    where 
        T: PartialOrd
    {
        if x > y { x } else { y }
    }
    
    let num1 = 10;
    let num2 = 20;
    let result = longest(&num1, &num2);
    println!("较大的数字: {}", result);
    
    // 15. 类型别名
    println!("15. 类型别名：");
    type Kilometers = i32;
    type Thunk = Box<dyn Fn() + Send + 'static>;
    
    let distance: Kilometers = 100;
    println!("距离: {} 公里", distance);
    
    let f: Thunk = Box::new(|| println!("这是一个 thunk"));
    f();
    
    println!("\n=== 第8章完成 ===");
} 