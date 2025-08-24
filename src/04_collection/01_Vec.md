# Rust std::vec::Vec 库教程（超级扩展版）

Rust 的 `std::vec::Vec<T>` 类型是标准库 `std::vec` 模块（以及相关 `std::collections` 中的 VecDeque 扩展）的核心组成部分，提供动态数组的实现，用于高效管理可增长的连续内存序列，支持元素插入、移除、迭代、容量控制、内存布局优化和高级内存操作。它抽象了底层内存分配器（使用 std::alloc::GlobalAlloc 或自定义 Allocator trait），确保跨平台兼容性和内存安全，并通过 `std::vec::Drain<'a, T>`、`std::vec::Splice<'a, T, I>`、`std::vec::IntoIter<T>` 或运行时 panic（如索引越界、容量溢出或无效切片）显式处理错误如分配失败或无效操作。`std::vec::Vec` 强调 Rust 的所有权、借用和零成本抽象模型：Vec 拥有元素，通过 push/pop/reserve 等方法动态调整大小，支持泛型 T 的任意类型（无需 Copy/Clone，除非指定方法要求）；提供 capacity/shrink_to_fit 以最小化内存使用；集成 Iterator/IntoIterator 以懒惰消费；支持 unsafe 方法如 set_len/as_mut_ptr 以低级控制。模块的设计优先高性能和灵活性，适用于通用数据存储场景（多线程用 Arc<Vec<T>>），并提供 VecDeque<T> 作为双端队列变体以 O(1) 前后操作。`std::vec::Vec` 与 `std::alloc`（自定义分配）、`std::slice`（&[T] 借用视图）、`std::iter`（迭代适配器）、`std::mem`（内存交换/forget）、`std::ptr`（指针操作）、`std::clone`（Vec Clone 深拷贝）和 `std::ops`（Index/IndexMut 到 &T/&mut T）深度集成，支持高级模式如零拷贝切片、Drain 排水迭代和 Splice 拼接操作。


## 1. std::vec::Vec 简介

- **导入和高级结构**：除了基本导入 `use std::vec::Vec;`，高级用法可包括 `use std::vec::{Drain, Splice, IntoIter};` 以访问迭代器变体，以及 `use std::alloc::Allocator;` 以自定义分配（alloc trait）。模块的内部结构包括 Vec 的 RawVec< T, A >（指针 + len + cap）、allocator 集成（Global 默认）和迭代器的状态机（ptr + end）。
    - **类型详解**：
        - `Vec<T, A: Allocator = Global>`：动态数组，支持 push/pop/insert/remove/reserve/shrink_to_fit/clear/len/capacity/is_empty/as_ptr/as_mut_ptr/set_len (unsafe)/into_boxed_slice 等；泛型 A 以自定义分配。
        - `Drain<'a, T, R: RangeBounds<usize> = Full>`：排水迭代器，支持 filter_map 以条件排水。
        - `Splice<'a, T, I: Iterator<Item = T>, R: RangeBounds<usize> = Full>`：拼接迭代器，支持 replace_with 以原子替换范围。
        - `IntoIter<T, A: Allocator = Global>`：消耗迭代器，支持 as_slice/as_mut_slice 以剩余视图。
        - `VecDeque<T, A: Allocator = Global>`：双端队列，支持 push_front/pop_front/rotate_left 等 O(1) 操作。
    - **函数和方法扩展**：`Vec::new` 创建、`Vec::with_capacity` 预分配、`Vec::from_raw_parts` unsafe 创建、`Vec::leak` 'static 泄漏、`Vec::spare_capacity_mut` mutable spare 视图 (1.48+)。
    - **宏**：`vec![]` 创建初始化 Vec。
