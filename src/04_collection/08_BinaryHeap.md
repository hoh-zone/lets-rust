# Rust std::collections::BinaryHeap 库教程

Rust 的 `std::collections::BinaryHeap<T>` 类型是标准库 `std::collections` 模块中实现二进制堆（Binary Heap）的核心组成部分，提供高效的优先级队列操作，支持 O(log n) 插入和移除最大/最小元素的动态集合，适用于需要按优先级处理的场景。它抽象了底层 Vec<T> 数组的堆结构（使用父子索引维护堆性质），确保跨平台兼容性和内存安全，并通过 `std::collections::binary_heap::Drain<'a, T>`、`std::collections::binary_heap::Iter<'a, T>`、`std::collections::binary_heap::PeekMut<'a, T>` 或运行时 panic（如容量溢出或无效比较）显式处理错误如分配失败或堆不变量违反。`std::collections::BinaryHeap` 强调 Rust 的所有权、借用和零成本抽象模型：BinaryHeap 拥有元素，通过 push/pop/peek/peek_mut/into_sorted_vec/append/drain/len/is_empty/capacity/iter 等方法动态调整，支持泛型 T 的 Ord（默认 max-heap，元素要求有序）；集成 Iterator/IntoIterator 以懒惰消费（无序，除非 into_sorted_vec）；支持 PeekMut 以 mut 访问堆顶而不移除。模块的设计优先堆性质和性能，适用于优先级队列、Dijkstra 算法和调度任务场景（对比 Vec 的无序 O(1) 追加），并作为 BinaryHeap 的扩展变体支持自定义分配器（alloc trait，1.36+）和与 MinHeap 的互转（用 Reverse 包裹元素）。`std::collections::BinaryHeap` 与 `std::cmp`（Ord trait 和 Ordering）、`std::alloc`（自定义分配）、`std::iter`（迭代适配器）、`std::mem`（内存交换/forget）、`std::clone`（BinaryHeap Clone 深拷贝）和 `std::ops`（无 Index，以堆不变量）深度集成，支持高级模式如 drain_sorted 有序排水、append 堆合并和与 Vec 的堆化。


## 1. std::collections::BinaryHeap 简介

- **导入和高级结构**：除了基本导入 `use std::collections::BinaryHeap;`，高级用法可包括 `use std::collections::binary_heap::{Drain, DrainSorted, Iter, PeekMut};` 以访问迭代器和 mut 变体，以及 `use std::cmp::Reverse;` 以 min-heap、`use std::alloc::Allocator;` 以自定义分配（alloc trait，1.36+）。模块的内部结构包括 BinaryHeap 的 Vec<T> 缓冲（堆索引父子关系）、Ord 元素比较和 Iter 的堆遍历状态机（无序，除 sorted）。
  - **类型详解**：
    - `BinaryHeap<T, A: Allocator = Global>`：二进制堆（max-heap 默认），支持 push/pop/peek/peek_mut/push_pop/replace/append/drain/drain_sorted/len/is_empty/capacity/iter/into_iter/into_sorted_vec/into_vec/clear 等；泛型 A 以自定义分配。
    - `Drain<'a, T>`：排水迭代器（无序），支持 filter_map 以条件排水。
    - `DrainSorted<'a, T>`：有序排水迭代器（pop 顺序）。
    - `Iter<'a, T>`：无序借用迭代器，支持 cloned 以值复制。
    - `IntoIter<T, A: Allocator = Global>`：消耗迭代器，支持 as_slice (Vec) 以剩余视图。
    - `PeekMut<'a, T>`：堆顶 mut 借用，支持 replace 以替换顶值。
  - **函数和方法扩展**：`BinaryHeap::new` 创建、`BinaryHeap::with_capacity` 预分配、`BinaryHeap::from_vec` heapify Vec、`BinaryHeap::leak` 'static 泄漏 (no, but drop empty)。
  - **宏**：无，但相关如 binaryheap! [v] proposal。
