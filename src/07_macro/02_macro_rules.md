# Rust macro_rules! 教程（超级详细版本）

Rust 的 `macro_rules!` 是声明式宏的定义方式，
是 Rust 宏系统的基础组成部分，提供了一种简单的元编程工具，
用于在编译时基于模式匹配生成代码，支持代码复用、语法糖和性能优化，
而无需运行时开销。
`macro_rules!` 宏允许定义规则集，通过模式匹配输入并转录输出，
生成有效的 Rust 代码。它是 Rust 宏的入门形式，
相比过程宏更简单但功能有限，主要用于重复代码生成、
变参函数和简单 DSL。`macro_rules!` 强调编译时扩展：
宏展开在词法分析后、语法解析前发生，生成抽象语法树（AST），
支持 hygiene（卫生性）以避免名称冲突和意外捕获；
规则通过臂（arm）匹配，允许多规则和递归，
但受限于 64 级深度以防栈溢。宏可以是公有的（pub macro_rules!）
，支持导出和导入（use crate::macro_name!;），
并可跨 crate 使用（需 macro_export 属性）。
`macro_rules!` 的设计优先易用性和安全性，
适用于 boilerplate 减少、trait 辅助和库 API 扩展场景（对比过程宏的强大 Token 操作），
并作为声明宏的扩展支持自定义模式和与过程宏的互操作。`macro_rules!` 与 `std::fmt`（格式化转录）、
`std::panic`（宏中 panic 传播）和 `std::attribute`（属性辅助）深度集成，支持高级模式如递归计数器、变参列表和条件转录。


## 1. macro_rules! 简介

`macro_rules!` 是 Rust 声明式宏的定义关键字，用于创建基于模式匹配的宏。宏在编译时展开，生成代码，类似于函数但在语法级别操作。

### 为什么使用 macro_rules!
- **减少重复代码**：生成类似但略异的代码块。
- **创建变参接口**：如 println! 支持任意参数。
- **定义 DSL**：如 sql! 用于查询语法。
- **性能优化**：编译时计算常量。

### 基本语法
```rust
macro_rules! macro_name {
    (pattern1) => { expansion1 };
    (pattern2) => { expansion2 };
    // ...
}
```

- `macro_name`: 宏名。
- `(pattern)`: 匹配输入的模式。
- `{ expansion }`: 转录输出的代码。
- 多臂以 ; 分隔，第一匹配使用。

### 示例1: 简单宏
```rust
macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}

fn main() {
    say_hello!(); // 展开为 println!("Hello, world!");
}
```

- 解释：空模式 () 匹配 say_hello!() 调用，展开 println!。

### 示例2: 带参数宏
```rust
macro_rules! greet {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

fn main() {
    greet!("Rust"); // Hello, Rust!
}
```

- 解释：`$name:expr` 匹配表达式，如字符串字面量。

### 高级语法元素
- **元变量类型**：$var:expr (表达式)、$var:ty (类型)、$var:ident (标识符)、$var:tt (令牌树)等。
- **卫生性**：宏展开的变量不会污染外部作用域。
- **可见性**：pub macro_rules! 导出宏；use 导入。

### 性能考虑
宏展开发生在编译时，运行时零开销；但复杂宏增加编译时间，用 rustc -Z time-passes 分析。

### 跨平台
宏是 compiler 级，无 OS 依赖；但过程宏 DLL/so 测试加载。

### 测试宏
用 cargo expand 查看展开；#[test] 测试宏调用。

### 常见陷阱
- 模式不匹配：编译错误。
- 无限递归：rustc 限 64 深，溢出错误。
- 非卫生：用 $crate 逃逸。

### 替代
过程宏更强大，但 macro_rules! 简单无 dep。

## 2. 模式和元变量详解

模式定义宏输入。

### 元变量类型
- block: 代码块 {}
- expr: 表达式
- ident: 标识符
- item: 项 (fn/struct 等)
- lifetime: 'a
- literal: 字面量
- meta: 属性元
- pat: 模式
- pat_param: 参数模式
- path: 路径 ::
- stmt: 语句
- tt: 任意令牌树
- ty: 类型
- vis: 可见性 pub/private

### 示例: 多类型元变量
```rust
macro_rules! define_struct {
    ($name:ident, $field:ident: $ty:ty) => {
        struct $name {
            $field: $ty,
        }
    };
}

define_struct!(MyStruct, id: i32); // 生成 struct MyStruct { id: i32 }
```

- 解释：$name:ident 匹配标识，$ty:ty 类型。

### 高级模式
- $var:literal: 匹配字面如 "str" 123。
- $var:meta: 匹配属性如 #[attr]。
- tt 任意，辅助复杂。

### 示例: tt 任意
```rust
macro_rules! wrap {
    ($tt:tt) => { $tt };
}

fn main() {
    wrap!(let x = 1;); // 展开 let x = 1;
}
```

- 解释：tt 匹配任意语法树。

### 陷阱
- 模式太宽：用 expr 而非 tt 限制。
- 错误类型：编译 Err "expected expr"。

