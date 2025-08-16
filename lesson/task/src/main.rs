// Rust 基础教程 - 交互式代码示例
// 完整的20章教程，所有示例都可以直接运行

use std::io::{self, Write};
use std::process;

mod examples;

fn main() {
    show_welcome();
    
    loop {
        show_menu();
        
        let choice = get_user_input();
        
        // 清屏
        clear_screen();
        
        match choice.as_str() {
            "1" => run_chapter_with_explanation("第1章：变量与常量", ChapterInfo::Variables, || examples::variables_and_constants()),
            "2" => run_chapter_with_explanation("第2章：数据类型", ChapterInfo::DataTypes, || examples::data_types()),
            "3" => run_chapter_with_explanation("第3章：函数", ChapterInfo::Functions, || examples::functions_demo()),
            "4" => run_chapter_with_explanation("第4章：控制流", ChapterInfo::ControlFlow, || examples::control_flow()),
            "5" => run_chapter_with_explanation("第5章：内存管理", ChapterInfo::Memory, || examples::memory_management()),
            "6" => run_chapter_with_explanation("第6章：所有权", ChapterInfo::Ownership, || examples::ownership()),
            "7" => run_chapter_with_explanation("第7章：借用机制", ChapterInfo::Borrowing, || examples::borrowing()),
            "8" => run_chapter_with_explanation("第8章：结构体", ChapterInfo::Structs, || examples::structs()),
            "9" => run_chapter_with_explanation("第9章：常用类型", ChapterInfo::CommonTypes, || examples::common_types()),
            "10" => run_chapter_with_explanation("第10章：枚举", ChapterInfo::Enums, || examples::enums()),
            "11" => run_chapter_with_explanation("第11章：泛型与特征", ChapterInfo::Generics, || examples::generics_traits()),
            "12" => run_chapter_with_explanation("第12章：生命周期", ChapterInfo::Lifetimes, || examples::lifetimes()),
            "13" => run_chapter_with_explanation("第13章：项目管理", ChapterInfo::ProjectManagement, || examples::project_management()),
            "14" => run_chapter_with_explanation("第14章：文档与测试", ChapterInfo::DocsAndTesting, || examples::docs_and_testing()),
            "15" => run_chapter_with_explanation("第15章：闭包", ChapterInfo::Closures, || examples::closures()),
            "16" => run_chapter_with_explanation("第16章：迭代器", ChapterInfo::Iterators, || examples::iterators()),
            "17" => run_chapter_with_explanation("第17章：智能指针", ChapterInfo::SmartPointers, || examples::smart_pointers()),
            "18" => run_chapter_with_explanation("第18章：常用智能指针", ChapterInfo::CommonSmartPointers, || examples::common_smart_pointers()),
            "19" => run_chapter_with_explanation("第19章：并发编程", ChapterInfo::Concurrency, || examples::concurrency()),
            "20" => run_chapter_with_explanation("第20章：Unsafe Rust", ChapterInfo::UnsafeRust, || examples::unsafe_rust()),
            "21" => run_chapter_with_explanation("第21章：宏系统", ChapterInfo::Macros, || examples::macros()),
            "22" => run_chapter_with_explanation("第22章：过程宏", ChapterInfo::ProceduralMacros, || examples::procedural_macros()),
            "0" => run_all_examples(),
            "basic" => run_basic_examples(),
            "advanced" => run_advanced_examples(),
            "demo" => run_interactive_demo(),
            "help" | "h" => show_command_info(),
            "exit" | "quit" | "q" => {
                println!("\n👋 感谢使用 Rust 基础教程！");
                println!("🎉 希望您已经掌握了 Rust 编程的核心概念！");
                println!("📚 继续学习：https://doc.rust-lang.org/book/");
                break;
            },
            "" => continue,
            other => {
                if let Ok(num) = other.parse::<usize>() {
                    if num > 22 {
                        println!("❌ 章节编号超出范围！请输入 1-22 之间的数字");
                    } else {
                        println!("❌ 无效的章节编号：{}", num);
                    }
                } else {
                    println!("❌ 无效选择：{}！输入 'help' 查看所有可用命令", other);
                }
                println!("💡 提示：输入数字 1-22 选择章节，或输入 q 退出");
                wait_for_enter();
            }
        }
    }
}

