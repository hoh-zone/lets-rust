# 第18章：常见智能指针及其应用

本章将深入学习 Rust 中最常用的智能指针类型，包括引用计数、弱引用、原子引用计数和内部可变性等概念。

## 18.1 Rc<T>：引用计数智能指针

### Rc<T> 基础概念

Rc<T>（Reference Counted）允许单个值有多个所有者，通过引用计数来管理内存。

```rust
use std::rc::Rc;

fn main() {
    // 创建 Rc
    let a = Rc::new(5);
    println!("a 的引用计数: {}", Rc::strong_count(&a));
    
    // 克隆引用
    let b = Rc::clone(&a);
    println!("a 的引用计数: {}", Rc::strong_count(&a));
    
    {
        let c = Rc::clone(&a);
        println!("a 的引用计数: {}", Rc::strong_count(&a));
    }
    
    // c 离开作用域后
    println!("a 的引用计数: {}", Rc::strong_count(&a));
}
```

### Rc<T> 与链表

```rust
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("a 的引用计数: {}", Rc::strong_count(&a));
    
    let b = Cons(3, Rc::clone(&a));
    println!("a 的引用计数: {}", Rc::strong_count(&a));
    
    let c = Cons(4, Rc::clone(&a));
    println!("a 的引用计数: {}", Rc::strong_count(&a));
    
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
}
```

### Rc<T> 的实际应用

```rust
use std::rc::Rc;
use std::collections::HashMap;

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Debug)]
struct Post {
    id: u32,
    title: String,
    author: Rc<User>,
}

#[derive(Debug)]
struct Comment {
    id: u32,
    content: String,
    author: Rc<User>,
    post: Rc<Post>,
}

fn main() {
    // 创建用户
    let user1 = Rc::new(User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    });
    
    let user2 = Rc::new(User {
        id: 2,
        name: "Bob".to_string(),
        email: "bob@example.com".to_string(),
    });
    
    // 创建帖子
    let post1 = Rc::new(Post {
        id: 1,
        title: "Rust 学习笔记".to_string(),
        author: Rc::clone(&user1),
    });
    
    // 创建评论
    let comment1 = Comment {
        id: 1,
        content: "很好的文章！".to_string(),
        author: Rc::clone(&user2),
        post: Rc::clone(&post1),
    };
    
    let comment2 = Comment {
        id: 2,
        content: "谢谢分享！".to_string(),
        author: Rc::clone(&user1),  // 同一个用户
        post: Rc::clone(&post1),
    };
    
    println!("用户1的引用计数: {}", Rc::strong_count(&user1));
    println!("帖子1的引用计数: {}", Rc::strong_count(&post1));
    
    println!("评论1: {:?}", comment1);
    println!("评论2: {:?}", comment2);
}
```

### Rc<T> 的性能考虑

```rust
use std::rc::Rc;
use std::time::Instant;

fn benchmark_rc_vs_clone() {
    let data = vec![1; 1000000];
    
    // 测试直接克隆
    let start = Instant::now();
    for _ in 0..1000 {
        let _clone = data.clone();
    }
    let clone_duration = start.elapsed();
    
    // 测试 Rc 克隆
    let rc_data = Rc::new(data);
    let start = Instant::now();
    for _ in 0..1000 {
        let _rc_clone = Rc::clone(&rc_data);
    }
    let rc_duration = start.elapsed();
    
    println!("直接克隆时间: {:?}", clone_duration);
    println!("Rc 克隆时间: {:?}", rc_duration);
}

fn main() {
    benchmark_rc_vs_clone();
}
```

## 18.2 Weak<T>：避免循环引用的弱引用

