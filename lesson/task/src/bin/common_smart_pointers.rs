// 教学示例 - 允许未使用的代码
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// 第18章：常用智能指针
// 演示 Cow、Weak、Pin 等常用智能指针

use std::borrow::Cow;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::pin::Pin;
use std::marker::PhantomPinned;

fn main() {
    println!("🐄 第18章：常用智能指针");
    println!("=====================================");
    
    // 1. Cow (Clone on Write)
    cow_demo();
    
    // 2. Weak 弱引用
    weak_reference_demo();
    
    // 3. Pin 固定指针
    pin_demo();
    
    // 4. 实际应用示例
    practical_examples();
}

// ============================================================================
// 1. Cow (Clone on Write)
// ============================================================================

fn cow_demo() {
    println!("\n🐄 1. Cow (Clone on Write)");
    println!("{}", "-".repeat(40));
    
    // 基本使用
    println!("  🔸 基本使用：");
    
    let s1 = "hello";
    let cow1: Cow<str> = Cow::Borrowed(s1);
    println!("    借用字符串: {:?}", cow1);
    
    let s2 = String::from("world");
    let cow2: Cow<str> = Cow::Owned(s2);
    println!("    拥有字符串: {:?}", cow2);
    
    // 从不同类型创建
    println!("\n  🔸 从不同类型创建：");
    
    let borrowed: Cow<str> = "hello".into();
    let owned: Cow<str> = String::from("world").into();
    
    println!("    从 &str: {:?}", borrowed);
    println!("    从 String: {:?}", owned);
    
    // 延迟克隆
    println!("\n  🔸 延迟克隆示例：");
    
    fn process_text(input: &str) -> Cow<str> {
        if input.contains("bad") {
            // 需要修改，进行克隆
            Cow::Owned(input.replace("bad", "good"))
        } else {
            // 不需要修改，直接借用
            Cow::Borrowed(input)
        }
    }
    
    let text1 = "This is a good example";
    let text2 = "This is a bad example";
    
    let result1 = process_text(text1);
    let result2 = process_text(text2);
    
    println!("    处理 '{}': {:?}", text1, result1);
    println!("    处理 '{}': {:?}", text2, result2);
    
    match result1 {
        Cow::Borrowed(_) => println!("    result1 是借用的"),
        Cow::Owned(_) => println!("    result1 是拥有的"),
    }
    
    match result2 {
        Cow::Borrowed(_) => println!("    result2 是借用的"),
        Cow::Owned(_) => println!("    result2 是拥有的"),
    }
    
    // 数组的 Cow
    println!("\n  🔸 数组的 Cow：");
    
    fn process_numbers(input: &[i32]) -> Cow<[i32]> {
        if input.iter().any(|&x| x < 0) {
            // 有负数，需要转换为正数
            let positive: Vec<i32> = input.iter().map(|&x| x.abs()).collect();
            Cow::Owned(positive)
        } else {
            // 没有负数，直接借用
            Cow::Borrowed(input)
        }
    }
    
    let nums1 = [1, 2, 3, 4, 5];
    let nums2 = [1, -2, 3, -4, 5];
    
    let result1 = process_numbers(&nums1);
    let result2 = process_numbers(&nums2);
    
    println!("    处理 {:?}: {:?}", nums1, result1);
    println!("    处理 {:?}: {:?}", nums2, result2);
    
    // to_mut 方法
    println!("\n  🔸 to_mut 方法：");
    
    let mut cow: Cow<str> = "hello".into();
    println!("    初始: {:?}", cow);
    
    // 获取可变引用，如果是借用的会自动克隆
    let mutable_ref = cow.to_mut();
    mutable_ref.push_str(" world");
    
    println!("    修改后: {:?}", cow);
    
    // 配置管理示例
    println!("\n  🔸 配置管理示例：");
    
    #[derive(Debug)]
    struct Config<'a> {
        database_url: Cow<'a, str>,
        port: u16,
        debug: bool,
    }
    
    impl<'a> Config<'a> {
        fn new(database_url: &'a str) -> Self {
            Config {
                database_url: Cow::Borrowed(database_url),
                port: 8080,
                debug: false,
            }
        }
        
        fn with_custom_url(database_url: String) -> Config<'static> {
            Config {
                database_url: Cow::Owned(database_url),
                port: 8080,
                debug: false,
            }
        }
        
        fn set_debug(&mut self, debug: bool) {
            self.debug = debug;
        }
        
        fn get_url(&self) -> &str {
            &self.database_url
        }
    }
    
    let default_url = "postgresql://localhost/mydb";
    let mut config1 = Config::new(default_url);
    config1.set_debug(true);
    
    let custom_url = format!("postgresql://{}:5432/custom", "remote-host");
    let config2 = Config::with_custom_url(custom_url);
    
    println!("    配置1: {:?}", config1);
    println!("    配置2: {:?}", config2);
}