fn clear_screen() {
    // 在不同平台上清屏
    if cfg!(target_os = "windows") {
        let _ = process::Command::new("cmd").args(&["/C", "cls"]).status();
    } else {
        let _ = process::Command::new("clear").status();
    }
    
    // 如果清屏命令失败，使用换行符模拟
    if !is_clear_successful() {
        print!("\n{}", "\n".repeat(2));
    }
}

fn is_clear_successful() -> bool {
    // 简单的检查方法，实际中可能需要更复杂的逻辑
    true
}

#[derive(Debug)]
enum ChapterInfo {
    Variables,
    DataTypes,
    Functions,
    ControlFlow,
    Memory,
    Ownership,
    Borrowing,
    Structs,
    CommonTypes,
    Enums,
    Generics,
    Lifetimes,
    ProjectManagement,
    DocsAndTesting,
    Closures,
    Iterators,
    SmartPointers,
    CommonSmartPointers,
    Concurrency,
    UnsafeRust,
    Macros,
    ProceduralMacros,
}

fn show_welcome() {
    println!("🦀 欢迎来到 Rust 基础教程交互式示例！");
    println!("=====================================");
    println!("📚 本教程包含20个完整章节，从基础到高级");
    println!("🎯 每个示例都可以直接运行，并包含详细的知识点讲解");
    println!("💡 输入章节号码开始学习，输入 'help' 查看帮助");
    println!("🚀 输入 'demo' 体验交互式演示");
    println!();
}

fn show_menu() {
    println!("┌────────────────────────────────────────────────────┐");
    println!("│               🦀 Rust 教程菜单                     │");
    println!("├────────────────────────────────────────────────────┤");
    println!("│ 🔰 基础教程 (第1-4章)                              │");
    println!("│  1. 变量与常量  2. 数据类型  3. 函数  4. 控制流    │");
    println!("├────────────────────────────────────────────────────┤");
    println!("│ 🚀 核心概念 (第5-8章)                              │");
    println!("│  5. 内存管理  6. 所有权  7. 借用机制  8. 结构体     │");
    println!("├────────────────────────────────────────────────────┤");
    println!("│ 📦 高级特性 (第9-12章)                             │");
    println!("│  9. 常用类型  10. 枚举  11. 泛型特征  12. 生命周期  │");
    println!("├────────────────────────────────────────────────────┤");
    println!("│ 🏗️ 项目开发 (第13-17章)                            │");
    println!("│  13. 项目管理  14. 文档测试  15. 闭包  16. 迭代器  │");
    println!("│  17. 智能指针                                      │");
    println!("├────────────────────────────────────────────────────┤");
    println!("│ 🎭 专业主题 (第18-22章)                            │");
    println!("│  18. 常用智能指针  19. 并发编程  20. Unsafe Rust   │");
    println!("│  21. 宏系统  22. 过程宏                            │");
    println!("├────────────────────────────────────────────────────┤");
    println!("│ 🎯 快速选项                                        │");
    println!("│  0: 运行所有章节  basic: 基础教程  advanced: 进阶  │");
    println!("│  demo: 交互演示  help: 详细帮助  q: 退出程序       │");
    println!("└────────────────────────────────────────────────────┘");
    print!("🎓 请选择章节 (1-22) 或命令: ");
    io::stdout().flush().unwrap();
}

fn run_chapter_with_explanation<F>(name: &str, chapter: ChapterInfo, example_fn: F) 
where 
    F: FnOnce(),
{
    // 显示章节介绍
    show_chapter_introduction(&chapter);
    
    // 运行代码示例
    println!("\n🔸 正在运行：{}", name);
    println!("{}", "═".repeat(60));
    example_fn();
    println!("{}", "═".repeat(60));
    println!("✅ {} 示例运行完成！", name);
    
    // 显示知识点总结
    show_chapter_summary(&chapter);
    
    // 显示相关命令
    show_related_commands(&chapter);
    
    wait_for_enter();
}