### 循环引用问题

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<Node> {
        Rc::new(Node {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }
    
    fn add_child(parent: &Rc<Node>, child: &Rc<Node>) {
        child.parent.borrow_mut().replace(Rc::downgrade(parent));
        parent.children.borrow_mut().push(Rc::clone(child));
    }
}

fn main() {
    let leaf = Node::new(3);
    println!("leaf strong count: {}", Rc::strong_count(&leaf));
    println!("leaf weak count: {}", Rc::weak_count(&leaf));
    
    {
        let branch = Node::new(5);
        Node::add_child(&branch, &leaf);
        
        println!("branch strong count: {}", Rc::strong_count(&branch));
        println!("branch weak count: {}", Rc::weak_count(&branch));
        
        println!("leaf strong count: {}", Rc::strong_count(&leaf));
        println!("leaf weak count: {}", Rc::weak_count(&leaf));
        
        // 访问父节点
        if let Some(parent) = leaf.parent.borrow().upgrade() {
            println!("leaf 的父节点值: {}", parent.value);
        }
    }
    
    // branch 离开作用域后
    println!("leaf strong count: {}", Rc::strong_count(&leaf));
    println!("leaf weak count: {}", Rc::weak_count(&leaf));
    
    // 尝试访问父节点（已被释放）
    match leaf.parent.borrow().upgrade() {
        Some(parent) => println!("leaf 的父节点值: {}", parent.value),
        None => println!("leaf 的父节点已被释放"),
    }
}
```

### Observer 模式实现

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

trait Observer {
    fn update(&self, message: &str);
}

struct Subject {
    observers: RefCell<Vec<Weak<dyn Observer>>>,
}

impl Subject {
    fn new() -> Self {
        Subject {
            observers: RefCell::new(vec![]),
        }
    }
    
    fn subscribe(&self, observer: Weak<dyn Observer>) {
        self.observers.borrow_mut().push(observer);
    }
    
    fn notify(&self, message: &str) {
        let mut observers = self.observers.borrow_mut();
        
        // 清理失效的弱引用
        observers.retain(|observer| {
            if let Some(observer) = observer.upgrade() {
                observer.update(message);
                true
            } else {
                false
            }
        });
    }
}

struct ConcreteObserver {
    name: String,
}

impl ConcreteObserver {
    fn new(name: &str) -> Rc<Self> {
        Rc::new(ConcreteObserver {
            name: name.to_string(),
        })
    }
}

impl Observer for ConcreteObserver {
    fn update(&self, message: &str) {
        println!("{} 收到消息: {}", self.name, message);
    }
}

fn main() {
    let subject = Subject::new();
    
    let observer1 = ConcreteObserver::new("观察者1");
    let observer2 = ConcreteObserver::new("观察者2");
    
    subject.subscribe(Rc::downgrade(&observer1));
    subject.subscribe(Rc::downgrade(&observer2));
    
    subject.notify("第一条消息");
    
    // 释放一个观察者
    drop(observer1);
    
    subject.notify("第二条消息");
}
```

### 缓存系统实现

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::HashMap;

struct Cache<K, V> {
    data: RefCell<HashMap<K, Weak<V>>>,
}

impl<K, V> Cache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
{
    fn new() -> Self {
        Cache {
            data: RefCell::new(HashMap::new()),
        }
    }
    
    fn get(&self, key: &K) -> Option<Rc<V>> {
        let mut data = self.data.borrow_mut();
        
        if let Some(weak_ref) = data.get(key) {
            if let Some(strong_ref) = weak_ref.upgrade() {
                return Some(strong_ref);
            } else {
                // 清理失效的弱引用
                data.remove(key);
            }
        }
        None
    }
    
    fn insert(&self, key: K, value: Rc<V>) {
        let weak_ref = Rc::downgrade(&value);
        self.data.borrow_mut().insert(key, weak_ref);
    }
    
    fn size(&self) -> usize {
        // 清理失效的引用并返回有效大小
        let mut data = self.data.borrow_mut();
        data.retain(|_, weak_ref| weak_ref.strong_count() > 0);
        data.len()
    }
}

fn main() {
    let cache: Cache<String, String> = Cache::new();
    
    {
        let value1 = Rc::new("value1".to_string());
        let value2 = Rc::new("value2".to_string());
        
        cache.insert("key1".to_string(), value1);
        cache.insert("key2".to_string(), value2);
        
        println!("缓存大小: {}", cache.size());
        
        if let Some(cached_value) = cache.get(&"key1".to_string()) {
            println!("从缓存获取: {}", cached_value);
        }
    }
    
    // 值离开作用域后
    println!("清理后缓存大小: {}", cache.size());
}
```

## 18.3 Arc<T>：原子引用计数智能指针

### Arc<T> 基础

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];
    
    for i in 0..5 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("线程 {} 看到的数据: {:?}", i, data_clone);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("主线程的数据: {:?}", data);
}
```

### Arc<T> 与 Mutex<T> 结合

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("结果: {}", *counter.lock().unwrap());
}
```

### 并发数据结构

```rust
use std::sync::{Arc, RwLock};
use std::thread;
use std::collections::HashMap;

