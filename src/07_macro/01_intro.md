# Macros 教程

Rust 的宏系统是语言的核心特性之一，提供了一种元编程方式，用于在编译时生成代码，支持代码复用、DSL（领域特定语言）和性能优化，而不牺牲类型安全。Rust 宏分为两大类：**声明宏**（declarative macros，使用 `macro_rules!` 定义，基于模式匹配的语法扩展）和**过程宏**（procedural macros，使用 `proc_macro` crate 定义，包括函数式宏、派生宏和属性宏，允许任意 Rust 代码生成）。宏系统抽象了 TokenStream（令牌流）的解析和扩展，确保跨平台兼容性和编译时检查，并通过编译错误或运行时 panic（如递归深度超限或无效 Token）显式处理问题如模式不匹配或语法错误。Rust 宏强调编译时执行：宏展开在类型检查前发生，支持 hygiene（卫生性）以避免名称冲突；声明宏简单易用，过程宏强大但需外部 crate。模块的设计优先表达力和安全性，适用于 boilerplate 代码生成、性能关键扩展和库 API 增强场景（对比 C 的预处理器宏的安全问题），并作为宏系统的扩展支持自定义解析器和与 TokenStream 的互操作。Rust 宏与 `proc_macro`（过程宏 API）、`syn`/`quote`（外部解析/生成 crate）、`std::fmt`（格式化宏参数）和 `std::panic`（宏中 panic 传播）深度集成，支持高级模式如递归宏、自定义派生和属性注入。


## 1. Rust 宏系统简介

- **导入和高级结构**：对于声明宏，无需导入（内置）；对于过程宏，导入 `use proc_macro;`（在 proc-macro crate）。高级用法可包括 `use proc_macro2::{TokenStream, TokenTree, Span, Group, Punct, Ident, Literal};` 以访问 Token 操作，以及 `use syn::{parse_macro_input, DeriveInput};` 以解析输入、`use quote::quote;` 以生成代码。宏系统的内部结构包括 TokenStream 的树状 TokenTree（Group/Ident/Literal/Punct）、Span 的源位置跟踪和 hygiene 的编译时名称解析。
    - **宏类型详解**：
        - **声明宏**：使用 `macro_rules! name { (pattern) => (expansion); }`，支持 $var:ty 模式变量、重复 $(...)* 和卫生名称。
        - **函数式过程宏**：#[proc_macro] fn name(input: TokenStream) -> TokenStream，支持任意代码生成。
        - **派生过程宏**：#[proc_macro_derive(Name)] fn name(input: TokenStream) -> TokenStream，用于 #[derive(Name)]。
        - `TokenStream`：宏输入/输出流，支持 into_iter 以 TokenTree 遍历、parse 以 syn 解析。
        - `TokenTree`：枚举（Group/Ident/Literal/Punct），支持 span() 位置。
        - `Span`：源代码跨度，支持 join/unresolved 以合并/默认。
    - **函数和方法扩展**：`proc_macro::TokenStream::new` 创建、`TokenStream::from` 从 TokenTree、`TokenStream::is_empty` 检查、`TokenStream::to_string` 字符串化、`Span::source_text` 源文本 (1.15+)。
    - **宏**：`macro_rules!` 定义声明宏；过程宏无宏，但用 macro_rules 辅助。
- **设计哲学扩展**：Rust 宏是卫生性的（避免意外捕获），编译时展开；声明宏简单 DSL，过程宏强大元编程；零成本 Token 操作；panic 在宏传播编译错误。宏是 'static，输入 TokenStream 'static。
- **跨平台详解**：宏展开 compiler 侧，无 OS 依赖；但过程宏 DLL/so，测试 Windows/Linux 加载。
- **性能详析**：宏展开 O(n) 于 Token，复杂模式慢；基准 rustc -Z time-passes；大宏文件编译慢，用 mod 分离。
- **常见用例扩展**：日志宏（log!）、派生 Serialize、DSL 如 sql!、测试宏(assert_eq!)、性能 vec!。
- **超级扩展概念**：与 syn::parse 集成 AST；与 quote::ToTokens 生成；错误 custom Diagnostic；与 macro_rules_transparency 检查卫生；高性能用 proc-macro-hack 旧 hack；与 tracing::macro 装饰；历史：从 1.0 macro_rules 到 1.30 proc_macro 稳定。

