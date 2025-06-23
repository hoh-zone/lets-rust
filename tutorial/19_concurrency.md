# 第19章：多线程与并发

Rust 的并发模型以安全性为核心，通过类型系统在编译时防止数据竞争，提供了强大而安全的并发编程能力。

## 19.1 并发与并行

### 基本概念

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // 并发：多个任务交替执行
    let handle1 = thread::spawn(|| {
        for i in 1..10 {
            println!("任务1: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    let handle2 = thread::spawn(|| {
        for i in 1..10 {
            println!("任务2: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // 主线程也可以做其他工作
    for i in 1..5 {
        println!("主线程: {}", i);
        thread::sleep(Duration::from_millis(200));
    }
    
    // 等待子线程完成
    handle1.join().unwrap();
    handle2.join().unwrap();
}
```

### 线程基础

```rust
use std::thread;

fn main() {
    // 创建线程
    let handle = thread::spawn(|| {
        println!("Hello from spawned thread!");
    });
    
    // 等待线程完成
    handle.join().unwrap();
    
    // 带参数的线程
    let data = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(move || {
        println!("数据: {:?}", data);
        data.len()
    });
    
    let result = handle.join().unwrap();
    println!("线程返回值: {}", result);
}
```

### 线程间数据传递

```rust
use std::thread;

fn main() {
    let data = String::from("Hello from main thread");
    
    // 使用 move 将数据移动到线程中
    let handle = thread::spawn(move || {
        println!("线程收到: {}", data);
        format!("{}的回复", data)
    });
    
    let response = handle.join().unwrap();
    println!("主线程收到回复: {}", response);
    
    // data 在这里不再可用，因为已被移动
    // println!("{}", data); // 编译错误
}
```

## 19.2 多线程及同步机制

### 消息传递

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // 单发送者，单接收者
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    for received in rx {
        println!("收到: {}", received);
    }
}
```

### 多发送者通道

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    
    thread::spawn(move || {
        let vals = vec![
            String::from("发送者1: hi"),
            String::from("发送者1: from"),
        ];
        
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(std::time::Duration::from_millis(100));
        }
    });
    
    thread::spawn(move || {
        let vals = vec![
            String::from("发送者2: more"),
            String::from("发送者2: messages"),
        ];
        
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(std::time::Duration::from_millis(100));
        }
    });
    
    for received in rx {
        println!("收到: {}", received);
    }
}
```

### 工作者池模式

```rust
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

enum Message {
    NewJob(Job),
    Terminate,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
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
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("发送终止信号给所有工作者");
        
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        
        println!("关闭所有工作者");
        
        for worker in &mut self.workers {
            println!("关闭工作者 {}", worker.id);
            
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            
            match message {
                Message::NewJob(job) => {
                    println!("工作者 {} 收到任务", id);
                    job();
                }
                Message::Terminate => {
                    println!("工作者 {} 接到终止信号", id);
                    break;
                }
            }
        });
        
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

fn main() {
    let pool = ThreadPool::new(4);
    
    for i in 0..8 {
        pool.execute(move || {
            println!("执行任务 {}", i);
            thread::sleep(std::time::Duration::from_millis(1000));
            println!("任务 {} 完成", i);
        });
    }
    
    thread::sleep(std::time::Duration::from_millis(5000));
}
```

### 互斥锁 (Mutex)

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

### 读写锁 (RwLock)

```rust
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = vec![];
    
    // 多个读取线程
    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let data = data.read().unwrap();
            println!("读取线程 {}: {:?}", i, *data);
            thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }
    
    // 一个写入线程
    {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            let mut data = data.write().unwrap();
            data.push(4);
            println!("写入线程: 添加了元素4");
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("最终数据: {:?}", *data.read().unwrap());
}
```

### 条件变量 (Condvar)

```rust
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    
    // 消费者线程
    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut ready = lock.lock().unwrap();
        
        while !*ready {
            println!("等待条件满足...");
            ready = cvar.wait(ready).unwrap();
        }
        
        println!("条件满足，继续执行");
    });
    
    // 生产者线程
    thread::sleep(Duration::from_millis(1000));
    
    let (lock, cvar) = &*pair;
    let mut ready = lock.lock().unwrap();
    *ready = true;
    cvar.notify_one();
    
    thread::sleep(Duration::from_millis(100));
}
```

### 原子操作

```rust
use std::sync::atomic::{AtomicI32, AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(AtomicI32::new(0));
    let flag = Arc::new(AtomicBool::new(false));
    let mut handles = vec![];
    
    // 增加计数器的线程
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let flag = Arc::clone(&flag);
        
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
            
            // 设置完成标志
            flag.store(true, Ordering::SeqCst);
        });
        handles.push(handle);
    }
    
    // 监控线程
    let counter_monitor = Arc::clone(&counter);
    let flag_monitor = Arc::clone(&flag);
    let monitor = thread::spawn(move || {
        while !flag_monitor.load(Ordering::SeqCst) {
            println!("当前计数: {}", counter_monitor.load(Ordering::SeqCst));
            thread::sleep(std::time::Duration::from_millis(100));
        }
    });
    
    for handle in handles {
        handle.join().unwrap();
    }
    monitor.join().unwrap();
    
    println!("最终计数: {}", counter.load(Ordering::SeqCst));
}
```

## 19.3 Send 与 Sync 解析

### Send trait

```rust
use std::thread;
use std::rc::Rc;

