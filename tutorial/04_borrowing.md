# 第4章：借用机制

## 4.1 引用与借用规则

### 什么是借用？

借用（Borrowing）是 Rust 中一个核心概念，它允许你访问数据而不获取其所有权。通过引用来实现借用。

```rust
fn main() {
    let s1 = String::from("hello");
    
    // 创建一个引用，不获取所有权
    let len = calculate_length(&s1);
    
    // s1 仍然有效，因为我们只是借用了它
    println!("'{}' 的长度是 {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
    // s 离开作用域，但因为它没有所有权，所以什么都不会发生
}
```

### 引用的基本规则

Rust 的借用规则确保内存安全：

1. **在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用**
2. **引用必须总是有效的**

```rust
fn main() {
    let mut s = String::from("hello");
    
    // 多个不可变引用是允许的
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // r1 和 r2 的作用域在这里结束
    
    // 可变引用
    let r3 = &mut s;
    r3.push_str(", world");
    println!("{}", r3);
}
```

### 不可变引用

不可变引用允许读取数据但不能修改：

```rust
fn main() {
    let s = String::from("hello");
    
    // 创建不可变引用
    let r1 = &s;
    let r2 = &s;  // 可以有多个不可变引用
    
    println!("r1: {}, r2: {}", r1, r2);
    
    // 尝试通过不可变引用修改会报错
    // r1.push_str(" world");  // 错误！不能通过不可变引用修改
}
```

### 可变引用

可变引用允许修改借用的数据：

```rust
fn main() {
    let mut s = String::from("hello");
    
    // 创建可变引用
    let r = &mut s;
    r.push_str(", world");
    
    println!("{}", r);
    
    // 同一时间只能有一个可变引用
    // let r2 = &mut s;  // 错误！已经有一个可变引用了
}
```

### 引用的作用域

引用的作用域从声明开始，到最后一次使用结束：

```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s;     // r1 的作用域开始
    let r2 = &s;     // r2 的作用域开始
    println!("{} and {}", r1, r2);
    // r1 和 r2 的作用域在这里结束
    
    let r3 = &mut s; // r3 的作用域开始，现在可以创建可变引用
    r3.push_str(", world");
    println!("{}", r3);
} // r3 的作用域结束
```

## 4.2 验证借用规则

### 规则一：不可变引用和可变引用不能同时存在

```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s;     // 不可变引用
    let r2 = &s;     // 不可变引用
    // let r3 = &mut s; // 错误！不能在有不可变引用时创建可变引用
    
    println!("{} and {}", r1, r2);
    // r1 和 r2 不再使用
    
    let r3 = &mut s; // 现在可以创建可变引用
    println!("{}", r3);
}
```

### 规则二：同一时间只能有一个可变引用

```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &mut s;
    // let r2 = &mut s; // 错误！不能同时有两个可变引用
    
    println!("{}", r1);
    
    // r1 作用域结束后，可以创建新的可变引用
    let r2 = &mut s;
    println!("{}", r2);
}
```

### 数据竞争的预防

借用规则防止了数据竞争。数据竞争发生在：
- 两个或更多指针同时访问同一数据
- 至少有一个指针用于写入数据
- 没有同步数据访问的机制

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    
    // 这会导致数据竞争（在其他语言中）
    // let first = &v[0];      // 不可变引用
    // v.push(6);              // 可变操作，可能导致重新分配
    // println!("{}", first);  // 使用可能已失效的引用
    
    // Rust 编译器会阻止这种情况
    let first = &v[0];
    // v.push(6);  // 编译错误！不能在有不可变引用时修改
    println!("{}", first);
}
```

### 引用的引用

可以创建引用的引用：

```rust
fn main() {
    let s = String::from("hello");
    let r1 = &s;      // &String
    let r2 = &r1;     // &&String
    let r3 = &r2;     // &&&String
    
    // 自动解引用
    println!("s: {}", s);
    println!("r1: {}", r1);
    println!("r2: {}", r2);  // 自动解引用
    println!("r3: {}", r3);  // 自动解引用
    
    // 显式解引用
    println!("*r1: {}", *r1);
    println!("**r2: {}", **r2);
    println!("***r3: {}", ***r3);
}
```

## 4.3 切片

切片（Slice）是对集合中一部分连续元素的引用。

### 字符串切片

```rust
fn main() {
    let s = String::from("hello world");
    
    // 字符串切片
    let hello = &s[0..5];   // 或 &s[..5]
    let world = &s[6..11];  // 或 &s[6..]
    let whole = &s[..];     // 整个字符串的切片
    
    println!("hello: {}", hello);
    println!("world: {}", world);
    println!("whole: {}", whole);
}
```

### 字符串切片的类型

```rust
fn main() {
    let s = String::from("hello world");
    
    // &str 是字符串切片的类型
    let word: &str = first_word(&s);
    println!("第一个单词是: {}", word);
    
    // 字符串字面量就是切片
    let s2 = "Hello, world!";  // s2 的类型是 &str
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}
```

### 改进的函数签名

```rust
// 更通用的函数签名
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