- **设计哲学扩展**：`std::vec::Vec` 遵循 "growable array"，通过指数增长容量（*2）减摊销；零成本迭代；unsafe 方法允许低级但需 invariant（如 len <= cap）；VecDeque 环形缓冲防移。Vec 是 Send + Sync 如果 T 是，允许线程转移。
- **跨平台详解**：分配用 malloc (Unix)/HeapAlloc (Windows)；对齐 T align_of；测试差异用 CI，焦点大 Vec 分配失败于低内存 OS。
- **性能详析**：push amortized O(1)，insert O(n)；reserve O(1) 分配；drain O(1) 迭代；大 T memmove 慢。基准用 criterion，profile 用 heaptrack 内存高峰。
- **常见用案扩展**：缓冲区（I/O）、栈模拟（push/pop）、队列（VecDeque）、游戏向量（物理模拟）、测试数据生成。

## 2. 创建 Vec：Vec::new 和 vec!

`Vec::new` 是入口，`vec!` 宏初始化。

### 示例：基本 Vec 创建（空和初始化扩展）
```rust
use std::vec::Vec;

fn main() {
    let v: Vec<i32> = Vec::new();
    println!("空: len {}, cap {}", v.len(), v.capacity());  // 0, 0

    let v2 = vec![1, 2, 3];
    println!("宏: {:?}", v2);
}
```

- **解释**：`new` 零 cap。`vec!` 预分配。性能：宏编译时大小。

### 示例：With Capacity（预分配扩展）
```rust
use std::vec::Vec;

fn main() {
    let mut v = Vec::with_capacity(10);
    for i in 0..10 {
        v.push(i);
    }
    println!("无重分配 cap: {}", v.capacity());  // 10
}
```

- **解释**：`with_capacity` 预分配避免重分配。扩展：用 reserve 动态。

### 示例：From Raw Parts（unsafe 创建扩展）
```rust
use std::vec::Vec;

fn main() {
    let ptr = std::alloc::alloc(std::alloc::Layout::array::<i32>(5).unwrap()) as *mut i32;
    unsafe {
        for i in 0..5 {
            *ptr.add(i) = i as i32;
        }
        let v = Vec::from_raw_parts(ptr, 5, 5);
        println!("raw: {:?}", v);
    }
}
```

- **解释**：`from_raw_parts` 手动 ptr/len/cap。unsafe 责任初始化。陷阱：无效 ptr UB。

### 示例：VecDeque 创建（双端扩展）
```rust
use std::collections::VecDeque;

fn main() {
    let mut dq = VecDeque::new();
    dq.push_front(1);
    dq.push_back(2);
    println!("dq: {:?}", dq);  // [1, 2]
}
```

- **解释**：`push_front` O(1)。扩展：用 rotate_left 循环移。

## 3. 操作 Vec：Push、Pop、Insert

操作调整大小。

### 示例：Push 和 Pop（追加移除扩展）
```rust
use std::vec::Vec;

fn main() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    println!("pop: {:?}", v.pop());  // Some(2)
}
```

- **解释**：`push` amortized O(1)。`pop` O(1)。

### 示例：Insert 和 Remove（位置操作扩展）
```rust
use std::vec::Vec;

fn main() {
    let mut v = vec![1, 2, 3];
    v.insert(1, 4);  // [1, 4, 2, 3]
    let removed = v.remove(2);  // 2, v=[1,4,3]
    println!("移除: {}", removed);
}
```

- **解释**：`insert`/`remove` O(n) 移。扩展：用 swap_remove O(1) 无序移除。

### 示例：Reserve 和 Shrink（容量管理扩展）
```rust
use std::vec::Vec;

fn main() {
    let mut v = Vec::with_capacity(10);
    v.extend(1..=5);
    v.reserve(20);  // cap >=25
    v.shrink_to_fit();  // cap=5
    println!("cap: {}", v.capacity());
}
```

- **解释**：`reserve` 确保 cap >= len + add。`shrink_to_fit` 最小化。

