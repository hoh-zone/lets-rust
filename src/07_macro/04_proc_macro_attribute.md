# proc_macro_attribute 详细教程

Rust 的 `proc_macro_attribute` 是过程宏系统中用于定义自定义属性宏的机制。它允许开发者在编译时修改或扩展代码项（如函数、结构体、模块等），通过接收属性参数和项 TokenStream，生成新的 TokenStream 来替换或增强原项。这是一种强大的元编程工具，可以用于代码注入、装饰器模式和语法增强。属性宏的签名是 `#[proc_macro_attribute] pub fn name(attr: TokenStream, item: TokenStream) -> TokenStream`，其中 `attr` 是属性参数的 TokenStream，`item` 是被修饰项的 TokenStream，返回值是新的 TokenStream。


## 1. proc_macro_attribute 简介

属性宏用于装饰代码项，例如 `#[my_attr]` fn f() {}，宏可以修改 f 的定义、添加代码或生成新项。

- **优势**：编译时执行，零运行时开销；支持任意 item 修改；可用于测试框架（如 #[test]）、性能提示（如 #[inline]）或自定义注解。
- **限制**：只能用于属性位置；输入 item 必须有效 TokenStream；复杂宏增加编译时间。
- **性能考虑**：宏执行时间取决于 Token 处理，简单宏 <1ms，复杂 syn parse 10-100ms；用 rustc -Z time-passes 测量。
- **跨平台**：宏 compiler 侧，一致；lib 作为 DLL/so，测试加载。
- **测试**：用 proc-macro-test crate 测试属性应用；cargo expand 查看展开代码。
- **常见用例**：添加日志、计时函数、忽略警告、自定义链接。
- **替代**：macro_rules! 属性有限；derive 只 struct/enum。
- **历史**：Rust 1.15 引入，1.30 稳定；1.80+ 优化 Span 处理。

## 2. 设置环境

创建 proc-macro lib 项目。

**Cargo.toml**:
```toml
[package]
name = "my_attr_macro"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true

[dependencies]
proc-macro2 = "1.0"
syn = { version = "2.0", features = ["full", "visit-mut", "extra-traits"] }  // full for complete parsing, visit-mut for modifying AST, extra-traits for debugging
quote = "1.0"
darling = "0.20"  // for attribute parsing
proc-macro-error = "1.0"  // for user-friendly errors
thiserror = "1.0"  // for custom errors
```

- **解释**：proc-macro = true 启用过程宏库；syn features 启用完整解析/修改/调试；darling 简化属性解析；proc-macro-error 提供 attr 友好错误报告。
- **性能**：syn full 增加编译时间 20-50%，但必要复杂宏；用 minimal features 优化简单宏。
- **陷阱**：无 proc-macro = true，编译 Err "not a proc-macro crate"；dep 版本不匹配导致 syn/quote 兼容问题。
- **优化**：使用 proc-macro2 for stable API；避免 unnecessary syn features for lightweight macros。
- **测试设置**：创建单独 test crate 依赖 my_attr_macro，use #[my_attr] in tests。
- **高级**：add build.rs 生成宏代码 (meta-meta programming)；use cargo-sync-readme 同步文档；enable nightly features like proc_macro_span for better error locations。
- **变体**：for bin proc-macro tools, use [bin] but lib for macros。

## 3. 基本属性宏

属性宏修改 item，attr 是参数。

