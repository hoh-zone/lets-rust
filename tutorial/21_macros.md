# 第21章：宏

宏是 Rust 中强大的元编程工具，允许我们编写生成代码的代码，在编译时展开为实际的 Rust 代码。

## 21.1 宏的定义与特点

### 宏与函数的区别

```rust
// 函数：运行时调用
fn add_function(a: i32, b: i32) -> i32 {
    a + b
}

// 宏：编译时展开
macro_rules! add_macro {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

fn main() {
    // 函数调用
    let result1 = add_function(3, 4);
    println!("函数结果: {}", result1);
    
    // 宏调用
    let result2 = add_macro!(3, 4);
    println!("宏结果: {}", result2);
    
    // 宏可以接受不同类型的参数
    let result3 = add_macro!(3.5, 4.2);
    println!("宏结果(浮点): {}", result3);
}
```

### 宏的基本语法

```rust
macro_rules! my_macro {
    // 模式 => 展开的代码
    () => {
        println!("这是一个简单的宏");
    };
    
    // 带参数的模式
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
    
    // 多个参数
    ($name:expr, $age:expr) => {
        println!("{} is {} years old", $name, $age);
    };
}

fn main() {
    my_macro!();
    my_macro!("Alice");
    my_macro!("Bob", 25);
}
```

### 宏参数类型

```rust
macro_rules! demo_types {
    // item: 语法项（函数、结构体等）
    ($item:item) => {
        $item
    };
    
    // block: 代码块
    ($block:block) => {
        println!("执行代码块:");
        $block
    };
    
    // stmt: 语句
    ($stmt:stmt) => {
        $stmt
    };
    
    // expr: 表达式
    ($expr:expr) => {
        println!("表达式的值: {}", $expr);
    };
    
    // ty: 类型
    ($ty:ty) => {
        let _: $ty;
        println!("类型: {}", stringify!($ty));
    };
    
    // pat: 模式
    ($pat:pat) => {
        let $pat = 42;
    };
    
    // ident: 标识符
    ($id:ident) => {
        let $id = "identifier";
        println!("{} = {}", stringify!($id), $id);
    };
}

fn main() {
    // item
    demo_types! {
        fn hello() {
            println!("Hello from macro-generated function!");
        }
    }
    hello();
    
    // block
    demo_types! {
        {
            let x = 5;
            println!("x = {}", x);
        }
    }
    
    // expr
    demo_types!(2 + 3);
    
    // ty
    demo_types!(Vec<i32>);
    
    // pat
    demo_types!(x);
    println!("x = {}", x);
    
    // ident
    demo_types!(my_var);
}
```

## 21.2 Rust中常见宏的应用

### 重复模式宏

```rust
// 创建向量的宏
macro_rules! vec_of {
    ($elem:expr; $n:expr) => {
        {
            let mut v = Vec::new();
            for _ in 0..$n {
                v.push($elem);
            }
            v
        }
    };
    
    ($($elem:expr),* $(,)?) => {
        {
            let mut v = Vec::new();
            $(v.push($elem);)*
            v
        }
    };
}

fn main() {
    // 重复相同元素
    let v1 = vec_of![0; 5];
    println!("v1: {:?}", v1);
    
    // 不同元素
    let v2 = vec_of![1, 2, 3, 4, 5];
    println!("v2: {:?}", v2);
    
    // 可选的尾随逗号
    let v3 = vec_of![1, 2, 3,];
    println!("v3: {:?}", v3);
}
```

### HashMap 创建宏

```rust
use std::collections::HashMap;

macro_rules! hashmap {
    () => {
        HashMap::new()
    };
    
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut map = HashMap::new();
            $(map.insert($key, $value);)*
            map
        }
    };
}

fn main() {
    let empty_map: HashMap<&str, i32> = hashmap!();
    println!("空映射: {:?}", empty_map);
    
    let map = hashmap! {
        "one" => 1,
        "two" => 2,
        "three" => 3,
    };
    println!("映射: {:?}", map);
}
```

### 条件编译宏

