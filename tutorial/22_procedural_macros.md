# 第22章：过程宏深入解析

过程宏是 Rust 中最强大的元编程工具，可以在编译时操作和生成任意的 Rust 代码，提供了比声明宏更强大的功能。

## 22.1 过程宏的基本概念

### 过程宏的类型

```rust
// 在 Cargo.toml 中需要配置
// [lib]
// proc-macro = true
// 
// [dependencies]
// proc-macro2 = "1.0"
// quote = "1.0"
// syn = { version = "2.0", features = ["full"] }

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

// 1. 派生宏 (derive macros)
#[proc_macro_derive(MyDerive)]
pub fn my_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let expanded = quote! {
        impl #name {
            pub fn hello(&self) {
                println!("Hello from {}", stringify!(#name));
            }
        }
    };
    
    TokenStream::from(expanded)
}

// 2. 属性宏 (attribute macros)
#[proc_macro_attribute]
pub fn my_attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let expanded = quote! {
        #input
        
        impl #name {
            pub fn with_attribute(&self) {
                println!("This struct has the attribute: {}", stringify!(#args));
            }
        }
    };
    
    TokenStream::from(expanded)
}

// 3. 函数式宏 (function-like macros)
#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    
    let expanded = quote! {
        {
            println!("Macro input: {}", #input_str);
            #input
        }
    };
    
    TokenStream::from(expanded)
}
```

### 使用过程宏

```rust
// 使用示例（在另一个 crate 中）
use my_proc_macros::{MyDerive, my_attribute, my_macro};

#[derive(MyDerive)]
struct MyStruct {
    field: i32,
}

#[my_attribute(some_arg)]
struct AttributeStruct {
    data: String,
}

fn main() {
    let s = MyStruct { field: 42 };
    s.hello();
    
    let a = AttributeStruct {
        data: "test".to_string(),
    };
    a.with_attribute();
    
    my_macro! {
        let x = 5;
        println!("x = {}", x);
    }
}
```

## 22.2 编写和使用过程宏

### 自动实现 Debug trait

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, DeriveInput, Data, Fields, 
    FieldsNamed, FieldsUnnamed, FieldsUnit
};

