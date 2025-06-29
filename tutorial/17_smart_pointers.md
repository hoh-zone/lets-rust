# 第17章：智能指针

智能指针是具有额外元数据和功能的数据结构，它们不仅保存数据的地址，还包含其他信息如引用计数等。

## 17.1 智能指针概述：什么是智能指针

### 智能指针与普通引用的区别

```rust
fn main() {
    // 普通引用
    let x = 5;
    let y = &x;  // y 是对 x 的引用
    
    // 智能指针
    let b = Box::new(5);  // b 是一个智能指针，拥有堆上数据的所有权
    
    println!("普通引用: {}", y);
    println!("智能指针: {}", b);
}
```

### 智能指针的特征

智能指针通常实现两个重要的 trait：

1. **Deref trait** - 允许智能指针表现得像普通引用
2. **Drop trait** - 允许自定义清理代码

```rust
use std::ops::Deref;

// 自定义智能指针
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 实现 Deref trait
impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// 实现 Drop trait
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("MyBox 正在被清理");
    }
}

fn main() {
    let x = MyBox::new(5);
    
    // 可以像普通引用一样使用
    assert_eq!(5, *x);
    
    // 当 x 离开作用域时，会自动调用 drop
}
```

### 常见的智能指针类型

```rust
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;

fn main() {
    // Box<T> - 堆分配
    let b = Box::new(5);
    
    // Rc<T> - 引用计数（单线程）
    let rc = Rc::new(5);
    let rc_clone = Rc::clone(&rc);
    
    // RefCell<T> - 内部可变性
    let ref_cell = RefCell::new(5);
    
    // Arc<T> - 原子引用计数（多线程）
    let arc = Arc::new(5);
    
    println!("Box: {}", b);
    println!("Rc: {}, 引用计数: {}", rc, Rc::strong_count(&rc));
    println!("RefCell: {}", ref_cell.borrow());
    println!("Arc: {}", arc);
}
```

## 17.2 Deref 特征 与 Drop 特征 的解析

### Deref 特征详解

```rust
use std::ops::Deref;

// 简单包装器
struct StringWrapper(String);

impl StringWrapper {
    fn new(s: &str) -> Self {
        StringWrapper(s.to_string())
    }
}

impl Deref for StringWrapper {
    type Target = String;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let wrapper = StringWrapper::new("hello");
    
    // 通过 Deref 强制转换使用 String 的方法
    println!("长度: {}", wrapper.len());
    println!("大写: {}", wrapper.to_uppercase());
    
    // 显式解引用
    let s: &String = &wrapper;
    println!("解引用: {}", s);
}
```

### Deref 强制转换

```rust
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let m = Box::new(String::from("Rust"));
    
    // Deref 强制转换链：Box<String> -> String -> str
    hello(&m);
    
    // 等价于以下手动转换
    hello(&(*m)[..]);
}
```

### DerefMut 特征

```rust
use std::ops::{Deref, DerefMut};

struct MutWrapper<T>(T);

impl<T> MutWrapper<T> {
    fn new(x: T) -> Self {
        MutWrapper(x)
    }
}

impl<T> Deref for MutWrapper<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MutWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    let mut wrapper = MutWrapper::new(vec![1, 2, 3]);
    
    // 可以使用 Vec 的可变方法
    wrapper.push(4);
    println!("Vec: {:?}", *wrapper);
}
```

### Drop 特征详解

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("清理数据: `{}`", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    
    println!("创建了智能指针");
    
    // 手动调用 drop（如果需要）
    drop(c);
    println!("手动 drop 了 c");
    
    // d 会在作用域结束时自动 drop
}
```

### 复杂的 Drop 实现

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("节点 {} 被清理", self.value);
    }
}

impl Node {
    fn new(value: i32) -> Rc<Self> {
        Rc::new(Node {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }
    
    fn add_child(parent: &Rc<Node>, child: Rc<Node>) {
        child.parent.borrow_mut().replace(Rc::downgrade(parent));
        parent.children.borrow_mut().push(child);
    }
}

fn main() {
    let root = Node::new(1);
    let child1 = Node::new(2);
    let child2 = Node::new(3);
    
    Node::add_child(&root, child1);
    Node::add_child(&root, child2);
    
    println!("树结构创建完成");
    // 所有节点在这里被自动清理
}
```

## 17.3 使用 Box<T> 管理堆上的数据

### Box<T> 的基本使用