fn show_chapter_introduction(chapter: &ChapterInfo) {
    println!("\n📚 章节介绍");
    println!("{}", "─".repeat(30));
    
    match chapter {
        ChapterInfo::Variables => {
            println!("🎯 学习目标：");
            println!("  • 理解变量的声明和赋值");
            println!("  • 掌握可变性 (mut) 的概念");
            println!("  • 学习常量和静态变量的区别");
            println!("  • 了解变量遮蔽 (shadowing)");
            println!("\n💡 核心概念：");
            println!("  • let 关键字：声明变量");
            println!("  • mut 关键字：使变量可变");
            println!("  • const 关键字：编译时常量");
            println!("  • static 关键字：全局变量");
        },
        ChapterInfo::DataTypes => {
            println!("🎯 学习目标：");
            println!("  • 掌握 Rust 的基本数据类型");
            println!("  • 理解标量类型和复合类型");
            println!("  • 学习类型推断和显式标注");
            println!("  • 了解类型转换的安全性");
            println!("\n💡 核心概念：");
            println!("  • 整数类型：i8, i16, i32, i64, isize, u8, u16, u32, u64, usize");
            println!("  • 浮点类型：f32, f64");
            println!("  • 布尔类型：bool");
            println!("  • 字符类型：char (Unicode)");
            println!("  • 复合类型：元组 (tuple), 数组 (array)");
        },
        ChapterInfo::Functions => {
            println!("🎯 学习目标：");
            println!("  • 掌握函数的定义和调用");
            println!("  • 理解参数传递机制");
            println!("  • 学习返回值的语法");
            println!("  • 了解表达式和语句的区别");
            println!("\n💡 核心概念：");
            println!("  • fn 关键字：函数定义");
            println!("  • 参数类型标注");
            println!("  • 返回类型箭头 ->");
            println!("  • 表达式返回值（无分号）");
        },
        ChapterInfo::ControlFlow => {
            println!("🎯 学习目标：");
            println!("  • 掌握条件分支 if/else");
            println!("  • 学习循环结构 loop/while/for");
            println!("  • 理解 match 模式匹配");
            println!("  • 了解控制流的表达式特性");
            println!("\n💡 核心概念：");
            println!("  • if 表达式：条件分支");
            println!("  • loop：无限循环");
            println!("  • while：条件循环");
            println!("  • for：遍历循环");
            println!("  • break/continue：循环控制");
        },
        ChapterInfo::Memory => {
            println!("🎯 学习目标：");
            println!("  • 理解程序内存布局");
            println!("  • 掌握栈和堆的区别");
            println!("  • 学习指针和引用概念");
            println!("  • 了解内存安全的重要性");
            println!("\n💡 核心概念：");
            println!("  • 栈内存：快速，固定大小，自动管理");
            println!("  • 堆内存：灵活，动态大小，手动管理");
            println!("  • 指针：内存地址");
            println!("  • 引用：安全的指针");
        },
        ChapterInfo::Ownership => {
            println!("🎯 学习目标：");
            println!("  • 理解 Rust 的所有权系统");
            println!("  • 掌握移动语义");
            println!("  • 学习 Copy 和 Clone trait");
            println!("  • 了解 Drop trait 的作用");
            println!("\n💡 核心概念：");
            println!("  • 所有权三原则");
            println!("  • 移动 (move)：转移所有权");
            println!("  • 复制 (copy)：浅拷贝");
            println!("  • 克隆 (clone)：深拷贝");
        },
        ChapterInfo::Borrowing => {
            println!("🎯 学习目标：");
            println!("  • 掌握引用和借用");
            println!("  • 理解可变引用和不可变引用");
            println!("  • 学习借用检查器规则");
            println!("  • 了解切片的概念");
            println!("\n💡 核心概念：");
            println!("  • & 引用：借用不可变");
            println!("  • &mut 引用：借用可变");
            println!("  • 借用规则：要么多个不可变，要么一个可变");
            println!("  • 切片：引用集合的一部分");
        },
        ChapterInfo::Structs => {
            println!("🎯 学习目标：");
            println!("  • 掌握结构体的定义和使用");
            println!("  • 学习方法和关联函数");
            println!("  • 理解结构体的内存布局");
            println!("  • 了解元组结构体和单元结构体");
            println!("\n💡 核心概念：");
            println!("  • struct 关键字：定义结构体");
            println!("  • impl 块：实现方法");
            println!("  • &self：方法的接收者");
            println!("  • Self::new()：关联函数");
        },
        ChapterInfo::Macros => {
            println!("🎯 学习目标：");
            println!("  • 理解宏的基本概念和语法");
            println!("  • 掌握声明宏的编写");
            println!("  • 学习宏参数类型和重复模式");
            println!("  • 了解宏的实际应用场景");
            println!("\n💡 核心概念：");
            println!("  • macro_rules!：声明宏定义");
            println!("  • 模式匹配：=> 语法");
            println!("  • 参数类型：expr, ident, ty, item 等");
            println!("  • 重复模式：$(...),*, $(...)?");
            println!("  • 元编程：编译时代码生成");
        },
        ChapterInfo::ProceduralMacros => {
            println!("🎯 学习目标：");
            println!("  • 理解过程宏的工作原理");
            println!("  • 掌握三种过程宏类型");
            println!("  • 学习 TokenStream 操作");
            println!("  • 了解过程宏的实际应用");
            println!("\n💡 核心概念：");
            println!("  • TokenStream：词法标记流");
            println!("  • syn：语法分析库");
            println!("  • quote：代码生成库");
            println!("  • proc-macro crate：独立的宏包");
            println!("  • 三种类型：派生宏、属性宏、函数式宏");
        },
        _ => {
            println!("📖 这是一个高级主题，将深入探讨 Rust 的强大特性。");
        }
    }
}