#[proc_macro_derive(CustomDebug)]
pub fn custom_debug_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let debug_impl = match &input.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(FieldsNamed { named, .. }) => {
                    let field_debug = named.iter().map(|field| {
                        let field_name = &field.ident;
                        let field_str = field_name.as_ref().unwrap().to_string();
                        quote! {
                            .field(#field_str, &self.#field_name)
                        }
                    });
                    
                    quote! {
                        impl std::fmt::Debug for #name {
                            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                f.debug_struct(stringify!(#name))
                                    #(#field_debug)*
                                    .finish()
                            }
                        }
                    }
                }
                Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                    let field_debug = unnamed.iter().enumerate().map(|(i, _)| {
                        let index = syn::Index::from(i);
                        quote! {
                            .field(&self.#index)
                        }
                    });
                    
                    quote! {
                        impl std::fmt::Debug for #name {
                            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                f.debug_tuple(stringify!(#name))
                                    #(#field_debug)*
                                    .finish()
                            }
                        }
                    }
                }
                Fields::Unit => {
                    quote! {
                        impl std::fmt::Debug for #name {
                            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                f.debug_struct(stringify!(#name)).finish()
                            }
                        }
                    }
                }
            }
        }
        Data::Enum(data_enum) => {
            let variant_arms = data_enum.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                match &variant.fields {
                    Fields::Named(fields) => {
                        let field_names: Vec<_> = fields.named.iter()
                            .map(|f| f.ident.as_ref().unwrap())
                            .collect();
                        let field_debug = fields.named.iter().map(|field| {
                            let field_name = field.ident.as_ref().unwrap();
                            let field_str = field_name.to_string();
                            quote! {
                                .field(#field_str, #field_name)
                            }
                        });
                        
                        quote! {
                            #name::#variant_name { #(#field_names),* } => {
                                f.debug_struct(&format!("{}::{}", stringify!(#name), stringify!(#variant_name)))
                                    #(#field_debug)*
                                    .finish()
                            }
                        }
                    }
                    Fields::Unnamed(fields) => {
                        let field_names: Vec<_> = (0..fields.unnamed.len())
                            .map(|i| syn::Ident::new(&format!("field{}", i), proc_macro2::Span::call_site()))
                            .collect();
                        
                        quote! {
                            #name::#variant_name(#(#field_names),*) => {
                                f.debug_tuple(&format!("{}::{}", stringify!(#name), stringify!(#variant_name)))
                                    #(.field(#field_names))*
                                    .finish()
                            }
                        }
                    }
                    Fields::Unit => {
                        quote! {
                            #name::#variant_name => {
                                f.debug_struct(&format!("{}::{}", stringify!(#name), stringify!(#variant_name)))
                                    .finish()
                            }
                        }
                    }
                }
            });
            
            quote! {
                impl std::fmt::Debug for #name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        match self {
                            #(#variant_arms)*
                        }
                    }
                }
            }
        }
        Data::Union(_) => {
            quote! {
                compile_error!("CustomDebug 不支持联合体");
            }
        }
    };
    
    TokenStream::from(debug_impl)
}
```

### Builder 模式生成器

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Type};

#[proc_macro_derive(Builder)]
pub fn builder_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let builder_name = syn::Ident::new(&format!("{}Builder", name), name.span());
    
    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Builder 只支持具名字段的结构体"),
        },
        _ => panic!("Builder 只支持结构体"),
    };
    
    // 生成 Builder 结构体的字段
    let builder_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! {
            #field_name: Option<#field_type>
        }
    });
    
    // 生成设置器方法
    let setters = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! {
            pub fn #field_name(mut self, value: #field_type) -> Self {
                self.#field_name = Some(value);
                self
            }
        }
    });
    
    // 生成构建方法
    let build_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            #field_name: self.#field_name.ok_or_else(|| {
                format!("字段 '{}' 未设置", stringify!(#field_name))
            })?
        }
    });
    
    let expanded = quote! {
        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#field_name: None),*
                }
            }
        }
        
        pub struct #builder_name {
            #(#builder_fields),*
        }
        
        impl #builder_name {
            #(#setters)*
            
            pub fn build(self) -> Result<#name, String> {
                Ok(#name {
                    #(#build_fields),*
                })
            }
        }
    };
    
    TokenStream::from(expanded)
}
```

### 序列化宏

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(SimpleSerialize)]
pub fn simple_serialize_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let serialize_impl = match &input.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                    let field_serializations = fields.named.iter().map(|field| {
                        let field_name = &field.ident;
                        let field_str = field_name.as_ref().unwrap().to_string();
                        quote! {
                            result.push_str(&format!("{}:{:?},", #field_str, self.#field_name));
                        }
                    });
                    
                    quote! {
                        impl #name {
                            pub fn serialize(&self) -> String {
                                let mut result = format!("{} {{", stringify!(#name));
                                #(#field_serializations)*
                                result.push('}');
                                result
                            }
                        }
                    }
                }
                _ => quote! {
                    compile_error!("SimpleSerialize 只支持具名字段的结构体");
                }
            }
        }
        _ => quote! {
            compile_error!("SimpleSerialize 只支持结构体");
        }
    };
    
    TokenStream::from(serialize_impl)
}
```

### 性能监控属性宏

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, parse::Parse, Token, LitStr};

struct TimingArgs {
    message: Option<LitStr>,
}

impl Parse for TimingArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(TimingArgs { message: None });
        }
        
        let message = input.parse::<LitStr>()?;
        Ok(TimingArgs { message: Some(message) })
    }
}

#[proc_macro_attribute]
pub fn timing(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as TimingArgs);
    let input_fn = parse_macro_input!(input as ItemFn);
    
    let fn_name = &input_fn.sig.ident;
    let fn_block = &input_fn.block;
    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;
    
    let message = match args.message {
        Some(msg) => msg.value(),
        None => format!("函数 {}", fn_name),
    };
    
    let expanded = quote! {
        #fn_vis #fn_sig {
            let start = std::time::Instant::now();
            let result = (|| #fn_block)();
            let duration = start.elapsed();
            println!("{} 执行时间: {:?}", #message, duration);
            result
        }
    };
    
    TokenStream::from(expanded)
}
```