// ============================================================================
// 2. Weak 弱引用
// ============================================================================

fn weak_reference_demo() {
    println!("\n🔗 2. Weak 弱引用");
    println!("{}", "-".repeat(40));
    
    // 基本使用
    println!("  🔸 基本使用：");
    
    let strong_ref = Rc::new(42);
    println!("    强引用计数: {}", Rc::strong_count(&strong_ref));
    println!("    弱引用计数: {}", Rc::weak_count(&strong_ref));
    
    let weak_ref = Rc::downgrade(&strong_ref);
    println!("    创建弱引用后 - 强引用计数: {}", Rc::strong_count(&strong_ref));
    println!("    创建弱引用后 - 弱引用计数: {}", Rc::weak_count(&strong_ref));
    
    // 尝试升级弱引用
    if let Some(strong_from_weak) = weak_ref.upgrade() {
        println!("    弱引用升级成功: {}", strong_from_weak);
        println!("    升级后强引用计数: {}", Rc::strong_count(&strong_ref));
    }
    
    // 释放强引用
    drop(strong_ref);
    
    // 再次尝试升级
    if let Some(_) = weak_ref.upgrade() {
        println!("    弱引用仍然有效");
    } else {
        println!("    弱引用已失效");
    }
    
    // 避免循环引用
    println!("\n  🔸 避免循环引用：");
    
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }
    
    impl Node {
        fn new(value: i32) -> Rc<Self> {
            Rc::new(Node {
                value,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(Vec::new()),
            })
        }
        
        fn add_child(parent: &Rc<Node>, child: Rc<Node>) {
            *child.parent.borrow_mut() = Rc::downgrade(parent);
            parent.children.borrow_mut().push(child);
        }
        
        fn get_parent(&self) -> Option<Rc<Node>> {
            self.parent.borrow().upgrade()
        }
        
        fn print_tree(&self, depth: usize) {
            let indent = "  ".repeat(depth);
            println!("{}节点值: {}", indent, self.value);
            
            for child in self.children.borrow().iter() {
                child.print_tree(depth + 1);
            }
        }
    }
    
    let root = Node::new(1);
    let child1 = Node::new(2);
    let child2 = Node::new(3);
    let grandchild = Node::new(4);
    
    Node::add_child(&root, child1.clone());
    Node::add_child(&root, child2.clone());
    Node::add_child(&child1, grandchild.clone());
    
    println!("    树结构:");
    root.print_tree(1);
    
    println!("    引用计数:");
    println!("      根节点强引用: {}", Rc::strong_count(&root));
    println!("      子节点1强引用: {}", Rc::strong_count(&child1));
    println!("      孙节点强引用: {}", Rc::strong_count(&grandchild));
    
    // 验证父子关系
    if let Some(parent) = grandchild.get_parent() {
        println!("    孙节点的父节点值: {}", parent.value);
    }
    
    // 缓存示例
    println!("\n  🔸 缓存示例：");
    
    use std::collections::HashMap;
    
    struct Cache {
        data: RefCell<HashMap<String, Weak<String>>>,
    }
    
    impl Cache {
        fn new() -> Self {
            Cache {
                data: RefCell::new(HashMap::new()),
            }
        }
        
        fn get(&self, key: &str) -> Option<Rc<String>> {
            let mut cache = self.data.borrow_mut();
            
            if let Some(weak_ref) = cache.get(key) {
                if let Some(strong_ref) = weak_ref.upgrade() {
                    println!("    缓存命中: {}", key);
                    return Some(strong_ref);
                } else {
                    // 弱引用已失效，移除
                    cache.remove(key);
                }
            }
            
            println!("    缓存未命中: {}", key);
            None
        }
        
        fn insert(&self, key: String, value: Rc<String>) {
            let weak_ref = Rc::downgrade(&value);
            self.data.borrow_mut().insert(key, weak_ref);
        }
        
        fn cleanup(&self) {
            let mut cache = self.data.borrow_mut();
            cache.retain(|_, weak_ref| weak_ref.upgrade().is_some());
            println!("    缓存清理完成");
        }
    }
    
    let cache = Cache::new();
    
    {
        let value1 = Rc::new(String::from("数据1"));
        let value2 = Rc::new(String::from("数据2"));
        
        cache.insert("key1".to_string(), value1.clone());
        cache.insert("key2".to_string(), value2.clone());
        
        // 第一次访问
        let _retrieved1 = cache.get("key1");
        let _retrieved2 = cache.get("key2");
        
    } // value1 和 value2 在这里被释放
    
    // 再次访问，应该缓存未命中
    let _retrieved1 = cache.get("key1");
    let _retrieved2 = cache.get("key2");
    
    cache.cleanup();
}

