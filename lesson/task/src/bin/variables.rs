// 教学示例 - 允许未使用的代码
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// 独立运行的变量与常量示例
// 使用命令：cargo run --bin variables

use task::examples;

fn main() {
    println!("🦀 Rust 基础教程 - 变量与常量示例");
    println!("================================\n");
    
    examples::variables_and_constants();
    
    println!("\n✅ 变量与常量示例运行完成！");
} 