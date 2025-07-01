// 教学示例 - 允许未使用的代码
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// 独立运行的数据类型示例
// 使用命令：cargo run --bin data_types

use task::examples;

fn main() {
    println!("🦀 Rust 基础教程 - 数据类型示例");
    println!("==============================\n");
    
    examples::data_types();
    
    println!("\n✅ 数据类型示例运行完成！");
} 