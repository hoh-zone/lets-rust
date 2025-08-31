# proc_macro_derive 教程

Rust 的 `proc_macro_derive` 是过程宏系统的高级组成部分，提供自定义派生宏的 API，用于在编译时为结构体、枚举或联合体自动生成 trait 实现，支持 boilerplate 代码减少和库扩展，而无需手动编写重复 impl。它是 Rust 元编程的强大工具，抽象了 DeriveInput 的解析和 TokenStream 的生成，确保类型安全和效率，并通过编译错误或运行时 panic（如无效输入或栈溢）显式处理问题如字段缺失或泛型不匹配。`proc_macro_derive` 强调编译时代码生成：宏在扩展阶段运行，接收 DeriveInput TokenStream，输出 trait impl TokenStream；支持三种主要用例：简单 trait 实现、字段处理和条件生成；需在 Cargo.toml 中启用 [lib] proc_macro = true，并使用 extern crate proc_macro;。crate 的设计优先灵活性和功率，适用于库如 serde 的 `#[derive(Serialize)]` 或自定义 ORM 实体；相比 macro_rules! 更强大，能处理复杂 AST。`proc_macro_derive` 与 `proc_macro2`（stable TokenStream）、`syn`（AST 解析）、`quote`（代码生成）、`darling`（属性解析）、`std::panic`（宏中 panic 传播）和 `std::attribute`（辅助属性）深度集成，支持高级模式如递归字段处理、自定义错误诊断和 hygienic 名称生成。

## 1. proc_macro_derive 简介

- **导入和基本结构**：对于 proc_macro_derive，无需特殊导入（proc_macro crate 自带），但函数标注 #[proc_macro_derive(Name, attributes(helper))] 以定义，Name 是 derive 名，attributes 允许 helper attr 如 #[helper]。系统基于 TokenStream，内部结构包括 DeriveInput 的 Data (Struct/Enum/Union)、Fields (Named/Unnamed/Unit)、Generics (params/where) 和 Attrs (attributes)。
    - **derive 类型详解**：函数 pub fn name(input: TokenStream) -> TokenStream，input 是 DeriveInput TokenStream。
    - **helper attr**：如 #[proc_macro_derive(MyTrait, attributes(my_attr))]，允许 struct 上 #[my_attr]。
    - **函数式辅助**：用 macro_rules! 辅助 derive 内部逻辑。
- **设计哲学扩展**：proc_macro_derive 遵循 "trait auto-impl"，编译时生成 impl；零成本 Token 操作；panic 在宏传播编译错误。derive 是 'static，input TokenStream 'static。
- **跨平台详解**：derive 展开 compiler 侧，无 OS 依；但 lib DLL/so，测试 Windows/Linux 加载。
- **性能详析**：derive 运行 O(n) 于字段，复杂 syn parse 100ms+；基准 rustc -Z time-passes；大 struct 编译慢，用 mod 分离。
- **常见用例扩展**：自动 Debug/Clone；ORM 实体 SQL；错误 enum 自动 From。
- **超级扩展概念**：与 syn::DeriveInput 集成自定义解析；与 quote::ToTokens 生成；错误 syn::Error to_compile_error；与 darling::FromDeriveInput 辅助 attr；高性能用 quote_fast 快速生成；与 tracing::instrument 装饰宏日志；历史：从 1.15 derive 到 1.30 stable。

## 2. 设置环境

创建 proc-macro lib。

Cargo.toml:
```toml
[package]
name = "my_derive"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true

[dependencies]
proc-macro2 = "1.0"
syn = { version = "2.0", features = ["full", "extra-traits"] }  // full 解析所有，extra-traits 调试
quote = "1.0"
darling = "0.20"  // attr 解析
thiserror = "1.0"  // 错误
```

- 解释：proc-macro = true 启用；syn features full/extra 完整解析/打印。
- 性能：syn full 重，编译慢；用 minimal features 优化。
- 陷阱：无 proc-macro = true Err "not proc-macro"；依赖版本 mismatch syn/quote Err。
- 优化：use proc-macro2 no_std 兼容。
- 测试：单独 test crate 用 my_derive。
- 高级：add build.rs 生成宏代码 (meta-meta)。

## 3. 基本派生宏

函数接收 DeriveInput，生成 impl。

### 示例: 简单派生 Hello
lib.rs
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Hello)]
pub fn hello_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics Hello for #name #ty_generics #where_clause {
            fn hello() {
                println!("Hello from {}", stringify!(#name));
            }
        }
    };

    TokenStream::from(expanded)
}
```

- 解释：parse_macro_input! 宏辅助 parse；split_for_impl 处理泛型；quote! 生成 impl；stringify! 转字符串。
- 性能：小 struct parse <1ms。
- 陷阱：无 generics，泛型 struct Err "no impl"；用 where_clause 支持 bound。
- 优化：quote_spanned #name.span() 位置。
- 测试：test crate derive struct 调用 hello。
- 高级：用 darling 解析 input.attrs 自定义行为。
- 变体：enum 用 match 变体生成。

### 示例: 字段派生 SumFields
lib.rs
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(SumFields)]
pub fn sum_fields_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let sum_code = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => {
                let field_sums = fields.named.iter().map(|f| {
                    let field_name = &f.ident;
                    quote! { sum += self.#field_name as i32; }
                });
                quote! { #(#field_sums)* }
            },
            _ => quote! { compile_error!("Only named fields supported"); },
        },
        _ => quote! { compile_error!("Only structs supported"); },
    };

    let expanded = quote! {
        impl #name {
            pub fn sum_fields(&self) -> i32 {
                let mut sum = 0;
                #sum_code
                sum
            }
        }
    };

    TokenStream::from(expanded)
}
```