// ============================================================================
// 3. Pin 固定指针
// ============================================================================

fn pin_demo() {
    println!("\n📌 3. Pin 固定指针");
    println!("{}", "-".repeat(40));
    
    // 基本概念
    println!("  🔸 基本概念：");
    
    let mut data = String::from("hello");
    let pinned = Pin::new(&mut data);
    
    println!("    固定的数据: {:?}", pinned);
    
    // 自引用结构体
    println!("\n  🔸 自引用结构体：");
    
    #[derive(Debug)]
    struct SelfReferential {
        data: String,
        pointer: *const u8,
        _pin: PhantomPinned,
    }
    
    impl SelfReferential {
        fn new(data: String) -> Pin<Box<Self>> {
            let mut boxed = Box::pin(SelfReferential {
                data,
                pointer: std::ptr::null(),
                _pin: PhantomPinned,
            });
            
            // 安全地设置自引用指针
            let ptr = boxed.data.as_ptr();
            unsafe {
                let mut_ref = Pin::as_mut(&mut boxed);
                Pin::get_unchecked_mut(mut_ref).pointer = ptr;
            }
            
            boxed
        }
        
        fn data(&self) -> &str {
            &self.data
        }
        
        fn pointer_valid(&self) -> bool {
            self.pointer == self.data.as_ptr()
        }
    }
    
    let pinned_struct = SelfReferential::new(String::from("pinned data"));
    println!("    数据: {}", pinned_struct.data());
    println!("    指针有效: {}", pinned_struct.pointer_valid());
    
    // 异步上下文中的 Pin
    println!("\n  🔸 异步上下文模拟：");
    
    use std::future::Future;
    use std::task::{Context, Poll, Waker};
    use std::time::{Duration, Instant};
    
    struct DelayFuture {
        duration: Duration,
        start_time: Option<Instant>,
    }
    
    impl DelayFuture {
        fn new(duration: Duration) -> Self {
            DelayFuture {
                duration,
                start_time: None,
            }
        }
    }
    
    impl Future for DelayFuture {
        type Output = ();
        
        fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.start_time.is_none() {
                self.start_time = Some(Instant::now());
                println!("    开始延迟计时");
                Poll::Pending
            } else {
                let elapsed = self.start_time.unwrap().elapsed();
                if elapsed >= self.duration {
                    println!("    延迟完成");
                    Poll::Ready(())
                } else {
                    println!("    延迟进行中... ({:?} / {:?})", elapsed, self.duration);
                    Poll::Pending
                }
            }
        }
    }
    
    // 模拟简单的执行器
    let mut future = Box::pin(DelayFuture::new(Duration::from_millis(100)));
    
    // 创建一个虚拟的 Waker
    use std::task::RawWaker;
    use std::task::RawWakerVTable;
    
    fn raw_waker() -> RawWaker {
        fn no_op(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker {
            raw_waker()
        }
        
        let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
        RawWaker::new(std::ptr::null::<()>(), vtable)
    }
    
    let waker = unsafe { Waker::from_raw(raw_waker()) };
    let mut context = Context::from_waker(&waker);
    
    // 简单的轮询
    for i in 0..3 {
        println!("    轮询 {}", i + 1);
        match future.as_mut().poll(&mut context) {
            Poll::Ready(_) => {
                println!("    Future 完成");
                break;
            }
            Poll::Pending => {
                println!("    Future 仍在等待");
                std::thread::sleep(Duration::from_millis(50));
            }
        }
    }
    
    // Pin 的移动限制
    println!("\n  🔸 Pin 的移动限制：");
    
    struct Immovable {
        data: String,
        slice: *const u8,
        _pin: PhantomPinned,
    }
    
    impl Immovable {
        fn new(data: String) -> Pin<Box<Self>> {
            let mut boxed = Box::pin(Immovable {
                data,
                slice: std::ptr::null(),
                _pin: PhantomPinned,
            });
            
            let slice = boxed.data.as_ptr();
            unsafe {
                let mut_ref = Pin::as_mut(&mut boxed);
                Pin::get_unchecked_mut(mut_ref).slice = slice;
            }
            
            boxed
        }
        
        fn get_slice(&self) -> &str {
            unsafe { 
                let len = self.data.len();
                let slice = std::slice::from_raw_parts(self.slice, len);
                std::str::from_utf8_unchecked(slice)
            }
        }
    }
    
    let immovable = Immovable::new(String::from("不可移动的数据"));
    println!("    切片内容: {}", immovable.get_slice());
    
    // 这里不能移动 immovable，因为它被 Pin 固定了
    // let moved = immovable; // 这会编译错误
}