fn show_chapter_summary(chapter: &ChapterInfo) {
    println!("\n📋 知识点总结");
    println!("{}", "─".repeat(30));
    
    match chapter {
        ChapterInfo::Variables => {
            println!("✅ 您已经学会了：");
            println!("  🔸 使用 let 声明变量");
            println!("  🔸 使用 mut 创建可变变量");
            println!("  🔸 定义常量和静态变量");
            println!("  🔸 理解变量遮蔽的机制");
            println!("\n🚀 下一步建议：");
            println!("  📚 学习第2章数据类型，了解 Rust 的类型系统");
        },
        ChapterInfo::DataTypes => {
            println!("✅ 您已经学会了：");
            println!("  🔸 Rust 的基本数据类型");
            println!("  🔸 类型推断和显式标注");
            println!("  🔸 元组和数组的使用");
            println!("  🔸 类型安全的重要性");
            println!("\n🚀 下一步建议：");
            println!("  📚 学习第3章函数，了解如何组织代码");
        },
        ChapterInfo::Functions => {
            println!("✅ 您已经学会了：");
            println!("  🔸 函数的定义和调用");
            println!("  🔸 参数和返回值的语法");
            println!("  🔸 表达式和语句的区别");
            println!("  🔸 函数作为程序的构建块");
            println!("\n🚀 下一步建议：");
            println!("  📚 学习第4章控制流，掌握程序流程控制");
        },
        ChapterInfo::ControlFlow => {
            println!("✅ 您已经学会了：");
            println!("  🔸 条件分支和循环结构");
            println!("  🔸 match 模式匹配");
            println!("  🔸 控制流表达式的特性");
            println!("  🔸 循环控制和跳转");
            println!("\n🚀 下一步建议：");
            println!("  📚 学习第5章内存管理，理解 Rust 的内存模型");
        },
        ChapterInfo::Memory => {
            println!("✅ 您已经学会了：");
            println!("  🔸 程序内存的基本布局");
            println!("  🔸 栈和堆的区别和使用");
            println!("  🔸 指针和引用的概念");
            println!("  🔸 内存安全的重要性");
            println!("\n🚀 下一步建议：");
            println!("  📚 学习第6章所有权，这是 Rust 的核心特性！");
        },
        ChapterInfo::Ownership => {
            println!("✅ 您已经学会了：");
            println!("  🔸 Rust 独特的所有权系统");
            println!("  🔸 移动语义和内存安全");
            println!("  🔸 Copy 和 Clone 的区别");
            println!("  🔸 RAII 和自动内存管理");
            println!("\n🚀 下一步建议：");
            println!("  📚 学习第7章借用机制，掌握引用的使用");
        },
        ChapterInfo::Borrowing => {
            println!("✅ 您已经学会了：");
            println!("  🔸 引用和借用的概念");
            println!("  🔸 借用检查器的规则");
            println!("  🔸 可变和不可变引用");
            println!("  🔸 切片的定义和使用");
            println!("\n🚀 下一步建议：");
            println!("  📚 学习第8章结构体，开始构建复杂数据类型");
        },
        ChapterInfo::Structs => {
            println!("✅ 您已经学会了：");
            println!("  🔸 结构体的定义和实例化");
            println!("  🔸 方法和关联函数");
            println!("  🔸 结构体的内存布局");
            println!("  🔸 面向对象编程的基础");
            println!("\n🚀 下一步建议：");
            println!("  📚 学习第9章常用类型，掌握标准库集合");
        },
        ChapterInfo::Macros => {
            println!("✅ 您已经学会了：");
            println!("  🔸 宏的基本概念和语法");
            println!("  🔸 声明宏的编写技巧");
            println!("  🔸 宏参数类型的使用");
            println!("  🔸 重复模式和代码生成");
            println!("\n🚀 下一步建议：");
            println!("  📚 学习第22章过程宏，掌握更强大的元编程");
        },
        ChapterInfo::ProceduralMacros => {
            println!("✅ 您已经学会了：");
            println!("  🔸 过程宏的工作原理");
            println!("  🔸 三种过程宏类型的应用");
            println!("  🔸 TokenStream 的基本操作");
            println!("  🔸 过程宏的实际用途");
            println!("\n🚀 恭喜完成所有教程：");
            println!("  🎉 您已经掌握了 Rust 的完整知识体系！");
            println!("  📚 继续探索 Rust 生态系统和高级应用");
        },
        _ => {
            println!("🎉 恭喜完成这个高级主题的学习！");
            println!("💪 继续保持学习的热情，探索更多 Rust 特性！");
        }
    }
}