## 2. 声明宏：macro_rules!

`macro_rules!` 定义模式匹配宏。

### 示例：基本 macro_rules（简单扩展）
```rust
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

fn main() {
    say_hello!();
}
```

- **解释**：空模式 () 展开 println。性能：编译时替换。

### 示例：参数模式（变量扩展）
```rust
macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

fn main() {
    println!("add: {}", add!(1, 2));  // 3
}
```

- **解释**：`$var:expr` 捕获表达式。扩展：用 $:ty 类型。

### 示例：重复模式（* + ?扩展）
```rust
macro_rules! vec_sum {
    ($($x:expr),*) => {{
        let mut sum = 0;
        $(
            sum += $x;
        )*
        sum
    }};
}

fn main() {
    println!("sum: {}", vec_sum!(1, 2, 3));  // 6
}
```

- **解释**：`$(...)*` 重复零或多。+ 一次或多，? 零或一。扩展：嵌套重复 $( $( $inner )* )*。

### 示例：卫生和可见性（hygiene扩展）
```rust
macro_rules! hygiene {
    () => {
        let x = 1;
    };
}

fn main() {
    let x = 2;
    hygiene!();
    println!("x: {}", x);  // 2, 宏 x 卫生不冲突
}
```

- **解释**：hygiene 宏变量不漏。扩展：use ::x 逃逸卫生。

### 示例：递归宏（树构建扩展）
```rust
macro_rules! nested {
    ($x:expr) => { $x };
    ($x:expr, $($rest:expr),+) => { nested!($x) + nested!($($rest),+) };
}

fn main() {
    println!("nested: {}", nested!(1, 2, 3));  // 6
}
```

- **解释**：递归展开。陷阱：深度 >64 panic，用迭代模式避。

## 4. 过程宏：proc_macro

过程宏需 proc-macro crate。

### 示例：函数式过程宏（hello扩展）
```rust
use proc_macro::TokenStream;

#[proc_macro]
pub fn hello(input: TokenStream) -> TokenStream {
    "println!(\"Hello from macro!\");".parse().unwrap()
}
```

- **解释**：输入 TokenStream，返回生成代码。性能：编译时执行。

### 示例：派生宏（CustomDerive扩展）
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MyTrait)]
pub fn my_trait(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    quote! {
        impl MyTrait for #name {
            fn method(&self) {}
        }
    }.into()
}
```

- **解释**：syn 解析 DeriveInput。quote 生成。扩展：darling 解析 attr。

### 示例：属性宏（attr扩展）
```rust
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn my_attr(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
```

- **解释**：attr TokenStream 是属性参数，item 是项。扩展：用 syn 解析 item。

## 4. TokenStream 操作

TokenStream 是宏 I/O。

### 示例：TokenTree 遍历（解析扩展）
```rust
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

fn main() {
    let ts: TokenStream = "fn main() {}".parse().unwrap();
    for tt in ts {
        match tt {
            TokenTree::Ident(i) => println!("ident: {}", i),
            TokenTree::Group(g) => println!("group: {:?}", g.delimiter()),
            _ => {},
        }
    }
}
```

- **解释**：TokenTree 枚举遍历。扩展：use syn::parse2 高级 AST。

## 5. 最佳实践和常见陷阱

- **宏最佳**：声明简单，过程复杂；hygiene 避冲突；递归限深。
- **性能**：宏展开慢，大文件分 mod；过程 Token 操作 O(n)。
- **错误**：模式不匹配编译 Err，用 $:tt 宽松。
- **安全**：过程 unsafe 限，macro_rules 安全。
- **跨平台**：宏 compiler 侧，一致。
- **测试**：cargo expand 检查；proc_macro test harness。
- **资源**：宏无运行时资源。
- **常见扩展**：
    - 卫生冲突：use $crate 逃逸。
    - 递归深：用 loop 迭代模式。
    - Token 无效：use syn error 友好。
    - 过程 dep：proc-macro = true lib。

