# 第16章：迭代器

迭代器是 Rust 中处理序列数据的核心抽象，提供了一种惰性、高效、可组合的数据处理方式。

## 16.1 迭代器的概念与类型

### 什么是迭代器

```rust
// Iterator trait 的定义
pub trait Iterator {
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
    
    // ... 其他提供的方法
}

// 基本使用示例
fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    
    // 创建迭代器
    let mut iter = vec.iter();
    
    // 手动调用 next
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), None);
}
```

### 三种迭代器类型

```rust
fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    
    // 1. iter() - 不可变引用迭代器
    let iter = vec.iter();
    for &item in iter {
        println!("不可变引用: {}", item);
        // item 的类型是 &i32
    }
    println!("vec 仍然可用: {:?}", vec);
    
    // 2. iter_mut() - 可变引用迭代器
    let mut vec_mut = vec![1, 2, 3, 4, 5];
    let iter_mut = vec_mut.iter_mut();
    for item in iter_mut {
        *item *= 2;
        // item 的类型是 &mut i32
    }
    println!("修改后的 vec: {:?}", vec_mut);
    
    // 3. into_iter() - 获取所有权的迭代器
    let vec_owned = vec![1, 2, 3, 4, 5];
    let into_iter = vec_owned.into_iter();
    for item in into_iter {
        println!("拥有所有权: {}", item);
        // item 的类型是 i32
    }
    // println!("{:?}", vec_owned); // 错误！vec_owned 已被移动
}
```

### 常见集合的迭代器

```rust
use std::collections::{HashMap, HashSet};

fn main() {
    // 数组迭代器
    let arr = [1, 2, 3, 4, 5];
    for &item in arr.iter() {
        println!("数组元素: {}", item);
    }
    
    // 切片迭代器
    let slice = &[10, 20, 30];
    for (index, &value) in slice.iter().enumerate() {
        println!("索引 {}: 值 {}", index, value);
    }
    
    // String 迭代器
    let s = String::from("hello");
    for ch in s.chars() {
        println!("字符: {}", ch);
    }
    for byte in s.bytes() {
        println!("字节: {}", byte);
    }
    
    // HashMap 迭代器
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
    
    // HashSet 迭代器
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    
    for value in &set {
        println!("集合元素: {}", value);
    }
}
```

### Range 迭代器

```rust
fn main() {
    // 基本范围
    for i in 0..5 {
        println!("范围: {}", i);
    }
    
    // 包含结束值的范围
    for i in 0..=5 {
        println!("包含结束: {}", i);
    }
    
    // 步进
    for i in (0..10).step_by(2) {
        println!("步进2: {}", i);
    }
    
    // 反向
    for i in (0..5).rev() {
        println!("反向: {}", i);
    }
    
    // 无限迭代器
    let mut counter = 0;
    for i in 0.. {
        if i > 5 { break; }
        counter += 1;
        println!("无限: {}", i);
    }
}
```

## 16.2 常见的迭代器方法

### 转换方法

```rust
fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    
    // map: 转换每个元素
    let squares: Vec<i32> = vec.iter()
        .map(|&x| x * x)
        .collect();
    println!("平方: {:?}", squares);
    
    // filter: 过滤元素
    let evens: Vec<&i32> = vec.iter()
        .filter(|&&x| x % 2 == 0)
        .collect();
    println!("偶数: {:?}", evens);
    
    // filter_map: 同时过滤和转换
    let result: Vec<i32> = vec.iter()
        .filter_map(|&x| {
            if x % 2 == 0 {
                Some(x * x)
            } else {
                None
            }
        })
        .collect();
    println!("偶数的平方: {:?}", result);
    
    // flat_map: 展平嵌套结构
    let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
    let flattened: Vec<i32> = nested.into_iter()
        .flat_map(|v| v.into_iter())
        .collect();
    println!("展平: {:?}", flattened);
}
```

### 消费方法

```rust
fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    
    // collect: 收集到集合
    let doubled: Vec<i32> = vec.iter()
        .map(|&x| x * 2)
        .collect();
    
    // sum: 求和
    let sum: i32 = vec.iter().sum();
    println!("总和: {}", sum);
    
    // product: 求积
    let product: i32 = vec.iter().product();
    println!("乘积: {}", product);
    
    // count: 计数
    let count = vec.iter().filter(|&&x| x > 2).count();
    println!("大于2的个数: {}", count);
    
    // fold: 自定义累积
    let result = vec.iter().fold(0, |acc, &x| acc + x * x);
    println!("平方和: {}", result);
    
    // reduce: 类似 fold 但使用第一个元素作为初始值
    let max = vec.iter().reduce(|a, b| if a > b { a } else { b });
    println!("最大值: {:?}", max);
}
```