fn main() {
    // i32 实现了 Send，可以在线程间传递
    let number = 42;
    thread::spawn(move || {
        println!("数字: {}", number);
    });
    
    // String 实现了 Send
    let text = String::from("Hello");
    thread::spawn(move || {
        println!("文本: {}", text);
    });
    
    // Rc<T> 没有实现 Send，不能在线程间传递
    let rc_data = Rc::new(vec![1, 2, 3]);
    // thread::spawn(move || {
    //     println!("Rc 数据: {:?}", rc_data); // 编译错误
    // });
    
    // 但是 Arc<T> 实现了 Send
    use std::sync::Arc;
    let arc_data = Arc::new(vec![1, 2, 3]);
    thread::spawn(move || {
        println!("Arc 数据: {:?}", arc_data);
    });
    
    thread::sleep(std::time::Duration::from_millis(100));
}
```

### Sync trait

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Mutex<T> 实现了 Sync，可以在多个线程间共享引用
    let data = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut num = data.lock().unwrap();
            *num += 1;
            println!("线程修改数据为: {}", *num);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

### 自定义 Send 和 Sync

```rust
use std::marker::{Send, Sync};
use std::cell::UnsafeCell;

// 线程安全的计数器
struct SafeCounter {
    value: UnsafeCell<i32>,
}

impl SafeCounter {
    fn new(value: i32) -> Self {
        SafeCounter {
            value: UnsafeCell::new(value),
        }
    }
    
    fn get(&self) -> i32 {
        unsafe { *self.value.get() }
    }
    
    fn increment(&self) {
        unsafe {
            let ptr = self.value.get();
            *ptr += 1;
        }
    }
}

// 手动实现 Send 和 Sync
unsafe impl Send for SafeCounter {}
unsafe impl Sync for SafeCounter {}

fn main() {
    use std::sync::Arc;
    use std::thread;
    
    let counter = Arc::new(SafeCounter::new(0));
    let mut handles = vec![];
    
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.increment();
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("最终值: {}", counter.get());
}
```

### Send 和 Sync 的组合

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::HashMap;

// 线程安全的缓存
struct ThreadSafeCache<K, V> {
    data: Arc<Mutex<HashMap<K, V>>>,
}

impl<K, V> ThreadSafeCache<K, V>
where
    K: Eq + std::hash::Hash + Clone + Send + 'static,
    V: Clone + Send + 'static,
{
    fn new() -> Self {
        ThreadSafeCache {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    fn get(&self, key: &K) -> Option<V> {
        let data = self.data.lock().unwrap();
        data.get(key).cloned()
    }
    
    fn insert(&self, key: K, value: V) {
        let mut data = self.data.lock().unwrap();
        data.insert(key, value);
    }
    
    fn clone_handle(&self) -> Self {
        ThreadSafeCache {
            data: Arc::clone(&self.data),
        }
    }
}

fn main() {
    let cache = ThreadSafeCache::new();
    let mut handles = vec![];
    
    // 写入线程
    for i in 0..5 {
        let cache = cache.clone_handle();
        let handle = thread::spawn(move || {
            cache.insert(i, format!("value_{}", i));
            println!("插入 key: {}", i);
        });
        handles.push(handle);
    }
    
    // 读取线程
    for i in 0..3 {
        let cache = cache.clone_handle();
        let handle = thread::spawn(move || {
            thread::sleep(std::time::Duration::from_millis(100));
            if let Some(value) = cache.get(&i) {
                println!("读取 key: {}, value: {}", i, value);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

## RWO 权限分析

### 线程间所有权转移

```rust
use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    
    // O: 所有权转移到新线程
    let handle = thread::spawn(move || {
        println!("线程拥有数据: {:?}", data);
        data.len()  // 返回值也转移所有权
    });
    
    // data 在这里不再可用
    // println!("{:?}", data);  // 编译错误
    
    // O: 从线程获取返回值的所有权
    let result = handle.join().unwrap();
    println!("线程返回: {}", result);
}
```

### 共享状态的权限管理

```rust
use std::sync::{Arc, Mutex};
use std::thread;

