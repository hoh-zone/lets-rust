// 第17章：智能指针
// 演示 Rust 中的智能指针：Box、Rc、RefCell、Arc、Mutex等

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("📦 第17章：智能指针");
    println!("=====================================");
    
    // 1. Box<T> - 堆分配
    box_pointer_demo();
    
    // 2. Rc<T> - 引用计数
    rc_pointer_demo();
    
    // 3. RefCell<T> - 内部可变性
    refcell_demo();
    
    // 4. Rc<RefCell<T>> 组合
    rc_refcell_combo();
    
    // 5. Arc<T> - 原子引用计数
    arc_demo();
    
    // 6. Mutex<T> - 互斥锁
    mutex_demo();
    
    // 7. 实际应用示例
    practical_examples();
}

// ============================================================================
// 1. Box<T> - 堆分配
// ============================================================================

fn box_pointer_demo() {
    println!("\n📦 1. Box<T> - 堆分配");
    println!("{}", "-".repeat(40));
    
    // 基本使用
    println!("  🔸 基本使用：");
    let b = Box::new(5);
    println!("    Box 中的值: {}", b);
    println!("    值的地址: {:p}", &*b);
    
    // 递归数据结构
    println!("\n  🔸 递归数据结构 - 链表：");
    
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    
    use List::{Cons, Nil};
    
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("    链表: {:?}", list);
    
    // 二叉树
    println!("\n  🔸 递归数据结构 - 二叉树：");
    
    #[derive(Debug)]
    struct TreeNode {
        value: i32,
        left: Option<Box<TreeNode>>,
        right: Option<Box<TreeNode>>,
    }
    
    impl TreeNode {
        fn new(value: i32) -> Self {
            TreeNode {
                value,
                left: None,
                right: None,
            }
        }
        
        fn insert(&mut self, value: i32) {
            if value < self.value {
                match &mut self.left {
                    Some(left) => left.insert(value),
                    None => self.left = Some(Box::new(TreeNode::new(value))),
                }
            } else {
                match &mut self.right {
                    Some(right) => right.insert(value),
                    None => self.right = Some(Box::new(TreeNode::new(value))),
                }
            }
        }
        
        fn contains(&self, value: i32) -> bool {
            if value == self.value {
                true
            } else if value < self.value {
                self.left.as_ref().map_or(false, |left| left.contains(value))
            } else {
                self.right.as_ref().map_or(false, |right| right.contains(value))
            }
        }
    }
    
    let mut tree = TreeNode::new(10);
    tree.insert(5);
    tree.insert(15);
    tree.insert(3);
    tree.insert(7);
    
    println!("    二叉搜索树: {:#?}", tree);
    println!("    包含 7: {}", tree.contains(7));
    println!("    包含 12: {}", tree.contains(12));
    
    // 大型数据
    println!("\n  🔸 大型数据的堆分配：");
    let large_array = Box::new([0; 1000]);
    println!("    大数组已分配到堆上，长度: {}", large_array.len());
}

// ============================================================================
// 2. Rc<T> - 引用计数
// ============================================================================

fn rc_pointer_demo() {
    println!("\n🔄 2. Rc<T> - 引用计数");
    println!("{}", "-".repeat(40));
    
    // 基本使用
    println!("  🔸 基本使用：");
    let a = Rc::new(5);
    println!("    创建 Rc，引用计数: {}", Rc::strong_count(&a));
    
    let b = Rc::clone(&a);
    println!("    克隆后，引用计数: {}", Rc::strong_count(&a));
    
    {
        let c = Rc::clone(&a);
        println!("    再次克隆，引用计数: {}", Rc::strong_count(&a));
    }
    println!("    作用域结束，引用计数: {}", Rc::strong_count(&a));
    
    // 共享数据结构
    println!("\n  🔸 共享数据结构 - 多个所有者的链表：");
    
    #[derive(Debug)]
    enum RcList {
        Cons(i32, Rc<RcList>),
        Nil,
    }
    
    use RcList::{Cons as RcCons, Nil as RcNil};
    
    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("    链表 a 的引用计数: {}", Rc::strong_count(&a));
    
    let b = RcCons(3, Rc::clone(&a));
    println!("    创建 b 后，a 的引用计数: {}", Rc::strong_count(&a));
    
    let c = RcCons(4, Rc::clone(&a));
    println!("    创建 c 后，a 的引用计数: {}", Rc::strong_count(&a));
    
    println!("    链表 b: {:?}", b);
    println!("    链表 c: {:?}", c);
    
    // 图结构
    println!("\n  🔸 图结构：");
    
    #[derive(Debug)]
    struct Node {
        value: i32,
        children: Vec<Rc<Node>>,
    }
    
    impl Node {
        fn new(value: i32) -> Rc<Self> {
            Rc::new(Node {
                value,
                children: Vec::new(),
            })
        }
    }
    
    let leaf = Node::new(3);
    let branch = Rc::new(Node {
        value: 5,
        children: vec![Rc::clone(&leaf)],
    });
    let root = Rc::new(Node {
        value: 10,
        children: vec![Rc::clone(&branch), Rc::clone(&leaf)],
    });
    
    println!("    叶子节点引用计数: {}", Rc::strong_count(&leaf));
    println!("    分支节点引用计数: {}", Rc::strong_count(&branch));
    println!("    根节点: {:#?}", root);
}