fn show_related_commands(chapter: &ChapterInfo) {
    println!("\n🛠️ 相关命令");
    println!("{}", "─".repeat(20));
    
    match chapter {
        ChapterInfo::Variables => {
            println!("💻 独立运行此章节：");
            println!("   cargo run --bin variables");
            println!("📖 查看源代码：");
            println!("   cat src/bin/variables.rs");
        },
        ChapterInfo::DataTypes => {
            println!("💻 独立运行此章节：");
            println!("   cargo run --bin data_types");
            println!("📖 查看源代码：");
            println!("   cat src/bin/data_types.rs");
        },
        ChapterInfo::Functions => {
            println!("💻 独立运行此章节：");
            println!("   cargo run --bin functions");
            println!("📖 查看源代码：");
            println!("   cat src/bin/functions.rs");
        },
        ChapterInfo::ControlFlow => {
            println!("💻 独立运行此章节：");
            println!("   cargo run --bin control_flow");
            println!("📖 查看源代码：");
            println!("   cat src/bin/control_flow.rs");
        },
        ChapterInfo::Memory => {
            println!("💻 独立运行此章节：");
            println!("   cargo run --bin memory_management");
            println!("🧪 运行内存相关测试：");
            println!("   cargo test memory");
        },
        ChapterInfo::Ownership => {
            println!("💻 独立运行此章节：");
            println!("   cargo run --bin ownership");
            println!("🧪 运行所有权测试：");
            println!("   cargo test ownership");
        },
        ChapterInfo::Borrowing => {
            println!("💻 独立运行此章节：");
            println!("   cargo run --bin borrowing");
            println!("🧪 运行借用测试：");
            println!("   cargo test borrowing");
        },
        ChapterInfo::Structs => {
            println!("💻 独立运行此章节：");
            println!("   cargo run --bin structs");
            println!("🧪 运行结构体测试：");
            println!("   cargo test structs");
        },
        ChapterInfo::Macros => {
            println!("💻 独立运行此章节：");
            println!("   cargo run --bin macros");
            println!("📖 查看源代码：");
            println!("   cat src/bin/macros.rs");
            println!("📚 查看教程文档：");
            println!("   cat tutorial/21_macros.md");
        },
        ChapterInfo::ProceduralMacros => {
            println!("💻 独立运行此章节：");
            println!("   cargo run --bin procedural_macros");
            println!("📖 查看源代码：");
            println!("   cat src/bin/procedural_macros.rs");
            println!("📚 查看教程文档：");
            println!("   cat tutorial/22_procedural_macros.md");
            println!("🔧 学习过程宏开发：");
            println!("   创建独立的 proc-macro crate");
        },
        _ => {
            println!("💻 查看所有可用命令：");
            println!("   ./run_examples.sh");
        }
    }
}

