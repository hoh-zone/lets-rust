// 教学示例 - 允许未使用的代码
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// 第15章：闭包
// 演示 Rust 中闭包的定义、使用和特征

use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    println!("🔒 第15章：闭包");
    println!("=====================================");
    
    // 1. 闭包基础
    closure_basics();
    
    // 2. 闭包语法和类型推断
    closure_syntax();
    
    // 3. 捕获环境
    capture_environment();
    
    // 4. 闭包特征：Fn、FnMut、FnOnce
    closure_traits();
    
    // 5. 闭包作为参数
    closures_as_parameters();
    
    // 6. 返回闭包
    returning_closures();
    
    // 7. 实际应用示例
    practical_examples();
}

// ============================================================================
// 1. 闭包基础
// ============================================================================

fn closure_basics() {
    println!("\n🎯 1. 闭包基础");
    println!("{}", "-".repeat(40));
    
    // 最简单的闭包
    let simple_closure = || {
        println!("  🔹 这是一个简单的闭包");
    };
    simple_closure();
    
    // 带参数的闭包
    let add_one = |x| x + 1;
    println!("  🔹 5 + 1 = {}", add_one(5));
    
    // 带多个参数的闭包
    let add = |x, y| x + y;
    println!("  🔹 3 + 4 = {}", add(3, 4));
    
    // 带类型注解的闭包
    let multiply: fn(i32, i32) -> i32 = |x, y| x * y;
    println!("  🔹 6 * 7 = {}", multiply(6, 7));
    
    // 闭包与函数的比较
    fn function_add(x: i32, y: i32) -> i32 {
        x + y
    }
    
    let closure_add = |x: i32, y: i32| -> i32 { x + y };
    
    println!("  🔹 函数结果: {}", function_add(2, 3));
    println!("  🔹 闭包结果: {}", closure_add(2, 3));
}

// ============================================================================
// 2. 闭包语法和类型推断
// ============================================================================

fn closure_syntax() {
    println!("\n📝 2. 闭包语法和类型推断");
    println!("{}", "-".repeat(40));
    
    // 不同的闭包语法形式
    println!("  🔸 不同的闭包语法：");
    
    // 完整语法
    let closure1 = |x: i32| -> i32 { x + 1 };
    println!("    完整语法: {}", closure1(5));
    
    // 省略类型注解
    let closure2 = |x| x + 1;
    println!("    省略类型: {}", closure2(5));
    
    // 单表达式省略大括号
    let closure3 = |x| x + 1;
    println!("    省略大括号: {}", closure3(5));
    
    // 无参数闭包
    let closure4 = || 42;
    println!("    无参数: {}", closure4());
    
    // 类型推断示例
    println!("\n  🔸 类型推断示例：");
    
    let example_closure = |x| x;
    
    // 第一次调用确定了类型
    let s = example_closure(String::from("hello"));
    println!("    字符串: {}", s);
    
    // 后续调用必须使用相同类型
    // let n = example_closure(5); // 这会编译错误
    
    // 演示闭包的灵活性
    let numbers = vec![1, 2, 3, 4, 5];
    
    // 使用闭包进行函数式编程
    let squares: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    println!("    平方: {:?}", squares);
    
    let evens: Vec<&i32> = numbers.iter().filter(|&x| x % 2 == 0).collect();
    println!("    偶数: {:?}", evens);
    
    let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);
    println!("    求和: {}", sum);
}

// ============================================================================
// 3. 捕获环境
// ============================================================================

