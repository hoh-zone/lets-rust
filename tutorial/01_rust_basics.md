# 第1章：Rust语言基础

## 1.1 变量与常量

### 变量声明

在 Rust 中，使用 `let` 关键字声明变量：

```rust
fn main() {
    let x = 5;
    println!("x 的值是: {}", x);
}
```

### 不可变性

Rust 中的变量默认是**不可变的（immutable）**。这是 Rust 安全性的重要特性之一：

```rust
fn main() {
    let x = 5;
    println!("x 的值是: {}", x);
    
    // 下面这行会报错：cannot assign twice to immutable variable
    // x = 6;
}
```

### 可变变量

如果需要改变变量的值，使用 `mut` 关键字：

```rust
fn main() {
    let mut x = 5;
    println!("x 的值是: {}", x);
    
    x = 6;  // 现在可以修改了
    println!("x 的新值是: {}", x);
}
```

### 变量遮蔽（Shadowing）

Rust 允许使用相同的名字声明新变量，新变量会遮蔽之前的变量：

```rust
fn main() {
    let x = 5;
    let x = x + 1;  // 遮蔽了前面的 x
    
    {
        let x = x * 2;  // 在内部作用域中遮蔽
        println!("内部作用域中 x 的值是: {}", x);  // 输出 12
    }
    
    println!("外部作用域中 x 的值是: {}", x);  // 输出 6
}
```

遮蔽与 `mut` 的区别：
- 遮蔽实际上创建了一个新变量，可以改变类型
- `mut` 只是使变量可变，不能改变类型

```rust
let spaces = "   ";
let spaces = spaces.len();  // 可以，类型从 &str 变为 usize

let mut spaces = "   ";
// spaces = spaces.len();  // 错误！不能改变类型
```

### 常量

常量使用 `const` 关键字声明，必须注明类型，并且只能设置为常量表达式：

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("3小时有 {} 秒", THREE_HOURS_IN_SECONDS);
}
```

常量与不可变变量的区别：
- 常量总是不可变的，不能使用 `mut`
- 常量必须注明类型
- 常量可以在任何作用域声明，包括全局作用域
- 常量只能设置为常量表达式，不能是函数调用的结果

## 1.2 数据类型

Rust 是静态类型语言，编译时必须知道所有变量的类型。

### 标量类型

#### 整数类型

| 长度    | 有符号  | 无符号  |
|---------|---------|---------|
| 8-bit   | `i8`    | `u8`    |
| 16-bit  | `i16`   | `u16`   |
| 32-bit  | `i32`   | `u32`   |
| 64-bit  | `i64`   | `u64`   |
| 128-bit | `i128`  | `u128`  |
| arch    | `isize` | `usize` |

```rust
fn main() {
    let a: i32 = 98_222;  // 十进制，可以使用下划线增加可读性
    let b: i32 = 0xff;    // 十六进制
    let c: i32 = 0o77;    // 八进制
    let d: i32 = 0b1111_0000; // 二进制
    let e: u8 = b'A';     // 字节（u8）
}
```

#### 浮点类型

Rust 有两种浮点类型：`f32` 和 `f64`（默认）

```rust
fn main() {
    let x = 2.0;      // f64
    let y: f32 = 3.0; // f32
    
    // 基本运算
    let sum = 5.0 + 10.0;
    let difference = 95.5 - 4.3;
    let product = 4.0 * 30.0;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;  // 只能用于整数
}
```

#### 布尔类型

```rust
fn main() {
    let t = true;
    let f: bool = false;
}
```

#### 字符类型

Rust 的 `char` 类型代表一个 Unicode 标量值：

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    
    println!("{} {} {}", c, z, heart_eyed_cat);
}
```

### 复合类型

#### 元组（Tuple）