### 查找方法

```rust
fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    
    // find: 查找第一个匹配的元素
    let first_even = vec.iter().find(|&&x| x % 2 == 0);
    println!("第一个偶数: {:?}", first_even);
    
    // position: 查找位置
    let pos = vec.iter().position(|&x| x == 3);
    println!("3的位置: {:?}", pos);
    
    // any: 是否存在匹配
    let has_even = vec.iter().any(|&x| x % 2 == 0);
    println!("是否有偶数: {}", has_even);
    
    // all: 是否全部匹配
    let all_positive = vec.iter().all(|&x| x > 0);
    println!("是否全部为正: {}", all_positive);
    
    // max/min: 最大最小值
    let max = vec.iter().max();
    let min = vec.iter().min();
    println!("最大: {:?}, 最小: {:?}", max, min);
}
```

### 组合和分组

```rust
fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec!['a', 'b', 'c'];
    
    // zip: 配对两个迭代器
    let pairs: Vec<(i32, char)> = vec1.iter()
        .cloned()
        .zip(vec2.iter().cloned())
        .collect();
    println!("配对: {:?}", pairs);
    
    // chain: 连接迭代器
    let vec3 = vec![4, 5, 6];
    let chained: Vec<i32> = vec1.iter()
        .chain(vec3.iter())
        .cloned()
        .collect();
    println!("连接: {:?}", chained);
    
    // partition: 分组
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let (evens, odds): (Vec<i32>, Vec<i32>) = numbers.into_iter()
        .partition(|&x| x % 2 == 0);
    println!("偶数: {:?}, 奇数: {:?}", evens, odds);
    
    // chunks: 分块
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let chunks: Vec<Vec<i32>> = data.chunks(3)
        .map(|chunk| chunk.to_vec())
        .collect();
    println!("分块: {:?}", chunks);
}
```

## 16.3 自定义迭代器

### 基本自定义迭代器

```rust
// 斐波那契数列迭代器
struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;
    
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next = current + self.next;
        
        Some(current)
    }
}

fn main() {
    let fib = Fibonacci::new();
    
    // 取前10个斐波那契数
    let fib_numbers: Vec<u64> = fib.take(10).collect();
    println!("斐波那契: {:?}", fib_numbers);
}
```

### 为自定义类型实现迭代器

```rust
// 自定义集合
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// 带状态的迭代器
struct RingBuffer<T> {
    data: Vec<T>,
    index: usize,
}

impl<T: Clone> RingBuffer<T> {
    fn new(data: Vec<T>) -> Self {
        RingBuffer { data, index: 0 }
    }
    
    fn iter(&self) -> RingBufferIter<T> {
        RingBufferIter {
            buffer: self,
            count: 0,
        }
    }
}

struct RingBufferIter<'a, T> {
    buffer: &'a RingBuffer<T>,
    count: usize,
}

impl<'a, T: Clone> Iterator for RingBufferIter<'a, T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.buffer.data.len() {
            return None;
        }
        
        let index = (self.buffer.index + self.count) % self.buffer.data.len();
        self.count += 1;
        Some(self.buffer.data[index].clone())
    }
}
```

### 双向迭代器

```rust
// 实现双向迭代器
struct BiCounter {
    front: i32,
    back: i32,
}

impl BiCounter {
    fn new(start: i32, end: i32) -> Self {
        BiCounter { front: start, back: end }
    }
}

impl Iterator for BiCounter {
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.front <= self.back {
            let result = self.front;
            self.front += 1;
            Some(result)
        } else {
            None
        }
    }
}

impl DoubleEndedIterator for BiCounter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front <= self.back {
            let result = self.back;
            self.back -= 1;
            Some(result)
        } else {
            None
        }
    }
}

fn main() {
    let counter = BiCounter::new(1, 5);
    
    // 从前往后
    let forward: Vec<i32> = counter.clone().collect();
    println!("正向: {:?}", forward);
    
    // 从后往前
    let backward: Vec<i32> = counter.rev().collect();
    println!("反向: {:?}", backward);
}
```

## 16.4 消费型与适配型迭代器

### 适配器链