fn main() {
    let my_string = String::from("hello world");
    
    // first_word 适用于 String 的切片
    let word = first_word(&my_string[..]);
    
    let my_string_literal = "hello world";
    
    // first_word 也适用于字符串字面量
    let word = first_word(my_string_literal);
    
    println!("第一个单词: {}", word);
}
```

### 数组切片

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    
    // 数组切片
    let slice = &a[1..3];  // 包含索引 1 和 2 的元素
    
    println!("slice: {:?}", slice);  // 输出: [2, 3]
    
    // 使用切片
    let sum = sum_slice(&a[..]);  // 传递整个数组的切片
    println!("总和: {}", sum);
}

fn sum_slice(slice: &[i32]) -> i32 {
    let mut sum = 0;
    for &item in slice {
        sum += item;
    }
    sum
}
```

### 可变切片

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    
    // 可变切片
    let slice = &mut v[2..4];
    slice[0] = 10;  // 修改第一个元素（原来的 v[2]）
    slice[1] = 20;  // 修改第二个元素（原来的 v[3]）
    
    println!("v: {:?}", v);  // 输出: [1, 2, 10, 20, 5]
}
```

## 4.4 悬垂引用

Rust 的借用检查器确保引用永远不会成为悬垂引用。

### 什么是悬垂引用？

悬垂引用指向的内存可能已经被分配给其他人：

```rust
// 这个函数试图返回悬垂引用
// fn dangle() -> &String {  // 错误！
//     let s = String::from("hello");
//     &s  // 返回 s 的引用
// }  // s 离开作用域并被丢弃，其内存被释放

// 正确的做法：返回 String
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // 所有权被移出
}

fn main() {
    let string = no_dangle();
    println!("{}", string);
}
```

### 生命周期基础

编译器通过生命周期来追踪引用的有效性：

```rust
fn main() {
    let r;                // r 的生命周期开始
    
    {
        let x = 5;        // x 的生命周期开始
        r = &x;           // 错误！x 的生命周期太短
    }                     // x 的生命周期结束
    
    // println!("r: {}", r); // r 是悬垂引用
}
```

### 函数中的生命周期

```rust
// 显式生命周期标注
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("最长的字符串是: {}", result);
}
```

### 结构体中的引用

```rust
// 结构体中存储引用需要生命周期标注
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("重要片段: {}", i.part);
}
```

### 静态生命周期

```rust
fn main() {
    // 字符串字面量有 'static 生命周期
    let s: &'static str = "I have a static lifetime.";
    
    // 可以在整个程序运行期间使用
    println!("{}", s);
}

// 返回静态生命周期的引用
fn get_static_str() -> &'static str {
    "This string lives forever!"
}
```

### 借用检查器的工作原理

```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s;      // 不可变借用开始
    let r2 = &s;      // 另一个不可变借用
    println!("{} and {}", r1, r2);
    // r1 和 r2 的最后使用
    
    let r3 = &mut s;  // 可变借用开始（之前的借用已结束）
    r3.push_str(", world");
    println!("{}", r3);
}
```

### 复杂的借用场景

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    
    // 同时借用不同部分是允许的
    let (first_half, second_half) = v.split_at_mut(3);
    first_half[0] = 10;
    second_half[0] = 20;
    
    println!("v: {:?}", v);  // 输出: [10, 2, 3, 20, 5]
}

// 迭代器和借用
fn iterate_example() {
    let v = vec![1, 2, 3, 4, 5];
    
    // 不可变借用进行迭代
    for i in &v {
        println!("{}", i);
    }
    
    // v 仍然可用
    println!("v: {:?}", v);
    
    let mut v = vec![1, 2, 3, 4, 5];
    
    // 可变借用进行迭代
    for i in &mut v {
        *i *= 2;
    }
    
    println!("v: {:?}", v);  // 输出: [2, 4, 6, 8, 10]
}
```

## 小结

本章深入学习了 Rust 的借用机制：

1. **引用与借用规则**：理解不可变引用和可变引用的使用规则
2. **验证借用规则**：掌握借用检查器如何保证内存安全
3. **切片**：学习字符串切片和数组切片的使用
4. **悬垂引用**：理解 Rust 如何防止悬垂引用

借用机制是 Rust 实现内存安全的关键特性之一，它允许我们在不转移所有权的情况下安全地访问数据。下一章我们将学习结构体，这是组织相关数据的重要方式。 