fn run_interactive_demo() {
    println!("🎭 交互式演示模式");
    println!("{}", "═".repeat(40));
    println!("🎯 这个模式将展示 Rust 的核心概念");
    println!();
    
    // 演示1：变量和可变性
    demo_variables();
    wait_for_enter();
    
    // 演示2：所有权
    demo_ownership();
    wait_for_enter();
    
    // 演示3：借用
    demo_borrowing();
    wait_for_enter();
    
    println!("🎉 交互式演示完成！");
    println!("💡 建议：从第1章开始系统学习");
}

fn demo_variables() {
    println!("📝 演示1：变量和可变性");
    println!("{}", "-".repeat(30));
    
    println!("💬 在 Rust 中，变量默认是不可变的：");
    println!("   let x = 5;        // 不可变变量");
    println!("   let mut y = 10;   // 可变变量");
    
    let x = 5;
    let mut y = 10;
    
    println!("\n🔍 当前值：");
    println!("   x = {}", x);
    println!("   y = {}", y);
    
    println!("\n🔄 修改可变变量：");
    y = 20;
    println!("   y = {} (已修改)", y);
    
    println!("\n⚠️  不可变变量 x 无法修改，否则编译错误！");
    // x = 10;  // 这行代码会导致编译错误
}

fn demo_ownership() {
    println!("\n🏠 演示2：所有权系统");
    println!("{}", "-".repeat(30));
    
    println!("💬 Rust 的所有权系统确保内存安全：");
    
    let s1 = String::from("hello");
    println!("   创建字符串: s1 = \"{}\"", s1);
    
    let s2 = s1;  // s1 的所有权移动到 s2
    println!("   移动所有权: s2 = \"{}\"", s2);
    println!("   ⚠️  s1 现在不再可用！");
    
    // println!("{}", s1);  // 这会导致编译错误
    
    println!("\n🔄 克隆创建新的所有权：");
    let s3 = s2.clone();
    println!("   克隆字符串: s3 = \"{}\"", s3);
    println!("   现在 s2 和 s3 都可用：s2 = \"{}\", s3 = \"{}\"", s2, s3);
}