## 22.3 过程宏的高级应用

### ORM 映射宏

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, DeriveInput, Data, Fields, 
    Attribute, Meta, Lit, NestedMeta
};

fn get_table_name(attrs: &[Attribute], struct_name: &syn::Ident) -> String {
    for attr in attrs {
        if attr.path.is_ident("table") {
            if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                if let Some(NestedMeta::Meta(Meta::NameValue(name_value))) = meta_list.nested.first() {
                    if name_value.path.is_ident("name") {
                        if let Lit::Str(lit_str) = &name_value.lit {
                            return lit_str.value();
                        }
                    }
                }
            }
        }
    }
    // 默认使用结构体名的小写形式
    struct_name.to_string().to_lowercase()
}

fn get_column_name(attrs: &[Attribute], field_name: &syn::Ident) -> String {
    for attr in attrs {
        if attr.path.is_ident("column") {
            if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                if let Some(NestedMeta::Meta(Meta::NameValue(name_value))) = meta_list.nested.first() {
                    if name_value.path.is_ident("name") {
                        if let Lit::Str(lit_str) = &name_value.lit {
                            return lit_str.value();
                        }
                    }
                }
            }
        }
    }
    field_name.to_string()
}

#[proc_macro_derive(Model, attributes(table, column))]
pub fn model_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let table_name = get_table_name(&input.attrs, name);
    
    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Model 只支持具名字段的结构体"),
        },
        _ => panic!("Model 只支持结构体"),
    };
    
    // 生成字段映射
    let field_mappings = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let column_name = get_column_name(&field.attrs, field_name);
        quote! {
            (#column_name, stringify!(#field_name))
        }
    });
    
    // 生成 SQL 查询方法
    let column_names: Vec<_> = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        get_column_name(&field.attrs, field_name)
    }).collect();
    
    let column_list = column_names.join(", ");
    let placeholders = (0..fields.len()).map(|_| "?").collect::<Vec<_>>().join(", ");
    
    let expanded = quote! {
        impl #name {
            pub fn table_name() -> &'static str {
                #table_name
            }
            
            pub fn field_mappings() -> Vec<(&'static str, &'static str)> {
                vec![#(#field_mappings),*]
            }
            
            pub fn select_all_sql() -> String {
                format!("SELECT {} FROM {}", #column_list, #table_name)
            }
            
            pub fn insert_sql() -> String {
                format!("INSERT INTO {} ({}) VALUES ({})", 
                    #table_name, #column_list, #placeholders)
            }
            
            pub fn find_by_id_sql() -> String {
                format!("SELECT {} FROM {} WHERE id = ?", #column_list, #table_name)
            }
        }
    };
    
    TokenStream::from(expanded)
}
```

### API 路由生成宏

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Attribute, Meta, Lit};

fn extract_route_info(attrs: &[Attribute]) -> Option<(String, String)> {
    for attr in attrs {
        if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
            if meta_list.path.is_ident("route") {
                let mut method = None;
                let mut path = None;
                
                for nested in &meta_list.nested {
                    if let syn::NestedMeta::Meta(Meta::NameValue(name_value)) = nested {
                        if name_value.path.is_ident("method") {
                            if let Lit::Str(lit_str) = &name_value.lit {
                                method = Some(lit_str.value());
                            }
                        } else if name_value.path.is_ident("path") {
                            if let Lit::Str(lit_str) = &name_value.lit {
                                path = Some(lit_str.value());
                            }
                        }
                    }
                }
                
                if let (Some(m), Some(p)) = (method, path) {
                    return Some((m, p));
                }
            }
        }
    }
    None
}

#[proc_macro_attribute]
pub fn api_route(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_name = &input_fn.sig.ident;
    
    if let Some((method, path)) = extract_route_info(&input_fn.attrs) {
        let route_registration = syn::Ident::new(
            &format!("register_{}", fn_name), 
            fn_name.span()
        );
        
        let expanded = quote! {
            #input_fn
            
            pub fn #route_registration(router: &mut Router) {
                router.route(#path, #method, #fn_name);
            }
        };
        
        TokenStream::from(expanded)
    } else {
        // 如果没有路由信息，直接返回原函数
        TokenStream::from(quote! { #input_fn })
    }
}
```

### 状态机生成宏

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse::Parse, Token, Ident, 
    punctuated::Punctuated, braced, parenthesized
};