type SharedMap = Arc<RwLock<HashMap<String, i32>>>;

fn main() {
    let shared_map: SharedMap = Arc::new(RwLock::new(HashMap::new()));
    let mut handles = vec![];
    
    // 写入线程
    for i in 0..5 {
        let map = Arc::clone(&shared_map);
        let handle = thread::spawn(move || {
            let mut map = map.write().unwrap();
            map.insert(format!("key{}", i), i);
            println!("写入线程 {} 完成", i);
        });
        handles.push(handle);
    }
    
    // 读取线程
    for i in 0..3 {
        let map = Arc::clone(&shared_map);
        let handle = thread::spawn(move || {
            thread::sleep(std::time::Duration::from_millis(100));
            let map = map.read().unwrap();
            println!("读取线程 {} 看到 {} 个元素", i, map.len());
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_map = shared_map.read().unwrap();
    println!("最终 map: {:?}", *final_map);
}
```

### 工作池实现

```rust
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

struct WorkerPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl WorkerPool {
    fn new(size: usize) -> WorkerPool {
        assert!(size > 0);
        
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        WorkerPool { workers, sender }
    }
    
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} 执行任务", id);
            job();
        });
        
        Worker { id, thread }
    }
}

fn main() {
    let pool = WorkerPool::new(4);
    
    for i in 0..8 {
        pool.execute(move || {
            println!("任务 {} 正在执行", i);
            thread::sleep(std::time::Duration::from_millis(1000));
            println!("任务 {} 完成", i);
        });
    }
    
    thread::sleep(std::time::Duration::from_millis(5000));
}
```

## 18.4 RefCell<T>：内部可变性与运行时借用检查

### RefCell<T> 基础

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);
    
    // 不可变借用
    {
        let borrowed = data.borrow();
        println!("借用的值: {}", *borrowed);
        // borrowed 在这里离开作用域
    }
    
    // 可变借用
    {
        let mut borrowed_mut = data.borrow_mut();
        *borrowed_mut += 10;
        println!("修改后的值: {}", *borrowed_mut);
    }
    
    println!("最终值: {}", *data.borrow());
}
```

### RefCell<T> 与 Rc<T> 结合

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<Node> {
        Rc::new(Node {
            value,
            children: RefCell::new(vec![]),
        })
    }
    
    fn add_child(&self, child: Rc<Node>) {
        self.children.borrow_mut().push(child);
    }
    
    fn print_tree(&self, depth: usize) {
        println!("{}{}", "  ".repeat(depth), self.value);
        for child in self.children.borrow().iter() {
            child.print_tree(depth + 1);
        }
    }
}

fn main() {
    let root = Node::new(1);
    let child1 = Node::new(2);
    let child2 = Node::new(3);
    let grandchild = Node::new(4);
    
    child1.add_child(grandchild);
    root.add_child(child1);
    root.add_child(child2);
    
    println!("树结构:");
    root.print_tree(0);
}
```

### Mock 对象实现

```rust
use std::cell::RefCell;
use std::collections::HashMap;

trait Database {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&self, key: &str, value: &str);
}

struct MockDatabase {
    data: RefCell<HashMap<String, String>>,
    call_log: RefCell<Vec<String>>,
}

impl MockDatabase {
    fn new() -> Self {
        MockDatabase {
            data: RefCell::new(HashMap::new()),
            call_log: RefCell::new(Vec::new()),
        }
    }
    
    fn get_call_log(&self) -> Vec<String> {
        self.call_log.borrow().clone()
    }
}

impl Database for MockDatabase {
    fn get(&self, key: &str) -> Option<String> {
        self.call_log.borrow_mut().push(format!("get({})", key));
        self.data.borrow().get(key).cloned()
    }
    
    fn set(&self, key: &str, value: &str) {
        self.call_log.borrow_mut().push(format!("set({}, {})", key, value));
        self.data.borrow_mut().insert(key.to_string(), value.to_string());
    }
}

fn use_database(db: &dyn Database) {
    db.set("user:1", "Alice");
    db.set("user:2", "Bob");
    
    if let Some(user) = db.get("user:1") {
        println!("找到用户: {}", user);
    }
    
    let _ = db.get("user:3"); // 不存在的用户
}