元组可以将多个不同类型的值组合在一起：

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    // 解构
    let (x, y, z) = tup;
    println!("y 的值是: {}", y);
    
    // 使用点号访问
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
}
```

#### 数组（Array）

数组中的每个元素必须是相同类型，且长度固定：

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    
    // 显式声明类型和长度
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    
    // 创建相同值的数组
    let a = [3; 5]; // 等同于 [3, 3, 3, 3, 3]
    
    // 访问元素
    let first = a[0];
    let second = a[1];
}
```

## 1.3 函数

### 函数定义

使用 `fn` 关键字定义函数：

```rust
fn main() {
    println!("Hello, world!");
    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

### 函数参数

函数参数必须声明类型：

```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
```

### 语句和表达式

Rust 是基于表达式的语言，理解语句和表达式的区别很重要：

- **语句**：执行操作但不返回值
- **表达式**：计算并产生一个值

```rust
fn main() {
    // 语句
    let y = 6;  // let 语句不返回值
    
    // 表达式
    let y = {
        let x = 3;
        x + 1  // 注意没有分号，这是一个表达式
    };
    
    println!("y 的值是: {}", y);  // 输出 4
}
```

### 函数返回值

使用 `->` 指定返回类型：

```rust
fn five() -> i32 {
    5  // 没有分号，作为表达式返回
}

fn plus_one(x: i32) -> i32 {
    x + 1  // 最后一个表达式作为返回值
}

fn main() {
    let x = five();
    let y = plus_one(5);
    
    println!("x = {}, y = {}", x, y);
}
```

注意：如果在应该返回值的地方加了分号，会导致编译错误：

```rust
fn plus_one(x: i32) -> i32 {
    x + 1;  // 错误！这变成了语句
}
```

## 1.4 程序控制流

### if 表达式

```rust
fn main() {
    let number = 3;
    
    if number < 5 {
        println!("条件为真");
    } else {
        println!("条件为假");
    }
}
```

#### 多重条件

```rust
fn main() {
    let number = 6;
    
    if number % 4 == 0 {
        println!("number 能被 4 整除");
    } else if number % 3 == 0 {
        println!("number 能被 3 整除");
    } else if number % 2 == 0 {
        println!("number 能被 2 整除");
    } else {
        println!("number 不能被 4、3 或 2 整除");
    }
}
```

#### 在 let 语句中使用 if

由于 if 是表达式，可以在 let 语句右边使用：

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    
    println!("number 的值是: {}", number);
}
```

### 循环

Rust 有三种循环：`loop`、`while` 和 `for`。

#### loop 循环

`loop` 创建无限循环：

```rust
fn main() {
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;  // break 可以返回值
        }
    };
    
    println!("结果是: {}", result);  // 输出 20
}
```

#### 循环标签

可以给循环指定标签，用于多重循环的 break 和 continue：

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        
        count += 1;
    }
    println!("End count = {}", count);
}
```

#### while 循环

```rust
fn main() {
    let mut number = 3;
    
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    
    println!("发射！");
}
```

#### for 循环

for 循环是最常用的循环形式：

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    
    // 遍历数组
    for element in a {
        println!("值是: {}", element);
    }
    
    // 使用范围（Range）
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("发射！");
}
```

### match 表达式

`match` 是 Rust 中强大的控制流运算符：

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = Coin::Penny;
    println!("硬币价值: {} 美分", value_in_cents(coin));
}
```

match 必须穷尽所有可能性，可以使用 `_` 通配符：

```rust
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => println!("获得帽子！"),
        7 => println!("失去帽子！"),
        _ => println!("继续前进"),  // 匹配所有其他值
    }
}
```

## 小结

本章介绍了 Rust 的基础语法：

1. **变量与常量**：理解不可变性、可变变量、变量遮蔽和常量的区别
2. **数据类型**：掌握标量类型（整数、浮点、布尔、字符）和复合类型（元组、数组）
3. **函数**：学会定义函数、传递参数、返回值，理解语句和表达式的区别
4. **控制流**：使用 if、loop、while、for 和 match 控制程序执行流程

这些是 Rust 编程的基石，后续章节将在此基础上深入探讨 Rust 的独特特性。 