- **设计哲学扩展**：`std::collections::BinaryHeap` 遵循 "max-heap by default"，通过 sift_up/sift_down 维护堆性质 O(log n）；零成本 peek；drain_sorted O(n log n) 排序排水；无 min-heap Built-in (用 Reverse<T> 包裹)。BinaryHeap 是 Send + Sync 如果 T 是，允许线程转移；无内置 custom ord (用 Reverse)。
- **跨平台详解**：缓冲分配用 malloc (Unix)/HeapAlloc (Windows)；对齐 Vec align_of；测试差异用 CI，焦点大 Heap 分配失败于低内存 OS 和 Ord 比较 endian。
- **性能详析**：push/pop O(log n)，peek O(1)；append + heapify O(n)；大 T sift 慢。基准用 criterion，profile 用 heaptrack 堆高度。
- **常见用例扩展**：优先级队列（任务调度）、Dijkstra 最短路径、Kth largest、游戏 AI 路径找、测试堆数据。
- **超级扩展概念**：与 std::cmp::Ordering 集成自定义 min-heap；与 std::panic::catch_unwind 安全 drop 大 Heap；错误 panic 于溢出；与 priority_queue::PriorityQueue 灵活替代；高吞吐用 binary-heap-plus::BinaryHeap min/max 切换；与 tracing::field BinaryHeap 日志；历史：从 1.0 BinaryHeap 到 1.62 append 优化。

## 2. 创建 BinaryHeap：BinaryHeap::new 和 from

`BinaryHeap::new` 是入口，`from` 转换。

### 示例：基本 BinaryHeap 创建（空和初始化扩展）
```rust
use std::collections::BinaryHeap;

fn main() {
    let heap: BinaryHeap<i32> = BinaryHeap::new();
    println!("空: len {}, cap {}", heap.len(), heap.capacity());  // 0, 0

    let heap2 = BinaryHeap::from(vec![3, 1, 2]);
    println!("from: {:?}", heap2.into_sorted_vec());  // [1, 2, 3] (sorted drain)
}
```

- **解释**：`new` 空 Vec。`from` heapify Vec O(n)。性能：from 快于逐 push O(n log n)。

### 示例：With Capacity（预分配扩展）
```rust
use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::with_capacity(10);
    for i in (1..=10).rev() {
        heap.push(i);
    }
    println!("无重分配 cap: {}", heap.capacity());  // >=10
}
```

- **解释**：`with_capacity` 预 Vec cap。扩展：use reserve 动态。

### 示例：Min Heap 用 Reverse（切换扩展）
```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let mut min_heap = BinaryHeap::<Reverse<i32>>::new();
    min_heap.push(Reverse(3));
    min_heap.push(Reverse(1));
    min_heap.push(Reverse(2));
    println!("min pop: {:?}", min_heap.pop());  // Reverse(1)
}
```

- **解释**：`Reverse` 反转 Ord 为 min-heap。扩展：自定义 Ord wrapper 复杂优先级。

## 3. 操作 BinaryHeap：Push、Pop、Peek

操作维护堆。

### 示例：Push 和 Pop（添加移除扩展）
```rust
use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();
    heap.push(1);
    heap.push(3);
    heap.push(2);
    println!("pop: {:?}", heap.pop());  // Some(3) max
}
```

- **解释**：`push` sift_up O(log n)。`pop` sift_down O(log n)。性能：max-heap pop 最大。

### 示例：Peek 和 PeekMut（检查修改扩展）
```rust
use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::from(vec![1, 3, 2]);
    println!("peek: {:?}", heap.peek());  // Some(&3)

    if let Some(mut_top) = heap.peek_mut() {
        *mut_top = 4;
    }
    println!("peek mut: {:?}", heap.peek());  // Some(&4)
}
```

- **解释**：`peek` &T Option。`peek_mut` PeekMut<'a, T> mut 访问，drop 时 sift_down 如果改小。扩展：use PeekMut::pop 移除顶。

### 示例：PushPop 和 Replace（组合扩展）
```rust
use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::from(vec![1, 2]);
    let max = heap.push_pop(3);  // push 3, pop 3 (max)
    println!("push_pop: {:?}", max);  // 3, heap [2,1]

    let old = heap.replace(4);  // replace top 2 with 4, return 2
    println!("replace: {:?}", old);  // 2, heap [4,1]
}
```

- **解释**：`push_pop` 组合 O(log n)。`replace` 替换顶返回旧。

## 4. 容量管理：Reserve、Shrink

基于 Vec 内部。

### 示例：Reserve 和 Shrink（管理扩展）
```rust
use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();
    heap.reserve(10);  // Vec cap >=10
    heap.extend(1..=5);
    heap.shrink_to_fit();  // cap~5
    println!("cap: {}", heap.capacity());
}
```

- **解释**：`reserve` 确保内部 Vec cap >= len + add。`shrink_to_fit` 最小化。

## 5. 迭代：Iter、Drain

迭代无序（堆布局）。

### 示例：Drain Sorted（有序排水扩展）
```rust
use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::from(vec![3, 1, 2]);
    let sorted: Vec<i32> = heap.drain_sorted().collect();
    println!("sorted: {:?}", sorted);  // [1,2,3]
    println!("heap 空: {}", heap.is_empty());
}
```

- **解释**：`drain_sorted` pop 顺序返回 iter 清 heap。

### 示例：IntoSortedVec（转换扩展）
```rust
use std::collections::BinaryHeap;

fn main() {
    let heap = BinaryHeap::from(vec![3, 1, 2]);
    let sorted = heap.into_sorted_vec();
    println!("sorted vec: {:?}", sorted);  // [1,2,3]
}
```

- **解释**：`into_sorted_vec` 消耗堆返回 sorted Vec O(n log n)。

## 6. 高级：Append、Custom Ord、Min Heap

- Append：合并堆。

### 示例：Append（追加扩展）
```rust
use std::collections::BinaryHeap;

fn main() {
    let mut heap1 = BinaryHeap::from(vec![1, 3]);
    let mut heap2 = BinaryHeap::from(vec![2, 4]);
    heap1.append(&mut heap2);  // heap1 重堆 [4,3,2,1], heap2 空
    println!("append: {:?}", heap1.into_sorted_vec());  // [1,2,3,4]
}
```

- **解释**：`append` 移动 heap2 到 heap1，heapify O(n)。

### 示例：Custom Ord Min Heap（优先级扩展）
```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let mut min_heap = BinaryHeap::<Reverse<i32>>::new();
    min_heap.push(Reverse(3));
    min_heap.push(Reverse(1));
    min_heap.push(Reverse(2));
    println!("min pop: {:?}", min_heap.pop());  // Reverse(1)
}
```

- **解释**：`Reverse` 反转为 min-heap。扩展：自定义 Ord wrapper 复杂优先级，如 (priority, timestamp) 元组。

## 7. 最佳实践和常见陷阱

- **Heap 最佳**：with_capacity 预分配；peek_mut 修改顶；append 合并。
- **性能**：O(log n) 均衡；into_sorted_vec O(n log n)。
- **错误**：panic 溢出，无 try_push。
- **安全**：T Ord 正确；peek_mut 后 sift 如果改。
- **跨平台**：cmp 一致。
- **测试**：loom 无，但 Ord fuzz。
- **资源**：drop 释放；clear 不回收 Vec。
- **常见扩展**：
  - 无序：heap 布局无序，用 sorted。
  - 改顶：peek_mut sift 维护堆。
  - 未找到：peek Option。
  - 内存高：用 smallvec 节点 (no, heap Vec)。

## 8. 练习建议

1. 编写优先队列：push priority，pop max。
2. 实现 Kth largest：heap push，pop >k。
3. 创建 min heap：用 Reverse 测试 pop min。
4. 处理大 Heap：append 测试大堆合并。
5. 基准：比较 push/pop vs Vec sort 时间，用 criterion。
6. 与 iter：用 drain_sorted 收集有序。
7. 错误框架：mock Ord panic 测试 push 恢复。
8. 高级 app：实现 Dijkstra：BinaryHeap<(Dist, Node)> pop min dist。

如果需要更多、集成或深入，请细节！