// ============================================================================
// 4. 实际应用示例
// ============================================================================

fn practical_examples() {
    println!("\n🚀 4. 实际应用示例");
    println!("{}", "-".repeat(40));
    
    // 配置系统
    config_system_example();
    
    // 观察者模式改进
    improved_observer_pattern();
    
    // 异步任务管理
    async_task_management();
}

fn config_system_example() {
    println!("\n  🔸 配置系统示例：");
    
    use std::collections::HashMap;
    
    #[derive(Debug, Clone)]
    struct AppConfig {
        settings: HashMap<String, Cow<'static, str>>,
    }
    
    impl AppConfig {
        fn new() -> Self {
            let mut settings = HashMap::new();
            
            // 默认配置（借用静态字符串）
            settings.insert("host".to_string(), Cow::Borrowed("localhost"));
            settings.insert("port".to_string(), Cow::Borrowed("8080"));
            settings.insert("debug".to_string(), Cow::Borrowed("false"));
            
            AppConfig { settings }
        }
        
        fn set(&mut self, key: String, value: String) {
            self.settings.insert(key, Cow::Owned(value));
        }
        
        fn get(&self, key: &str) -> Option<&str> {
            self.settings.get(key).map(|cow| cow.as_ref())
        }
        
        fn override_from_env(&mut self) {
            // 模拟从环境变量覆盖配置
            if let Ok(port) = std::env::var("APP_PORT") {
                self.set("port".to_string(), port);
            }
            
            // 模拟设置自定义主机
            self.set("host".to_string(), "0.0.0.0".to_string());
        }
    }
    
    let mut config = AppConfig::new();
    println!("    默认配置: {:?}", config);
    
    config.override_from_env();
    println!("    环境变量覆盖后: {:?}", config);
    
    println!("    主机: {:?}", config.get("host"));
    println!("    端口: {:?}", config.get("port"));
}