struct StateMachine {
    name: Ident,
    states: Vec<Ident>,
    transitions: Vec<Transition>,
}

struct Transition {
    from: Ident,
    to: Ident,
    event: Ident,
}

impl Parse for StateMachine {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![struct]>()?;
        let name = input.parse::<Ident>()?;
        
        let content;
        braced!(content in input);
        
        // 解析状态
        content.parse::<Token![states]>()?;
        content.parse::<Token![:]>()?;
        let states_content;
        braced!(states_content in content);
        let states = Punctuated::<Ident, Token![,]>::parse_terminated(&states_content)?
            .into_iter()
            .collect();
        
        // 解析转换
        content.parse::<Token![transitions]>()?;
        content.parse::<Token![:]>()?;
        let transitions_content;
        braced!(transitions_content in content);
        
        let mut transitions = Vec::new();
        while !transitions_content.is_empty() {
            let from = transitions_content.parse::<Ident>()?;
            transitions_content.parse::<Token![->]>()?;
            let to = transitions_content.parse::<Ident>()?;
            transitions_content.parse::<Token![on]>()?;
            let event = transitions_content.parse::<Ident>()?;
            
            transitions.push(Transition { from, to, event });
            
            if transitions_content.peek(Token![,]) {
                transitions_content.parse::<Token![,]>()?;
            }
        }
        
        Ok(StateMachine { name, states, transitions })
    }
}

#[proc_macro]
pub fn state_machine(input: TokenStream) -> TokenStream {
    let state_machine = parse_macro_input!(input as StateMachine);
    let name = &state_machine.name;
    let states = &state_machine.states;
    
    // 生成状态枚举
    let state_enum_name = syn::Ident::new(&format!("{}State", name), name.span());
    
    // 生成转换方法
    let transition_methods = state_machine.transitions.iter().map(|transition| {
        let method_name = &transition.event;
        let from_state = &transition.from;
        let to_state = &transition.to;
        
        quote! {
            pub fn #method_name(&mut self) -> Result<(), String> {
                match self.state {
                    #state_enum_name::#from_state => {
                        self.state = #state_enum_name::#to_state;
                        Ok(())
                    }
                    _ => Err(format!("不能从状态 {:?} 执行事件 {}", 
                        self.state, stringify!(#method_name)))
                }
            }
        }
    });
    
    let expanded = quote! {
        #[derive(Debug, Clone, PartialEq)]
        pub enum #state_enum_name {
            #(#states),*
        }
        
        pub struct #name {
            state: #state_enum_name,
        }
        
        impl #name {
            pub fn new() -> Self {
                Self {
                    state: #state_enum_name::#(#states)*, // 使用第一个状态作为初始状态
                }
            }
            
            pub fn current_state(&self) -> &#state_enum_name {
                &self.state
            }
            
            #(#transition_methods)*
        }
    };
    
    TokenStream::from(expanded)
}
```

## RWO 权限分析

### 过程宏生成的所有权代码

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(OwnershipAnalysis)]
pub fn ownership_analysis_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => &fields.named,
            _ => return TokenStream::new(),
        },
        _ => return TokenStream::new(),
    };
    
    // 生成不同权限的访问方法
    let read_methods = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        let method_name = syn::Ident::new(&format!("read_{}", field_name.as_ref().unwrap()), field_name.span());
        
        quote! {
            // R: 只读访问
            pub fn #method_name(&self) -> &#field_type {
                &self.#field_name
            }
        }
    });
    
    let write_methods = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        let method_name = syn::Ident::new(&format!("write_{}", field_name.as_ref().unwrap()), field_name.span());
        
        quote! {
            // W: 可写访问
            pub fn #method_name(&mut self) -> &mut #field_type {
                &mut self.#field_name
            }
        }
    });
    
    let own_methods = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        let take_method = syn::Ident::new(&format!("take_{}", field_name.as_ref().unwrap()), field_name.span());
        let replace_method = syn::Ident::new(&format!("replace_{}", field_name.as_ref().unwrap()), field_name.span());
        
        quote! {
            // O: 获取所有权（需要 Default）
            pub fn #take_method(&mut self) -> #field_type 
            where 
                #field_type: Default 
            {
                std::mem::take(&mut self.#field_name)
            }
            
            // O: 替换并返回旧值
            pub fn #replace_method(&mut self, new_value: #field_type) -> #field_type {
                std::mem::replace(&mut self.#field_name, new_value)
            }
        }
    });
    
    let expanded = quote! {
        impl #name {
            #(#read_methods)*
            #(#write_methods)*
            #(#own_methods)*
            
            // 整体克隆（如果所有字段都实现了 Clone）
            pub fn clone_all(&self) -> Self 
            where 
                Self: Clone 
            {
                self.clone()
            }
            
            // 消费自身，返回所有字段
            pub fn into_parts(self) -> (#(#field_type),*) {
                (self.#field_name),*
            }
        }
    };
    
    TokenStream::from(expanded)
}
```