```rust
fn main() {
    // 在堆上分配单个值
    let b = Box::new(5);
    println!("b = {}", b);
    
    // 大型数据结构
    let large_array = Box::new([0; 1000000]);
    println!("数组长度: {}", large_array.len());
    
    // 递归类型
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    
    use List::{Cons, Nil};
    
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("链表: {:?}", list);
}
```

### Box<T> 与递归数据结构

```rust
// 二叉搜索树
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
            match self.left {
                Some(ref mut left) => left.insert(value),
                None => self.left = Some(Box::new(TreeNode::new(value))),
            }
        } else {
            match self.right {
                Some(ref mut right) => right.insert(value),
                None => self.right = Some(Box::new(TreeNode::new(value))),
            }
        }
    }
    
    fn search(&self, value: i32) -> bool {
        if value == self.value {
            true
        } else if value < self.value {
            self.left.as_ref().map_or(false, |left| left.search(value))
        } else {
            self.right.as_ref().map_or(false, |right| right.search(value))
        }
    }
}

fn main() {
    let mut root = TreeNode::new(10);
    root.insert(5);
    root.insert(15);
    root.insert(3);
    root.insert(7);
    
    println!("树结构: {:#?}", root);
    println!("查找 7: {}", root.search(7));
    println!("查找 12: {}", root.search(12));
}
```

### Box<T> 的性能特性

```rust
use std::time::Instant;

fn stack_allocation() -> [i32; 1000] {
    [0; 1000]
}

fn heap_allocation() -> Box<[i32; 1000]> {
    Box::new([0; 1000])
}

fn main() {
    // 栈分配测试
    let start = Instant::now();
    for _ in 0..100000 {
        let _array = stack_allocation();
    }
    let stack_duration = start.elapsed();
    
    // 堆分配测试
    let start = Instant::now();
    for _ in 0..100000 {
        let _array = heap_allocation();
    }
    let heap_duration = start.elapsed();
    
    println!("栈分配时间: {:?}", stack_duration);
    println!("堆分配时间: {:?}", heap_duration);
}
```

### Box<T> 与 trait 对象

```rust
trait Animal {
    fn make_sound(&self);
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Animal for Dog {
    fn make_sound(&self) {
        println!("{} says Woof!", self.name);
    }
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("{} says Meow!", self.name);
    }
}

fn main() {
    // 使用 Box 存储不同类型的动物
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog { name: "Buddy".to_string() }),
        Box::new(Cat { name: "Whiskers".to_string() }),
    ];
    
    for animal in animals {
        animal.make_sound();
    }
}
```

### 自定义分配器

```rust
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

// 统计分配的内存
struct CountingAllocator;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
        }
        ret
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        ALLOCATED.fetch_sub(layout.size(), Ordering::SeqCst);
    }
}

#[global_allocator]
static GLOBAL: CountingAllocator = CountingAllocator;

fn main() {
    println!("初始分配: {} bytes", ALLOCATED.load(Ordering::SeqCst));
    
    let numbers = Box::new(vec![1, 2, 3, 4, 5]);
    println!("创建 Box 后: {} bytes", ALLOCATED.load(Ordering::SeqCst));
    
    drop(numbers);
    println!("释放后: {} bytes", ALLOCATED.load(Ordering::SeqCst));
}
```

## RWO 权限分析

### Box<T> 的所有权语义

```rust
fn main() {
    let box1 = Box::new(5);
    
    // O: Box 拥有堆上数据的所有权
    let box2 = box1;  // 所有权转移
    // println!("{}", box1);  // 错误！box1 已被移动
    
    println!("box2: {}", box2);
    
    // R: 可以借用 Box 中的数据
    let box3 = Box::new(String::from("hello"));
    let borrowed: &String = &box3;
    println!("借用的值: {}", borrowed);
    println!("原始 Box: {}", box3);  // box3 仍然可用
    
    // W: 可变借用
    let mut box4 = Box::new(vec![1, 2, 3]);
    let vec_ref: &mut Vec<i32> = &mut box4;
    vec_ref.push(4);
    println!("修改后: {:?}", box4);
}
```

### 智能指针的权限传递

```rust
use std::ops::Deref;

struct Container<T> {
    value: Box<T>,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Container {
            value: Box::new(value),
        }
    }
    
    // R: 返回不可变引用
    fn get(&self) -> &T {
        &self.value
    }
    
    // W: 返回可变引用
    fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }
    
    // O: 消费并返回内部值
    fn into_inner(self) -> T {
        *self.value
    }
}

impl<T> Deref for Container<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn main() {
    let mut container = Container::new(String::from("hello"));
    
    // R: 通过 Deref 访问
    println!("长度: {}", container.len());
    
    // W: 可变访问
    container.get_mut().push_str(" world");
    println!("修改后: {}", container.get());
    
    // O: 获取所有权
    let owned_string = container.into_inner();
    println!("拥有的字符串: {}", owned_string);
    // container 不再可用
}
```