fn improved_observer_pattern() {
    println!("\n  🔸 改进的观察者模式：");
    
    trait Observer {
        fn notify(&self, event: &str);
    }
    
    struct EmailNotifier {
        email: String,
    }
    
    impl Observer for EmailNotifier {
        fn notify(&self, event: &str) {
            println!("    📧 发送邮件到 {}: {}", self.email, event);
        }
    }
    
    struct EventManager {
        observers: RefCell<Vec<Weak<dyn Observer>>>,
    }
    
    impl EventManager {
        fn new() -> Self {
            EventManager {
                observers: RefCell::new(Vec::new()),
            }
        }
        
        fn subscribe(&self, observer: Rc<dyn Observer>) {
            self.observers.borrow_mut().push(Rc::downgrade(&observer));
        }
        
        fn notify_all(&self, event: &str) {
            let mut observers = self.observers.borrow_mut();
            
            // 保留仍然有效的观察者
            observers.retain(|weak_observer| {
                if let Some(observer) = weak_observer.upgrade() {
                    observer.notify(event);
                    true
                } else {
                    false
                }
            });
        }
        
        fn cleanup(&self) {
            let mut observers = self.observers.borrow_mut();
            observers.retain(|weak_observer| weak_observer.upgrade().is_some());
            println!("    清理了无效的观察者");
        }
    }
    
    let event_manager = EventManager::new();
    
    {
        let email_observer = Rc::new(EmailNotifier {
            email: "user@example.com".to_string(),
        });
        
        event_manager.subscribe(email_observer.clone());
        event_manager.notify_all("用户登录");
        
    } // email_observer 在这里被释放
    
    event_manager.notify_all("用户注销"); // 应该没有输出
    event_manager.cleanup();
}

fn async_task_management() {
    println!("\n  🔸 异步任务管理示例：");
    
    use std::collections::VecDeque;
    
    #[derive(Debug)]
    struct Task {
        id: usize,
        name: String,
        _pinned: PhantomPinned,
    }
    
    impl Task {
        fn new(id: usize, name: String) -> Pin<Box<Self>> {
            Box::pin(Task {
                id,
                name,
                _pinned: PhantomPinned,
            })
        }
        
        fn execute(&self) {
            println!("    执行任务 {}: {}", self.id, self.name);
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
    
    struct TaskManager {
        tasks: RefCell<VecDeque<Pin<Box<Task>>>>,
        completed: RefCell<Vec<usize>>,
    }
    
    impl TaskManager {
        fn new() -> Self {
            TaskManager {
                tasks: RefCell::new(VecDeque::new()),
                completed: RefCell::new(Vec::new()),
            }
        }
        
        fn add_task(&self, task: Pin<Box<Task>>) {
            self.tasks.borrow_mut().push_back(task);
        }
        
        fn run_next_task(&self) -> bool {
            if let Some(task) = self.tasks.borrow_mut().pop_front() {
                task.execute();
                self.completed.borrow_mut().push(task.id);
                true
            } else {
                false
            }
        }
        
        fn run_all_tasks(&self) {
            while self.run_next_task() {
                // 继续执行直到没有任务
            }
            println!("    所有任务执行完成");
            println!("    完成的任务 ID: {:?}", self.completed.borrow());
        }
    }
    
    let task_manager = TaskManager::new();
    
    // 添加一些任务
    for i in 1..=5 {
        let task = Task::new(i, format!("任务{}", i));
        task_manager.add_task(task);
    }
    
    // 执行所有任务
    task_manager.run_all_tasks();
} 