### 安全的智能指针生成

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, Fields};

#[proc_macro_attribute]
pub fn safe_pointer(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(input as ItemStruct);
    let name = &input_struct.ident;
    let pointer_name = syn::Ident::new(&format!("{}Pointer", name), name.span());
    
    let fields = match &input_struct.fields {
        Fields::Named(fields) => &fields.named,
        _ => panic!("safe_pointer 只支持具名字段"),
    };
    
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    let field_names: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();
    
    let expanded = quote! {
        #input_struct
        
        pub struct #pointer_name<'a> {
            inner: &'a mut #name,
        }
        
        impl<'a> #pointer_name<'a> {
            // R: 创建智能指针，借用原始数据
            pub fn new(inner: &'a mut #name) -> Self {
                Self { inner }
            }
            
            // R: 获取不可变引用
            pub fn as_ref(&self) -> &#name {
                self.inner
            }
            
            // W: 获取可变引用
            pub fn as_mut(&mut self) -> &mut #name {
                self.inner
            }
            
            // 为每个字段生成安全访问器
            #(
                pub fn #field_names(&self) -> &#field_types {
                    &self.inner.#field_names
                }
                
                paste::paste! {
                    pub fn [<#field_names _mut>](&mut self) -> &mut #field_types {
                        &mut self.inner.#field_names
                    }
                }
            )*
        }
        
        impl<'a> std::ops::Deref for #pointer_name<'a> {
            type Target = #name;
            
            fn deref(&self) -> &Self::Target {
                self.inner
            }
        }
        
        impl<'a> std::ops::DerefMut for #pointer_name<'a> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.inner
            }
        }
    };
    
    TokenStream::from(expanded)
}
```

### 生命周期分析宏

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Signature, FnArg, ReturnType, Type};

fn extract_lifetimes(sig: &Signature) -> Vec<syn::Lifetime> {
    let mut lifetimes = Vec::new();
    
    // 从泛型参数中提取生命周期
    for param in &sig.generics.params {
        if let syn::GenericParam::Lifetime(lifetime_def) = param {
            lifetimes.push(lifetime_def.lifetime.clone());
        }
    }
    
    lifetimes
}

fn analyze_borrowing(sig: &Signature) -> Vec<String> {
    let mut analysis = Vec::new();
    
    for (i, input) in sig.inputs.iter().enumerate() {
        match input {
            FnArg::Typed(pat_type) => {
                match &*pat_type.ty {
                    Type::Reference(type_ref) => {
                        let mutability = if type_ref.mutability.is_some() {
                            "可变"
                        } else {
                            "不可变"
                        };
                        analysis.push(format!("参数 {}: {} 借用", i, mutability));
                    }
                    _ => {
                        analysis.push(format!("参数 {}: 所有权转移", i));
                    }
                }
            }
            FnArg::Receiver(receiver) => {
                if receiver.reference.is_some() {
                    let mutability = if receiver.mutability.is_some() {
                        "可变"
                    } else {
                        "不可变"
                    };
                    analysis.push(format!("self: {} 借用", mutability));
                } else {
                    analysis.push("self: 所有权转移".to_string());
                }
            }
        }
    }
    
    // 分析返回类型
    match &sig.output {
        ReturnType::Type(_, return_type) => {
            match &**return_type {
                Type::Reference(_) => {
                    analysis.push("返回: 借用引用".to_string());
                }
                _ => {
                    analysis.push("返回: 所有权转移".to_string());
                }
            }
        }
        ReturnType::Default => {
            analysis.push("返回: ()".to_string());
        }
    }
    
    analysis
}

#[proc_macro_attribute]
pub fn analyze_ownership(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let analysis = analyze_borrowing(&input_fn.sig);
    
    let analysis_items = analysis.iter().map(|item| {
        quote! {
            println!("  {}", #item);
        }
    });
    
    let expanded = quote! {
        #input_fn
        
        // 生成分析函数
        paste::paste! {
            pub fn [<analyze_ #fn_name>]() {
                println!("函数 {} 的所有权分析:", stringify!(#fn_name));
                #(#analysis_items)*
            }
        }
    };
    
    TokenStream::from(expanded)
}
```