### Drop 与权限的关系

```rust
use std::cell::RefCell;

struct Resource {
    name: String,
    data: RefCell<Vec<i32>>,
}

impl Resource {
    fn new(name: &str) -> Self {
        Resource {
            name: name.to_string(),
            data: RefCell::new(vec![]),
        }
    }
    
    // R: 只读访问
    fn read_data(&self) -> Vec<i32> {
        self.data.borrow().clone()
    }
    
    // W: 修改数据
    fn add_data(&self, value: i32) {
        self.data.borrow_mut().push(value);
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("清理资源: {}", self.name);
        // 注意：这里只能使用 &mut self，所以只能获取可变引用
        let data = self.data.get_mut();
        data.clear();
        println!("数据已清空");
    }
}

fn main() {
    let resource = Box::new(Resource::new("数据库连接"));
    
    // R: 读取操作
    resource.add_data(1);
    resource.add_data(2);
    let data = resource.read_data();
    println!("数据: {:?}", data);
    
    // O: 当 resource 离开作用域时，会调用 drop
}
```

### 智能指针的生命周期

```rust
use std::ops::Deref;

struct LifetimeWrapper<'a, T> {
    value: Box<T>,
    reference: &'a T,
}

impl<'a, T> LifetimeWrapper<'a, T> {
    fn new(value: T, reference: &'a T) -> Self {
        LifetimeWrapper {
            value: Box::new(value),
            reference,
        }
    }
}

impl<'a, T> Deref for LifetimeWrapper<'a, T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

// 返回 Box 的函数
fn create_boxed_string(s: &str) -> Box<String> {
    Box::new(s.to_string())
}

// 接受 Box 参数的函数
fn process_box(boxed: Box<String>) -> usize {
    boxed.len()  // Box 被移动到函数内
}

fn main() {
    let original = String::from("hello");
    
    // 创建包含引用的包装器
    let wrapper = LifetimeWrapper::new(
        String::from("world"),
        &original,
    );
    
    println!("包装的值: {}", *wrapper);
    println!("引用的值: {}", wrapper.reference);
    
    // 函数调用
    let boxed = create_boxed_string("test");
    let length = process_box(boxed);
    println!("长度: {}", length);
    // boxed 在这里不再可用
}
```

### 智能指针与并发

```rust
use std::sync::{Arc, Mutex};
use std::thread;

struct ThreadSafeCounter {
    value: Arc<Mutex<i32>>,
}

impl ThreadSafeCounter {
    fn new() -> Self {
        ThreadSafeCounter {
            value: Arc::new(Mutex::new(0)),
        }
    }
    
    // R: 读取当前值
    fn get(&self) -> i32 {
        *self.value.lock().unwrap()
    }
    
    // W: 增加值
    fn increment(&self) {
        let mut val = self.value.lock().unwrap();
        *val += 1;
    }
    
    // 克隆用于多线程
    fn clone_handle(&self) -> Self {
        ThreadSafeCounter {
            value: Arc::clone(&self.value),
        }
    }
}

fn main() {
    let counter = ThreadSafeCounter::new();
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter_clone = counter.clone_handle();
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                counter_clone.increment();
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("最终计数: {}", counter.get());
}
```

## 小结

本章深入学习了 Rust 的智能指针系统：

1. **智能指针概述**：
   - 智能指针不仅存储数据地址，还包含额外的元数据
   - 通常实现 Deref 和 Drop trait
   - 常见类型：Box<T>、Rc<T>、RefCell<T>、Arc<T>

2. **Deref 和 Drop trait**：
   - Deref 允许智能指针表现得像普通引用
   - Drop 允许自定义清理逻辑
   - Deref 强制转换提供了便利的API

3. **Box<T>**：
   - 在堆上分配数据
   - 适用于递归数据结构
   - 零开销抽象
   - 支持 trait 对象

4. **RWO 权限分析**：
   - **R**：智能指针可以被借用，提供对内部数据的访问
   - **W**：支持可变借用，允许修改内部数据
   - **O**：智能指针拥有数据的所有权，可以转移
   - Drop trait 在所有权结束时自动调用
   - 生命周期确保智能指针的安全使用

智能指针是 Rust 内存管理的核心概念，理解它们对于编写安全、高效的 Rust 代码至关重要。下一章我们将学习常见的智能指针及其应用。 