使用：
```rust
#[derive(SumFields)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{}", p.sum_fields()); // 3
}
```

- 解释：match Fields 生成 field sum；quote! 重复 #(#field_sums)*。
- 性能：多字段 quote 慢，用 iter collect<String> 辅助。
- 陷阱：unnamed fields 用 index self.0；enum 用 variant match。
- 优化：quote! 用 loop 而非展开大字段。
- 测试：不同 fields struct 测试 sum。
- 高级：用 darling::FromField 解析 field attr 如 #[skip]。
- 变体：union 用 panic 不支持。

### 示例: 枚举派生 VariantCount
lib.rs
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data};

#[proc_macro_derive(VariantCount)]
pub fn variant_count_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let count = if let Data::Enum(data_enum) = input.data {
        data_enum.variants.len()
    } else {
        return quote! { compile_error!("Only enums supported"); }.into();
    };

    quote! {
        impl #name {
            pub const VARIANT_COUNT: usize = #count;
        }
    }.into()
}
```

使用：
```rust
#[derive(VariantCount)]
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    println!("{}", Color::VARIANT_COUNT); // 3
}
```

- 解释：match Data::Enum 计算 variants.len()。
- 性能：enum 变体多 parse 慢。
- 陷阱：variant fields 忽略，只计数。
- 优化：quote const 编译时计算。
- 测试：enum variant 测试 count。
- 高级：用 variant attr #[count = false] 跳过。
- 变体：struct 用 1。

## 4. 处理属性

属性如 #[derive(MyTrait)] #[my_attr = "value"]。

### 示例: 属性处理
lib.rs
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Attribute};

#[proc_macro_derive(AttrDerive, attributes(my_attr))]
pub fn attr_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let attr_value = input.attrs.iter().find(|a| a.path.is_ident("my_attr")).map(|a| a.parse_meta().unwrap().lit()).unwrap_or(syn::Lit::Str(syn::LitStr::new("default", proc_macro::Span::call_site())));

    quote! {
        impl #name {
            pub fn attr_value() -> &'static str {
                #attr_value
            }
        }
    }.into()
}
```

使用：
```rust
#[derive(AttrDerive)]
#[my_attr = "custom"]
struct MyStruct;

fn main() {
    println!("{}", MyStruct::attr_value()); // custom
}
```

- 解释：attrs.iter().find 找 my_attr，parse_meta lit 值。
- 性能：多 attr iter 慢。
- 陷阱：无 attr 默认；lit 类型检查。
- 优化：use darling::FromMeta 解析复杂 attr。
- 测试：不同 attr 测试 value。
- 高级：use a.tokens TokenStream 自定义解析。
- 变体：多 attr 用 filter_map 收集。

## 5. 处理泛型和 where

### 示例: 泛型支持
lib.rs
```rust
#[proc_macro_derive(GenDerive)]
pub fn gen_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut where_clause = where_clause.cloned().unwrap_or_default();
    where_clause.predicates.push(parse_quote! { Self: Sized });

    quote! {
        impl #impl_generics GenDerive for #name #ty_generics #where_clause {
            fn gen() {}
        }
    }.into()
}
```

- 解释：split_for_impl 生成 impl 头；push 附加 bound。
- 性能：generics 大 parse 慢。
- 陷阱：lifetime 参数需处理。
- 优化：quote #where_clause 保留原。
- 测试：泛型 struct 测试 impl。
- 高级：use generics.params iter 处理 lifetime/type/trait_bound。
- 变体：const generics 用 generics.const_params。

## 6. 枚举和联合体

### 示例: 枚举派生
类似 struct，用 Data::Enum，variants.iter() 处理变体。

- 解释：variant.fields 处理字段。
- 性能：多变体 iter 慢。
- 陷阱：discriminant 值用 variant.discriminant。
- 优化：quote match self { Self::Var => ... }。
- 测试：enum 测试派生。
- 高级：use variant.attrs 变体属性。
- 变体：union 用 Data::Union fields。

## 7. 错误处理

### 示例: 错误
lib.rs
```rust
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Error};
use quote::quote;

#[proc_macro_derive(ErrorDerive)]
pub fn error_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let syn::Data::Union(_) = input.data {
        return Error::new(input.span(), "Union not supported").to_compile_error().into();
    }

    // 生成
    TokenStream::new()
}
```

- 解释：Error::new(span, msg) 生成 Err；to_compile_error 生成 TokenStream Err。
- 性能：早 Err 减展开。
- 陷阱：span call_site 默认，用 input.span 指定位置。
- 优化：多 Err 用 spanned 多位置。
- 测试：无效 input 测试 Err 消息。
- 高级：use syn::spanned::Spanned trait 指定 field.span()。
- 变体：use darling Error 辅助 attr Err。

## 8. 高级：helper attr、递归、测试

- helper attr：attributes(helper) 允许 #[helper]。

### 示例: helper
#[proc_macro_derive(My, attributes(helper))]
helper 处理内部逻辑。

- 递归：过程不支，用 loop Token。
- 测试：test crate 用 #[cfg(test)] mod tests { use super::*; #[test] fn t() { /* use derive */ } }。

## 9. 最佳实践

- 用 syn/quote/darling 标准栈。
- 处理 generics/attr/错误。
- 测试多种输入/边缘。
- 文档 derive 用法/attr。
- 避免 panic，用 to_compile_error。

## 10. 练习

1. 派生 Sum 为 struct 字段和。
2. 处理 enum 变体生成 const COUNT。
3. 用 attr #[skip] 跳过字段。
4. 测试 derive 输出 snapshot。
5. 基准 derive 编译时间。
6. 与 darling 解析 attr 辅助。
7. 错误处理：无效 to_compile_error。
8. 高级：实现 Json 用于 enum 变体字符串化。
