# proc_macro 教程

Rust 的 `proc_macro` crate 是 Rust 元编程系统的核心组成部分，提供过程宏的 API，用于在编译时生成自定义代码，支持语法扩展、trait 派生和属性注入，而不牺牲类型安全和性能。`proc_macro` 允许开发者创建像编译器插件一样的宏，通过 TokenStream 处理输入，生成新的 TokenStream 插入代码中。它是 Rust 高级宏的基石，抽象了编译器的 Token 处理，确保效率和隔离，并通过编译错误或 panic（如无效 Token 或内存溢出）显式处理问题如解析失败或无限循环。`proc_macro` 强调编译时计算：宏在扩展阶段执行，访问 TokenStream 而非完整 AST（用 syn 桥接）；支持函数宏、属性宏、派生宏和未来函数式变体；需在 Cargo.toml 启用 [lib] proc_macro = true，并 extern crate proc_macro;。crate 的设计优先功率和灵活性，适用于复杂代码生成、库增强和 DSL，相比 macro_rules! 更通用，但需 dep。`proc_macro` 与 `proc_macro2`（stable TokenStream）、`syn`（AST 解析）、`quote`（代码生成）、`darling`（属性解析）、`std::panic`（panic 传播）和 `std::attribute`（属性处理）深度集成，支持高级模式如递归 Token 处理、自定义诊断和 hygienic 名称。

## 1. proc_macro 简介

- **导入和基本结构**：proc_macro 提供 TokenStream、Span 等；导入 use proc_macro::TokenStream;。结构包括 TokenStream Vec<TokenTree>、TokenTree 枚举（Group/Ident/Literal/Punct）和 Span 位置。
    - **类型详解**：TokenStream 流，支持 from/into_iter/extend；TokenTree 子类型，Group Delimiter (Brace/Paren/Bracket/None)；Span call_site/mixed_site/join。
    - **函数宏**：#[proc_macro] fn (input: TokenStream) -> TokenStream，生成代码。
    - **属性宏**：#[proc_macro_attribute] fn (attr: TokenStream, item: TokenStream) -> TokenStream，修改项。
    - **派生宏**：#[proc_macro_derive(Name)] fn (input: TokenStream) -> TokenStream，生成 impl。
- **设计哲学扩展**：proc_macro 编译时，生成 Token 而非文本；零成本运行；panic 传播 Err。proc_macro 是 'static，input TokenStream 'static。
- **跨平台详解**：proc lib DLL/so，Windows/Linux 测试加载；Token OS 无依。
- **性能详析**：proc 执行 O(n) Token，复杂 100ms+；基准 rustc -Z time-passes；大 input 慢，用 chunk 处理。
- **常见用例扩展**：trait 派生（serde）、属性注入（rocket）、函数 DSL (lazy_static!)。
- **超级扩展概念**：与 syn::parse 深度 AST；与 quote::ToTokens 生成；错误 proc_macro::compile_error!；与 darling::FromMeta 属性；高性能 quote_fast 替代；与 tracing::instrument 宏日志；历史：从 1.15 experimental 到 1.30 stable。

## 2. 设置环境

创建 proc lib。

Cargo.toml:
```toml
[package]
name = "my_proc"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true

[dependencies]
proc-macro2 = "1.0"
syn = { version = "2.0", features = ["full", "visit-mut", "extra-traits"] }  // full 解析，visit-mut 修改，extra 调试
quote = "1.0"
darling = "0.20"  // attr
thiserror = "1.0"  // 错误
proc-macro-error = "1.0"  // 友好 Err
```

- 解释：proc-macro = true 启用；syn features full/visit-mut/extra 完整/修改/打印。
- 性能：syn full 重，编译慢；用 minimal features 优化。
- 陷阱：无 proc-macro = true Err "not proc"；dep 版本 mismatch syn/quote Err。
- 优化：proc-macro2 no_std 兼容；use proc-macro-error attr 友好 Err。
- 测试：test crate 用 my_proc，#[test] fn t() { let _ = my_macro!(input); }。
- 高级：add build.rs 生成 proc 代码 (meta-meta)；use cargo-sync-readme 文档同步。
- 变体：bin proc-macro 用于工具。

## 3. 基本函数宏

#[proc_macro] fn name(input: TokenStream) -> TokenStream

### 示例: 简单函数
lib.rs
```rust
use proc_macro::TokenStream;

#[proc_macro]
pub fn my_fn(input: TokenStream) -> TokenStream {
    "1 + 2".parse().unwrap()
}
```

使用：
```rust
use my_proc::my_fn;

let x = my_fn!(); // 3 但宏生成代码
```

- 解释：生成固定 Token。性能：<1ms。
- 陷阱：input 忽略，实际用 parse。
- 优化：quote! { 1 + 2 } 生成。
- 测试：test crate 调用 my_fn!() 编译/运行。
- 高级：use syn::Expr parse input 生成动态。
- 变体：use input.is_empty() 检查空。

### 示例: 参数处理
lib.rs
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::LitInt;