fn capture_environment() {
    println!("\n🎪 3. 捕获环境");
    println!("{}", "-".repeat(40));
    
    // 不可变借用捕获
    println!("  🔸 不可变借用捕获：");
    let x = 4;
    let equal_to_x = |z| z == x;  // 捕获 x 的不可变引用
    
    let y = 4;
    println!("    {} == {} ? {}", y, x, equal_to_x(y));
    
    // 可变借用捕获
    println!("\n  🔸 可变借用捕获：");
    let mut list = vec![1, 2, 3];
    println!("    修改前: {:?}", list);
    
    let mut borrows_mutably = || list.push(7);
    // println!("    {:?}", list); // 这里不能使用 list，因为被可变借用了
    borrows_mutably();
    println!("    修改后: {:?}", list);
    
    // 获取所有权捕获
    println!("\n  🔸 获取所有权捕获：");
    let list2 = vec![1, 2, 3];
    println!("    移动前: {:?}", list2);
    
    let takes_ownership = move || {
        println!("    闭包内部: {:?}", list2);
        list2
    };
    
    let owned_list = takes_ownership();
    println!("    移动后: {:?}", owned_list);
    // println!("    {:?}", list2); // 这里不能使用 list2，因为已被移动
    
    // 在线程中使用 move 闭包
    println!("\n  🔸 线程中的 move 闭包：");
    let data = vec![1, 2, 3, 4, 5];
    
    let handle = thread::spawn(move || {
        println!("    线程中的数据: {:?}", data);
        data.len()
    });
    
    let result = handle.join().unwrap();
    println!("    线程返回: {}", result);
}

// ============================================================================
// 4. 闭包特征：Fn、FnMut、FnOnce
// ============================================================================

fn closure_traits() {
    println!("\n🎭 4. 闭包特征：Fn、FnMut、FnOnce");
    println!("{}", "-".repeat(40));
    
    println!("  🔸 Fn - 可以多次调用，不可变借用环境：");
    let x = 5;
    let fn_closure = |y| x + y;  // 实现 Fn
    println!("    第一次调用: {}", fn_closure(3));
    println!("    第二次调用: {}", fn_closure(4));
    
    println!("\n  🔸 FnMut - 可以多次调用，可变借用环境：");
    let mut counter = 0;
    let mut fn_mut_closure = || {  // 实现 FnMut
        counter += 1;
        counter
    };
    println!("    第一次调用: {}", fn_mut_closure());
    println!("    第二次调用: {}", fn_mut_closure());
    
    println!("\n  🔸 FnOnce - 只能调用一次，获取环境所有权：");
    let data = String::from("hello");
    let fn_once_closure = move || {  // 实现 FnOnce
        println!("    消费数据: {}", data);
        data  // 返回 data，消费所有权
    };
    let result = fn_once_closure();
    println!("    返回的数据: {}", result);
    // fn_once_closure(); // 这会编译错误，因为只能调用一次
    
    // 演示特征层次
    println!("\n  🔸 特征层次关系：");
    println!("    Fn: Send + Sync");
    println!("    FnMut: Send");
    println!("    FnOnce");
    println!("    所有 Fn 都实现 FnMut，所有 FnMut 都实现 FnOnce");
}

// ============================================================================
// 5. 闭包作为参数
// ============================================================================

fn closures_as_parameters() {
    println!("\n📤 5. 闭包作为参数");
    println!("{}", "-".repeat(40));
    
    // 使用泛型参数
    fn call_with_one<F>(f: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(1)
    }
    
    let double = |x| x * 2;
    let square = |x| x * x;
    
    println!("  🔸 使用泛型参数：");
    println!("    double(1) = {}", call_with_one(double));
    println!("    square(1) = {}", call_with_one(square));
    
    // 使用 Box<dyn Fn>
    fn call_boxed_closure(f: Box<dyn Fn(i32) -> i32>) -> i32 {
        f(5)
    }
    
    println!("\n  🔸 使用 Box<dyn Fn>：");
    let boxed_closure = Box::new(|x| x + 10);
    println!("    boxed_closure(5) = {}", call_boxed_closure(boxed_closure));
    
    // 不同的闭包特征作为参数
    fn execute_fn<F>(f: F) -> i32
    where
        F: Fn() -> i32,
    {
        f()
    }
    
    fn execute_fn_mut<F>(mut f: F) -> i32
    where
        F: FnMut() -> i32,
    {
        f()
    }
    
    fn execute_fn_once<F>(f: F) -> i32
    where
        F: FnOnce() -> i32,
    {
        f()
    }
    
    println!("\n  🔸 不同特征的闭包：");
    
    let value = 42;
    let fn_closure = || value;
    println!("    Fn 闭包: {}", execute_fn(fn_closure));
    println!("    Fn 闭包作为 FnMut: {}", execute_fn_mut(fn_closure));
    println!("    Fn 闭包作为 FnOnce: {}", execute_fn_once(fn_closure));
    
    let mut counter = 0;
    let fn_mut_closure = || {
        counter += 1;
        counter
    };
    // println!("    FnMut 闭包: {}", execute_fn(fn_mut_closure)); // 编译错误
    println!("    FnMut 闭包: {}", execute_fn_mut(fn_mut_closure));
    
         let data = String::from("test");
     let fn_once_closure = move || {
         data.len() as i32
     };
     println!("    FnOnce 闭包: {}", execute_fn_once(fn_once_closure));
}