```rust
fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 复杂的适配器链
    let result: Vec<i32> = data.iter()
        .filter(|&&x| x % 2 == 0)      // 过滤偶数
        .map(|&x| x * x)                // 平方
        .filter(|&x| x < 50)            // 过滤小于50的
        .take(3)                        // 取前3个
        .collect();
    
    println!("结果: {:?}", result);
    
    // 惰性求值演示
    let iter = data.iter()
        .map(|x| {
            println!("映射: {}", x);
            x * 2
        });
    
    println!("迭代器已创建，但还未执行");
    
    // 只有在消费时才会执行
    let _: Vec<i32> = iter.take(3).collect();
}
```

### 自定义适配器

```rust
// 自定义适配器：跳过偶数位置的元素
struct SkipEven<I> {
    iter: I,
    index: usize,
}

impl<I> SkipEven<I> {
    fn new(iter: I) -> Self {
        SkipEven { iter, index: 0 }
    }
}

impl<I> Iterator for SkipEven<I>
where
    I: Iterator,
{
    type Item = I::Item;
    
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(item) => {
                    self.index += 1;
                    if self.index % 2 == 1 {
                        return Some(item);
                    }
                }
                None => return None,
            }
        }
    }
}

// 为 Iterator trait 添加扩展方法
trait IteratorExt: Iterator {
    fn skip_even(self) -> SkipEven<Self>
    where
        Self: Sized,
    {
        SkipEven::new(self)
    }
}

impl<T: Iterator> IteratorExt for T {}

fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6];
    let result: Vec<i32> = vec.into_iter()
        .skip_even()
        .collect();
    println!("跳过偶数位置: {:?}", result);
}
```

### 性能优化

```rust
use std::time::Instant;

fn main() {
    let data: Vec<i32> = (0..1_000_000).collect();
    
    // 使用迭代器（推荐）
    let start = Instant::now();
    let sum1: i32 = data.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * 2)
        .sum();
    let duration1 = start.elapsed();
    
    // 使用循环
    let start = Instant::now();
    let mut sum2 = 0;
    for &x in &data {
        if x % 2 == 0 {
            sum2 += x * 2;
        }
    }
    let duration2 = start.elapsed();
    
    println!("迭代器: {:?}, 结果: {}", duration1, sum1);
    println!("循环: {:?}, 结果: {}", duration2, sum2);
    
    // 并行迭代器（需要 rayon crate）
    // use rayon::prelude::*;
    // let sum3: i32 = data.par_iter()
    //     .filter(|&&x| x % 2 == 0)
    //     .map(|&x| x * 2)
    //     .sum();
}
```

## RWO 权限分析

### 迭代器的权限类型

```rust
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    
    // R: iter() - 不可变借用
    {
        let iter = vec.iter();
        for item in iter {
            // item: &i32
            println!("读取: {}", item);
        }
    }
    vec.push(6);  // vec 仍然可用
    
    // W: iter_mut() - 可变借用
    {
        let iter = vec.iter_mut();
        for item in iter {
            // item: &mut i32
            *item *= 2;
        }
    }
    println!("修改后: {:?}", vec);
    
    // O: into_iter() - 获取所有权
    let vec_owned = vec![1, 2, 3];
    let iter = vec_owned.into_iter();
    for item in iter {
        // item: i32
        println!("拥有: {}", item);
    }
    // vec_owned 不再可用
}
```

### 迭代器适配器的权限传递

```rust
fn process_data() {
    let data = vec![
        String::from("hello"),
        String::from("world"),
        String::from("rust"),
    ];
    
    // R: 保持不可变引用
    let lengths: Vec<usize> = data.iter()
        .map(|s| s.len())
        .collect();
    println!("长度: {:?}", lengths);
    println!("原始数据仍可用: {:?}", data);
    
    // O: 转移所有权
    let uppercase: Vec<String> = data.into_iter()
        .map(|s| s.to_uppercase())
        .collect();
    println!("大写: {:?}", uppercase);
    // println!("{:?}", data);  // 错误！data 已被移动
    
    // 混合权限
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    // 创建新集合，原集合保持不变
    let doubled: Vec<i32> = numbers.iter()
        .map(|&x| x * 2)
        .collect();
    
    // 原地修改
    numbers.iter_mut()
        .for_each(|x| *x *= 3);
    
    println!("翻倍: {:?}", doubled);
    println!("原地修改: {:?}", numbers);
}
```

### 自定义迭代器的权限设计