```rust
macro_rules! cfg_if {
    // 基础情况：只有一个 if
    (if #[cfg($meta:meta)] { $($it:item)* }) => {
        $(#[cfg($meta)] $it)*
    };
    
    // 递归情况：if-else
    (
        if #[cfg($i_met:meta)] { $($i_it:item)* }
        else if #[cfg($e_met:meta)] { $($e_it:item)* }
        $(else if #[cfg($else_met:meta)] { $($else_it:item)* })*
        $(else { $($else_it2:item)* })?
    ) => {
        cfg_if! {
            @__items () ;
            [{ #[cfg($i_met)] $($i_it)* }]
            [{ #[cfg($e_met)] $($e_it)* }]
            $([{ #[cfg($else_met)] $($else_it)* }])*
            $([{ $($else_it2)* }])?
        }
    };
    
    // 内部递归处理
    (@__items ($($not:meta,)*) ; ) => {};
    (@__items ($($not:meta,)*) ; [{ $($it:item)* }] $($rest:tt)*) => {
        $(#[cfg(all($($not,)*))] $it)*
        cfg_if! { @__items ($($not,)* not($($rest)*),) ; $($rest)* }
    };
}

cfg_if! {
    if #[cfg(unix)] {
        fn platform_specific() {
            println!("这是 Unix 平台特定的代码");
        }
    } else if #[cfg(windows)] {
        fn platform_specific() {
            println!("这是 Windows 平台特定的代码");
        }
    } else {
        fn platform_specific() {
            println!("这是其他平台的代码");
        }
    }
}

fn main() {
    platform_specific();
}
```

### 断言宏

```rust
macro_rules! assert_eq_verbose {
    ($left:expr, $right:expr) => {
        {
            let left_val = $left;
            let right_val = $right;
            if left_val != right_val {
                panic!(
                    "断言失败: {} != {}\n  左值: {:?}\n  右值: {:?}\n  位置: {}:{}:{}",
                    stringify!($left),
                    stringify!($right),
                    left_val,
                    right_val,
                    file!(),
                    line!(),
                    column!()
                );
            } else {
                println!("断言成功: {} == {}", stringify!($left), stringify!($right));
            }
        }
    };
}

fn main() {
    let a = 5;
    let b = 5;
    assert_eq_verbose!(a, b);
    
    let x = 10;
    let y = 20;
    // assert_eq_verbose!(x, y); // 这会 panic
}
```

### 日志宏

```rust
macro_rules! log {
    (ERROR, $($arg:tt)*) => {
        eprintln!("[ERROR] {}: {}", 
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
            format!($($arg)*)
        );
    };
    
    (WARN, $($arg:tt)*) => {
        println!("[WARN]  {}: {}", 
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
            format!($($arg)*)
        );
    };
    
    (INFO, $($arg:tt)*) => {
        println!("[INFO]  {}: {}", 
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
            format!($($arg)*)
        );
    };
    
    (DEBUG, $($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!("[DEBUG] {}: {}", 
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
            format!($($arg)*)
        );
    };
}

// 简化版本（不依赖 chrono）
macro_rules! simple_log {
    (ERROR, $($arg:tt)*) => {
        eprintln!("[ERROR] {}", format!($($arg)*));
    };
    
    (WARN, $($arg:tt)*) => {
        println!("[WARN]  {}", format!($($arg)*));
    };
    
    (INFO, $($arg:tt)*) => {
        println!("[INFO]  {}", format!($($arg)*));
    };
    
    (DEBUG, $($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!("[DEBUG] {}", format!($($arg)*));
    };
}

fn main() {
    simple_log!(INFO, "应用程序启动");
    simple_log!(WARN, "这是一个警告: {}", "内存使用较高");
    simple_log!(ERROR, "发生错误: {}", "文件未找到");
    simple_log!(DEBUG, "调试信息: 变量 x = {}", 42);
}
```

## 21.3 编写与使用声明宏

### 计算宏

```rust
macro_rules! calc {
    // 加法
    ($a:expr + $b:expr) => {
        $a + $b
    };
    
    // 减法
    ($a:expr - $b:expr) => {
        $a - $b
    };
    
    // 乘法
    ($a:expr * $b:expr) => {
        $a * $b
    };
    
    // 除法
    ($a:expr / $b:expr) => {
        if $b != 0 {
            $a / $b
        } else {
            panic!("除零错误");
        }
    };
    
    // 复杂表达式
    (($($expr:tt)*)) => {
        calc!($($expr)*)
    };
    
    // 递归计算
    ($a:expr + $($rest:tt)*) => {
        $a + calc!($($rest)*)
    };
    
    ($a:expr - $($rest:tt)*) => {
        $a - calc!($($rest)*)
    };
    
    ($a:expr * $($rest:tt)*) => {
        $a * calc!($($rest)*)
    };
    
    ($a:expr / $($rest:tt)*) => {
        if calc!($($rest)*) != 0 {
            $a / calc!($($rest)*)
        } else {
            panic!("除零错误");
        }
    };
}

fn main() {
    println!("2 + 3 = {}", calc!(2 + 3));
    println!("10 - 4 = {}", calc!(10 - 4));
    println!("5 * 6 = {}", calc!(5 * 6));
    println!("15 / 3 = {}", calc!(15 / 3));
    
    // 复杂表达式
    println!("1 + 2 + 3 = {}", calc!(1 + 2 + 3));
}
```