// ============================================================================
// 6. 返回闭包
// ============================================================================

fn returning_closures() {
    println!("\n📥 6. 返回闭包");
    println!("{}", "-".repeat(40));
    
    // 返回 Box<dyn Fn>
    fn make_adder(x: i32) -> Box<dyn Fn(i32) -> i32> {
        Box::new(move |y| x + y)
    }
    
    let add_5 = make_adder(5);
    println!("  🔸 返回的闭包: add_5(3) = {}", add_5(3));
    
    // 返回不同类型的闭包
    fn make_closure(condition: bool) -> Box<dyn Fn(i32) -> i32> {
        if condition {
            Box::new(|x| x * 2)
        } else {
            Box::new(|x| x + 1)
        }
    }
    
    let closure1 = make_closure(true);
    let closure2 = make_closure(false);
    
    println!("  🔸 条件返回闭包:");
    println!("    closure1(5) = {}", closure1(5));
    println!("    closure2(5) = {}", closure2(5));
    
    // 使用 impl Fn 语法（需要所有分支返回相同类型）
    fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
        move |x| x * factor
    }
    
    let triple = make_multiplier(3);
    println!("  🔸 impl Fn 语法: triple(4) = {}", triple(4));
    
    // 闭包工厂
    fn create_counter() -> impl FnMut() -> i32 {
        let mut count = 0;
        move || {
            count += 1;
            count
        }
    }
    
    let mut counter = create_counter();
    println!("  🔸 闭包工厂:");
    println!("    第一次: {}", counter());
    println!("    第二次: {}", counter());
    println!("    第三次: {}", counter());
}

// ============================================================================
// 7. 实际应用示例
// ============================================================================

fn practical_examples() {
    println!("\n🚀 7. 实际应用示例");
    println!("{}", "-".repeat(40));
    
    // 缓存/记忆化
    cache_example();
    
    // 事件处理
    event_handling_example();
    
    // 函数式编程
    functional_programming_example();
    
    // 配置和策略模式
    strategy_pattern_example();
}

fn cache_example() {
    println!("\n  🔸 缓存/记忆化示例：");
    
    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }
    
    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }
        
        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }
    
    let expensive_calculation = |num| {
        println!("    计算中... (模拟耗时操作)");
        thread::sleep(Duration::from_millis(100));
        num * 2
    };
    
    let mut cacher = Cacher::new(expensive_calculation);
    
    println!("    第一次调用:");
    let result1 = cacher.value(5);
    println!("    结果: {}", result1);
    
    println!("    第二次调用 (使用缓存):");
    let result2 = cacher.value(5);
    println!("    结果: {}", result2);
}

