// 教学示例 - 允许未使用的代码
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// 第16章：迭代器
// 演示 Rust 中迭代器的使用、适配器和消费者

use std::collections::HashMap;

fn main() {
    println!("🔄 第16章：迭代器");
    println!("=====================================");
    
    // 1. 迭代器基础
    iterator_basics();
    
    // 2. 迭代器适配器
    iterator_adapters();
    
    // 3. 消费适配器
    consuming_adapters();
    
    // 4. 自定义迭代器
    custom_iterators();
    
    // 5. 实际应用示例
    practical_examples();
}

// ============================================================================
// 1. 迭代器基础
// ============================================================================

fn iterator_basics() {
    println!("\n🎯 1. 迭代器基础");
    println!("{}", "-".repeat(40));
    
    // 三种迭代器类型
    println!("  🔸 三种迭代器类型：");
    
    let vec = vec![1, 2, 3, 4, 5];
    
    // iter() - 产生不可变引用
    println!("    iter() - 不可变引用：");
    for item in vec.iter() {
        println!("      &{}", item);
    }
    println!("    原始向量仍可用: {:?}", vec);
    
    // into_iter() - 获取所有权
    let vec2 = vec![1, 2, 3, 4, 5];
    println!("\n    into_iter() - 获取所有权：");
    for item in vec2.into_iter() {
        println!("      {}", item);
    }
    // println!("    {:?}", vec2); // 编译错误：vec2 已被移动
    
    // iter_mut() - 产生可变引用
    let mut vec3 = vec![1, 2, 3, 4, 5];
    println!("\n    iter_mut() - 可变引用：");
    for item in vec3.iter_mut() {
        *item *= 2;
        println!("      修改为: {}", item);
    }
    println!("    修改后的向量: {:?}", vec3);
    
    // 迭代器是惰性的
    println!("\n  🔸 迭代器的惰性特性：");
    let v1 = vec![1, 2, 3];
    let _v1_iter = v1.iter(); // 创建迭代器，但没有消费
    println!("    迭代器已创建，但没有执行任何操作");
    
    // 手动调用 next()
    println!("\n  🔸 手动调用 next()：");
    let v2 = vec![1, 2, 3];
    let mut iter = v2.iter();
    
    println!("    第一次 next(): {:?}", iter.next());
    println!("    第二次 next(): {:?}", iter.next());
    println!("    第三次 next(): {:?}", iter.next());
    println!("    第四次 next(): {:?}", iter.next()); // None
}

// ============================================================================
// 2. 迭代器适配器
// ============================================================================

fn iterator_adapters() {
    println!("\n🔧 2. 迭代器适配器");
    println!("{}", "-".repeat(40));
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // map - 转换每个元素
    println!("  🔸 map - 转换每个元素：");
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("    原始: {:?}", numbers);
    println!("    翻倍: {:?}", doubled);
    
    // filter - 过滤元素
    println!("\n  🔸 filter - 过滤元素：");
    let evens: Vec<&i32> = numbers.iter().filter(|&x| x % 2 == 0).collect();
    println!("    偶数: {:?}", evens);
    
    let greater_than_5: Vec<&i32> = numbers.iter().filter(|&x| *x > 5).collect();
    println!("    大于5: {:?}", greater_than_5);
    
    // enumerate - 添加索引
    println!("\n  🔸 enumerate - 添加索引：");
    let with_index: Vec<(usize, &i32)> = numbers.iter().enumerate().collect();
    println!("    带索引: {:?}", with_index);
    
    // zip - 组合两个迭代器
    println!("\n  🔸 zip - 组合两个迭代器：");
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![30, 25, 35];
    let pairs: Vec<(&str, &i32)> = names.iter().zip(ages.iter()).map(|(&name, age)| (name, age)).collect();
    println!("    姓名年龄对: {:?}", pairs);
    
    // take - 取前 n 个元素
    println!("\n  🔸 take - 取前 n 个元素：");
    let first_three: Vec<&i32> = numbers.iter().take(3).collect();
    println!("    前三个: {:?}", first_three);
    
    // skip - 跳过前 n 个元素
    println!("\n  🔸 skip - 跳过前 n 个元素：");
    let skip_three: Vec<&i32> = numbers.iter().skip(3).collect();
    println!("    跳过前三个: {:?}", skip_three);
    
    // step_by - 按步长迭代
    println!("\n  🔸 step_by - 按步长迭代：");
    let every_second: Vec<&i32> = numbers.iter().step_by(2).collect();
    println!("    每隔一个: {:?}", every_second);
    
    // rev - 反向迭代
    println!("\n  🔸 rev - 反向迭代：");
    let reversed: Vec<&i32> = numbers.iter().rev().collect();
    println!("    反向: {:?}", reversed);
    
    // 链式操作
    println!("\n  🔸 链式操作：");
    let result: Vec<i32> = numbers
        .iter()
        .filter(|&x| x % 2 == 0)    // 筛选偶数
        .map(|x| x * x)             // 平方
        .filter(|&x| x > 10)        // 筛选大于10
        .collect();
    println!("    偶数的平方中大于10的: {:?}", result);
}

