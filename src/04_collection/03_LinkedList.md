# Rust std::collections::LinkedList 库教程

Rust 的 `std::collections::LinkedList<T>` 类型是标准库 `std::collections` 模块中实现双向链表（Doubly-Linked List）的核心组成部分，提供高效的在任意位置插入/移除元素的动态序列，支持 O(1) 前后端操作、游标（Cursor）定位和链表分割/拼接。它抽象了底层节点分配（使用 Box<Node<T>> 的链式结构），确保跨平台兼容性和内存安全，并通过 `std::collections::linked_list::Cursor<'a, T>`、`std::collections::linked_list::CursorMut<'a, T>`、`std::collections::linked_list::Iter<'a, T>` 或运行时 panic（如无效游标、溢出或借用冲突）显式处理错误如分配失败或无效操作。`std::collections::LinkedList` 强调 Rust 的所有权、借用和零成本抽象模型：LinkedList 拥有节点，通过 push_front/push_back/pop_front/pop_back/append/split_off/remove 等方法动态调整，支持泛型 T 的任意类型（无需 Copy/Clone，除非指定方法要求）；提供 len/is_empty 以查询大小，但无 capacity（链表无预分配概念）；集成 Iterator/IntoIterator 以懒惰消费；支持 Cursor 以 O(1) 定位任意元素进行插入/移除。模块的设计优先灵活性和节点级操作，适用于频繁插入/删除的场景（对比 Vec 的连续内存优势），并作为链表的扩展变体支持拼接和游标导航。`std::collections::LinkedList` 与 `std::alloc`（自定义分配）、`std::iter`（迭代适配器）、`std::mem`（内存交换/forget）、`std::ptr`（指针操作）、`std::clone`（LinkedList Clone 深拷贝）和 `std::ops`（Index 到 &T 但无 mut，以防无效化）深度集成，支持高级模式如原子链表拼接、Cursor 游标遍历和与 Vec 的互转。

## 1. std::collections::LinkedList 简介

- **导入和高级结构**：除了基本导入 `use std::collections::LinkedList;`，高级用法可包括 `use std::collections::linked_list::{Cursor, CursorMut, Iter, IntoIter};` 以访问游标和迭代器变体，以及 `use std::alloc::Allocator;` 以自定义分配（alloc trait，future）。模块的内部结构包括 LinkedList 的 双向 Node<Box<Node<T>>> 链（head/tail 指针 + len）、游标的 &mut LinkedList + Option<&mut Node> 定位和迭代器的链遍历状态机。
    - **类型详解**：
        - `LinkedList<T>`：双向链表，支持 push_front/push_back/pop_front/pop_back/append/split_off/insert_before/insert_after/remove/front/back/front_mut/back_mut/len/is_empty/iter/iter_mut/into_iter/cursor/cursor_mut/clear 等；无 capacity，但 len O(1)。
        - `Cursor<'a, T>`/`CursorMut<'a, T>`：借用游标，支持 move_next/move_prev/insert_after/insert_before/remove_current/split_before/split_after/splice_before/splice_after 等原子操作。
        - `Iter<'a, T>`/`IterMut<'a, T>`：借用迭代器，支持 rev() 双端遍历、peekable 等适配。
        - `IntoIter<T>`：消耗迭代器，支持 as_slice (no, 但 future) 以剩余视图。
    - **函数和方法扩展**：`LinkedList::new` 创建、`LinkedList::from_iter` 从迭代器、`LinkedList::append` 原子拼接、`LinkedList::split_off` 分割返回新 list、`LinkedList::leak` 'static 泄漏 (no, but drop empty)。
    - **宏**：无，但相关如 linkedlist![] proposal。
- **设计哲学扩展**：`std::collections::LinkedList` 遵循 "node-based deque"，通过双向指针 O(1) 任意插入/移除（对比 Vec O(n)）；零成本迭代；无预分配容量以最小内存；Cursor 提供原子节点操作以防无效化。LinkedList 是 Send + Sync 如果 T 是，允许线程转移；无内置 alloc trait (future)。
- **跨平台详解**：节点分配用 malloc (Unix)/HeapAlloc (Windows)；对齐 Box align_of；测试差异用 CI，焦点大 List 分配失败于低内存 OS。
- **性能详析**：push_front/back O(1) 分配；append O(1) 链接；cursor insert O(1)；大 T Box 分配慢。基准用 criterion，profile 用 heaptrack 节点高峰。
- **常见用例扩展**：编译器 AST 链表、任务调度队列、历史 undo/redo、游戏事件链、测试序列模拟。
- **超级扩展概念**：与 std::alloc::alloc 集成自定义节点；与 std::panic::catch_unwind 安全 drop 大 List；错误 panic 于越界；与 intrusive-collections::LinkedList 高性能入侵式替代；高吞吐用 linked-list-allocator 池化节点；与 tracing::span List 日志；历史：从 1.0 LinkedList 到 1.60 Cursor::splice 优化。

## 2. 创建 LinkedList：LinkedList::new 和 from_iter

`LinkedList::new` 是入口，`from_iter` 转换。

### 示例：基本 LinkedList 创建（空和初始化扩展）
```rust
use std::collections::LinkedList;

fn main() {
    let list: LinkedList<i32> = LinkedList::new();
    println!("空: len {}", list.len());  // 0

    let list2: LinkedList<i32> = (1..4).collect();
    println!("collect: {:?}", list2);  // [1, 2, 3]
}
```

- **解释**：`new` 零节点。`collect` 从 iter 构建。性能：O(n) 分配于元素。

### 示例：From Iter 高级（链式构建扩展）
```rust
use std::collections::LinkedList;

fn main() {
    let list = LinkedList::from_iter(1..=5);
    println!("from_iter: {:?}", list);  // [1, 2, 3, 4, 5]
}
```