fn event_handling_example() {
    println!("\n  🔸 事件处理示例：");
    
    struct EventHandler {
        handlers: Vec<Box<dyn Fn(&str)>>,
    }
    
    impl EventHandler {
        fn new() -> Self {
            EventHandler {
                handlers: Vec::new(),
            }
        }
        
        fn add_handler<F>(&mut self, handler: F)
        where
            F: Fn(&str) + 'static,
        {
            self.handlers.push(Box::new(handler));
        }
        
        fn trigger_event(&self, event: &str) {
            for handler in &self.handlers {
                handler(event);
            }
        }
    }
    
    let mut event_handler = EventHandler::new();
    
    // 添加不同的事件处理器
    event_handler.add_handler(|event| {
        println!("    日志处理器: 记录事件 '{}'", event);
    });
    
    event_handler.add_handler(|event| {
        println!("    邮件处理器: 发送关于 '{}' 的邮件", event);
    });
    
    let notification_count = std::cell::RefCell::new(0);
    event_handler.add_handler(move |event| {
        let mut count = notification_count.borrow_mut();
        *count += 1;
        println!("    通知处理器: 第 {} 次通知 '{}'", *count, event);
    });
    
    // 触发事件
    println!("    触发 '用户登录' 事件:");
    event_handler.trigger_event("用户登录");
}

fn functional_programming_example() {
    println!("\n  🔸 函数式编程示例：");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 链式操作
    let result: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)  // 筛选偶数
        .map(|&x| x * x)           // 计算平方
        .filter(|&x| x > 10)       // 筛选大于10的数
        .collect();
    
    println!("    偶数的平方中大于10的数: {:?}", result);
    
    // 使用 fold 进行归约
    let sum = numbers
        .iter()
        .fold(0, |acc, &x| acc + x);
    
    println!("    数字总和: {}", sum);
    
    // 使用 reduce
    let max = numbers
        .iter()
        .reduce(|a, b| if a > b { a } else { b });
    
    println!("    最大值: {:?}", max);
    
    // 复杂的数据处理
    let words = vec!["hello", "world", "rust", "programming", "language"];
    
    let long_words: Vec<String> = words
        .into_iter()
        .filter(|word| word.len() > 4)
        .map(|word| word.to_uppercase())
        .collect();
    
    println!("    长单词 (大写): {:?}", long_words);
}

fn strategy_pattern_example() {
    println!("\n  🔸 策略模式示例：");
    
    struct Calculator {
        strategy: Box<dyn Fn(f64, f64) -> f64>,
    }
    
    impl Calculator {
        fn new<F>(strategy: F) -> Self
        where
            F: Fn(f64, f64) -> f64 + 'static,
        {
            Calculator {
                strategy: Box::new(strategy),
            }
        }
        
        fn calculate(&self, a: f64, b: f64) -> f64 {
            (self.strategy)(a, b)
        }
        
        fn set_strategy<F>(&mut self, strategy: F)
        where
            F: Fn(f64, f64) -> f64 + 'static,
        {
            self.strategy = Box::new(strategy);
        }
    }
    
    // 不同的计算策略
    let add_strategy = |a: f64, b: f64| a + b;
    let multiply_strategy = |a: f64, b: f64| a * b;
    let power_strategy = |a: f64, b: f64| a.powf(b);
    
    let mut calc = Calculator::new(add_strategy);
    println!("    加法策略: 5 + 3 = {}", calc.calculate(5.0, 3.0));
    
    calc.set_strategy(multiply_strategy);
    println!("    乘法策略: 5 * 3 = {}", calc.calculate(5.0, 3.0));
    
    calc.set_strategy(power_strategy);
    println!("    幂运算策略: 5^3 = {}", calc.calculate(5.0, 3.0));
    
    // 配置驱动的行为
    let config = HashMap::from([
        ("debug", true),
        ("verbose", false),
    ]);
    
         let logger = if *config.get("debug").unwrap_or(&false) {
        Box::new(|msg: &str| println!("    [DEBUG] {}", msg)) as Box<dyn Fn(&str)>
    } else {
        Box::new(|msg: &str| println!("    [INFO] {}", msg)) as Box<dyn Fn(&str)>
    };
    
    logger("这是一条日志消息");
} 