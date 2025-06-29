# 第8章：泛型与特征

泛型（Generics）和特征（Traits）是 Rust 实现代码复用和抽象的核心机制。

## 8.1 泛型数据类型

### 泛型函数

```rust
// 没有泛型的版本
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

// 使用泛型
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("最大的数字是 {}", result);
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("最大的字符是 {}", result);
}
```

### 泛型结构体

```rust
// 单个泛型参数
struct Point<T> {
    x: T,
    y: T,
}

// 多个泛型参数
struct Point2<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    
    // 不同类型
    let mixed = Point2 { x: 5, y: 4.0 };
    
    println!("integer.x = {}", integer.x);
    println!("float.y = {}", float.y);
    println!("mixed: ({}, {})", mixed.x, mixed.y);
}
```

### 泛型枚举

```rust
// Option<T> 是泛型枚举
enum Option<T> {
    Some(T),
    None,
}

// Result<T, E> 有两个泛型参数
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 自定义泛型枚举
enum Tree<T> {
    Empty,
    Node(T, Box<Tree<T>>, Box<Tree<T>>),
}

impl<T> Tree<T> {
    fn new() -> Self {
        Tree::Empty
    }
    
    fn leaf(value: T) -> Self {
        Tree::Node(value, Box::new(Tree::Empty), Box::new(Tree::Empty))
    }
}
```

### 泛型方法

```rust
struct Point<T> {
    x: T,
    y: T,
}

// 为所有 T 实现方法
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    
    fn x(&self) -> &T {
        &self.x
    }
}

// 只为特定类型实现方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 混合泛型
impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p = Point::new(5, 10);
    println!("p.x = {}", p.x());
    
    let p2 = Point::new(3.0_f32, 4.0_f32);
    println!("距离原点: {}", p2.distance_from_origin());
    
    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

### 泛型的性能

```rust
// 泛型没有运行时开销（单态化）
fn generic_function<T>(x: T) -> T {
    x
}

fn main() {
    // 编译器会为每个具体类型生成代码
    let integer = generic_function(5);      // 生成 i32 版本
    let float = generic_function(3.14);     // 生成 f64 版本
    let string = generic_function("hello"); // 生成 &str 版本
    
    // 相当于编译器生成了：
    // fn generic_function_i32(x: i32) -> i32 { x }
    // fn generic_function_f64(x: f64) -> f64 { x }
    // fn generic_function_str(x: &str) -> &str { x }
}
```

## 8.2 特征

### 定义特征

```rust
// 定义一个特征
trait Summary {
    fn summarize(&self) -> String;
}

// 带有默认实现的特征
trait Display {
    fn fmt(&self) -> String {
        String::from("(默认格式)")
    }
}

// 关联类型的特征
trait Iterator {
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
}
```

### 实现特征

```rust
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
}

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
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("当然，正如你可能已经知道的，人们"),
        reply: false,
        retweet: false,
    };
    
    println!("1 条新推文: {}", tweet.summarize());
}
```

### 默认实现

```rust
trait Summary {
    fn summarize_author(&self) -> String;
    
    // 默认实现可以调用其他方法
    fn summarize(&self) -> String {
        format!("(阅读更多来自 {} 的内容...)", self.summarize_author())
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // 使用默认的 summarize 实现
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("当然，正如你可能已经知道的，人们"),
        reply: false,
        retweet: false,
    };
    
    println!("摘要: {}", tweet.summarize());
}
```

### 特征作为参数

```rust
// 使用 impl Trait 语法
fn notify(item: &impl Summary) {
    println!("突发新闻！{}", item.summarize());
}

// 特征约束语法（与上面等价）
fn notify2<T: Summary>(item: &T) {
    println!("突发新闻！{}", item.summarize());
}

// 多个特征约束
fn notify3(item: &(impl Summary + Display)) {
    println!("突发新闻！{}", item.summarize());
}

// 使用 where 子句
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // 函数体
}
```

### 返回实现特征的类型

```rust
// 返回实现了特征的类型
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("当然，正如你可能已经知道的，人们"),
        reply: false,
        retweet: false,
    }
}

// 注意：只能返回单一类型
// 这个不能编译：
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle { ... }
//     } else {
//         Tweet { ... }
//     }
// }
```

### 常用特征

```rust
use std::fmt::{self, Display};

// Display 特征
struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Clone 特征
#[derive(Clone)]
struct Circle {
    center: Point,
    radius: f64,
}