fn demo_borrowing() {
    println!("\n🔗 演示3：借用机制");
    println!("{}", "-".repeat(30));
    
    println!("💬 借用允许使用值而不获取所有权：");
    
    let s = String::from("hello world");
    println!("   原始字符串: s = \"{}\"", s);
    
    let len = calculate_length(&s);  // 借用 s
    println!("   通过借用计算长度: {} 字符", len);
    println!("   原始字符串仍可用: s = \"{}\"", s);
    
    println!("\n🔄 可变借用允许修改：");
    let mut s2 = String::from("hello");
    println!("   可变字符串: s2 = \"{}\"", s2);
    
    change_string(&mut s2);  // 可变借用
    println!("   修改后: s2 = \"{}\"", s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s 离开作用域，但因为它是借用，所以不会释放内存

fn change_string(s: &mut String) {
    s.push_str(", world");
}

fn show_command_info() {
    println!("📖 Rust 基础教程 - 完整功能说明");
    println!("═══════════════════════════════════════════");
    
    println!("\n🔰 基础教程 (第1-4章):");
    println!("  1. 变量与常量     - 学习变量声明、可变性和常量");
    println!("  2. 数据类型       - 掌握基本和复合数据类型");  
    println!("  3. 函数           - 理解函数定义和调用");
    println!("  4. 控制流         - 掌握条件语句和循环");
    
    println!("\n🚀 核心概念 (第5-8章):");
    println!("  5. 内存管理       - 理解栈和堆的区别");
    println!("  6. 所有权         - 掌握 Rust 的核心特性");
    println!("  7. 借用机制       - 学习引用和借用");
    println!("  8. 结构体         - 构建自定义数据类型");
    
    println!("\n📦 高级特性 (第9-12章):");
    println!("  9. 常用类型       - Vector、String、HashMap");
    println!("  10. 枚举          - 枚举定义和模式匹配");
    println!("  11. 泛型与特征    - 泛型编程和特征系统");
    println!("  12. 生命周期      - 引用有效性管理");
    
    println!("\n🏗️ 项目开发 (第13-17章):");
    println!("  13. 项目管理      - 模块系统和包管理");
    println!("  14. 文档与测试    - 文档注释和测试编写");
    println!("  15. 闭包          - 函数式编程特性");
    println!("  16. 迭代器        - 高效的数据处理");
    println!("  17. 智能指针      - 高级内存管理");
    
    println!("\n🎭 专业主题 (第18-20章):");
    println!("  18. 常用智能指针  - Rc、Arc、RefCell 等");
    println!("  19. 并发编程      - 线程和异步编程");
    println!("  20. Unsafe Rust   - 底层系统编程");
    
    println!("\n💻 运行方式:");
    println!("  🎯 交互式学习:");
    println!("    • 输入数字 (1-20) - 运行对应章节");
    println!("    • 输入 'basic'    - 运行基础教程 (1-4章)");
    println!("    • 输入 'advanced' - 运行进阶教程 (5-20章)");
    println!("    • 输入 '0'        - 运行所有章节");
    println!("    • 输入 'demo'     - 交互式演示");
    
    println!("\n  🚀 独立运行:");
    println!("    cargo run --bin variables     # 第1章");
    println!("    cargo run --bin ownership     # 第6章");
    println!("    cargo run --bin concurrency   # 第19章");
    
    println!("\n  🧪 测试和验证:");
    println!("    cargo test                    # 运行所有测试");
    println!("    cargo check                   # 检查代码");
    println!("    cargo build --release         # 优化构建");
    
    println!("\n🎓 学习建议:");
    println!("  📚 新手路径: 1→2→3→4→5→6→7→8");
    println!("  🚀 进阶路径: 9→10→11→12→13→14→15→16→17");
    println!("  🎭 专业路径: 18→19→20");
    
    println!("\n🛠️ 额外功能:");
    println!("  • 每章包含详细的知识点讲解");
    println!("  • 提供相关命令和下一步建议");
    println!("  • 支持清屏和美化输出");
    println!("  • 包含交互式演示模式");
    
    wait_for_enter();
}

fn get_user_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_lowercase(),
        Err(_) => {
            println!("❌ 输入读取失败，请重试");
            String::new()
        }
    }
}