// ============================================================================
// 3. 消费适配器
// ============================================================================

fn consuming_adapters() {
    println!("\n🍽️ 3. 消费适配器");
    println!("{}", "-".repeat(40));
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // collect - 收集到集合
    println!("  🔸 collect - 收集到集合：");
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("    收集到 Vec: {:?}", doubled);
    
    // reduce - 归约操作
    println!("\n  🔸 reduce - 归约操作：");
    let sum = numbers.iter().reduce(|acc, x| {
        println!("    累加: {} + {} = {}", acc, x, acc + x);
        if acc > x { acc } else { x } // 返回较大值以避免类型错误
    });
    println!("    最终结果: {:?}", sum);
    
    let max = numbers.iter().reduce(|acc, x| if acc > x { acc } else { x });
    println!("    最大值: {:?}", max);
    
    // fold - 带初始值的归约
    println!("\n  🔸 fold - 带初始值的归约：");
    let sum_fold = numbers.iter().fold(0, |acc, x| {
        println!("    fold: {} + {} = {}", acc, x, acc + x);
        acc + x
    });
    println!("    fold 结果: {}", sum_fold);
    
    let product = numbers.iter().fold(1, |acc, x| acc * x);
    println!("    乘积: {}", product);
    
    // for_each - 对每个元素执行操作
    println!("\n  🔸 for_each - 对每个元素执行操作：");
    numbers.iter().for_each(|x| print!("{} ", x));
    println!();
    
    // find - 查找第一个匹配的元素
    println!("\n  🔸 find - 查找第一个匹配的元素：");
    let found = numbers.iter().find(|&x| *x > 5);
    println!("    第一个大于5的数: {:?}", found);
    
    let not_found = numbers.iter().find(|&x| *x > 15);
    println!("    第一个大于15的数: {:?}", not_found);
    
    // any - 检查是否有任何元素满足条件
    println!("\n  🔸 any - 检查是否有任何元素满足条件：");
    let has_even = numbers.iter().any(|&x| x % 2 == 0);
    println!("    是否有偶数: {}", has_even);
    
    let has_negative = numbers.iter().any(|&x| x < 0);
    println!("    是否有负数: {}", has_negative);
    
    // all - 检查是否所有元素都满足条件
    println!("\n  🔸 all - 检查是否所有元素都满足条件：");
    let all_positive = numbers.iter().all(|&x| x > 0);
    println!("    是否都是正数: {}", all_positive);
    
    let all_even = numbers.iter().all(|&x| x % 2 == 0);
    println!("    是否都是偶数: {}", all_even);
    
    // count - 计数
    println!("\n  🔸 count - 计数：");
    let even_count = numbers.iter().filter(|&x| x % 2 == 0).count();
    println!("    偶数个数: {}", even_count);
    
    // min 和 max
    println!("\n  🔸 min 和 max：");
    let min_val = numbers.iter().min();
    let max_val = numbers.iter().max();
    println!("    最小值: {:?}, 最大值: {:?}", min_val, max_val);
    
    // sum 和 product（需要实现相应的 trait）
    println!("\n  🔸 sum 和 product：");
    let sum: i32 = numbers.iter().sum();
    let product: i32 = numbers.iter().product();
    println!("    和: {}, 乘积: {}", sum, product);
}

// ============================================================================
// 4. 自定义迭代器
// ============================================================================

fn custom_iterators() {
    println!("\n🛠️ 4. 自定义迭代器");
    println!("{}", "-".repeat(40));
    
    // 简单的计数器迭代器
    println!("  🔸 计数器迭代器：");
    
    struct Counter {
        current: usize,
        max: usize,
    }
    
    impl Counter {
        fn new(max: usize) -> Counter {
            Counter { current: 0, max }
        }
    }
    
    impl Iterator for Counter {
        type Item = usize;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.max {
                let current = self.current;
                self.current += 1;
                Some(current)
            } else {
                None
            }
        }
    }
    
    let counter = Counter::new(5);
    for num in counter {
        println!("    计数: {}", num);
    }
    
    // 使用自定义迭代器的适配器
    println!("\n  🔸 自定义迭代器与适配器：");
    let counter = Counter::new(10);
    let sum: usize = counter
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .sum();
    println!("    偶数平方和: {}", sum);
    
    // 斐波那契迭代器
    println!("\n  🔸 斐波那契迭代器：");
    
    struct Fibonacci {
        current: u32,
        next: u32,
    }
    
    impl Fibonacci {
        fn new() -> Self {
            Fibonacci { current: 0, next: 1 }
        }
    }
    
    impl Iterator for Fibonacci {
        type Item = u32;
        
        fn next(&mut self) -> Option<Self::Item> {
            let current = self.current;
            self.current = self.next;
            self.next = current + self.next;
            
            // 防止溢出
            if self.next < self.current {
                None
            } else {
                Some(current)
            }
        }
    }
    
    let fib = Fibonacci::new();
    let fib_numbers: Vec<u32> = fib.take(10).collect();
    println!("    前10个斐波那契数: {:?}", fib_numbers);
}