### 结构体生成宏

```rust
macro_rules! create_struct {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field_vis:vis $field_name:ident: $field_type:ty
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis struct $name {
            $(
                $(#[$field_meta])*
                $field_vis $field_name: $field_type,
            )*
        }
        
        impl $name {
            pub fn new($($field_name: $field_type),*) -> Self {
                Self {
                    $($field_name),*
                }
            }
            
            $(
                paste::paste! {
                    pub fn [<get_ $field_name>](&self) -> &$field_type {
                        &self.$field_name
                    }
                    
                    pub fn [<set_ $field_name>](&mut self, value: $field_type) {
                        self.$field_name = value;
                    }
                }
            )*
        }
    };
}

// 简化版本（不使用 paste）
macro_rules! simple_struct {
    (
        $vis:vis struct $name:ident {
            $($field_name:ident: $field_type:ty),* $(,)?
        }
    ) => {
        $vis struct $name {
            $($field_name: $field_type,)*
        }
        
        impl $name {
            pub fn new($($field_name: $field_type),*) -> Self {
                Self {
                    $($field_name),*
                }
            }
        }
    };
}

simple_struct! {
    pub struct Person {
        name: String,
        age: u32,
        email: String,
    }
}

fn main() {
    let person = Person::new(
        "Alice".to_string(),
        30,
        "alice@example.com".to_string(),
    );
    
    println!("姓名: {}", person.name);
    println!("年龄: {}", person.age);
    println!("邮箱: {}", person.email);
}
```

### 枚举生成宏

```rust
macro_rules! create_enum {
    (
        $vis:vis enum $name:ident {
            $($variant:ident$(($($field:ty),*))?),* $(,)?
        }
    ) => {
        $vis enum $name {
            $($variant$(($($field),*))?),*
        }
        
        impl $name {
            pub fn variant_name(&self) -> &'static str {
                match self {
                    $(Self::$variant$((..))? => stringify!($variant)),*
                }
            }
            
            pub fn all_variants() -> Vec<&'static str> {
                vec![$(stringify!($variant)),*]
            }
        }
    };
}

create_enum! {
    pub enum Color {
        Red,
        Green,
        Blue,
        Rgb(u8, u8, u8),
        Hsl(f32, f32, f32),
    }
}

fn main() {
    let red = Color::Red;
    let blue = Color::Blue;
    let custom = Color::Rgb(255, 128, 0);
    
    println!("红色变体名: {}", red.variant_name());
    println!("蓝色变体名: {}", blue.variant_name());
    println!("自定义颜色变体名: {}", custom.variant_name());
    
    println!("所有变体: {:?}", Color::all_variants());
}
```

### 测试宏

```rust
macro_rules! test_case {
    ($name:ident: $input:expr => $expected:expr) => {
        #[cfg(test)]
        mod $name {
            use super::*;
            
            #[test]
            fn test() {
                let result = $input;
                assert_eq!(result, $expected, 
                    "测试失败: {} 预期 {} 但得到 {}", 
                    stringify!($input), 
                    $expected, 
                    result
                );
            }
        }
    };
    
    ($name:ident: $input:expr => $expected:expr, $description:expr) => {
        #[cfg(test)]
        mod $name {
            use super::*;
            
            #[test]
            fn test() {
                let result = $input;
                assert_eq!(result, $expected, 
                    "{}: {} 预期 {} 但得到 {}", 
                    $description,
                    stringify!($input), 
                    $expected, 
                    result
                );
            }
        }
    };
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

test_case!(test_add_positive: add(2, 3) => 5);
test_case!(test_add_negative: add(-1, -1) => -2);
test_case!(test_multiply: multiply(4, 5) => 20, "乘法测试");

fn main() {
    println!("运行 'cargo test' 来执行测试");
}
```

### 配置宏

```rust
macro_rules! config {
    (
        $(
            $key:ident = $value:expr
        ),* $(,)?
    ) => {
        pub struct Config {
            $(pub $key: String),*
        }
        
        impl Config {
            pub fn new() -> Self {
                Self {
                    $($key: $value.to_string()),*
                }
            }
            
            pub fn from_env() -> Self {
                Self {
                    $(
                        $key: std::env::var(stringify!($key))
                            .unwrap_or_else(|_| $value.to_string())
                    ),*
                }
            }
            
            pub fn print(&self) {
                println!("配置:");
                $(println!("  {}: {}", stringify!($key), self.$key);)*
            }
        }
    };
}

config! {
    database_url = "localhost:5432",
    api_key = "default_key",
    log_level = "info",
    max_connections = "100",
}

fn main() {
    let config = Config::new();
    config.print();
    
    println!("\n从环境变量加载的配置:");
    let env_config = Config::from_env();
    env_config.print();
}
```