// ============================================================================
// 3. RefCell<T> - 内部可变性
// ============================================================================

fn refcell_demo() {
    println!("\n🔄 3. RefCell<T> - 内部可变性");
    println!("{}", "-".repeat(40));
    
    // 基本使用
    println!("  🔸 基本使用：");
    let data = RefCell::new(5);
    
    println!("    初始值: {}", data.borrow());
    
    *data.borrow_mut() = 10;
    println!("    修改后: {}", data.borrow());
    
    // 运行时借用检查
    println!("\n  🔸 运行时借用检查：");
    let value = RefCell::new(42);
    
    // 多个不可变借用
    {
        let borrow1 = value.borrow();
        let borrow2 = value.borrow();
        println!("    多个不可变借用: {} 和 {}", *borrow1, *borrow2);
    }
    
    // 一个可变借用
    {
        let mut borrow_mut = value.borrow_mut();
        *borrow_mut = 100;
        println!("    可变借用修改: {}", *borrow_mut);
    }
    
    // Mock 对象模式
    println!("\n  🔸 Mock 对象模式：");
    
    trait Messenger {
        fn send(&self, msg: &str);
    }
    
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }
    
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }
    
    let mock_messenger = MockMessenger::new();
    mock_messenger.send("Hello");
    mock_messenger.send("World");
    
    println!("    发送的消息: {:?}", mock_messenger.sent_messages.borrow());
    
    // 限额跟踪器
    struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }
    
    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }
        
        fn set_value(&mut self, value: usize) {
            self.value = value;
            
            let percentage_of_max = self.value as f64 / self.max as f64;
            
            if percentage_of_max >= 1.0 {
                self.messenger.send("错误：超出配额！");
            } else if percentage_of_max >= 0.9 {
                self.messenger.send("紧急警告：已使用配额的90%以上");
            } else if percentage_of_max >= 0.75 {
                self.messenger.send("警告：已使用配额的75%以上");
            }
        }
    }
    
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
    limit_tracker.set_value(80);
    limit_tracker.set_value(95);
    limit_tracker.set_value(105);
    
    println!("    限额跟踪消息: {:?}", mock_messenger.sent_messages.borrow());
}

// ============================================================================
// 4. Rc<RefCell<T>> 组合
// ============================================================================