#[proc_macro]
pub fn add_one(input: TokenStream) -> TokenStream {
    let num = parse_macro_input!(input as LitInt);
    let val = num.base10_parse::<i32>().unwrap() + 1;
    quote! { #val }
}
```

使用：
```rust
let y = add_one!(41); // 42
```

- 解释：parse_macro_input! 辅助 syn parse；quote 生成 literals。
- 性能：小 input 快。
- 陷阱：无效 input parse Err，用 .unwrap_or_else 返回 compile_error!。
- 优化：quote #val 插值。
- 测试：不同 lit 测试 add_one。
- 高级：use syn::parse::Parse trait 自定义 parse。
- 变体：multi arg 用 punctuated。

## 4. 属性宏

#[proc_macro_attribute] fn name(attr: TokenStream, item: TokenStream) -> TokenStream

### 示例: 简单属性
lib.rs
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn logged(attr: TokenStream, item: TokenStream) -> TokenStream {
    let fn_item = parse_macro_input!(item as ItemFn);
    let fn_name = &fn_item.sig.ident;
    let block = &fn_item.block;

    quote! {
        #fn_item.sig {
            println!("进入 {}", stringify!(#fn_name));
            let result = { #block };
            println!("退出 {}", stringify!(#fn_name));
            result
        }
    }.into()
}
```

使用：
```rust
#[logged]
fn my_fn() {
    println!("inside");
}
```

- 解释：parse ItemFn；quote 包装 block 添加 log。
- 性能：fn 块大 quote 慢。
- 陷阱：async fn 用 quote async。
- 优化：quote_spanned fn_item.span() 位置。
- 测试：test crate 用 #[logged] fn，检查输出。
- 高级：use attr parse_lit 自定义参数。
- 变体：use item.to_string() 简单，但丢失 Span。

### 示例: 属性参数
lib.rs
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};

#[proc_macro_attribute]
pub fn log_level(attr: TokenStream, item: TokenStream) -> TokenStream {
    let level = if attr.is_empty() {
        "info".to_string()
    } else {
        let lit = parse_macro_input!(attr as LitStr);
        lit.value()
    };

    let fn_item = parse_macro_input!(item as ItemFn);
    let fn_name = &fn_item.sig.ident;
    let block = &fn_item.block;

    quote! {
        #fn_item.sig {
            println!("[{}] 进入 {}", #level, stringify!(#fn_name));
            #block
        }
    }.into()
}
```

使用：
```rust
#[log_level = "debug"]
fn my_fn() {}
```

- 解释：parse attr as LitStr。
- 性能：小 attr 快。
- 陷阱：非 str attr parse Err。
- 优化：use darling::FromMeta 多类型 attr。
- 测试：不同 attr 测试 log。
- 高级：use punctuated 多参数。
- 变体：attr TokenStream 用 if attr.is_empty() 默认。

## 4. 派生宏

#[proc_macro_derive(Name, attributes(helper))]

### 示例: 派生 struct
见上。

- 解释：DeriveInput input.data match Struct/Enum/Union。
- 性能：多字段慢。
- 陷阱：vis pub 用 input.vis。
- 优化：quote loop 字段。
- 测试：export derive 测试 impl。
- 高级：use input.attrs 派生条件。
- 变体：enum variant.discriminant 值处理。

### 示例: 枚举派生
lib.rs
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Variant};

#[proc_macro_derive(EnumToString)]
pub fn enum_to_string_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let variants = if let Data::Enum(data_enum) = input.data {
        data_enum.variants.iter().map(|v| {
            let v_name = &v.ident;
            let str_name = v_name.to_string();
            quote! { #name::#v_name => #str_name }
        }).collect::<Vec<_>>()
    } else {
        return quote! { compile_error!("Only enums"); }.into();
    };

    quote! {
        impl #name {
            pub fn to_string(&self) -> &'static str {
                match self {
                    #( #variants , )*
                }
            }
        }
    }.into()
}
```

使用：
```rust
#[derive(EnumToString)]
enum Color {
    Red,
    Green,
}

fn main() {
    println!("{}", Color::Red.to_string()); // "Red"
}
```

- 解释：variants.iter() 生成 match 臂。
- 性能：多变体 quote 慢。
- 陷阱：variant fields 忽略，用 v.fields if empty。
- 优化：quote match self { ... }。
- 测试：enum 测试 to_string。
- 高级：use v.attrs #[to_str = "custom"] 自定义。
- 变体：union 不支，用 panic。

## 5. 错误处理

用 syn::Error 或 compile_error!。

### 示例: syn Error
lib.rs
```rust
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Error};
use quote::quote;

#[proc_macro_derive(ErrDerive)]
pub fn err_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let syn::Data::Union(u) = &input.data {
        return Error::new(u.union_token.span, "Union not supported").to_compile_error().into();
    }

    // 生成
    TokenStream::new()
}
```

- 解释：Error::new(span, msg)；to_compile_error 生成 Token Err。
- 性能：早 Err 减展开。
- 陷阱：span def_site 默认，用 field.span 指定。
- 优化：多个 Error 用 spanned::Spanned。
- 测试：无效 input 测试 Err 消息。
- 高级：use darling Error 辅助。
- 变体：use thiserror derive Err 类型。

## 6. 高级：helper attr、generic、test

- helper attr：attributes(helper) 允许 #[helper]。

### 示例: helper attr
lib.rs
```rust
#[proc_macro_derive(HelperDerive, attributes(helper))]
pub fn helper_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let helper_attrs = input.attrs.iter().filter(|a| a.path.is_ident("helper")).collect::<Vec<_>>();
    // 处理
    TokenStream::new()
}
```

- 解释：filter find helper attr。

- generic：用 split_for_impl。

- test：test crate 用 #[derive] struct 测试方法。

## 7. 最佳实践

- 用 syn/quote/darling/thiserror 栈。
- 处理 generic/attr/err。
- 测试多种输入/边缘。
- 文档 derive/attr。
- 避免 panic，用 to_compile_error。

## 8. 练习

1. 派生 ToVec 为 struct 字段 vec。
2. 处理 enum 变体生成 from_variant 方法。
3. 用 helper #[ignore] 跳过字段。
4. 测试 derive 输出 use snapshot。
5. 基准 derive 时间。
6. 与 darling 解析 helper attr。
7. 错误：无效用 to_compile_error。
8. 高级：实现 Clone for union (custom)。