### 示例：Drain 和 Splice（范围操作扩展）
```rust
use std::vec::Vec;

fn main() {
    let mut v = vec![1, 2, 3, 4];
    let drained: Vec<i32> = v.drain(1..3).collect();  // [2,3], v=[1,4]
    println!("drain: {:?}", drained);

    v.splice(1..1, [5, 6]);  // 插入 [5,6], v=[1,5,6,4]
}
```

- **解释**：`drain` 移除范围返回迭代器。`splice` 替换范围。扩展：drain_filter 条件移除。

## 4. 迭代和访问：Iter、AsSlice

迭代返回借用。

### 示例：Iter 和 MutIter（借用扩展）
```rust
use std::vec::Vec;

fn main() {
    let v = vec![1, 2, 3];
    let sum: i32 = v.iter().sum();
    println!("sum: {}", sum);

    let mut v_mut = v;
    v_mut.iter_mut().for_each(|x| *x *= 2);
}
```

- **解释**：`iter` &T，`iter_mut` &mut T。扩展：use chunks 块迭代。

### 示例：AsSlice 和 AsMutSlice（视图扩展）
```rust
use std::vec::Vec;

fn main() {
    let v = vec![1, 2, 3];
    let slice = v.as_slice();
    println!("slice: {:?}", slice);

    let mut v_mut = v;
    let mut_slice = v_mut.as_mut_slice();
    mut_slice[0] = 10;
}
```

- **解释**：`as_slice` &[T]。扩展：use split_at 分割。

## 4. 高级：Unsafe、Alloc 和 Deque

- Unsafe：低级控制。

### 示例：SetLen Unsafe（长度设置扩展）
```rust
use std::vec::Vec;

fn main() {
    let mut v: Vec<i32> = Vec::with_capacity(5);
    unsafe { v.set_len(5); }  // 假设初始化
    // 未初始化 UB
}
```

- **解释**：`set_len` 改变 len 无检查。unsafe 责任初始化。

### 示例：Custom Alloc（分配器扩展）
```rust
use std::vec::Vec;
use std::alloc::Global;

fn main() {
    let mut v = Vec::with_capacity_in(10, Global);
    v.push(1);
}
```

- **解释**：`with_capacity_in` 用 Allocator。扩展：用 jemalloc 全局。

### 示例：VecDeque 操作（双端扩展）
```rust
use std::collections::VecDeque;

fn main() {
    let mut dq = VecDeque::new();
    dq.push_front(1);
    dq.pop_back();
    dq.rotate_left(1);  // 循环左移
}
```

- **解释**：O(1) 前后。扩展：用 make_contiguous 连续视图。

## 5. 错误和panic：越界、溢出

Vec panic 于错误。

### 示例：Index Panic（越界扩展）
```rust
use std::vec::Vec;

fn main() {
    let v = vec![1];
    // v[1];  // panic "index out of bounds"
    if let Some(&val) = v.get(1) {
        println!("{}", val);
    } else {
        println!("越界");
    }
}
```

- **解释**：`get` Option 安全。扩展：use checked_index crate。

## 6. 高级主题：Drain、Splice、IntoIter 和 集成

- Drain：移除迭代。

### 示例：Drain Filter（条件排水扩展）
```rust
use std::vec::Vec;

fn main() {
    let mut v = vec![1, 2, 3, 4];
    let even: Vec<i32> = v.drain_filter(|x| *x % 2 == 0).collect();
    println!("even: {:?}", even);  // [2, 4]
    println!("v: {:?}", v);  // [1, 3]
}
```

- **解释**：`drain_filter` 条件移除返回迭代器。扩展：用 retain 就地保留。

### 示例：Splice 拼接（替换扩展）
```rust
use std::vec::Vec;

fn main() {
    let mut v = vec![1, 2, 3];
    let spliced: Vec<i32> = v.splice(1..2, [4, 5]).collect();
    println!("spliced: {:?}", spliced);  // [2]
    println!("v: {:?}", v);  // [1, 4, 5, 3]
}
```

