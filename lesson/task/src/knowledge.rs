// 知识点详细解释模块
// 为每个章节提供深入的概念讲解

/// 知识点结构
#[derive(Debug, Clone)]
pub struct KnowledgePoint {
    pub title: String,
    pub description: String,
    pub key_concepts: Vec<String>,
    pub code_examples: Vec<CodeExample>,
    pub common_mistakes: Vec<String>,
    pub best_practices: Vec<String>,
    pub related_topics: Vec<String>,
}

/// 代码示例结构
#[derive(Debug, Clone)]
pub struct CodeExample {
    pub title: String,
    pub code: String,
    pub explanation: String,
    pub output: Option<String>,
}

/// 获取章节的详细知识点
pub fn get_chapter_knowledge(chapter_id: u8) -> Option<Vec<KnowledgePoint>> {
    match chapter_id {
        1 => Some(get_variables_knowledge()),
        2 => Some(get_data_types_knowledge()),
        3 => Some(get_functions_knowledge()),
        4 => Some(get_control_flow_knowledge()),
        5 => Some(get_memory_management_knowledge()),
        6 => Some(get_ownership_knowledge()),
        7 => Some(get_borrowing_knowledge()),
        8 => Some(get_structs_knowledge()),
        _ => None,
    }
}

/// 第1章：变量与常量的知识点
fn get_variables_knowledge() -> Vec<KnowledgePoint> {
    vec![
        KnowledgePoint {
            title: "变量声明与可变性".to_string(),
            description: "Rust 中变量默认是不可变的，这是 Rust 安全性和并发性的基础。".to_string(),
            key_concepts: vec![
                "let 关键字用于声明变量".to_string(),
                "变量默认不可变 (immutable)".to_string(),
                "mut 关键字使变量可变".to_string(),
                "不可变性有助于防止意外修改".to_string(),
            ],
            code_examples: vec![
                CodeExample {
                    title: "基本变量声明".to_string(),
                    code: "let x = 5;\nlet mut y = 10;\ny = 15;".to_string(),
                    explanation: "使用 let 声明变量，默认不可变。加上 mut 关键字使变量可变。".to_string(),
                    output: Some("y 的值从 10 变为 15".to_string()),
                },
            ],
            common_mistakes: vec![
                "忘记使用 mut 关键字就尝试修改变量".to_string(),
                "混淆变量遮蔽和变量修改的概念".to_string(),
            ],
            best_practices: vec![
                "默认使用不可变变量，只在需要时使用 mut".to_string(),
                "使用描述性的变量名".to_string(),
            ],
            related_topics: vec![
                "内存安全".to_string(),
                "并发编程".to_string(),
            ],
        },
    ]
}

fn get_data_types_knowledge() -> Vec<KnowledgePoint> {
    vec![]
}

fn get_functions_knowledge() -> Vec<KnowledgePoint> {
    vec![]
}

fn get_control_flow_knowledge() -> Vec<KnowledgePoint> {
    vec![]
}

fn get_memory_management_knowledge() -> Vec<KnowledgePoint> {
    vec![]
}

fn get_ownership_knowledge() -> Vec<KnowledgePoint> {
    vec![]
}

fn get_borrowing_knowledge() -> Vec<KnowledgePoint> {
    vec![]
}

fn get_structs_knowledge() -> Vec<KnowledgePoint> {
    vec![]
}

/// 显示知识点详情
pub fn display_knowledge_point(kp: &KnowledgePoint) {
    println!("📚 {}", kp.title);
    println!("{}", "═".repeat(50));
    
    println!("\n📖 概述：");
    println!("   {}", kp.description);
    
    println!("\n🎯 核心概念：");
    for concept in &kp.key_concepts {
        println!("   • {}", concept);
    }
}

/// 获取通用的编程技巧
pub fn get_programming_tips() -> Vec<String> {
    vec![
        "🎯 每次只学习一个概念，确保理解后再继续".to_string(),
        "💻 多写代码，通过实践加深理解".to_string(),
        "🔍 使用 Rust 编译器的错误信息学习".to_string(),
        "📚 阅读标准库文档了解最佳实践".to_string(),
    ]
} 