## RWO 权限分析

### 宏中的所有权处理

```rust
macro_rules! take_ownership {
    ($value:expr) => {
        {
            let owned = $value; // O: 获取所有权
            println!("获取所有权: {:?}", owned);
            owned // O: 转移所有权给调用者
        }
    };
}

macro_rules! borrow_value {
    ($value:expr) => {
        {
            let borrowed = &$value; // R: 借用引用
            println!("借用值: {:?}", borrowed);
            // 不转移所有权
        }
    };
}

macro_rules! modify_value {
    ($value:expr, $new_val:expr) => {
        {
            let mut_ref = &mut $value; // W: 可变借用
            *mut_ref = $new_val;
            println!("修改后的值: {:?}", $value);
        }
    };
}

fn main() {
    let data = vec![1, 2, 3];
    
    // R: 借用
    borrow_value!(data);
    println!("原始数据仍可用: {:?}", data);
    
    // O: 转移所有权
    let owned_data = take_ownership!(data);
    println!("新拥有的数据: {:?}", owned_data);
    // data 在这里不再可用
    
    // W: 可变操作
    let mut mutable_data = vec![1, 2, 3];
    modify_value!(mutable_data, vec![4, 5, 6]);
}
```

### 宏生成的代码权限

```rust
macro_rules! create_getter_setter {
    ($struct_name:ident, $field_name:ident, $field_type:ty) => {
        impl $struct_name {
            // R: 获取不可变引用
            pub fn $field_name(&self) -> &$field_type {
                &self.$field_name
            }
            
            // O: 获取所有权的副本（需要 Clone）
            paste::paste! {
                pub fn [<$field_name _clone>](&self) -> $field_type 
                where 
                    $field_type: Clone 
                {
                    self.$field_name.clone()
                }
                
                // W: 可变引用
                pub fn [<$field_name _mut>](&mut self) -> &mut $field_type {
                    &mut self.$field_name
                }
                
                // O: 设置新值（转移所有权）
                pub fn [<set_ $field_name>](&mut self, value: $field_type) {
                    self.$field_name = value;
                }
            }
        }
    };
}

// 简化版本
macro_rules! simple_getter_setter {
    ($struct_name:ident, $field_name:ident, $field_type:ty) => {
        impl $struct_name {
            pub fn get_field(&self) -> &$field_type {
                &self.$field_name
            }
            
            pub fn get_field_mut(&mut self) -> &mut $field_type {
                &mut self.$field_name
            }
            
            pub fn set_field(&mut self, value: $field_type) {
                self.$field_name = value;
            }
        }
    };
}

struct MyStruct {
    data: Vec<i32>,
}

simple_getter_setter!(MyStruct, data, Vec<i32>);

fn main() {
    let mut my_struct = MyStruct {
        data: vec![1, 2, 3],
    };
    
    // R: 不可变借用
    let data_ref = my_struct.get_field();
    println!("借用的数据: {:?}", data_ref);
    
    // W: 可变借用
    {
        let data_mut = my_struct.get_field_mut();
        data_mut.push(4);
    }
    
    // O: 设置新值
    my_struct.set_field(vec![5, 6, 7]);
    println!("新数据: {:?}", my_struct.get_field());
}
```

### 宏中的生命周期处理

```rust
macro_rules! create_wrapper {
    ($name:ident, $inner_type:ty) => {
        struct $name<'a> {
            inner: &'a $inner_type,
        }
        
        impl<'a> $name<'a> {
            // R: 创建包装器，借用内部数据
            fn new(inner: &'a $inner_type) -> Self {
                Self { inner }
            }
            
            // R: 获取内部数据的引用
            fn get(&self) -> &'a $inner_type {
                self.inner
            }
            
            // 转换为内部类型的引用
            fn as_ref(&self) -> &$inner_type {
                self.inner
            }
        }
        
        impl<'a> std::ops::Deref for $name<'a> {
            type Target = $inner_type;
            
            fn deref(&self) -> &Self::Target {
                self.inner
            }
        }
    };
}

create_wrapper!(StringWrapper, String);
create_wrapper!(VecWrapper, Vec<i32>);

fn main() {
    let text = String::from("Hello, world!");
    let numbers = vec![1, 2, 3, 4, 5];
    
    // 创建包装器（借用原始数据）
    let string_wrapper = StringWrapper::new(&text);
    let vec_wrapper = VecWrapper::new(&numbers);
    
    // 通过包装器访问数据
    println!("包装的字符串: {}", string_wrapper.get());
    println!("包装的向量: {:?}", vec_wrapper.get());
    
    // 通过 Deref 自动解引用
    println!("字符串长度: {}", string_wrapper.len());
    println!("向量长度: {}", vec_wrapper.len());
    
    // 原始数据仍然可用
    println!("原始文本: {}", text);
    println!("原始数字: {:?}", numbers);
}
```