fn run_all_examples() {
    println!("🚀 运行所有20章 Rust 基础教程");
    println!("{}", "═".repeat(50));
    println!("⏱️  预计运行时间：约3-5分钟");
    println!("📚 将依次展示所有章节的核心概念");
    println!("{}", "═".repeat(50));
    
    let chapters = [
        ("第1章：变量与常量", examples::variables_and_constants as fn()),
        ("第2章：数据类型", examples::data_types),
        ("第3章：函数", examples::functions_demo),
        ("第4章：控制流", examples::control_flow),
        ("第5章：内存管理", examples::memory_management),
        ("第6章：所有权", examples::ownership),
        ("第7章：借用机制", examples::borrowing),
        ("第8章：结构体", examples::structs),
        ("第9章：常用类型", examples::common_types),
        ("第10章：枚举", examples::enums),
        ("第11章：泛型与特征", examples::generics_traits),
        ("第12章：生命周期", examples::lifetimes),
        ("第13章：项目管理", examples::project_management),
        ("第14章：文档与测试", examples::docs_and_testing),
        ("第15章：闭包", examples::closures),
        ("第16章：迭代器", examples::iterators),
        ("第17章：智能指针", examples::smart_pointers),
        ("第18章：常用智能指针", examples::common_smart_pointers),
        ("第19章：并发编程", examples::concurrency),
        ("第20章：Unsafe Rust", examples::unsafe_rust),
    ];
    
    for (i, (name, func)) in chapters.iter().enumerate() {
        println!("\n🔹 [{}/20] {}", i + 1, name);
        println!("{}", "─".repeat(40));
        func();
        println!("✅ {} 完成", name);
        
        if i < chapters.len() - 1 {
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }
    
    println!("\n🎉 所有20章教程示例运行完成！");
    println!("🏆 您已经掌握了 Rust 编程的核心知识！");
    println!("📚 建议继续深入学习官方文档：https://doc.rust-lang.org/book/");
    wait_for_enter();
}

fn run_basic_examples() {
    println!("📚 运行基础教程 (第1-4章)");
    println!("{}", "═".repeat(40));
    println!("🎯 这些章节涵盖 Rust 的基础语法");
    println!("{}", "─".repeat(40));
    
    let basic_chapters = [
        ("第1章：变量与常量", examples::variables_and_constants as fn()),
        ("第2章：数据类型", examples::data_types),
        ("第3章：函数", examples::functions_demo),
        ("第4章：控制流", examples::control_flow),
    ];
    
    for (i, (name, func)) in basic_chapters.iter().enumerate() {
        println!("\n🔹 [{}/4] {}", i + 1, name);
        println!("{}", "-".repeat(30));
        func();
        println!("✅ {} 完成", name);
        
        if i < basic_chapters.len() - 1 {
            std::thread::sleep(std::time::Duration::from_millis(300));
        }
    }
    
    println!("\n✅ 基础教程完成！");
    println!("🚀 建议继续学习核心概念 (第5-8章)");
    println!("💡 输入 'advanced' 运行进阶教程");
    wait_for_enter();
}

fn run_advanced_examples() {
    println!("🔥 运行进阶教程 (第5-20章)");
    println!("{}", "═".repeat(40));
    println!("🎯 这些章节涵盖 Rust 的核心和高级特性");
    println!("{}", "─".repeat(40));
    
    let advanced_chapters = [
        ("第5章：内存管理", examples::memory_management as fn()),
        ("第6章：所有权", examples::ownership),
        ("第7章：借用机制", examples::borrowing),
        ("第8章：结构体", examples::structs),
        ("第9章：常用类型", examples::common_types),
        ("第10章：枚举", examples::enums),
        ("第11章：泛型与特征", examples::generics_traits),
        ("第12章：生命周期", examples::lifetimes),
        ("第13章：项目管理", examples::project_management),
        ("第14章：文档与测试", examples::docs_and_testing),
        ("第15章：闭包", examples::closures),
        ("第16章：迭代器", examples::iterators),
        ("第17章：智能指针", examples::smart_pointers),
        ("第18章：常用智能指针", examples::common_smart_pointers),
        ("第19章：并发编程", examples::concurrency),
        ("第20章：Unsafe Rust", examples::unsafe_rust),
    ];
    
    for (i, (name, func)) in advanced_chapters.iter().enumerate() {
        println!("\n🔹 [{}/16] {}", i + 1, name);
        println!("{}", "-".repeat(30));
        func();
        println!("✅ {} 完成", name);
        
        if i < advanced_chapters.len() - 1 {
            std::thread::sleep(std::time::Duration::from_millis(300));
        }
    }
    
    println!("\n🎉 进阶教程完成！");
    println!("🏆 您已掌握 Rust 的核心和高级特性！");
    println!("🌟 您现在可以开始构建实际的 Rust 项目了！");
    wait_for_enter();
}

fn wait_for_enter() {
    print!("\n⏎ 按回车键继续...");
    io::stdout().flush().unwrap();
    let mut _temp = String::new();
    let _ = io::stdin().read_line(&mut _temp);
    clear_screen();
}