fn rc_refcell_combo() {
    println!("\n🔄📦 4. Rc<RefCell<T>> 组合");
    println!("{}", "-".repeat(40));
    
    println!("  🔸 多所有权 + 内部可变性：");
    
    #[derive(Debug)]
    struct Node {
        value: RefCell<i32>,
        children: RefCell<Vec<Rc<Node>>>,
    }
    
    impl Node {
        fn new(value: i32) -> Rc<Self> {
            Rc::new(Node {
                value: RefCell::new(value),
                children: RefCell::new(Vec::new()),
            })
        }
        
        fn add_child(&self, child: Rc<Node>) {
            self.children.borrow_mut().push(child);
        }
        
        fn set_value(&self, value: i32) {
            *self.value.borrow_mut() = value;
        }
        
        fn get_value(&self) -> i32 {
            *self.value.borrow()
        }
    }
    
    let root = Node::new(1);
    let child1 = Node::new(2);
    let child2 = Node::new(3);
    
    root.add_child(Rc::clone(&child1));
    root.add_child(Rc::clone(&child2));
    
    println!("    根节点值: {}", root.get_value());
    println!("    子节点数量: {}", root.children.borrow().len());
    
    // 修改共享节点的值
    child1.set_value(20);
    child2.set_value(30);
    
    println!("    修改后 child1 值: {}", child1.get_value());
    println!("    修改后 child2 值: {}", child2.get_value());
    
    // 共享状态
    println!("\n  🔸 共享状态示例：");
    
    #[derive(Debug)]
    struct Counter {
        count: Rc<RefCell<i32>>,
    }
    
    impl Counter {
        fn new() -> Self {
            Counter {
                count: Rc::new(RefCell::new(0)),
            }
        }
        
        fn increment(&self) {
            *self.count.borrow_mut() += 1;
        }
        
        fn get_count(&self) -> i32 {
            *self.count.borrow()
        }
        
        fn clone_counter(&self) -> Counter {
            Counter {
                count: Rc::clone(&self.count),
            }
        }
    }
    
    let counter1 = Counter::new();
    let counter2 = counter1.clone_counter();
    let counter3 = counter1.clone_counter();
    
    counter1.increment();
    counter2.increment();
    counter3.increment();
    
    println!("    counter1 计数: {}", counter1.get_count());
    println!("    counter2 计数: {}", counter2.get_count());
    println!("    counter3 计数: {}", counter3.get_count());
    println!("    所有计数器共享同一个值！");
}

// ============================================================================
// 5. Arc<T> - 原子引用计数
// ============================================================================

fn arc_demo() {
    println!("\n⚛️ 5. Arc<T> - 原子引用计数");
    println!("{}", "-".repeat(40));
    
    // 基本使用
    println!("  🔸 基本使用：");
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    
    println!("    创建 Arc，引用计数: {}", Arc::strong_count(&data));
    
    let data_clone = Arc::clone(&data);
    println!("    克隆后，引用计数: {}", Arc::strong_count(&data));
    
    // 多线程共享
    println!("\n  🔸 多线程共享数据：");
    
    let shared_data = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];
    
    for i in 0..3 {
        let data = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            println!("    线程 {} 访问数据: {:?}", i, data);
            data.len()
        });
        handles.push(handle);
    }
    
    for handle in handles {
        let result = handle.join().unwrap();
        println!("    线程返回数据长度: {}", result);
    }
    
    println!("    主线程中的数据: {:?}", shared_data);
    println!("    最终引用计数: {}", Arc::strong_count(&shared_data));
    
    // 并行计算
    println!("\n  🔸 并行计算示例：");
    
    let numbers = Arc::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let mut handles = vec![];
    
    // 将数据分成两部分并行处理
    for chunk_id in 0..2 {
        let data = Arc::clone(&numbers);
        let handle = thread::spawn(move || {
            let start = chunk_id * 5;
            let end = start + 5;
            let sum: i32 = data[start..end].iter().sum();
            println!("    线程 {} 处理 [{}, {}): sum = {}", chunk_id, start, end, sum);
            sum
        });
        handles.push(handle);
    }
    
    let mut total = 0;
    for handle in handles {
        total += handle.join().unwrap();
    }
    
    println!("    并行计算总和: {}", total);
}

// ============================================================================
// 6. Mutex<T> - 互斥锁
// ============================================================================

