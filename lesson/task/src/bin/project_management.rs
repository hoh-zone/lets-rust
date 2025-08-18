// 教学示例 - 允许未使用的代码
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// 第13章：项目管理
// 演示 Rust 的模块系统、包管理和工作空间

use std::collections::HashMap;

fn main() {
    println!("🏗️ 第13章：项目管理");
    println!("=====================================");
    
    // 1. 模块系统演示
    module_system_demo();
    
    // 2. 可见性控制演示
    visibility_demo();
    
    // 3. use 语句演示
    use_statements_demo();
    
    // 4. 包和 crate 概念
    package_crate_demo();
}

// ============================================================================
// 1. 模块系统演示
// ============================================================================

fn module_system_demo() {
    println!("\n📦 1. 模块系统演示");
    println!("{}", "-".repeat(40));
    
    // 使用不同模块中的功能
    restaurant::eat_at_restaurant();
    
    // 使用嵌套模块
    println!("\n🏪 餐厅管理系统：");
    restaurant::front_of_house::hosting::add_to_waitlist();
    restaurant::front_of_house::hosting::seat_at_table();
    
    // 使用后厨模块
    restaurant::back_of_house::prepare_order();
}

// 餐厅模块
mod restaurant {
    // 公开函数
    pub fn eat_at_restaurant() {
        println!("🍽️ 顾客来餐厅用餐");
        
        // 绝对路径调用
        crate::restaurant::front_of_house::hosting::add_to_waitlist();
        
        // 相对路径调用
        front_of_house::hosting::add_to_waitlist();
        
        // 调用后厨
        back_of_house::prepare_order();
    }
    
    // 前厅模块
    pub mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {
                println!("  📝 添加到等待列表");
            }
            
            pub fn seat_at_table() {
                println!("  🪑 安排座位");
            }
        }
        
        pub mod serving {
            pub fn take_order() {
                println!("  📋 接受订单");
            }
            
            pub fn serve_order() {
                println!("  🍽️ 上菜");
            }
            
            pub fn take_payment() {
                println!("  💰 收款");
            }
        }
    }
    
    // 后厨模块
    pub mod back_of_house {
        pub fn prepare_order() {
            println!("  👨‍🍳 准备订单");
            cook_order();
        }
        
        fn cook_order() {
            println!("  🔥 烹饪中...");
        }
        
        // 公开的结构体
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String, // 私有字段
        }
        
        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("桃子"),
                }
            }
        }
        
        // 公开的枚举（所有变体都是公开的）
        pub enum Appetizer {
            Soup,
            Salad,
        }
    }
}

// ============================================================================
// 2. 可见性控制演示
// ============================================================================

fn visibility_demo() {
    println!("\n👁️ 2. 可见性控制演示");
    println!("{}", "-".repeat(40));
    
    // 使用公开的结构体
    let mut meal = restaurant::back_of_house::Breakfast::summer("黑麦面包");
    meal.toast = String::from("小麦面包");
    println!("🍞 早餐：{} 配时令水果", meal.toast);
    
    // 使用公开的枚举
    let order1 = restaurant::back_of_house::Appetizer::Soup;
    let _order2 = restaurant::back_of_house::Appetizer::Salad;
    
    match order1 {
        restaurant::back_of_house::Appetizer::Soup => println!("🍲 点了汤"),
        restaurant::back_of_house::Appetizer::Salad => println!("🥗 点了沙拉"),
    }
    
    // 演示不同的可见性级别
    visibility_levels::demo();
}

mod visibility_levels {
    pub fn demo() {
        println!("\n🔒 可见性级别示例：");
        
        // pub(crate) - 整个 crate 内可见
        pub(crate) fn crate_visible() {
            println!("  📦 crate 内可见的函数");
        }
        
        // pub(super) - 父模块内可见
        pub(super) fn parent_visible() {
            println!("  ⬆️ 父模块内可见的函数");
        }
        
        // pub(in path) - 指定路径内可见
        pub(in crate::visibility_levels) fn path_visible() {
            println!("  🛤️ 指定路径内可见的函数");
        }
        
        crate_visible();
        parent_visible();
        path_visible();
    }
}