### 内存安全检查宏

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(MemorySafe)]
pub fn memory_safe_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => &fields.named,
            _ => return TokenStream::new(),
        },
        _ => return TokenStream::new(),
    };
    
    // 生成内存安全的方法
    let safe_methods = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        let get_method = syn::Ident::new(&format!("safe_get_{}", field_name.as_ref().unwrap()), field_name.span());
        let set_method = syn::Ident::new(&format!("safe_set_{}", field_name.as_ref().unwrap()), field_name.span());
        
        quote! {
            // 安全的获取方法
            pub fn #get_method(&self) -> Option<&#field_type> {
                Some(&self.#field_name)
            }
            
            // 安全的设置方法
            pub fn #set_method(&mut self, value: #field_type) -> Result<#field_type, String> {
                let old_value = std::mem::replace(&mut self.#field_name, value);
                Ok(old_value)
            }
        }
    });
    
    let expanded = quote! {
        impl #name {
            #(#safe_methods)*
            
            // 内存安全检查
            pub fn memory_check(&self) -> bool {
                // 基本的内存有效性检查
                true
            }
            
            // 安全的克隆
            pub fn safe_clone(&self) -> Result<Self, String> 
            where 
                Self: Clone 
            {
                Ok(self.clone())
            }
            
            // 安全的销毁
            pub fn safe_destroy(self) -> Result<(), String> {
                drop(self);
                Ok(())
            }
        }
    };
    
    TokenStream::from(expanded)
}
```

## 小结

本章深入学习了过程宏：

1. **过程宏基础**：
   - 三种类型：派生宏、属性宏、函数式宏
   - TokenStream 处理
   - syn 和 quote 库的使用

2. **实用过程宏**：
   - Debug trait 自动实现
   - Builder 模式生成
   - 序列化宏
   - 性能监控宏

3. **高级应用**：
   - ORM 映射
   - API 路由生成
   - 状态机生成
   - 复杂的代码生成

4. **RWO 权限分析**：
   - **R**：过程宏可以生成提供借用访问的代码
   - **W**：生成可变借用和修改功能的代码
   - **O**：处理所有权转移、获取和释放
   - 过程宏在编译时分析和生成代码
   - 保持 Rust 的内存安全保证
   - 生成的代码遵循所有权规则

过程宏是 Rust 最强大的元编程工具，允许我们在编译时生成复杂的代码，同时保持类型安全和内存安全。通过合理使用过程宏，我们可以大大减少样板代码，提高开发效率。

这样，我们就完成了 Rust 教程的全部 22 章内容，从基础语法到高级特性，从内存管理到并发编程，从智能指针到宏系统，全面覆盖了 Rust 语言的核心概念和实用技能。 