fn mutex_demo() {
    println!("\n🔒 6. Mutex<T> - 互斥锁");
    println!("{}", "-".repeat(40));
    
    // 基本使用
    println!("  🔸 基本使用：");
    let m = Mutex::new(5);
    
    {
        let mut num = m.lock().unwrap();
        *num = 6;
        println!("    修改后的值: {}", *num);
    }
    
    println!("    锁外访问: {:?}", m);
    
    // 多线程计数器
    println!("\n  🔸 多线程计数器：");
    
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("    线程 {} 增加计数器", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("    最终计数: {}", *counter.lock().unwrap());
    
    // 共享状态修改
    println!("\n  🔸 共享状态修改：");
    
    let data = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];
    
    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut vec = data.lock().unwrap();
            vec.push(i);
            println!("    线程 {} 添加数据", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("    最终数据: {:?}", *data.lock().unwrap());
}

// ============================================================================
// 7. 实际应用示例
// ============================================================================

fn practical_examples() {
    println!("\n🚀 7. 实际应用示例");
    println!("{}", "-".repeat(40));
    
    // 缓存系统
    cache_system_example();
    
    // 观察者模式
    observer_pattern_example();
    
    // 线程池
    thread_pool_example();
}

fn cache_system_example() {
    println!("\n  🔸 缓存系统：");
    
    use std::collections::HashMap;
    
    #[derive(Debug)]
    struct Cache {
        data: Arc<Mutex<HashMap<String, String>>>,
    }
    
    impl Cache {
        fn new() -> Self {
            Cache {
                data: Arc::new(Mutex::new(HashMap::new())),
            }
        }
        
        fn get(&self, key: &str) -> Option<String> {
            let cache = self.data.lock().unwrap();
            cache.get(key).cloned()
        }
        
        fn set(&self, key: String, value: String) {
            let mut cache = self.data.lock().unwrap();
            cache.insert(key, value);
        }
        
        fn clone_cache(&self) -> Cache {
            Cache {
                data: Arc::clone(&self.data),
            }
        }
    }
    
    let cache = Cache::new();
    
    // 模拟多个线程访问缓存
    let cache1 = cache.clone_cache();
    let cache2 = cache.clone_cache();
    
    let handle1 = thread::spawn(move || {
        cache1.set("user:1".to_string(), "Alice".to_string());
        println!("    线程1 设置缓存: user:1 = Alice");
    });
    
    let handle2 = thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(10));
        if let Some(value) = cache2.get("user:1") {
            println!("    线程2 读取缓存: user:1 = {}", value);
        } else {
            println!("    线程2 缓存未命中");
        }
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
    
    cache.set("user:2".to_string(), "Bob".to_string());
    println!("    主线程读取: user:2 = {:?}", cache.get("user:2"));
}

fn observer_pattern_example() {
    println!("\n  🔸 观察者模式：");
    
    trait Observer {
        fn update(&self, message: &str);
    }
    
    struct ConcreteObserver {
        id: usize,
    }
    
    impl Observer for ConcreteObserver {
        fn update(&self, message: &str) {
            println!("    观察者 {} 收到消息: {}", self.id, message);
        }
    }
    
    struct Subject {
        observers: RefCell<Vec<Rc<dyn Observer>>>,
    }
    
    impl Subject {
        fn new() -> Self {
            Subject {
                observers: RefCell::new(Vec::new()),
            }
        }
        
        fn attach(&self, observer: Rc<dyn Observer>) {
            self.observers.borrow_mut().push(observer);
        }
        
        fn notify(&self, message: &str) {
            for observer in self.observers.borrow().iter() {
                observer.update(message);
            }
        }
    }
    
    let subject = Subject::new();
    
    let observer1 = Rc::new(ConcreteObserver { id: 1 });
    let observer2 = Rc::new(ConcreteObserver { id: 2 });
    let observer3 = Rc::new(ConcreteObserver { id: 3 });
    
    subject.attach(observer1);
    subject.attach(observer2);
    subject.attach(observer3);
    
    subject.notify("第一条消息");
    subject.notify("第二条消息");
}

fn thread_pool_example() {
    println!("\n  🔸 简单线程池：");
    
    use std::sync::mpsc;
    
    struct ThreadPool {
        workers: Vec<Worker>,
        sender: mpsc::Sender<Job>,
    }
    
    type Job = Box<dyn FnOnce() + Send + 'static>;
    
    struct Worker {
        id: usize,
        thread: thread::JoinHandle<()>,
    }
    
    impl ThreadPool {
        fn new(size: usize) -> ThreadPool {
            assert!(size > 0);
            
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));
            
            let mut workers = Vec::with_capacity(size);
            
            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }
            
            ThreadPool { workers, sender }
        }
        
        fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);
            self.sender.send(job).unwrap();
        }
    }
    
    impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
            let thread = thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("    Worker {} 开始执行任务", id);
                job();
            });
            
            Worker { id, thread }
        }
    }
    
    let pool = ThreadPool::new(3);
    
    for i in 0..5 {
        pool.execute(move || {
            println!("    任务 {} 正在执行", i);
            thread::sleep(std::time::Duration::from_millis(100));
            println!("    任务 {} 完成", i);
        });
    }
    
    // 等待一段时间让任务完成
    thread::sleep(std::time::Duration::from_millis(1000));
    println!("    所有任务已提交到线程池");
} 