### 示例1: 简单包装函数
lib.rs
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn log_entry(attr: TokenStream, item: TokenStream) -> TokenStream {
    let fn_item = parse_macro_input!(item as ItemFn);
    let fn_name = &fn_item.sig.ident;
    let block = &fn_item.block;

    quote! {
        #fn_item.sig {
            println!("Entering {}", stringify!(#fn_name));
            #block
        }
    }.into()
}
```

使用：
```rust
#[log_entry]
fn my_function() {
    println!("Inside");
}
```

- **解释**：parse_macro_input! 解析 ItemFn；quote! 包装 block 添加 print；stringify! 转 fn_name 字符串。
- **性能**：小 fn parse <1ms，quote O(n) Token。
- **陷阱**：async fn 需要 quote async sig；attr 忽略。
- **优化**：use quote_spanned fn_item.span() 保留位置 for better errors。
- **测试**：test crate use #[log_entry] fn my_test() {}, run check "Entering my_test" output。
- **高级**：use fn_item.vis 保留 visibility 如 pub。
- **变体**：if fn_item.sig.asyncness.is_some() add async log。
- **分析**：this macro adds logging without runtime overhead beyond println!；for production, use tracing crate integration。

### 示例2: 添加返回日志
Extend example1 to log exit.

lib.rs (extend)
```rust
quote! {
    #fn_item.sig {
        println!("Entering {}", stringify!(#fn_name));
        let result = { #block };
        println!("Exiting {}", stringify!(#fn_name));
        result
    }
}
```

- **解释**：wrap block in let result to log exit and return。
- **性能**：negligible added code。
- **陷阱**：void fn no return, use if fn_item.sig.output is -> () no result。
- **优化**：use syn::ReturnType match output。
- **测试**：check output "Entering" and "Exiting"。
- **高级**：add timing with Instant now/elapsed in generated code。
- **变体**：for async, quote async and .await result if needed (but attribute on async fn requires care)。

### 示例3: 属性忽略特定函数
Use attr to conditional log.

lib.rs
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitBool};

#[proc_macro_attribute]
pub fn conditional_log(attr: TokenStream, item: TokenStream) -> TokenStream {
    let enable = if attr.is_empty() {
        true
    } else {
        let lit = parse_macro_input!(attr as LitBool);
        lit.value
    };

    let fn_item = parse_macro_input!(item as ItemFn);
    let fn_name = &fn_item.sig.ident;
    let block = &fn_item.block;

    if enable {
        quote! {
            #fn_item.sig {
                println!("Enter {}", stringify!(#fn_name));
                #block
            }
        }.into()
    } else {
        item
    }
}
```

使用：
```rust
#[conditional_log(false)]
fn no_log() {}
```

- **解释**：parse attr as LitBool；if false return original item。
- **性能**：empty attr fast parse。
- **陷阱**：attr not bool parse Err。
- **优化**：use darling for robust attr parse。
- **测试**：true/false attr check log presence。
- **高级**：use Meta for key=value attr like #[log(enable = true)]。
- **变体**：attr as ident for levels like "debug"。

### 示例4: 修改结构体添加字段
lib.rs
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, FieldsNamed};

#[proc_macro_attribute]
pub fn add_id(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(FieldsNamed { named: ref mut fields, .. }) = item_struct.fields {
        fields.push(parse_quote! { id: u32 });
    } else {
        return quote! { compile_error!("Only named fields"); }.into();
    }

    quote! { #item_struct }.into()
}
```

使用：
```rust
#[add_id]
struct MyStruct {
    name: String,
}

// 展开 struct MyStruct { name: String, id: u32 }
```

- **解释**：parse ItemStruct；push field 添加 id。
- **性能**：小 struct fast。
- **陷阱**：unnamed/unit fields match Err。
- **优化**：quote #item_struct 修改后。
- **测试**：expand check added field。
- **高级**：use visit_mut 修改 AST deeper。
- **变体**：for enum add variant。

### 示例5: 添加方法到 trait
For trait item, add method.

lib.rs
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemTrait, TraitItem, TraitItemMethod};

#[proc_macro_attribute]
pub fn add_trait_method(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_trait = parse_macro_input!(item as ItemTrait);

    item_trait.items.push(TraitItem::Method(TraitItemMethod {
        attrs: vec![],
        sig: parse_quote! { fn added(&self); },
        default: None,
        semi_token: Some(syn::token::Semi { spans: [proc_macro::Span::call_site()] }),
    }));

    quote! { #item_trait }.into()
}
```

使用：
```rust
#[add_trait_method]
trait MyTrait {
    fn existing(&self);
}

// 展开 trait MyTrait { fn existing(&self); fn added(&self); }
```

- **解释**：parse ItemTrait；push TraitItemMethod 添加方法。
- **性能**：小 trait fast。
- **陷阱**：default None for sig only。
- **优化**：use parse_quote! 方便。
- **测试**：expand check added method。
- **高级**：use attr 指定 method name/sig。
- **变体**：for impl add body。

### 示例6: 包装模块添加 use
For module item, add use.

lib.rs
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemMod, Item, UseTree};

#[proc_macro_attribute]
pub fn add_use(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_mod = parse_macro_input!(item as ItemMod);

    if let Some((_, ref mut content)) = item_mod.content {
        content.insert(0, Item::Use(syn::ItemUse {
            attrs: vec![],
            vis: syn::Visibility::Inherited,
            use_token: syn::token::Use { span: proc_macro::Span::call_site() },
            leading_colon: None,
            tree: UseTree::Path(syn::UsePath {
                ident: syn::Ident::new("std", proc_macro::Span::call_site()),
                colon2_token: syn::token::Colon2 { spans: [proc_macro::Span::call_site()] },
                tree: Box::new(UseTree::Name(syn::UseName {
                    ident: syn::Ident::new("collections", proc_macro::Span::call_site()),
                })),
            }),
            semi_token: syn::token::Semi { spans: [proc_macro::Span::call_site()] },
        }));
    }

    quote! { #item_mod }.into()
}
```

使用：
```rust
#[add_use]
mod my_mod {
    // 添加 use std::collections;
}
```

- **解释**：parse ItemMod；insert Item::Use 添加 use。
- **性能**：小 mod fast。
- **陷阱**：no content (extern mod) Err。
- **优化**：parse_quote! Item::Use。
- **测试**：expand check added use。
- **高级**：use attr 指定 use path。
- **变体**：for crate root 添加 use。

### 示例7: 函数参数添加默认
Modify fn sig add default.

lib.rs
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, FnArg, PatType};

#[proc_macro_attribute]
pub fn default_arg(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut fn_item = parse_macro_input!(item as ItemFn);

    for arg in fn_item.sig.inputs.iter_mut() {
        if let FnArg::Typed(PatType { ty, .. }) = arg {
            if **ty == parse_quote! { i32 } {
                // 添加默认 = 0
            }
        }
    }

    quote! { #fn_item }.into()
}
```

- **解释**：iter_mut sig.inputs 修改 arg 添加 default (syn support default 1.39+)。
- **性能**：小 sig fast。
- **陷阱**：default 需 ty 支持。
- **优化**：use visit_mut 修改。
- **测试**：expand check default。
- **高级**：attr 指定 which arg default value。
- **变体**：for method self arg。

### 示例8: 结构体实现 trait
Add impl for struct.

lib.rs
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn impl_trait(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(item as ItemStruct);
    let name = item_struct.ident;

    let impl_code = quote! {
        impl MyTrait for #name {
            fn method(&self) {}
        }
    };

    quote! {
        #item_struct
        #impl_code
    }.into()
}
```

使用：
```rust
#[impl_trait]
struct MyStruct;
```

- **解释**：生成 #item_struct + impl。
- **性能**：fast。
- **陷阱**：duplicate impl Err，用 if not exist。
- **优化**：append to item。
- **测试**：check impl method。
- **高级**：use attr 指定 trait name。
- **变体**：for enum 生成 impl。

### 示例9: 模块添加 item
Add const to mod.

lib.rs
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemMod};

#[proc_macro_attribute]
pub fn add_const(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_mod = parse_macro_input!(item as ItemMod);

    if let Some((_, ref mut content)) = item_mod.content {
        content.push(parse_quote! { const ADDED: i32 = 42; });
    }

    quote! { #item_mod }.into()
}
```

使用：
```rust
#[add_const]
mod my_mod {
    // 添加 const ADDED = 42;
}
```

- **解释**：push parse_quote! Item。
- **性能**：fast。
- **陷阱**：no content extern mod。
- **优化**：use if content.is_some()。
- **测试**：expand check const。
- **高级**：use attr 指定 const value。
- **变体**：add fn 或 struct。

### 示例10: trait 添加 method
lib.rs
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemTrait, TraitItem, TraitItemMethod};

#[proc_macro_attribute]
pub fn add_method(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_trait = parse_macro_input!(item as ItemTrait);

    item_trait.items.push(TraitItem::Method(parse_quote! { fn added(&self); }));

    quote! { #item_trait }.into()
}
```

使用：
```rust
#[add_method]
trait MyTrait {
    fn existing(&self);
}

// 展开 trait MyTrait { fn existing(&self); fn added(&self); }
```

- **解释**：push TraitItemMethod。
- **性能**：fast。
- **陷阱**：semi_token 需要。
- **优化**：parse_quote! 方便。
- **测试**：expand check added method。
- **高级**：use attr 指定 method sig。
- **变体**：add associated type/const。

### 示例11: 处理模块内 item
Use visit to modify inner items.

lib.rs
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemMod, visit_mut::VisitMut};

struct AddPrint;

impl VisitMut for AddPrint {
    fn visit_item_fn_mut(&mut self, i: &mut syn::ItemFn) {
        i.block.stmts.insert(0, parse_quote! { println!("added"); });
    }
}

#[proc_macro_attribute]
pub fn add_print_to_fns(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_mod = parse_macro_input!(item as ItemMod);

    if let Some((_, ref mut content)) = item_mod.content {
        let mut visitor = AddPrint;
        for item in content.iter_mut() {
            visitor.visit_item_mut(item);
        }
    }

    quote! { #item_mod }.into()
}
```

使用：
```rust
#[add_print_to_fns]
mod my_mod {
    fn f1() {}
    fn f2() {}
}

// 展开 fn f1 { println!("added"); } fn f2 { println!("added"); }
```

- **解释**：VisitMut 修改 mod 内 fn 添加 stmt。
- **性能**：mod 大 visit 慢。
- **陷阱**：visit_mut 需要 features ["visit-mut"]。
- **优化**：针对 fn visit_item_fn_mut。
- **测试**：expand check added print。
- **高级**：递归 visit mod 内 mod。
- **变体**：add to struct method。

### 示例12: attr 多参数
Use punctuated parse attr.

lib.rs
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, punctuated::Punctuated, Token, LitStr};

#[proc_macro_attribute]
pub fn multi_attr(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args: Punctuated<LitStr, Token![,]> = Punctuated::parse_terminated.parse(attr).unwrap();
    let messages = args.iter().map(|lit| lit.value()).collect::<Vec<_>>();

    let fn_item = parse_macro_input!(item as ItemFn);
    let block = &fn_item.block;

    let prints = messages.iter().map(|msg| quote! { println!("{}", #msg); });

    quote! {
        #fn_item.sig {
            #(#prints)*
            #block
        }
    }.into()
}
```

使用：
```rust
#[multi_attr("msg1", "msg2")]
fn my_fn() {}
```

- **解释**：Punctuated parse comma sep LitStr。
- **性能**：小 args 快。
- **陷阱**：无 comma 或非 str Err。
- **优化**：use darling FromMeta 多类型。
- **测试**：不同 args 测试 prints。
- **高级**：use MetaList for nested (key = val)。
- **变体**：attr as expr 用 parse::Parser。

### 示例13: 修改 trait impl
For impl item, add method impl.

lib.rs
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemImpl, ImplItem, ImplItemMethod};

#[proc_macro_attribute]
pub fn add_impl_method(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_impl = parse_macro_input!(item as ItemImpl);

    item_impl.items.push(ImplItem::Method(parse_quote! { fn added(&self) { } }));

    quote! { #item_impl }.into()
}
```

使用：
```rust
#[add_impl_method]
impl MyTrait for MyStruct {
    fn existing(&self) {}
}

// 展开 impl MyTrait for MyStruct { fn existing(&self) {} fn added(&self) {} }
```

- **解释**：push ImplItemMethod 添加方法。
- **性能**：fast。
- **陷阱**：sig 匹配 trait。
- **优化**：parse_quote!。
- **测试**：expand check added method。
- **高级**：use attr 指定 body。
- **变体**：for trait def add sig。

### 示例14: 删除 item
Return empty TokenStream delete item.

lib.rs
```rust
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn remove(attr: TokenStream, item: TokenStream) -> TokenStream {
    TokenStream::new()  // 删除 item
}
```

使用：
```rust
#[remove]
fn removed() {}  // 展开为空，移除 fn
```

- **解释**：空返回删除。
- **性能**：fast。
- **陷阱**：删除必要 item Err。
- **优化**：条件删除。
- **测试**：expand check no fn。
- **高级**：use if attr "true" 删除。
- **变体**：return modified or empty。

### 示例15: 注入全局代码
Add const outside item.

lib.rs
```rust
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn inject_const(attr: TokenStream, item: TokenStream) -> TokenStream {
    quote! {
        const INJECTED: i32 = 42;
        #item
    }.into()
}
```

使用：
```rust
#[inject_const]
mod my_mod {}

// 展开 const INJECTED = 42; mod my_mod {}
```

- **解释**：quote 添加 const + #item。
- **性能**：fast。
- **陷阱**：duplicate const Err。
- **优化**：check if exist (no, compile time no)。
- **测试**：expand check const。
- **高级**：inject use 或 extern。
- **变体**：inject in mod if ItemMod。

### 示例16: visit_mut 修改 block
Use visit_mut add stmt to fn block.

lib.rs
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, visit_mut::VisitMut};

struct AddStmt;

impl VisitMut for AddStmt {
    fn visit_block_mut(&mut self, b: &mut syn::Block) {
        b.stmts.insert(0, parse_quote! { println!("added"); });
    }
}

#[proc_macro_attribute]
pub fn add_stmt(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut fn_item = parse_macro_input!(item as ItemFn);
    let mut visitor = AddStmt;
    visitor.visit_item_fn_mut(&mut fn_item);
    quote! { #fn_item }.into()
}
```

使用：
```rust
#[add_stmt]
fn my_fn() {
    // 添加 println!("added");
}
```

- **解释**：VisitMut 修改 block insert stmt。
- **性能**：block 大 visit 慢。
- **陷阱**：features ["visit-mut"] 需要。
- **优化**：针对 block visit_block_mut。
- **测试**：expand check added stmt。
- **高级**：递归 visit all nested block。
- **变体**：add to struct init block。

### 示例17: 处理 trait
Add default method to trait.

lib.rs
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemTrait, TraitItem, TraitItemMethod};

#[proc_macro_attribute]
pub fn default_method(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_trait = parse_macro_input!(item as ItemTrait);

    item_trait.items.push(TraitItem::Method(TraitItemMethod {
        attrs: vec![],
        sig: parse_quote! { fn default_method(&self) { println!("default"); } },
        default: Some(parse_quote! { { println!("default"); } }),
        semi_token: None,
    }));

    quote! { #item_trait }.into()
}
```

使用：
```rust
#[default_method]
trait MyTrait {}

// 展开 trait MyTrait { fn default_method(&self) { println!("default"); } }
```

- **解释**：push TraitItemMethod with default block。
- **性能**：fast。
- **陷阱**：default Some for body。
- **优化**：parse_quote!。
- **测试**：expand check default method。
- **高级**：use attr 指定 body。
- **变体**：add to impl default impl。

### 示例18: 模块级注入
Inject item to mod.

lib.rs
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemMod};

#[proc_macro_attribute]
pub fn inject_item(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_mod = parse_macro_input!(item as ItemMod);

    if let Some((_, ref mut content)) = item_mod.content {
        content.push(parse_quote! { fn injected() { } });
    } else {
        // extern mod, append after
        return quote! { #item_mod fn injected() { } }.into();
    }

    quote! { #item_mod }.into()
}
```

使用：
```rust
#[inject_item]
mod my_mod {}

// 展开 mod my_mod { fn injected() { } }
```

- **解释**：push to content or append。
- **性能**：fast。
- **陷阱**：extern mod no content。
- **优化**：parse_quote! Item。
- **测试**：expand check injected。
- **高级**：inject use 或 const。
- **变体**：inject to crate root。

### 示例19: 泛型 item 处理
For generic fn, preserve generics.

lib.rs
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn gen_attr(attr: TokenStream, item: TokenStream) -> TokenStream {
    let fn_item = parse_macro_input!(item as ItemFn);
    let generics = &fn_item.generics;
    let (impl_gen, ty_gen, where_clause) = generics.split_for_impl();

    // 生成
    quote! { #fn_item }.into()
}
```

- **解释**：split_for_impl 保留 generic。
- **性能**：fast。
- **陷阱**：generic params 处理。
- **优化**：quote #impl_gen 等。
- **测试**：generic fn 测试展开。
- **高级**：add bound to where_clause。
- **变体**：for struct generic。

### 示例20: visit 修改 nested
Use visit_mut modify inner expr.

lib.rs
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, visit_mut::VisitMut, Expr};

struct ReplaceLit;

impl VisitMut for ReplaceLit {
    fn visit_expr_mut(&mut self, e: &mut syn::Expr) {
        if let Expr::Lit(lit) = e {
            if let syn::Lit::Int(int) = &lit.lit {
                if int.base10_parse::<i32>().unwrap() == 42 {
                    *e = parse_quote! { 43 };
                }
            }
        }
    }
}

#[proc_macro_attribute]
pub fn replace_42(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut fn_item = parse_macro_input!(item as ItemFn);
    let mut visitor = ReplaceLit;
    visitor.visit_item_fn_mut(&mut fn_item);
    quote! { #fn_item }.into()
}
```

使用：
```rust
#[replace_42]
fn my_fn() -> i32 {
    42
}
```

展开：
fn my_fn() -> i32 {
43
}

- **解释**：VisitMut 修改 expr lit 42 to 43。
- **性能**：expr 多 visit 慢。
- **陷阱**：features ["visit-mut"] 需要。
- **优化**：针对 expr visit_expr_mut。
- **测试**：expand check replaced。
- **高级**：递归 visit all nested expr。
- **变体**：replace ident 或 type。

## 9. 总结

proc_macro_attribute 是强大装饰工具，结合 syn/quote/darling 高效开发。20 示例覆盖基础到高级，练习应用。