// ============================================================================
// 3. use 语句演示
// ============================================================================

fn use_statements_demo() {
    println!("\n📥 3. use 语句演示");
    println!("{}", "-".repeat(40));
    
    // 基本 use
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    println!("📋 HashMap: {:?}", map);
    
    // 重命名导入
    use std::collections::HashMap as Map;
    let mut renamed_map = Map::new();
    renamed_map.insert("renamed", "value");
    println!("🏷️ 重命名的 Map: {:?}", renamed_map);
    
    // 嵌套路径
    use std::{
        collections::{HashMap as HMap, BTreeMap},
        io::{self, Write},
    };
    
    let mut btree = BTreeMap::new();
    btree.insert("b", 2);
    btree.insert("a", 1);
    println!("🌳 BTreeMap (有序): {:?}", btree);
    
    // glob 导入（谨慎使用）
    use std::collections::*;
    let mut hash_set = HashSet::new();
    hash_set.insert("item1");
    hash_set.insert("item2");
    println!("🎯 HashSet: {:?}", hash_set);
    
    // 重导出
    pub use restaurant::front_of_house::hosting;
    hosting::add_to_waitlist();
    
    // 抑制未使用警告
    let _ = io::stdout();
}

// ============================================================================
// 4. 包和 crate 概念
// ============================================================================

fn package_crate_demo() {
    println!("\n📦 4. 包和 crate 概念");
    println!("{}", "-".repeat(40));
    
    println!("🏗️ Rust 项目结构：");
    println!("  📁 my_project/");
    println!("  ├── 📄 Cargo.toml        # 包配置文件");
    println!("  ├── 📁 src/");
    println!("  │   ├── 📄 main.rs       # 二进制 crate 根");
    println!("  │   ├── 📄 lib.rs        # 库 crate 根");
    println!("  │   └── 📁 bin/          # 额外的二进制文件");
    println!("  │       └── 📄 another.rs");
    println!("  ├── 📁 tests/            # 集成测试");
    println!("  ├── 📁 examples/         # 示例代码");
    println!("  └── 📁 benches/          # 基准测试");
    
    println!("\n📋 Cargo.toml 配置示例：");
    println!(r#"
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
tokio = {{ version = "1", features = ["full"] }}

[[bin]]
name = "my_app"
path = "src/main.rs"

[workspace]
members = ["crate1", "crate2"]
"#);
    
    println!("\n🎯 crate 类型：");
    println!("  📚 库 crate：提供功能给其他程序使用");
    println!("  🚀 二进制 crate：可执行程序");
    
    // 演示条件编译
    conditional_compilation_demo();
}

fn conditional_compilation_demo() {
    println!("\n🔧 条件编译示例：");
    
    #[cfg(target_os = "windows")]
    fn platform_specific() {
        println!("  🪟 Windows 特定代码");
    }
    
    #[cfg(target_os = "linux")]
    fn platform_specific() {
        println!("  🐧 Linux 特定代码");
    }
    
    #[cfg(target_os = "macos")]
    fn platform_specific() {
        println!("  🍎 macOS 特定代码");
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    fn platform_specific() {
        println!("  🌍 其他平台代码");
    }
    
    platform_specific();
    
    #[cfg(debug_assertions)]
    println!("  🐛 调试模式");
    
    #[cfg(not(debug_assertions))]
    println!("  🚀 发布模式");
    
    println!("\n📝 项目管理最佳实践：");
    println!("  ✅ 使用清晰的模块层次结构");
    println!("  ✅ 合理控制可见性");
    println!("  ✅ 使用 use 语句简化路径");
    println!("  ✅ 遵循 Rust 命名约定");
    println!("  ✅ 编写良好的文档");
    println!("  ✅ 使用工作空间管理大型项目");
} 