- **解释**：`from_iter` 泛型 FromIterator。扩展：用 extend 追加 iter。

## 3. 操作 LinkedList：Push、Pop、Append

操作调整链。

### 示例：Push 和 Pop（前后追加移除扩展）
```rust
use std::collections::LinkedList;

fn main() {
    let mut list = LinkedList::new();
    list.push_front(1);
    list.push_back(2);
    println!("pop_front: {:?}", list.pop_front());  // Some(1)
    println!("pop_back: {:?}", list.pop_back());    // Some(2)
}
```

- **解释**：`push_front/back` O(1) 分配。`pop_front/back` O(1)。

### 示例：Append 和 SplitOff（拼接分割扩展）
```rust
use std::collections::LinkedList;

fn main() {
    let mut list1 = LinkedList::from_iter(1..=3);
    let mut list2 = LinkedList::from_iter(4..=6);
    list1.append(&mut list2);  // list1 [1,2,3,4,5,6], list2 空
    let split = list1.split_off(3);  // list1 [1,2,3], split [4,5,6]
    println!("split: {:?}", split);
}
```

- **解释**：`append` O(1) 链接链。`split_off` O(n) 遍历到位置。扩展：用 splice_before Cursor 原子。

### 示例：Insert 和 Remove（位置操作扩展）
```rust
use std::collections::LinkedList;

fn main() {
    let mut list = LinkedList::from_iter(1..=3);
    list.insert_before(&mut list.front_mut().unwrap(), 0);  // no, use Cursor
    // Cursor 示例见下
}
```

- **解释**：无直接位置 insert，用 Cursor O(1) 如果定位。

## 4. 游标：Cursor 和 CursorMut

游标定位操作。

### 示例：Cursor 基本（导航扩展）
```rust
use std::collections::LinkedList;

fn main() {
    let mut list = LinkedList::from_iter(1..=3);
    let mut cursor = list.cursor_front_mut();
    cursor.insert_after(4);  // [1,4,2,3]
    cursor.move_next();
    cursor.remove_current();  // [1,4,3]
}
```

- **解释**：`cursor_front_mut` 起始游标。`insert_after` O(1)。`remove_current` O(1)。

### 示例：Cursor Splice（拼接扩展）
```rust
use std::collections::LinkedList;

fn main() {
    let mut list1 = LinkedList::from_iter(1..=3);
    let mut list2 = LinkedList::from_iter(4..=6);
    let mut cursor = list1.cursor_back_mut();
    cursor.splice_after(list2);  // list1 [1,2,3,4,5,6], list2 空
}
```

- **解释**：`splice_after` O(1) 拼接链。扩展：splice_before 前插。

## 4. 迭代：Iter、IntoIter

迭代返回借用。

### 示例：Iter 和 MutIter（借用扩展）
```rust
use std::collections::LinkedList;

fn main() {
    let list = LinkedList::from_iter(1..=3);
    let sum: i32 = list.iter().sum();
    println!("sum: {}", sum);

    let mut list_mut = list;
    list_mut.iter_mut().for_each(|x| *x *= 2);
}
```

- **解释**：`iter` &T，`iter_mut` &mut T。扩展：use rev 双端反转。

## 5. 高级：Unsafe、Alloc 和 集成

- Unsafe：低级。

### 示例：Unsafe Mut（指针扩展）
```rust
use std::collections::LinkedList;

fn main() {
    let mut list = LinkedList::from_iter(1..=3);
    unsafe {
        let ptr = list.front_mut().as_mut_ptr();
        *ptr = 10;
    }
}
```

- **解释**：`as_mut_ptr` *mut T。unsafe 责任不无效。

### 示例：Custom Alloc（分配器扩展）
```rust
use std::collections::VecDeque;  // VecDeque 有，LinkedList future
// LinkedList 无 alloc，use Vec for ex
```

- **解释**：LinkedList 无 alloc trait，用 Box 内部。

## 6. 错误和panic：LinkedList

LinkedList panic 于无效。

### 示例：Cursor Invalid（操作扩展）
```rust
use std::collections::LinkedList;

fn main() {
    let mut list = LinkedList::new();
    let mut cursor = list.cursor_front_mut();
    // cursor.remove_current();  // panic "no current"
}
```

- **解释**：无效 cursor 操作 panic。用 if cursor.current().is_some() 检查。

## 7. 最佳实践和常见陷阱

- **List 最佳**：用 append 合并；Cursor 定位操作；split_off 分割。
- **性能**：O(1) 前后；O(n) 中间遍历。
- **错误**：panic 无效 Cursor，用 check。
- **安全**：unsafe mut 需不破链。
- **跨平台**：alloc 一致。
- **测试**：miri UB；fuzz push/pop。
- **资源**：drop 释放链。
- **常见扩展**：
    - 无效 Cursor：current is_some 检查。
    - 内存碎片：链表高开销于小 T，用 Vec。
    - 未释放：循环 Weak 解决 (Rc/Arc)。
    - 遍历慢：用 Vec 连续。

## 8. 练习建议

1. 编写链表队列：push_back，pop_front 操作。
2. 实现合并排序：用 append 分治合并。
3. 创建 Cursor 编辑器：insert/remove 文本链表。
4. 处理大 List：split_off 测试大链分割。
5. 基准：比较 LinkedList append vs Vec append 时间，用 criterion。
6. 与 iter：用 iter_mut map 修改链。
7. 错误框架：mock invalid Cursor 测试 panic 恢复。
8. 高级 app：实现编译器符号链：LinkedList<Symbol> append 作用域。

