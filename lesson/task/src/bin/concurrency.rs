// 教学示例 - 允许未使用的代码
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// 第19章：并发编程
// 演示 Rust 中的线程、消息传递、共享状态等并发概念

use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex, RwLock, Condvar};
use std::sync::mpsc;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    println!("🧵 第19章：并发编程");
    println!("=====================================");
    
    // 1. 线程基础
    thread_basics();
    
    // 2. 消息传递
    message_passing();
    
    // 3. 共享状态
    shared_state();
    
    // 4. 原子操作
    atomic_operations();
    
    // 5. 条件变量
    condition_variables();
    
    // 6. 实际应用示例
    practical_examples();
}

// ============================================================================
// 1. 线程基础
// ============================================================================

fn thread_basics() {
    println!("\n🧵 1. 线程基础");
    println!("{}", "-".repeat(40));
    
    // 创建线程
    println!("  🔸 创建线程：");
    
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("    子线程: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    for i in 1..=3 {
        println!("    主线程: {}", i);
        thread::sleep(Duration::from_millis(150));
    }
    
    handle.join().unwrap();
    println!("    所有线程完成");
    
    // 带返回值的线程
    println!("\n  🔸 带返回值的线程：");
    
    let handle = thread::spawn(|| {
        let mut sum = 0;
        for i in 1..=10 {
            sum += i;
        }
        sum
    });
    
    let result = handle.join().unwrap();
    println!("    计算结果: {}", result);
    
    // move 闭包
    println!("\n  🔸 move 闭包：");
    
    let data = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(move || {
        println!("    线程中的数据: {:?}", data);
        data.iter().sum::<i32>()
    });
    
    let sum = handle.join().unwrap();
    println!("    数据总和: {}", sum);
    
    // 多个线程
    println!("\n  🔸 多个线程：");
    
    let mut handles = vec![];
    
    for i in 0..3 {
        let handle = thread::spawn(move || {
            println!("    线程 {} 开始工作", i);
            thread::sleep(Duration::from_millis(100 * (i + 1) as u64));
            println!("    线程 {} 完成工作", i);
            i * 10
        });
        handles.push(handle);
    }
    
    for (i, handle) in handles.into_iter().enumerate() {
        let result = handle.join().unwrap();
        println!("    线程 {} 返回: {}", i, result);
    }
    
    // 线程构建器
    println!("\n  🔸 线程构建器：");
    
    let builder = thread::Builder::new()
        .name("worker-thread".into())
        .stack_size(32 * 1024);
    
    let handle = builder.spawn(|| {
        println!("    自定义线程: {}", thread::current().name().unwrap_or("unnamed"));
        42
    }).unwrap();
    
    let result = handle.join().unwrap();
    println!("    自定义线程返回: {}", result);
}

// ============================================================================
// 2. 消息传递
// ============================================================================