- **解释**：`splice` 替换范围返回迭代器。扩展：用 replace_with 原子。

### 示例：IntoIter 消耗（所有权扩展）
```rust
use std::vec::Vec;

fn main() {
    let v = vec![1, 2, 3];
    let iter = v.into_iter();
    let sum: i32 = iter.sum();
    // v 移动
}
```

- **解释**：`into_iter` 转移所有权。扩展：use as_slice 剩余视图。

## 4. 性能优化：Reserve、Shrink

容量管理减分配。

### 示例：Reserve Exact（精确扩展）
```rust
use std::vec::Vec;

fn main() {
    let mut v = Vec::new();
    v.reserve_exact(100);
    for i in 0..100 {
        v.push(i);
    }
    v.shrink_to(50);  // cap >=50, 尝试缩
}
```

- **解释**：`reserve_exact` 最小分配。`shrink_to` 缩到 >= len。

## 5. Unsafe Vec：FromRaw、SetLen

低级控制。

### 示例：FromRawPartsIn（alloc 扩展）
```rust
use std::vec::Vec;
use std::alloc::{Global, Layout};

fn main() {
    let layout = Layout::array::<i32>(5).unwrap();
    let ptr = unsafe { Global.alloc(layout).cast::<i32>().as_ptr() };
    unsafe {
        for i in 0..5 {
            *ptr.add(i) = i as i32;
        }
        let v = Vec::from_raw_parts_in(ptr, 5, 5, Global);
        println!("v: {:?}", v);
    }
}
```

- **解释**：`from_raw_parts_in` 用 Allocator。unsafe 管理。

## 6. VecDeque：双端

环形缓冲。

### 示例：Rotate 和 MakeContiguous（操作扩展）
```rust
use std::collections::VecDeque;

fn main() {
    let mut dq = VecDeque::from(vec![1, 2, 3]);
    dq.rotate_left(1);  // [2, 3, 1]
    let cont = dq.make_contiguous();
    println!("连续: {:?}", cont);  // &[2, 3, 1]
}
```

- **解释**：`rotate_left` 循环移。`make_contiguous` 重组连续。

## 7. 错误和panic：Vec

Vec panic 于无效。

### 示例：Capacity Overflow（大分配扩展）
```rust
use std::vec::Vec;

fn main() {
    let mut v: Vec<u8> = Vec::new();
    // v.reserve(usize::MAX);  // panic "capacity overflow"
    if let Err(e) = v.try_reserve(usize::MAX) {
        println!("错误: {}", e);  // CapacityOverflow
    }
}
```

- **解释**：`try_reserve` Result 安全。扩展：用 checked_add 计算 cap。

## 8. 最佳实践和常见陷阱

- **Vec 最佳**：reserve 预分配；shrink 回收；drain 批量移除。
- **性能**：指数增长减重分配；deque 前后 O(1)。
- **错误**：panic 越界，用 get；OOM alloc 失败。
- **安全**：unsafe set_len 需初始化；deque contiguous 防 UB。
- **跨平台**：alloc 一致。
- **测试**：miri UB；fuzz push/pop。
- **资源**：drop 释放；leak 'static。
- **常见扩展**：
    - 越界：get Option。
    - OOM：try_reserve 处理。
    - 未初始化：set_len UB，用 resize。
    - 移开销：use swap_remove 无序。

## 9. 练习建议

1. 编写缓冲：Vec<u8> push，reserve 增长。
2. 实现栈：push/pop，capacity 管理。
3. 创建环队列：VecDeque push_front/pop_back。
4. 处理大 Vec：try_reserve 测试 OOM 恢复。
5. 基准：比较 push vs smallvec push 时间，用 criterion。
6. 与 alloc：用 custom allocator 测试 Vec::with_capacity_in。
7. 错误框架：mock alloc fail 测试 try_reserve。
8. 高级 app：实现渲染缓冲：Vec<Vec3> push，drain 清帧。

