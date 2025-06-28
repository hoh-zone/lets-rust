// Rust 基础教程 - 交互式代码示例
// 完整的20章教程，所有示例都可以直接运行

use std::io::{self, Write};

mod examples;

fn main() {
    show_welcome();
    
    loop {
        show_menu();
        
        let choice = get_user_input();
        
        // 清屏或添加分隔符
        println!("\n{}", "─".repeat(50));
        
        match choice.as_str() {
            "1" => run_example("第1章：变量与常量", || examples::variables_and_constants()),
            "2" => run_example("第2章：数据类型", || examples::data_types()),
            "3" => run_example("第3章：函数", || examples::functions_demo()),
            "4" => run_example("第4章：控制流", || examples::control_flow()),
            "5" => run_example("第5章：内存管理", || examples::memory_management()),
            "6" => run_example("第6章：所有权", || examples::ownership()),
            "7" => run_example("第7章：借用机制", || examples::borrowing()),
            "8" => run_example("第8章：结构体", || examples::structs()),
            "9" => run_example("第9章：常用类型", || examples::common_types()),
            "10" => run_example("第10章：枚举", || examples::enums()),
            "11" => run_example("第11章：泛型与特征", || examples::generics_traits()),
            "12" => run_example("第12章：生命周期", || examples::lifetimes()),
            "13" => run_example("第13章：项目管理", || examples::project_management()),
            "14" => run_example("第14章：文档与测试", || examples::docs_and_testing()),
            "15" => run_example("第15章：闭包", || examples::closures()),
            "16" => run_example("第16章：迭代器", || examples::iterators()),
            "17" => run_example("第17章：智能指针", || examples::smart_pointers()),
            "18" => run_example("第18章：常用智能指针", || examples::common_smart_pointers()),
            "19" => run_example("第19章：并发编程", || examples::concurrency()),
            "20" => run_example("第20章：Unsafe Rust", || examples::unsafe_rust()),
            "0" => run_all_examples(),
            "basic" => run_basic_examples(),
            "advanced" => run_advanced_examples(),
            "help" | "h" => show_command_info(),
            "q" | "quit" | "exit" => {
                println!("\n👋 感谢使用 Rust 基础教程！");
                println!("🎉 希望您已经掌握了 Rust 编程的核心概念！");
                break;
            },
            "" => {
                // 空输入，直接继续
                continue;
            },
            _ => {
                println!("❌ 无效选择！输入 'help' 查看所有可用命令");
                println!("💡 提示：输入数字 1-20 选择章节，或输入 q 退出");
                wait_for_enter();
            }
        }
    }
}

fn show_welcome() {
    println!("🦀 欢迎来到 Rust 基础教程交互式示例！");
    println!("=====================================");
    println!("📚 本教程包含20个完整章节，从基础到高级");
    println!("🎯 输入章节号码开始学习，输入 'help' 查看帮助");
    println!();
}

fn show_menu() {
    println!("┌─────────────────────────────────────────┐");
    println!("│            🦀 Rust 教程菜单             │");
    println!("├─────────────────────────────────────────┤");
    println!("│ 📚 基础 (1-4)  🚀 核心 (5-8)  📦 高级  │");
    println!("│ 🏗️  项目 (13-17) 🎭 专业 (18-20)        │");
    println!("├─────────────────────────────────────────┤");
    println!("│ 🎯 快速选项:                            │");
    println!("│ • 0: 运行所有章节                       │");
    println!("│ • basic: 基础教程 (1-4)                 │");
    println!("│ • advanced: 进阶教程 (5-20)             │");
    println!("│ • help: 显示详细帮助                    │");
    println!("│ • q: 退出程序                           │");
    println!("└─────────────────────────────────────────┘");
    print!("请输入选择 (1-20 或命令): ");
    io::stdout().flush().unwrap();
}