fn message_passing() {
    println!("\n📨 2. 消息传递");
    println!("{}", "-".repeat(40));
    
    // 基本消息传递
    println!("  🔸 基本消息传递：");
    
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let messages = vec!["hello", "from", "thread"];
        
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    for received in rx {
        println!("    收到消息: {}", received);
    }
    
    // 多个发送者
    println!("\n  🔸 多个发送者：");
    
    let (tx, rx) = mpsc::channel();
    
    for i in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let message = format!("来自线程 {} 的消息", i);
            tx_clone.send(message).unwrap();
        });
    }
    
    drop(tx); // 关闭原始发送者
    
    for received in rx {
        println!("    收到: {}", received);
    }
    
    // 同步通道
    println!("\n  🔸 同步通道：");
    
    let (tx, rx) = mpsc::sync_channel(2); // 缓冲区大小为2
    
    let sender_handle = thread::spawn(move || {
        for i in 1..=5 {
            println!("    发送: {}", i);
            tx.send(i).unwrap();
            println!("    发送完成: {}", i);
        }
    });
    
    thread::sleep(Duration::from_millis(500));
    
    for received in rx {
        println!("    接收: {}", received);
        thread::sleep(Duration::from_millis(200));
    }
    
    sender_handle.join().unwrap();
    
    // 选择性接收
    println!("\n  🔸 选择性接收：");
    
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        tx1.send("快速消息").unwrap();
    });
    
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(200));
        tx2.send("慢速消息").unwrap();
    });
    
    // 简化的选择性接收
    let mut received = false;
    while !received {
        if let Ok(msg) = rx1.try_recv() {
            println!("    从通道1收到: {}", msg);
            received = true;
        } else if let Ok(msg) = rx2.try_recv() {
            println!("    从通道2收到: {}", msg);
            received = true;
        } else {
            thread::sleep(Duration::from_millis(10));
        }
    }
    
    // 工作者模式
    println!("\n  🔸 工作者模式：");
    
    let (job_sender, job_receiver) = mpsc::channel();
    let job_receiver = Arc::new(Mutex::new(job_receiver));
    
    let mut workers = vec![];
    
    for id in 0..3 {
        let receiver = Arc::clone(&job_receiver);
        let worker = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv();
                match job {
                    Ok(job) => {
                        println!("    工作者 {} 处理任务: {}", id, job);
                        thread::sleep(Duration::from_millis(100));
                    }
                    Err(_) => {
                        println!("    工作者 {} 退出", id);
                        break;
                    }
                }
            }
        });
        workers.push(worker);
    }
    
    for i in 1..=9 {
        job_sender.send(format!("任务{}", i)).unwrap();
    }
    
    drop(job_sender);
    
    for worker in workers {
        worker.join().unwrap();
    }
}

// ============================================================================
// 3. 共享状态
// ============================================================================

