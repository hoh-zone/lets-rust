# 第3章：所有权机制

## 3.1 目的与核心思想

### 为什么需要所有权？

传统的内存管理方式主要有两种：

1. **手动管理**（如 C/C++）：程序员负责分配和释放内存
   - 优点：灵活、高效
   - 缺点：容易出错（内存泄漏、野指针、双重释放等）

2. **垃圾回收**（如 Java、Python）：运行时自动管理内存
   - 优点：安全、方便
   - 缺点：运行时开销、不可预测的暂停

Rust 选择了第三条路：**所有权系统**
- 在编译时检查内存安全
- 没有运行时开销
- 防止内存错误和数据竞争

### 核心思想

所有权系统的核心思想是：**每个值都有一个所有者，并且同时只能有一个所有者**。

```rust
fn main() {
    // s 是 "hello" 的所有者
    let s = String::from("hello");
    
    // 当所有者离开作用域时，值会被自动清理
} // s 离开作用域，内存被释放
```

### 内存安全保证

所有权系统在编译时防止以下问题：

```rust
// 1. 使用后释放（Use After Free）
fn use_after_free_prevented() {
    let s = String::from("hello");
    let r = &s;
    drop(s);  // 显式释放 s
    // println!("{}", r);  // 编译错误！不能使用已释放的引用
}

// 2. 双重释放（Double Free）
fn double_free_prevented() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 的所有权移动到 s2
    // drop(s1);  // 编译错误！s1 不再拥有值
    drop(s2);     // 只有 s2 能释放内存
}

// 3. 空悬指针（Dangling Pointer）
fn no_dangling_pointer() -> &'static str {
    let s = String::from("hello");
    // &s  // 编译错误！不能返回局部变量的引用
    "hello"  // 返回静态字符串引用是安全的
}
```

## 3.2 所有权规则

Rust 的所有权有三条基本规则：

1. **每个值都有一个所有者**
2. **值在任一时刻只能有一个所有者**
3. **当所有者离开作用域时，值被丢弃**

### 规则一：每个值都有一个所有者

```rust
fn main() {
    let x = 5;           // x 拥有值 5
    let s = String::from("hello");  // s 拥有字符串 "hello"
    let v = vec![1, 2, 3];         // v 拥有向量
    
    // 复合类型
    struct Person {
        name: String,
        age: u32,
    }
    
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };  // person 拥有整个结构体，包括其字段
}
```

### 规则二：值在任一时刻只能有一个所有者

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // 所有权从 s1 移动到 s2
    
    // println!("{}", s1);  // 编译错误！s1 不再有效
    println!("{}", s2);     // 正常工作
}
```

#### 移动语义详解

```rust
fn main() {
    // 对于实现了 Copy trait 的类型，赋值是复制
    let x = 5;
    let y = x;  // x 被复制到 y
    println!("x = {}, y = {}", x, y);  // 两个都可用
    
    // 对于堆分配的类型，赋值是移动
    let s1 = String::from("hello");
    let s2 = s1;  // s1 被移动到 s2
    
    // 函数调用也会发生移动
    let s = String::from("world");
    takes_ownership(s);  // s 的所有权移动到函数
    // println!("{}", s);  // 编译错误！
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}  // some_string 离开作用域，内存被释放
```

### 规则三：当所有者离开作用域时，值被丢弃

```rust
fn main() {
    {
        let s = String::from("hello");  // s 在这里有效
        // 使用 s
    }  // s 离开作用域，drop 函数被自动调用
    
    // println!("{}", s);  // 编译错误！s 不在作用域内
}
```

#### Drop trait

当值离开作用域时，Rust 会自动调用 `drop` 函数：

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}  // d 先被 drop，然后是 c（LIFO 顺序）
```

## 3.3 验证规则

### Copy 和 Clone

#### Copy trait

一些类型实现了 `Copy` trait，赋值时会复制而不是移动：

```rust
fn main() {
    // 所有整数类型都是 Copy
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);  // 都可用
    
    // 浮点数、布尔值、字符也是 Copy
    let a = 3.14;
    let b = a;
    
    let flag1 = true;
    let flag2 = flag1;
    
    let ch1 = 'A';
    let ch2 = ch1;
    
    // 元组（如果所有元素都是 Copy）
    let tup1 = (1, 2.0, true);
    let tup2 = tup1;
    println!("{:?}", tup1);  // 仍然可用
}
```

#### Clone trait