fn show_command_info() {
    println!("📖 Rust 基础教程 - 完整章节列表");
    println!("═══════════════════════════════════════════");
    
    println!("\n🔰 基础教程 (第1-4章):");
    println!("  1. 变量与常量     2. 数据类型");
    println!("  3. 函数          4. 控制流");
    
    println!("\n🚀 核心概念 (第5-8章):");
    println!("  5. 内存管理      6. 所有权");
    println!("  7. 借用机制      8. 结构体");
    
    println!("\n📦 高级特性 (第9-12章):");
    println!("  9. 常用类型      10. 枚举");
    println!("  11. 泛型与特征   12. 生命周期");
    
    println!("\n🏗️ 项目开发 (第13-17章):");
    println!("  13. 项目管理     14. 文档与测试");
    println!("  15. 闭包        16. 迭代器");
    println!("  17. 智能指针");
    
    println!("\n🎭 专业主题 (第18-20章):");
    println!("  18. 常用智能指针  19. 并发编程");
    println!("  20. Unsafe Rust");
    
    println!("\n💡 使用提示:");
    println!("  • 输入数字 (1-20) 运行对应章节");
    println!("  • 输入 'basic' 运行基础教程");
    println!("  • 输入 'advanced' 运行进阶教程");
    println!("  • 输入 '0' 运行所有章节");
    println!("  • 输入 'q' 退出程序");
    
    println!("\n🎓 建议学习路径:");
    println!("  新手: 1→2→3→4→5→6→7→8");
    println!("  进阶: 9→10→11→12→13→14→15→16→17");
    println!("  专业: 18→19→20");
    
    wait_for_enter();
}

fn get_user_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_lowercase(),
        Err(_) => {
            println!("输入读取失败，请重试");
            String::new()
        }
    }
}

fn run_example<F>(name: &str, example_fn: F) 
where 
    F: FnOnce(),
{
    println!("🔹 正在运行：{}", name);
    println!("{}", "=".repeat(50));
    example_fn();
    println!("{}", "=".repeat(50));
    println!("✅ {} 示例运行完成！", name);
    
    show_chapter_info(name);
    wait_for_enter();
}

fn show_chapter_info(chapter_name: &str) {
    println!("\n📖 关于{}：", chapter_name);
    
    match chapter_name {
        name if name.contains("变量与常量") => {
            println!("   ✓ 学习了变量声明、可变性和常量定义");
            println!("   📝 下一步：学习第2章数据类型");
        },
        name if name.contains("数据类型") => {
            println!("   ✓ 掌握了基本数据类型和复合类型");
            println!("   📝 下一步：学习第3章函数");
        },
        name if name.contains("函数") => {
            println!("   ✓ 学习了函数定义、参数和返回值");
            println!("   📝 下一步：学习第4章控制流");
        },
        name if name.contains("控制流") => {
            println!("   ✓ 掌握了条件语句和循环结构");
            println!("   📝 下一步：学习第5章内存管理");
        },
        name if name.contains("内存管理") => {
            println!("   ✓ 理解了栈和堆的区别");
            println!("   📝 下一步：学习第6章所有权系统");
        },
        name if name.contains("所有权") => {
            println!("   ✓ 掌握了 Rust 独特的所有权系统");
            println!("   📝 下一步：学习第7章借用机制");
        },
        name if name.contains("借用") => {
            println!("   ✓ 学习了引用和借用的概念");
            println!("   📝 下一步：学习第8章结构体");
        },
        name if name.contains("结构体") => {
            println!("   ✓ 掌握了结构体定义和方法");
            println!("   📝 下一步：学习第9章常用类型");
        },
        name if name.contains("常用类型") => {
            println!("   ✓ 掌握了 Vector、String、HashMap");
            println!("   📝 下一步：学习第10章枚举");
        },
        name if name.contains("枚举") => {
            println!("   ✓ 学习了枚举和模式匹配");
            println!("   📝 下一步：学习第11章泛型与特征");
        },
        name if name.contains("泛型") => {
            println!("   ✓ 掌握了泛型编程和特征系统");
            println!("   📝 下一步：学习第12章生命周期");
        },
        name if name.contains("生命周期") => {
            println!("   ✓ 理解了生命周期和引用有效性");
            println!("   📝 下一步：学习第13章项目管理");
        },
        name if name.contains("项目管理") => {
            println!("   ✓ 学习了模块系统和包管理");
            println!("   📝 下一步：学习第14章文档与测试");
        },
        name if name.contains("文档与测试") => {
            println!("   ✓ 掌握了文档注释和测试编写");
            println!("   📝 下一步：学习第15章闭包");
        },
        name if name.contains("闭包") => {
            println!("   ✓ 理解了闭包和函数式编程");
            println!("   📝 下一步：学习第16章迭代器");
        },
        name if name.contains("迭代器") => {
            println!("   ✓ 掌握了迭代器模式和惰性求值");
            println!("   📝 下一步：学习第17章智能指针");
        },
        name if name.contains("智能指针") && !name.contains("常用") => {
            println!("   ✓ 学习了 Box、Rc、RefCell、Arc、Mutex");
            println!("   📝 下一步：学习第18章常用智能指针");
        },
        name if name.contains("常用智能指针") => {
            println!("   ✓ 掌握了 Cow、Weak、Pin 等高级指针");
            println!("   📝 下一步：学习第19章并发编程");
        },
        name if name.contains("并发编程") => {
            println!("   ✓ 学习了线程、消息传递、共享状态");
            println!("   📝 下一步：学习第20章 Unsafe Rust");
        },
        name if name.contains("Unsafe") => {
            println!("   ✓ 掌握了 unsafe 代码和底层操作");
            println!("   🎉 恭喜！您已完成所有教程！");
        },
        _ => {}
    }
}