### 宏中的错误处理权限

```rust
macro_rules! try_operation {
    ($operation:expr) => {
        match $operation {
            Ok(value) => value, // O: 成功时转移所有权
            Err(e) => {
                eprintln!("操作失败: {}", e);
                return Err(e); // O: 错误时转移错误所有权
            }
        }
    };
}

macro_rules! safe_unwrap {
    ($option:expr, $default:expr) => {
        match $option {
            Some(value) => value, // O: 有值时转移所有权
            None => $default,     // O: 无值时使用默认值
        }
    };
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("除零错误".to_string())
    } else {
        Ok(a / b)
    }
}

fn process_numbers() -> Result<f64, String> {
    let result1 = try_operation!(divide(10.0, 2.0));
    let result2 = try_operation!(divide(20.0, 4.0));
    
    Ok(result1 + result2)
}

fn main() {
    match process_numbers() {
        Ok(result) => println!("计算结果: {}", result),
        Err(e) => println!("处理失败: {}", e),
    }
    
    // 使用 safe_unwrap
    let maybe_value: Option<String> = None;
    let value = safe_unwrap!(maybe_value, "默认值".to_string());
    println!("安全解包的值: {}", value);
}
```

### 宏生成的智能指针

```rust
macro_rules! create_smart_pointer {
    ($name:ident, $inner_type:ty) => {
        struct $name {
            data: Box<$inner_type>,
        }
        
        impl $name {
            // O: 创建智能指针，获取数据所有权
            fn new(data: $inner_type) -> Self {
                Self {
                    data: Box::new(data),
                }
            }
            
            // O: 消费智能指针，返回内部数据
            fn into_inner(self) -> $inner_type {
                *self.data
            }
            
            // R: 获取不可变引用
            fn as_ref(&self) -> &$inner_type {
                &self.data
            }
            
            // W: 获取可变引用
            fn as_mut(&mut self) -> &mut $inner_type {
                &mut self.data
            }
        }
        
        impl std::ops::Deref for $name {
            type Target = $inner_type;
            
            fn deref(&self) -> &Self::Target {
                &self.data
            }
        }
        
        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.data
            }
        }
        
        impl Clone for $name 
        where 
            $inner_type: Clone 
        {
            fn clone(&self) -> Self {
                Self::new(self.data.as_ref().clone())
            }
        }
    };
}

create_smart_pointer!(SmartString, String);
create_smart_pointer!(SmartVec, Vec<i32>);

fn main() {
    // O: 创建智能指针
    let mut smart_string = SmartString::new("Hello".to_string());
    let smart_vec = SmartVec::new(vec![1, 2, 3]);
    
    // R: 通过 Deref 访问
    println!("字符串长度: {}", smart_string.len());
    println!("向量长度: {}", smart_vec.len());
    
    // W: 可变访问
    smart_string.as_mut().push_str(", World!");
    println!("修改后的字符串: {}", smart_string.as_ref());
    
    // 克隆智能指针
    let cloned_vec = smart_vec.clone();
    println!("克隆的向量: {:?}", cloned_vec.as_ref());
    
    // O: 提取内部数据
    let inner_string = smart_string.into_inner();
    println!("提取的字符串: {}", inner_string);
    // smart_string 在这里不再可用
}
```

## 小结

本章深入学习了 Rust 的宏系统：

1. **宏的基础**：
   - 宏在编译时展开
   - 与函数的区别
   - 基本语法和参数类型

2. **常见宏应用**：
   - 数据结构创建
   - 条件编译
   - 日志和断言
   - 配置管理

3. **声明宏编写**：
   - 模式匹配
   - 重复模式
   - 递归宏
   - 代码生成

4. **RWO 权限分析**：
   - **R**：宏可以生成借用数据的代码
   - **W**：宏可以生成修改数据的代码
   - **O**：宏处理所有权转移，包括参数和返回值
   - 宏展开时保持 Rust 的所有权语义
   - 生命周期在宏生成的代码中正确传播

宏是 Rust 强大的元编程工具，允许我们编写更简洁、更通用的代码，同时保持类型安全和内存安全。下一章我们将学习过程宏。 