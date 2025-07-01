// 教学示例 - 允许未使用的代码
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// 独立运行的函数示例
// 使用命令：cargo run --bin functions

use task::examples;

fn main() {
    println!("🦀 Rust 基础教程 - 函数示例");
    println!("==========================\n");
    
    examples::functions_demo();
    
    println!("\n✅ 函数示例运行完成！");
} 