fn shared_state() {
    println!("\n🔒 3. 共享状态");
    println!("{}", "-".repeat(40));
    
    // Mutex 互斥锁
    println!("  🔸 Mutex 互斥锁：");
    
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
    
    println!("    最终计数: {}", *counter.lock().unwrap());
    
    // RwLock 读写锁
    println!("\n  🔸 RwLock 读写锁：");
    
    let data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];
    
    // 多个读者
    for i in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let data = data.read().unwrap();
            println!("    读者 {} 读取数据: {:?}", i, *data);
            thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }
    
    // 一个写者
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        let mut data = data_clone.write().unwrap();
        data.push(6);
        println!("    写者添加元素: 6");
    });
    handles.push(handle);
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("    最终数据: {:?}", *data.read().unwrap());
    
    // 死锁避免
    println!("\n  🔸 死锁避免：");
    
    let resource1 = Arc::new(Mutex::new(1));
    let resource2 = Arc::new(Mutex::new(2));
    
    let res1_clone = Arc::clone(&resource1);
    let res2_clone = Arc::clone(&resource2);
    
    let handle1 = thread::spawn(move || {
        let _guard1 = res1_clone.lock().unwrap();
        println!("    线程1 获取资源1");
        thread::sleep(Duration::from_millis(100));
        
        let _guard2 = res2_clone.lock().unwrap();
        println!("    线程1 获取资源2");
    });
    
    let res1_clone = Arc::clone(&resource1);
    let res2_clone = Arc::clone(&resource2);
    
    let handle2 = thread::spawn(move || {
        let _guard1 = res1_clone.lock().unwrap(); // 同样的顺序避免死锁
        println!("    线程2 获取资源1");
        thread::sleep(Duration::from_millis(100));
        
        let _guard2 = res2_clone.lock().unwrap();
        println!("    线程2 获取资源2");
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
    
    println!("    避免了死锁");
}

// ============================================================================
// 4. 原子操作
// ============================================================================

fn atomic_operations() {
    println!("\n⚛️ 4. 原子操作");
    println!("{}", "-".repeat(40));
    
    // 基本原子操作
    println!("  🔸 基本原子操作：");
    
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("    原子计数器结果: {}", counter.load(Ordering::SeqCst));
    
    // 不同的内存顺序
    println!("\n  🔸 不同的内存顺序：");
    
    let flag = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let data = Arc::new(AtomicUsize::new(0));
    
    let flag_clone = Arc::clone(&flag);
    let data_clone = Arc::clone(&data);
    
    let producer = thread::spawn(move || {
        data_clone.store(42, Ordering::Relaxed);
        flag_clone.store(true, Ordering::Release); // Release 语义
        println!("    生产者设置数据和标志");
    });
    
    let flag_clone = Arc::clone(&flag);
    let data_clone = Arc::clone(&data);
    
    let consumer = thread::spawn(move || {
        while !flag_clone.load(Ordering::Acquire) { // Acquire 语义
            thread::sleep(Duration::from_millis(1));
        }
        let value = data_clone.load(Ordering::Relaxed);
        println!("    消费者读取数据: {}", value);
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
    
    // Compare and Swap
    println!("\n  🔸 Compare and Swap：");
    
    let value = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    for i in 0..5 {
        let value = Arc::clone(&value);
        let handle = thread::spawn(move || {
            loop {
                let current = value.load(Ordering::SeqCst);
                let new_value = current + i + 1;
                
                match value.compare_exchange_weak(
                    current,
                    new_value,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                ) {
                    Ok(_) => {
                        println!("    线程 {} 成功更新: {} -> {}", i, current, new_value);
                        break;
                    }
                    Err(_) => {
                        // 重试
                        thread::yield_now();
                    }
                }
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("    最终值: {}", value.load(Ordering::SeqCst));
}

// ============================================================================
// 5. 条件变量
// ============================================================================

fn condition_variables() {
    println!("\n🚦 5. 条件变量");
    println!("{}", "-".repeat(40));
    
    // 基本条件变量
    println!("  🔸 基本条件变量：");
    
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);
    
    thread::spawn(move || {
        let (lock, cvar) = &*pair_clone;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
        println!("    工作线程通知主线程");
    });
    
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }
    println!("    主线程收到通知");
    
    // 生产者-消费者模式
    println!("\n  🔸 生产者-消费者模式：");
    
    let buffer = Arc::new((Mutex::new(Vec::new()), Condvar::new()));
    let buffer_clone = Arc::clone(&buffer);
    
    // 生产者
    let producer = thread::spawn(move || {
        let (lock, cvar) = &*buffer_clone;
        for i in 1..=5 {
            let mut buffer = lock.lock().unwrap();
            buffer.push(i);
            println!("    生产者生产: {}", i);
            cvar.notify_one();
            drop(buffer);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // 消费者
    let buffer_clone = Arc::clone(&buffer);
    let consumer = thread::spawn(move || {
        let (lock, cvar) = &*buffer_clone;
        for _ in 1..=5 {
            let mut buffer = lock.lock().unwrap();
            while buffer.is_empty() {
                buffer = cvar.wait(buffer).unwrap();
            }
            let item = buffer.remove(0);
            println!("    消费者消费: {}", item);
            drop(buffer);
            thread::sleep(Duration::from_millis(150));
        }
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
    
    // 多个等待者
    println!("\n  🔸 多个等待者：");
    
    let pair = Arc::new((Mutex::new(0), Condvar::new()));
    let mut handles = vec![];
    
    for i in 0..3 {
        let pair = Arc::clone(&pair);
        let handle = thread::spawn(move || {
            let (lock, cvar) = &*pair;
            let mut count = lock.lock().unwrap();
            while *count < 10 {
                count = cvar.wait(count).unwrap();
            }
            println!("    等待者 {} 被唤醒，计数: {}", i, *count);
        });
        handles.push(handle);
    }
    
    thread::sleep(Duration::from_millis(100));
    
    let (lock, cvar) = &*pair;
    let mut count = lock.lock().unwrap();
    *count = 10;
    cvar.notify_all(); // 唤醒所有等待者
    drop(count);
    
    for handle in handles {
        handle.join().unwrap();
    }
}

// ============================================================================
// 6. 实际应用示例
// ============================================================================

fn practical_examples() {
    println!("\n🚀 6. 实际应用示例");
    println!("{}", "-".repeat(40));
    
    // 并行计算
    parallel_computation();
    
    // 线程池
    thread_pool_example();
    
    // 生产者消费者队列
    producer_consumer_queue();
}

fn parallel_computation() {
    println!("\n  🔸 并行计算示例：");
    
    let data: Vec<i32> = (1..=1000).collect();
    let chunk_size = data.len() / 4;
    let mut handles = vec![];
    
    for chunk in data.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let handle = thread::spawn(move || {
            chunk.iter().map(|&x| x * x).sum::<i32>()
        });
        handles.push(handle);
    }
    
    let mut total = 0;
    for handle in handles {
        total += handle.join().unwrap();
    }
    
    println!("    并行计算平方和: {}", total);
    
    // 验证结果
    let sequential_sum: i32 = data.iter().map(|&x| x * x).sum();
    println!("    顺序计算平方和: {}", sequential_sum);
    println!("    结果一致: {}", total == sequential_sum);
}

fn thread_pool_example() {
    println!("\n  🔸 线程池示例：");
    
    struct ThreadPool {
        workers: Vec<Worker>,
        sender: mpsc::Sender<Job>,
    }
    
    type Job = Box<dyn FnOnce() + Send + 'static>;
    
    struct Worker {
        id: usize,
        thread: Option<thread::JoinHandle<()>>,
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
    
    impl Drop for ThreadPool {
        fn drop(&mut self) {
            drop(self.sender.clone());
            
            for worker in &mut self.workers {
                if let Some(thread) = worker.thread.take() {
                    thread.join().unwrap();
                }
            }
        }
    }
    
    impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
            let thread = thread::spawn(move || loop {
                match receiver.lock().unwrap().recv() {
                    Ok(job) => {
                        println!("    Worker {} 开始执行任务", id);
                        job();
                    }
                    Err(_) => {
                        println!("    Worker {} 退出", id);
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
    
    let pool = ThreadPool::new(3);
    
    for i in 0..8 {
        pool.execute(move || {
            println!("    执行任务 {}", i);
            thread::sleep(Duration::from_millis(100));
            println!("    任务 {} 完成", i);
        });
    }
    
    thread::sleep(Duration::from_millis(1000));
    println!("    线程池示例完成");
}

fn producer_consumer_queue() {
    println!("\n  🔸 生产者消费者队列：");
    
    use std::collections::VecDeque;
    
    struct BoundedQueue<T> {
        queue: Mutex<VecDeque<T>>,
        not_empty: Condvar,
        not_full: Condvar,
        capacity: usize,
    }
    
    impl<T> BoundedQueue<T> {
        fn new(capacity: usize) -> Self {
            BoundedQueue {
                queue: Mutex::new(VecDeque::new()),
                not_empty: Condvar::new(),
                not_full: Condvar::new(),
                capacity,
            }
        }
        
        fn push(&self, item: T) {
            let mut queue = self.queue.lock().unwrap();
            while queue.len() == self.capacity {
                queue = self.not_full.wait(queue).unwrap();
            }
            queue.push_back(item);
            self.not_empty.notify_one();
        }
        
        fn pop(&self) -> T {
            let mut queue = self.queue.lock().unwrap();
            while queue.is_empty() {
                queue = self.not_empty.wait(queue).unwrap();
            }
            let item = queue.pop_front().unwrap();
            self.not_full.notify_one();
            item
        }
    }
    
    let queue = Arc::new(BoundedQueue::new(3));
    let mut handles = vec![];
    
    // 生产者
    for i in 0..2 {
        let queue = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            for j in 1..=5 {
                let item = format!("Producer{}-Item{}", i, j);
                println!("    生产: {}", item);
                queue.push(item);
                thread::sleep(Duration::from_millis(100));
            }
        });
        handles.push(handle);
    }
    
    // 消费者
    for i in 0..2 {
        let queue = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            for _ in 1..=5 {
                let item = queue.pop();
                println!("    消费者{} 消费: {}", i, item);
                thread::sleep(Duration::from_millis(150));
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("    生产者消费者队列示例完成");
} 