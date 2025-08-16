// 教学示例 - 允许未使用的代码
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// 独立运行的控制流示例
// 使用命令：cargo run --bin control_flow

use task::examples;

fn main() {
    println!("🦀 Rust 基础教程 - 控制流示例");
    println!("============================\n");
    
    examples::control_flow();
    
    println!("\n✅ 控制流示例运行完成！");
} 