// PartialEq 特征
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("点: {}", p);  // 使用 Display
    
    let c1 = Circle { center: p.clone(), radius: 10.0 };
    let c2 = c1.clone();  // 使用 Clone
    
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    println!("p1 == p2: {}", p1 == p2);  // 使用 PartialEq
}
```

### 派生特征

```rust
// 使用 derive 自动实现特征
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Priority(i32);

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    // Debug
    println!("{:?}", person);
    
    // Clone
    let person2 = person.clone();
    
    // PartialEq
    println!("相等？{}", person == person2);
    
    // 优先级比较
    let p1 = Priority(1);
    let p2 = Priority(2);
    println!("p1 < p2: {}", p1 < p2);
}
```

## 8.3 用特征约束泛型

### 特征约束

```rust
use std::fmt::Display;

// 单个特征约束
fn print_it<T: Display>(item: T) {
    println!("{}", item);
}

// 多个特征约束
fn compare_and_display<T: Display + PartialOrd>(a: T, b: T) {
    if a > b {
        println!("{} 大于 {}", a, b);
    } else {
        println!("{} 小于等于 {}", a, b);
    }
}

// 使用 where 子句
fn complex_function<T, U, V>(t: T, u: U, v: V) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
    V: Display + Debug,
{
    // 函数实现
    42
}
```

### 有条件地实现方法

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 只有当 T 实现了 Display + PartialOrd 时才实现此方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("最大的成员是 x = {}", self.x);
        } else {
            println!("最大的成员是 y = {}", self.y);
        }
    }
}

fn main() {
    let pair = Pair::new(5, 10);
    pair.cmp_display();
    
    // 对于没有实现 Display + PartialOrd 的类型，cmp_display 不可用
    let pair2 = Pair::new(vec![1], vec![2]);
    // pair2.cmp_display();  // 错误！Vec<i32> 没有实现 Display
}
```

### 覆盖实现（Blanket Implementations）

```rust
// 标准库中的例子
// impl<T: Display> ToString for T {
//     fn to_string(&self) -> String {
//         // ...
//     }
// }

// 自定义覆盖实现
trait MyTrait {
    fn my_method(&self);
}

// 为所有实现了 Display 的类型实现 MyTrait
impl<T: Display> MyTrait for T {
    fn my_method(&self) {
        println!("MyTrait for: {}", self);
    }
}

fn main() {
    let x = 42;
    x.my_method();  // 因为 i32 实现了 Display
    
    let s = "hello";
    s.my_method();  // 因为 &str 实现了 Display
}
```

### 特征对象的限制

```rust
// 静态分发
fn static_dispatch<T: Draw>(item: &T) {
    item.draw();
}

// 动态分发（特征对象）
fn dynamic_dispatch(item: &dyn Draw) {
    item.draw();
}

trait Draw {
    fn draw(&self);
}

struct Button;
impl Draw for Button {
    fn draw(&self) {
        println!("绘制按钮");
    }
}

struct SelectBox;
impl Draw for SelectBox {
    fn draw(&self) {
        println!("绘制选择框");
    }
}

fn main() {
    let button = Button;
    let select = SelectBox;
    
    // 静态分发
    static_dispatch(&button);
    static_dispatch(&select);
    
    // 动态分发
    dynamic_dispatch(&button);
    dynamic_dispatch(&select);
    
    // 特征对象的集合
    let drawables: Vec<Box<dyn Draw>> = vec![
        Box::new(Button),
        Box::new(SelectBox),
    ];
    
    for drawable in &drawables {
        drawable.draw();
    }
}
```

### 高级特征

```rust
// 关联类型
trait Container {
    type Item;
    
    fn add(&mut self, item: Self::Item);
    fn get(&self) -> Option<&Self::Item>;
}

struct MyContainer {
    value: Option<i32>,
}

impl Container for MyContainer {
    type Item = i32;
    
    fn add(&mut self, item: Self::Item) {
        self.value = Some(item);
    }
    
    fn get(&self) -> Option<&Self::Item> {
        self.value.as_ref()
    }
}

// 默认泛型参数
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = p1 + p2;
    
    println!("{:?} + {:?} = {:?}", p1, p2, p3);
}
```

### 特征的继承

```rust
use std::fmt;

// 特征可以依赖其他特征
trait OutlinePrint: fmt::Display {
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

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

fn main() {
    let p = Point { x: 1, y: 3 };
    p.outline_print();
}
```

## 小结

本章学习了 Rust 的泛型和特征系统：

1. **泛型数据类型**：使用泛型参数编写灵活、可复用的代码
2. **特征**：定义共享的行为，类似于其他语言的接口
3. **特征约束**：限制泛型必须实现的功能

泛型和特征是 Rust 类型系统的核心，它们使得我们能够编写既灵活又类型安全的代码。下一章我们将深入学习生命周期，这是 Rust 独特的概念之一。 