对于需要深拷贝的类型，使用 `clone` 方法：

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();  // 深拷贝
    
    println!("s1 = {}, s2 = {}", s1, s2);  // 都可用
    
    // 自定义类型实现 Clone
    #[derive(Clone)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p1 = Point { x: 10, y: 20 };
    let p2 = p1.clone();
    
    println!("p1: ({}, {})", p1.x, p1.y);
    println!("p2: ({}, {})", p2.x, p2.y);
}
```

### 所有权和函数

```rust
fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);  // 传递引用，不转移所有权
    println!("'{}' 的长度是 {}", s, len);  // s 仍然可用
    
    let s2 = String::from("world");
    let s3 = take_and_return(s2);  // s2 的所有权被转移并返回
    // println!("{}", s2);  // 错误！
    println!("{}", s3);     // s3 拥有所有权
}

fn calculate_length(s: &String) -> usize {
    s.len()  // 不拥有所有权，只是借用
}

fn take_and_return(s: String) -> String {
    s  // 返回所有权
}
```

### 所有权链

```rust
fn main() {
    let s1 = give_ownership();         // give_ownership 移动返回值到 s1
    let s2 = String::from("hello");    // s2 进入作用域
    let s3 = takes_and_gives_back(s2); // s2 被移动到函数，返回值移动到 s3
    
    println!("{}", s1);
    // println!("{}", s2);  // 错误！s2 已被移动
    println!("{}", s3);
}

fn give_ownership() -> String {
    let some_string = String::from("yours");
    some_string  // 返回并移动所有权
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // 返回并移动所有权
}
```

### 部分移动

结构体可以部分移动：

```rust
fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }
    
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    // 移动 name 字段
    let name = person.name;
    
    // person 部分移动了，不能整体使用
    // println!("{:?}", person);  // 错误！
    
    // 但可以使用未移动的字段
    println!("Age: {}", person.age);  // 正常
    
    // 解构时的部分移动
    let person2 = Person {
        name: String::from("Bob"),
        age: 25,
    };
    
    let Person { name, age } = person2;
    println!("Name: {}, Age: {}", name, age);
    // person2 完全移动了
}
```

### 验证所有权的例子

```rust
// 编译器如何验证所有权
fn ownership_validation() {
    let mut s = String::from("hello");
    
    let r1 = &s;      // 不可变借用
    let r2 = &s;      // 可以有多个不可变借用
    println!("{} and {}", r1, r2);
    // r1 和 r2 作用域结束
    
    let r3 = &mut s;  // 可变借用
    r3.push_str(" world");
    println!("{}", r3);
    // r3 作用域结束
    
    // 现在可以再次借用
    let r4 = &s;
    println!("{}", r4);
}

// 常见的所有权错误模式
fn common_ownership_mistakes() {
    // 1. 尝试使用已移动的值
    let v = vec![1, 2, 3];
    let v2 = v;
    // v.push(4);  // 错误！v 已被移动
    
    // 2. 在循环中移动
    let v = vec![String::from("a"), String::from("b")];
    // for s in v {  // v 被移动
    //     println!("{}", s);
    // }
    // println!("{:?}", v);  // 错误！
    
    // 正确做法：借用
    for s in &v {
        println!("{}", s);
    }
    println!("{:?}", v);  // 现在可以使用
    
    // 3. 从函数返回引用
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s  // 错误！返回局部变量的引用
    // }
}
```

### 所有权的好处

1. **内存安全**：编译时防止内存错误
2. **并发安全**：防止数据竞争
3. **零成本**：没有运行时开销
4. **可预测**：确定性的资源管理

```rust
// 展示所有权如何防止常见错误
fn ownership_benefits() {
    // 自动内存管理
    {
        let v = vec![1, 2, 3, 4, 5];
        // 使用 v
    }  // v 自动被清理，不会泄漏
    
    // 防止迭代器失效
    let mut v = vec![1, 2, 3];
    // let first = &v[0];
    // v.push(4);  // 错误！不能在借用时修改
    // println!("{}", first);
    
    // RAII 模式
    use std::fs::File;
    {
        let f = File::open("hello.txt");
        // 使用文件
    }  // 文件自动关闭
}
```

## 小结

本章深入学习了 Rust 的所有权机制：

1. **核心思想**：每个值有唯一的所有者，离开作用域时自动清理
2. **三条规则**：理解并应用所有权的基本规则
3. **验证规则**：掌握 Copy、Clone、移动语义等概念

所有权是 Rust 最独特的特性，它使 Rust 能够在没有垃圾回收的情况下保证内存安全。下一章我们将学习借用机制，它让我们能够在不转移所有权的情况下使用值。 