// ============================================================================
// 5. 实际应用示例
// ============================================================================

fn practical_examples() {
    println!("\n🚀 5. 实际应用示例");
    println!("{}", "-".repeat(40));
    
    // 数据处理管道
    data_processing_pipeline();
    
    // 文本分析
    text_analysis();
    
    // 分组和聚合
    grouping_and_aggregation();
}

fn data_processing_pipeline() {
    println!("\n  🔸 数据处理管道：");
    
    #[derive(Debug, Clone)]
    struct Sale {
        product: String,
        amount: f64,
        region: String,
        month: u32,
    }
    
    let sales = vec![
        Sale { product: "笔记本".to_string(), amount: 1200.0, region: "北京".to_string(), month: 1 },
        Sale { product: "手机".to_string(), amount: 800.0, region: "上海".to_string(), month: 1 },
        Sale { product: "笔记本".to_string(), amount: 1500.0, region: "北京".to_string(), month: 2 },
        Sale { product: "平板".to_string(), amount: 600.0, region: "深圳".to_string(), month: 1 },
        Sale { product: "手机".to_string(), amount: 900.0, region: "上海".to_string(), month: 2 },
    ];
    
    // 分析第1月北京地区的销售
    let beijing_jan_total: f64 = sales
        .iter()
        .filter(|sale| sale.region == "北京" && sale.month == 1)
        .map(|sale| sale.amount)
        .sum();
    
    println!("    北京1月总销售额: {:.2}", beijing_jan_total);
    
    // 找出销售额最高的产品
    let top_sale = sales
        .iter()
        .max_by(|a, b| a.amount.partial_cmp(&b.amount).unwrap());
    
    println!("    最高销售额: {:?}", top_sale);
    
    // 按地区分组计算总销售额
    let mut region_totals: HashMap<String, f64> = HashMap::new();
    sales
        .iter()
        .for_each(|sale| {
            *region_totals.entry(sale.region.clone()).or_insert(0.0) += sale.amount;
        });
    
    println!("    各地区销售额:");
    for (region, total) in region_totals {
        println!("      {}: {:.2}", region, total);
    }
}

fn text_analysis() {
    println!("\n  🔸 文本分析：");
    
    let text = "Rust is a systems programming language that runs blazingly fast, \
                prevents segfaults, and guarantees thread safety. Rust is great!";
    
    // 单词统计
    let word_count = text
        .split_whitespace()
        .count();
    
    println!("    总单词数: {}", word_count);
    
    // 长单词（超过5个字符）
    let long_words: Vec<&str> = text
        .split_whitespace()
        .filter(|word| word.len() > 5)
        .collect();
    
    println!("    长单词: {:?}", long_words);
    
    // 首字母大写的单词
    let capitalized_words: Vec<&str> = text
        .split_whitespace()
        .filter(|word| word.chars().next().unwrap_or('a').is_uppercase())
        .collect();
    
    println!("    首字母大写的单词: {:?}", capitalized_words);
}

fn grouping_and_aggregation() {
    println!("\n  🔸 分组和聚合：");
    
    #[derive(Debug)]
    struct Student {
        name: String,
        grade: u32,
        subject: String,
        score: f64,
    }
    
    let students = vec![
        Student { name: "Alice".to_string(), grade: 10, subject: "数学".to_string(), score: 85.0 },
        Student { name: "Bob".to_string(), grade: 10, subject: "数学".to_string(), score: 92.0 },
        Student { name: "Charlie".to_string(), grade: 11, subject: "数学".to_string(), score: 78.0 },
        Student { name: "Alice".to_string(), grade: 10, subject: "物理".to_string(), score: 88.0 },
        Student { name: "Bob".to_string(), grade: 10, subject: "物理".to_string(), score: 85.0 },
    ];
    
    // 按年级分组计算平均分
    let mut grade_scores: HashMap<u32, Vec<f64>> = HashMap::new();
    students
        .iter()
        .for_each(|student| {
            grade_scores.entry(student.grade).or_insert_with(Vec::new).push(student.score);
        });
    
    println!("    各年级平均分:");
    for (grade, scores) in grade_scores {
        let average = scores.iter().sum::<f64>() / scores.len() as f64;
        println!("      {}年级: {:.2}", grade, average);
    }
    
    // 找出每个科目的最高分
    let mut subject_max: HashMap<String, f64> = HashMap::new();
    students
        .iter()
        .for_each(|student| {
            let current_max = subject_max.entry(student.subject.clone()).or_insert(0.0);
            if student.score > *current_max {
                *current_max = student.score;
            }
        });
    
    println!("    各科目最高分:");
    for (subject, max_score) in subject_max {
        println!("      {}: {:.2}", subject, max_score);
    }
} 