### 优化
用具体 specifier 快匹配。

## 3. 重复详解

重复用 $(...) sep op，其中 op * + ?，sep , ; 等。

### 语法
- * : 0+
- + : 1+
- ? : 0或1，无 sep

### 示例: 重复参数
```rust
macro_rules! vec_create {
    ($($x:expr),*) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )*
            v
        }
    };
}

fn main() {
    let v = vec_create!(1, 2, 3); // Vec [1,2,3]
}
```

- 解释：, 分隔，* 重复 push。

### 示例: + 至少一
```rust
macro_rules! non_empty {
    ($head:expr $(, $tail:expr)+) => {
        $head $(+ $tail)*
    };
}

fn main() {
    println!("{}", non_empty!(1, 2, 3)); // 1+2+3 = 6
}
```

- 解释：+ 确保至少一 tail。

### 示例: ? 可选
```rust
macro_rules! optional {
    ($x:expr $(, $y:expr)?) => {
        $x $(+ $y)?
    };
}

fn main() {
    println!("{}", optional!(1)); // 1
    println!("{}", optional!(1, 2)); // 3
}
```

- **解释**：? 可选 $y，无 sep。

### 高级重复
嵌套 $( $( $inner )sep )op

### 示例: 嵌套
```rust
macro_rules! matrix {
    ( $( [ $( $x:expr ),* ] ),* ) => {
        vec![
            $(
                vec![ $( $x ),* ],
            )*
        ]
    };
}

fn main() {
    let m = matrix![ [1,2], [3,4] ]; // vec![vec![1,2], vec![3,4]]
}
```

- 解释：外 * 内 * 匹配矩阵。

### 陷阱
- 重复不匹配：编译 Err。
- 转录限制：$var 必须同重复级。

### 优化
用 * 而非 + 允许空；sep 匹配输入。

## 4. 卫生性详解

卫生性防止宏变量与外部冲突。

### 示例: 卫生变量
```rust
macro_rules! local_var {
    () => {
        let var = 1;
    };
}

fn main() {
    let var = 2;
    local_var!();
    println!("{}", var); // 2
}
```

- 解释：宏 var 卫生，不覆盖外部。

### 示例: 逃逸卫生
```rust
macro_rules! use_external {
    () => {
        println!("{}", $crate::SOME_GLOBAL);
    };
}

const SOME_GLOBAL: i32 = 42;

fn main() {
    use_external!(); // 42
}
```

- 解释：$crate 引用 crate 根。

### 高级卫生
- 标签/变量 定义现场查找。
- 其他 调用现场查找。

### 示例: 卫生标签
```rust
macro_rules! loop_label {
    () => {
        'label: loop {}
    };
}

fn main() {
    loop_label!();
    break 'label; // 错误，'label 定义在宏
}
```

- 解释：标签卫生于定义。

### 陷阱
- 非卫生需 allow(unused) 辅助。
- 调用现场路径需 qualify。

### 优化
用 hygiene 减 bug。

## 5. 高级用法

### 递归宏
用于树/列表。

### 示例: 递归加
```rust
macro_rules! rec_add {
    ($x:expr) => { $x };
    ($x:expr, $($rest:expr),+) => { $x + rec_add!($($rest),+) };
}

fn main() {
    println!("{}", rec_add!(1, 2, 3)); // 6
}
```

- 解释：递归展开 + 。

### 条件转录
用 if/else 在转录。

### 示例: 条件
```rust
macro_rules! cond {
    ($cond:expr => $true:expr, $false:expr) => {
        if $cond { $true } else { $false }
    };
}

fn main() {
    println!("{}", cond!(true => 1, 2)); // 1
}
```

- 解释：转录时条件。

### 转录 Token
用 stringify! 转字符串，concat! 连接。

### 示例: stringify
```rust
macro_rules! str_macro {
    ($x:expr) => {
        println!("{}", stringify!($x));
    };
}

fn main() {
    str_macro!(1 + 2); // "1 + 2"
}
```

- 解释：stringify! Token 转 str。

### 高级: TTL 辅助宏
用 macro_rules! 辅助过程宏。

## 6. 陷阱、错误和调试

- **陷阱**：模式太宽捕获错；递归深 overflow；卫生意外冲突。
- **错误**：不匹配 "no rules expected this token"；用 tt 宽松。
- **调试**：cargo expand 查看展开；rustc -Z unstable-options --pretty expanded。
- **测试**：#[macro_export] 导出；test mod 测试调用。

## 7. 最佳实践

- 小宏 macro_rules，大过程宏。
- 文档宏规则和例子。
- 用 ? + * 灵活参数。
- 转录用 {} 块隔离。
- 避免递归，用迭代 *。

## 8. 练习

1. 写 count! 宏计算参数数。
2. 实现 json! 对象宏。
3. 创建 rec_list! 递归列表。
4. 用 stringify 生成 const 字符串。
5. 测试宏展开 cargo expand。
6. 辅助宏过程宏 syn quote。
7. 处理模式 Err 用 tt fallback。
8. 高级：实现 builder 宏生成 struct 方法。