fn main() {
    let mock_db = MockDatabase::new();
    use_database(&mock_db);
    
    println!("调用日志:");
    for call in mock_db.get_call_log() {
        println!("  {}", call);
    }
}
```

### 运行时借用检查

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);
    
    // 多个不可变借用是允许的
    let borrow1 = data.borrow();
    let borrow2 = data.borrow();
    println!("borrow1: {}, borrow2: {}", *borrow1, *borrow2);
    drop(borrow1);
    drop(borrow2);
    
    // 一个可变借用
    let mut borrow_mut = data.borrow_mut();
    *borrow_mut = 10;
    drop(borrow_mut);
    
    // 这会在运行时 panic
    // let borrow1 = data.borrow();
    // let borrow_mut = data.borrow_mut(); // panic!
}
```

## RWO 权限分析

### Rc<T> 的权限语义

```rust
use std::rc::Rc;

fn main() {
    let data = Rc::new(String::from("hello"));
    
    // R: Rc 只允许共享不可变引用
    let clone1 = Rc::clone(&data);
    let clone2 = Rc::clone(&data);
    
    // 可以同时存在多个不可变引用
    println!("data: {}", data);
    println!("clone1: {}", clone1);
    println!("clone2: {}", clone2);
    
    // 无法获取可变引用
    // let mut_ref = Rc::get_mut(&mut data); // 需要唯一引用
    
    // 但是如果只有一个强引用，可以获取可变引用
    drop(clone1);
    drop(clone2);
    
    // 现在 data 是唯一的强引用
    if let Some(mut_ref) = Rc::get_mut(&mut data) {
        mut_ref.push_str(" world");
        println!("修改后: {}", data);
    }
}
```

### RefCell<T> 的动态借用

```rust
use std::cell::RefCell;
use std::rc::Rc;

struct Container {
    data: Rc<RefCell<Vec<i32>>>,
}

impl Container {
    fn new() -> Self {
        Container {
            data: Rc::new(RefCell::new(vec![])),
        }
    }
    
    // R: 返回不可变访问器
    fn read(&self) -> std::cell::Ref<Vec<i32>> {
        self.data.borrow()
    }
    
    // W: 返回可变访问器
    fn write(&self) -> std::cell::RefMut<Vec<i32>> {
        self.data.borrow_mut()
    }
    
    // O: 克隆 Rc，共享所有权
    fn clone_handle(&self) -> Self {
        Container {
            data: Rc::clone(&self.data),
        }
    }
    
    // 尝试获取独占所有权
    fn try_into_inner(self) -> Result<Vec<i32>, Self> {
        match Rc::try_unwrap(self.data) {
            Ok(ref_cell) => Ok(ref_cell.into_inner()),
            Err(rc) => Err(Container { data: rc }),
        }
    }
}

fn main() {
    let container = Container::new();
    let handle1 = container.clone_handle();
    let handle2 = container.clone_handle();
    
    // R: 多个不可变借用
    {
        let read1 = handle1.read();
        let read2 = handle2.read();
        println!("读取1: {:?}", *read1);
        println!("读取2: {:?}", *read2);
    }
    
    // W: 可变借用（一次只能一个）
    {
        let mut write = handle1.write();
        write.push(1);
        write.push(2);
    }
    
    println!("最终数据: {:?}", *container.read());
    
    // O: 尝试获取独占所有权（会失败，因为有多个引用）
    match container.try_into_inner() {
        Ok(_) => println!("获取独占所有权成功"),
        Err(_) => println!("获取独占所有权失败：存在多个引用"),
    }
}
```

### Arc<T> 与线程安全