struct SharedResource {
    data: Arc<Mutex<Vec<i32>>>,
}

impl SharedResource {
    fn new() -> Self {
        SharedResource {
            data: Arc::new(Mutex::new(vec![])),
        }
    }
    
    // R: 读取数据（通过锁获取临时访问权）
    fn read(&self) -> Vec<i32> {
        let data = self.data.lock().unwrap();
        data.clone()
    }
    
    // W: 写入数据（通过锁获取独占访问权）
    fn write(&self, value: i32) {
        let mut data = self.data.lock().unwrap();
        data.push(value);
    }
    
    // O: 克隆共享句柄
    fn clone_handle(&self) -> Self {
        SharedResource {
            data: Arc::clone(&self.data),
        }
    }
    
    // 尝试获取独占所有权
    fn try_into_inner(self) -> Result<Vec<i32>, Self> {
        match Arc::try_unwrap(self.data) {
            Ok(mutex) => Ok(mutex.into_inner().unwrap()),
            Err(arc) => Err(SharedResource { data: arc }),
        }
    }
}

fn main() {
    let resource = SharedResource::new();
    let mut handles = vec![];
    
    // 写入线程
    for i in 0..5 {
        let resource = resource.clone_handle();
        let handle = thread::spawn(move || {
            resource.write(i);
            println!("线程 {} 写入数据", i);
        });
        handles.push(handle);
    }
    
    // 读取线程
    for i in 0..3 {
        let resource = resource.clone_handle();
        let handle = thread::spawn(move || {
            thread::sleep(std::time::Duration::from_millis(100));
            let data = resource.read();
            println!("线程 {} 读取到 {} 个元素", i, data.len());
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    // 尝试获取独占所有权
    match resource.try_into_inner() {
        Ok(data) => println!("获取独占所有权: {:?}", data),
        Err(_) => println!("无法获取独占所有权"),
    }
}
```

### 消息传递的权限转移

```rust
use std::sync::mpsc;
use std::thread;

enum Message {
    Data(Vec<i32>),
    Text(String),
    Request { id: u32, callback: mpsc::Sender<String> },
}

fn main() {
    let (tx, rx) = mpsc::channel();
    
    // 发送者线程
    let sender_handle = thread::spawn(move || {
        // O: 发送数据，转移所有权
        let data = vec![1, 2, 3, 4, 5];
        tx.send(Message::Data(data)).unwrap();
        
        let text = String::from("Hello from sender");
        tx.send(Message::Text(text)).unwrap();
        
        // 发送回调请求
        let (callback_tx, callback_rx) = mpsc::channel();
        tx.send(Message::Request { 
            id: 1, 
            callback: callback_tx 
        }).unwrap();
        
        // 等待回调响应
        let response = callback_rx.recv().unwrap();
        println!("收到回调响应: {}", response);
    });
    
    // 接收者线程
    let receiver_handle = thread::spawn(move || {
        for message in rx {
            match message {
                // O: 接收数据的所有权
                Message::Data(data) => {
                    println!("接收到数据: {:?}", data);
                    // data 现在归这个线程所有
                }
                Message::Text(text) => {
                    println!("接收到文本: {}", text);
                    // text 现在归这个线程所有
                }
                Message::Request { id, callback } => {
                    println!("处理请求: {}", id);
                    // O: 通过回调发送响应
                    callback.send(format!("请求 {} 已处理", id)).unwrap();
                }
            }
        }
    });
    
    sender_handle.join().unwrap();
    receiver_handle.join().unwrap();
}
```

### 生命周期与并发

```rust
use std::sync::Arc;
use std::thread;

struct Data<'a> {
    content: &'a str,
}

// 需要 'static 生命周期的数据才能发送到线程
fn spawn_with_static_data() {
    let static_str: &'static str = "static string";
    
    thread::spawn(move || {
        println!("静态字符串: {}", static_str);
    });
}

// 使用 Arc 共享数据
fn spawn_with_shared_data() {
    let data = Arc::new(String::from("shared data"));
    let mut handles = vec![];
    
    for i in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("线程 {} 访问: {}", i, data);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

// 作用域线程
fn scoped_threads() {
    let data = String::from("scoped data");
    
    thread::scope(|s| {
        s.spawn(|| {
            println!("作用域线程访问: {}", data);
        });
        
        s.spawn(|| {
            println!("另一个作用域线程访问: {}", data);
        });
    });
    
    // data 在这里仍然有效
    println!("主线程仍可访问: {}", data);
}

fn main() {
    spawn_with_static_data();
    spawn_with_shared_data();
    scoped_threads();
}
```

### 异步并发权限

```rust
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

struct AsyncResource {
    data: Arc<Mutex<Vec<i32>>>,
    cache: Arc<RwLock<String>>,
}

impl AsyncResource {
    fn new() -> Self {
        AsyncResource {
            data: Arc::new(Mutex::new(vec![])),
            cache: Arc::new(RwLock::new(String::new())),
        }
    }
    
    // R: 异步读取
    async fn read(&self) -> Vec<i32> {
        let data = self.data.lock().await;
        data.clone()
    }
    
    // W: 异步写入
    async fn write(&self, value: i32) {
        let mut data = self.data.lock().await;
        data.push(value);
    }
    
    // R: 异步读取缓存
    async fn read_cache(&self) -> String {
        let cache = self.cache.read().await;
        cache.clone()
    }
    
    // W: 异步更新缓存
    async fn update_cache(&self, content: String) {
        let mut cache = self.cache.write().await;
        *cache = content;
    }
    
    fn clone_handle(&self) -> Self {
        AsyncResource {
            data: Arc::clone(&self.data),
            cache: Arc::clone(&self.cache),
        }
    }
}

#[tokio::main]
async fn main() {
    let resource = AsyncResource::new();
    let mut handles = vec![];
    
    // 异步写入任务
    for i in 0..5 {
        let resource = resource.clone_handle();
        let handle = tokio::spawn(async move {
            resource.write(i).await;
            resource.update_cache(format!("Updated by task {}", i)).await;
            println!("任务 {} 完成写入", i);
        });
        handles.push(handle);
    }
    
    // 异步读取任务
    for i in 0..3 {
        let resource = resource.clone_handle();
        let handle = tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
            let data = resource.read().await;
            let cache = resource.read_cache().await;
            println!("任务 {} 读取到 {} 个元素, 缓存: {}", i, data.len(), cache);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
}
```

## 小结

本章深入学习了 Rust 的并发编程：

1. **并发基础**：
   - 线程创建和管理
   - 数据在线程间的移动
   - join 等待线程完成

2. **同步机制**：
   - 消息传递（mpsc）
   - 互斥锁（Mutex）
   - 读写锁（RwLock）
   - 条件变量（Condvar）
   - 原子操作

3. **Send 和 Sync trait**：
   - Send：类型可以在线程间转移所有权
   - Sync：类型可以在多线程间安全共享引用
   - 自动推导和手动实现

4. **RWO 权限分析**：
   - **R**：通过锁机制提供临时读权限
   - **W**：通过互斥锁提供独占写权限
   - **O**：线程间所有权转移，Arc 共享所有权
   - 消息传递实现所有权转移
   - 作用域线程允许借用本地数据

Rust 的并发模型通过类型系统确保线程安全，避免了数据竞争等常见并发问题。下一章我们将学习 Unsafe Rust。 