fn run_all_examples() {
    println!("🚀 运行所有20章 Rust 基础教程");
    println!("{}", "=".repeat(50));
    println!("⏱️  预计运行时间：约3-5分钟");
    println!("{}", "=".repeat(50));
    
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
        println!("{}", "-".repeat(30));
        func();
        println!("✅ {} 完成", name);
        
        if i < chapters.len() - 1 {
            std::thread::sleep(std::time::Duration::from_millis(300));
        }
    }
    
    println!("\n🎉 所有20章教程示例运行完成！");
    println!("🏆 您已经掌握了 Rust 编程的核心知识！");
    wait_for_enter();
}

fn run_basic_examples() {
    println!("📚 运行基础教程 (第1-4章)");
    println!("{}", "=".repeat(40));
    
    let basic_chapters = [
        ("第1章：变量与常量", examples::variables_and_constants as fn()),
        ("第2章：数据类型", examples::data_types),
        ("第3章：函数", examples::functions_demo),
        ("第4章：控制流", examples::control_flow),
    ];
    
    for (i, (name, func)) in basic_chapters.iter().enumerate() {
        println!("\n🔹 [{}/4] {}", i + 1, name);
        println!("{}", "-".repeat(25));
        func();
        println!("✅ {} 完成", name);
        
        if i < basic_chapters.len() - 1 {
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
    }
    
    println!("\n✅ 基础教程完成！建议继续学习核心概念");
    wait_for_enter();
}

fn run_advanced_examples() {
    println!("🔥 运行进阶教程 (第5-20章)");
    println!("{}", "=".repeat(40));
    
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
        println!("{}", "-".repeat(25));
        func();
        println!("✅ {} 完成", name);
        
        if i < advanced_chapters.len() - 1 {
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
    }
    
    println!("\n🎉 进阶教程完成！您已掌握 Rust 高级特性！");
    wait_for_enter();
}

fn wait_for_enter() {
    print!("\n按回车键继续...");
    io::stdout().flush().unwrap();
    let mut _temp = String::new();
    let _ = io::stdin().read_line(&mut _temp);
}