```rust
// 拥有所有权的迭代器
struct OwnedIterator<T> {
    items: Vec<T>,
    index: usize,
}

impl<T> OwnedIterator<T> {
    fn new(items: Vec<T>) -> Self {
        OwnedIterator { items, index: 0 }
    }
}

impl<T> Iterator for OwnedIterator<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.items.len() {
            // 使用 swap_remove 避免移动整个向量
            Some(self.items.swap_remove(self.index))
        } else {
            None
        }
    }
}

// 借用的迭代器
struct BorrowedIterator<'a, T> {
    items: &'a [T],
    index: usize,
}

impl<'a, T> BorrowedIterator<'a, T> {
    fn new(items: &'a [T]) -> Self {
        BorrowedIterator { items, index: 0 }
    }
}

impl<'a, T> Iterator for BorrowedIterator<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.items.len() {
            let item = &self.items[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

// 可变借用的迭代器
struct MutBorrowedIterator<'a, T> {
    items: &'a mut [T],
    index: usize,
}

impl<'a, T> MutBorrowedIterator<'a, T> {
    fn new(items: &'a mut [T]) -> Self {
        MutBorrowedIterator { items, index: 0 }
    }
}

impl<'a, T> Iterator for MutBorrowedIterator<'a, T> {
    type Item = &'a mut T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.items.len() {
            let item = unsafe {
                &mut *(self.items.as_mut_ptr().add(self.index))
            };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}
```

### 生命周期与迭代器

```rust
// 带生命周期的迭代器包装器
struct LifetimeIterator<'a, T> {
    data: &'a [T],
    current: usize,
}

impl<'a, T> LifetimeIterator<'a, T> {
    fn new(data: &'a [T]) -> Self {
        LifetimeIterator { data, current: 0 }
    }
}

impl<'a, T> Iterator for LifetimeIterator<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.data.len() {
            let item = &self.data[self.current];
            self.current += 1;
            Some(item)
        } else {
            None
        }
    }
}

// 生命周期传播
fn filter_and_collect<'a>(data: &'a [i32]) -> Vec<&'a i32> {
    data.iter()
        .filter(|&&x| x > 0)
        .collect()
}

// 多个生命周期
struct PairIterator<'a, 'b, T, U> {
    first: &'a [T],
    second: &'b [U],
    index: usize,
}

impl<'a, 'b, T, U> Iterator for PairIterator<'a, 'b, T, U> {
    type Item = (&'a T, &'b U);
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.first.len() && self.index < self.second.len() {
            let item = (&self.first[self.index], &self.second[self.index]);
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}
```

### 并发迭代器的权限

```rust
use std::sync::{Arc, Mutex};
use std::thread;

// 线程安全的迭代器
struct ConcurrentIterator<T> {
    items: Arc<Mutex<Vec<T>>>,
}

impl<T: Send + 'static> ConcurrentIterator<T> {
    fn new(items: Vec<T>) -> Self {
        ConcurrentIterator {
            items: Arc::new(Mutex::new(items)),
        }
    }
    
    // 并行处理每个元素
    fn parallel_foreach<F>(&self, f: F)
    where
        F: Fn(T) + Send + 'static + Clone,
        T: Clone,
    {
        let items = self.items.lock().unwrap().clone();
        let mut handles = vec![];
        
        for item in items {
            let f = f.clone();
            let handle = thread::spawn(move || {
                f(item);
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
    }
}

// 使用 crossbeam 的 scope 实现更安全的并发
// use crossbeam::scope;
// 
// fn scoped_iteration<T: Sync>(data: &[T]) {
//     scope(|s| {
//         for item in data {
//             s.spawn(move |_| {
//                 process(item);
//             });
//         }
//     }).unwrap();
// }
```

## 小结

本章深入学习了 Rust 的迭代器系统：

1. **基本概念**：
   - Iterator trait 定义了 next 方法
   - 三种迭代器：iter()、iter_mut()、into_iter()
   - 迭代器是惰性的，只在消费时执行

2. **常用方法**：
   - 转换：map、filter、filter_map、flat_map
   - 消费：collect、sum、fold、reduce
   - 查找：find、position、any、all
   - 组合：zip、chain、partition

3. **自定义迭代器**：
   - 实现 Iterator trait
   - 可选实现 DoubleEndedIterator
   - 注意生命周期和权限设计

4. **性能考虑**：
   - 迭代器通常比循环更高效
   - 编译器能够优化迭代器链
   - 可以使用并行迭代器提升性能

5. **RWO 权限分析**：
   - **R (iter)**：产生不可变引用，原数据仍可用
   - **W (iter_mut)**：产生可变引用，独占访问
   - **O (into_iter)**：转移所有权，原数据不可用
   - 适配器会传递权限类型
   - 自定义迭代器需要正确处理权限

迭代器是 Rust 中函数式编程的核心，掌握迭代器能让代码更简洁、高效、安全。下一章我们将学习智能指针。