```rust
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

struct ThreadSafeData {
    // 读写锁保护的数据
    rw_data: Arc<RwLock<Vec<i32>>>,
    // 互斥锁保护的计数器
    counter: Arc<Mutex<usize>>,
}

impl ThreadSafeData {
    fn new() -> Self {
        ThreadSafeData {
            rw_data: Arc::new(RwLock::new(vec![])),
            counter: Arc::new(Mutex::new(0)),
        }
    }
    
    // R: 读取数据
    fn read_data(&self) -> Vec<i32> {
        let data = self.rw_data.read().unwrap();
        data.clone()
    }
    
    // W: 写入数据
    fn write_data(&self, value: i32) {
        let mut data = self.rw_data.write().unwrap();
        data.push(value);
        
        let mut counter = self.counter.lock().unwrap();
        *counter += 1;
    }
    
    // R: 读取计数器
    fn get_count(&self) -> usize {
        let counter = self.counter.lock().unwrap();
        *counter
    }
    
    // 克隆用于多线程
    fn clone_handle(&self) -> Self {
        ThreadSafeData {
            rw_data: Arc::clone(&self.rw_data),
            counter: Arc::clone(&self.counter),
        }
    }
}

fn main() {
    let data = ThreadSafeData::new();
    let mut handles = vec![];
    
    // 写入线程
    for i in 0..5 {
        let data_clone = data.clone_handle();
        let handle = thread::spawn(move || {
            data_clone.write_data(i);
            println!("线程 {} 写入数据", i);
        });
        handles.push(handle);
    }
    
    // 读取线程
    for i in 0..3 {
        let data_clone = data.clone_handle();
        let handle = thread::spawn(move || {
            thread::sleep(std::time::Duration::from_millis(50));
            let values = data_clone.read_data();
            println!("线程 {} 读取到 {} 个值", i, values.len());
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("最终数据: {:?}", data.read_data());
    println!("写入次数: {}", data.get_count());
}
```

### 弱引用的权限管理

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Parent {
    children: RefCell<Vec<Rc<Child>>>,
}

struct Child {
    value: i32,
    parent: RefCell<Weak<Parent>>,
}

impl Parent {
    fn new() -> Rc<Self> {
        Rc::new(Parent {
            children: RefCell::new(vec![]),
        })
    }
    
    // O: 添加子节点，获取所有权
    fn add_child(self: &Rc<Self>, value: i32) -> Rc<Child> {
        let child = Rc::new(Child {
            value,
            parent: RefCell::new(Rc::downgrade(self)),
        });
        
        self.children.borrow_mut().push(Rc::clone(&child));
        child
    }
    
    // R: 读取子节点
    fn get_children(&self) -> Vec<Rc<Child>> {
        self.children.borrow().clone()
    }
}

impl Child {
    // R: 尝试获取父节点的强引用
    fn get_parent(&self) -> Option<Rc<Parent>> {
        self.parent.borrow().upgrade()
    }
    
    // R: 检查父节点是否还存在
    fn has_parent(&self) -> bool {
        self.parent.borrow().strong_count() > 0
    }
}

fn main() {
    let parent = Parent::new();
    
    // 创建子节点
    let child1 = parent.add_child(1);
    let child2 = parent.add_child(2);
    
    println!("父节点引用计数: {}", Rc::strong_count(&parent));
    println!("子节点1引用计数: {}", Rc::strong_count(&child1));
    
    // 子节点可以访问父节点
    if let Some(p) = child1.get_parent() {
        println!("子节点1找到父节点，有 {} 个子节点", p.get_children().len());
    }
    
    // 释放父节点
    drop(parent);
    
    // 子节点仍然存在，但父节点已被释放
    println!("父节点被释放后:");
    println!("子节点1是否有父节点: {}", child1.has_parent());
    
    match child1.get_parent() {
        Some(_) => println!("意外：父节点仍然存在"),
        None => println!("正确：父节点已被释放"),
    }
}
```

## 小结

本章深入学习了 Rust 中常见的智能指针：

1. **Rc<T>**：
   - 引用计数智能指针，允许多个所有者
   - 只允许不可变访问
   - 适用于需要共享数据的单线程场景

2. **Weak<T>**：
   - 弱引用，不影响引用计数
   - 避免循环引用问题
   - 需要升级为强引用才能使用

3. **Arc<T>**：
   - 线程安全的引用计数智能指针
   - 可以在多线程间安全共享
   - 通常与 Mutex 或 RwLock 结合使用

4. **RefCell<T>**：
   - 提供内部可变性
   - 运行时借用检查
   - 允许在不可变结构中修改数据

5. **RWO 权限分析**：
   - **R**：Rc/Arc 提供共享只读访问
   - **W**：RefCell 通过运行时检查提供可变访问
   - **O**：智能指针管理共享所有权
   - Weak 引用不拥有数据，只能尝试获取访问权

这些智能指针组合使用可以构建复杂的数据结构，同时保证内存安全。下一章我们将学习多线程与并发。 