# Rust std::collections::BTreeSet 库教程

Rust 的 `std::collections::BTreeSet<T>` 类型是标准库 `std::collections` 模块中实现有序集合（Ordered Set）的核心组成部分，提供高效的唯一元素存储、查找、插入和删除操作，支持 O(log n) 时间复杂度的平衡二叉搜索树（B-Tree 变体），适用于需要按元素有序访问的唯一集数据结构。它抽象了底层 B 树节点分配（使用 Box<Node<T>> 的平衡树结构），确保跨平台兼容性和内存安全，并通过 `std::collections::btree_set::Iter<'a, T>`、`std::collections::btree_set::Range<'a, T>` 或运行时 panic（如容量溢出或无效元素比较）显式处理错误如分配失败或元素不存在。`std::collections::BTreeSet` 强调 Rust 的所有权、借用和零成本抽象模型：BTreeSet 拥有元素，通过 insert/remove/contains/first/last/pop_first/pop_last/range/range_mut/lower_bound/upper_bound/len/is_empty/iter/iter_mut/drain_filter/retain/clear 等方法动态调整，支持泛型 T 的 Ord（元素要求有序）；集成 Iterator/IntoIterator 以懒惰消费元素，按序遍历；支持 RangeBounds 以范围查询 &T 视图。模块的设计优先有序性和平衡性，适用于排序集合、范围检查和唯一有序列表场景（对比 HashSet 的无序 O(1) 平均），并作为 BTreeSet 的扩展变体支持自定义分配器（alloc trait，1.36+）和与 BTreeMap 的互转。`std::collections::BTreeSet` 与 `std::cmp`（Ord trait 和 Ordering）、`std::alloc`（自定义分配）、`std::iter`（迭代适配器）、`std::mem`（内存交换/forget）、`std::clone`（BTreeSet Clone 深拷贝）和 `std::ops`（RangeBounds 到 Iter）深度集成，支持高级模式如范围排水迭代、原子 retain 操作和与 Vec 的有序合并。

## 1. std::collections::BTreeSet 简介

- **导入和高级结构**：除了基本导入 `use std::collections::BTreeSet;`，高级用法可包括 `use std::collections::btree_set::{Iter, Range};` 以访问迭代器，以及 `use std::cmp::Reverse;` 以反转元素序、`use std::alloc::Allocator;` 以自定义分配（alloc trait，1.36+）。模块的内部结构包括 BTreeSet 的 BTree<Node<T>>（平衡树节点 Box，高度 log n）、Ord 元素比较和 Iter 的树遍历状态机。
  - **类型详解**：
    - `BTreeSet<T, A: Allocator + Clone = Global>`：有序集合，支持 insert/remove/contains/first/last/pop_first/pop_last/range/lower_bound/upper_bound/len/is_empty/iter/drain_filter/retain/clear/union/intersection/difference/symmetric_difference/is_subset/is_superset/is_disjoint 等；泛型 A 以自定义分配。
    - `Iter<'a, T>`：有序迭代器，支持 rev() 反转、peekable 等适配。
    - `Range<'a, T>`：范围迭代器，支持 next/next_back 以双端有序遍历。
    - `IntoIter<T, A: Allocator = Global>`：消耗迭代器，支持 as_slice (no, but drain)。
  - **函数和方法扩展**：`BTreeSet::new` 创建、`BTreeSet::with_allocator` 自定义 A、`BTreeSet::split_off` 分割返回新 set、`BTreeSet::append` from other set、`BTreeSet::lower_bound`/`upper_bound` 边界查找、`BTreeSet::drain_filter` 条件排水。
  - **宏**：无，但相关如 btreeset! {v} (std::collections proposal)。
- **设计哲学扩展**：`std::collections::BTreeSet` 遵循 "balanced ordered set"，通过 B 树保持元素有序（Ord 比较），log n 操作均衡；零成本范围迭代；无负载因子（树自平衡）；drain_filter 原子减查找。BTreeSet 是 Send + Sync 如果 T 是，允许线程转移；无内置 custom ord (用 Reverse 包裹元素)。
- **跨平台详解**：节点分配用 malloc (Unix)/HeapAlloc (Windows)；对齐 Box align_of；测试差异用 CI，焦点大 Set 分配失败于低内存 OS 和 Ord 比较 endian。
- **性能详析**：insert/contains O(log n)，range O(log n + k) 于输出；drain O(n)；大 T Box 分配慢。基准用 criterion，profile 用 heaptrack 树高度。
- **常见用例扩展**：有序唯一集（时间序列去重）、范围检查（IP 地址段）、优先级集合（任务调度）、游戏有序实体、测试有序数据。
- **超级扩展概念**：与 std::cmp::Ordering 集成自定义逆序；与 std::panic::catch_unwind 安全 drop 大 Set；错误 panic 于溢出；与 indexset::IndexSet 混合有序 hash 替代；高吞吐用 btree-slab::BTreeSet slab 分配节点；与 tracing::field BTreeSet 日志；历史：从 1.0 BTreeSet 到 1.56 range_mut 优化。

## 2. 创建 BTreeSet：BTreeSet::new 和 from

`BTreeSet::new` 是入口，`from` 转换。

### 示例：基本 BTreeSet 创建（空和初始化扩展）
```rust
use std::collections::BTreeSet;

fn main() {
    let set: BTreeSet<i32> = BTreeSet::new();
    println!("空: len {}", set.len());  // 0

    let set2 = BTreeSet::from([2, 1, 3]);
    println!("from: {:?}", set2);  // {1, 2, 3} (有序)
}
```

- **解释**：`new` 空树。`from` 从数组/iter，有序插入去重。性能：O(n log n) 构建。

### 示例：With Allocator（自定义分配扩展）
```rust
use std::collections::BTreeSet;
use std::alloc::Global;

fn main() {
    let set = BTreeSet::with_allocator(Global);
}
```

- **解释**：`with_allocator` 用 A。扩展：用 jemalloc 全局优化大 Set。

### 示例：From Iter 高级（链式构建扩展）
```rust
use std::collections::BTreeSet;

fn main() {
    let set = (1..=5).collect::<BTreeSet<_>>();
    println!("collect: {:?}", set);  // {1, 2, 3, 4, 5}
}
```

- **解释**：`collect` 从 iter 构建，去重有序。

## 3. 操作 BTreeSet：Insert、Remove、Contains

操作调整树。

### 示例：Insert 和 Remove（添加移除扩展）
```rust
use std::collections::BTreeSet;

fn main() {
    let mut set = BTreeSet::new();
    let new = set.insert(1);
    println!("new: {}", new);  // true

    let removed = set.remove(&1);
    println!("removed: {}", removed);  // true
}
```

- **解释**：`insert` 返回 bool（是否新）。`remove` 返回 bool（是否存在）。性能：O(log n)。

### 示例：Contains 和 Get（检查扩展）
```rust
use std::collections::BTreeSet;

fn main() {
    let set = BTreeSet::from([1, 2, 3]);
    println!("包含 2？{}", set.contains(&2));  // true

    println!("get: {:?}", set.get(&2));  // Some(&2)
}
```

- **解释**：`contains` bool 检查。`get` &T Option。

## 4. 范围查询：Range、LowerBound

范围返回有序子视图。

### 示例：Range（查询扩展）
```rust
use std::collections::BTreeSet;

fn main() {
    let set = BTreeSet::from([1, 2, 3, 4, 5]);
    let range: Vec<&i32> = set.range(2..4).cloned().collect();
    println!("range: {:?}", range);  // [2, 3]
}
```

- **解释**：`range` 返回 Range iter，有序子集。扩展：range_mut &mut T (no, Set 无 mut)。

### 示例：LowerBound/UpperBound（边界扩展）
```rust
use std::collections::BTreeSet;

fn main() {
    let set = BTreeSet::from([1, 3, 5]);
    let lower = set.lower_bound(std::cmp::Bound::Included(&4));
    println!("lower: {:?}", lower.next());  // Some(&3) wait adjust

    let upper = set.upper_bound(std::cmp::Bound::Excluded(&3));
    println!("upper: {:?}", upper.next());  // Some(&3) adjust
}
```

- **解释**：`lower_bound` >= key 迭代器。`upper_bound` > key。

## 5. 边界操作：First、Last、Pop

访问/移除 min/max。

### 示例：First/Last/Pop（扩展）
```rust
use std::collections::BTreeSet;

fn main() {
    let mut set = BTreeSet::from([1, 2, 3]);
    println!("first: {:?}", set.first());  // Some(&1)
    println!("last: {:?}", set.last());    // Some(&3)

    let popped_first = set.pop_first();  // Some(1)
    let popped_last = set.pop_last();    // Some(3)
    println!("popped: {:?}, {:?}", popped_first, popped_last);
}
```

- **解释**：`first/last` 返回 min/max &T。`pop_first/last` 移除 min/max。

## 6. 迭代：Iter、Drain

迭代返回有序借用。

### 示例：Drain Filter（条件排水扩展）
```rust
use std::collections::BTreeSet;

fn main() {
    let mut set = BTreeSet::from([1, 2, 3, 4]);
    let drained: BTreeSet<i32> = set.drain_filter(|&x| x % 2 == 0).collect();
    println!("drained: {:?}", drained);  // {2,4}
    println!("set: {:?}", set);  // {1,3}
}
```

- **解释**：`drain_filter` 条件移除返回 T iter，有序。

### 示例：Retain（就地保留扩展）
```rust
use std::collections::BTreeSet;

fn main() {
    let mut set = BTreeSet::from([1, 2, 3, 4]);
    set.retain(|&x| x % 2 == 0);
    println!("retain: {:?}", set);  // {2,4}
}
```

- **解释**：`retain` 条件保留，移除 false。

## 7. 集合运算：Union、Intersection

运算返回有序迭代器。

### 示例：Union 和 Intersection（运算扩展）
```rust
use std::collections::BTreeSet;

fn main() {
    let set1 = BTreeSet::from([1, 2, 3]);
    let set2 = BTreeSet::from([3, 4, 5]);

    let union: BTreeSet<i32> = set1.union(&set2).cloned().collect();
    println!("union: {:?}", union);  // {1,2,3,4,5}

    let intersection: BTreeSet<i32> = set1.intersection(&set2).cloned().collect();
    println!("intersection: {:?}", intersection);  // {3}
}
```

- **解释**：`union` 返回 &T iter 并集，有序。`cloned` 转 T。扩展：difference/symmetric_difference 差/对称差。

### 示例：IsSubset 和 IsDisjoint（检查扩展）
```rust
use std::collections::BTreeSet;

fn main() {
    let set1 = BTreeSet::from([1, 2]);
    let set2 = BTreeSet::from([1, 2, 3]);
    println!("子集？{}", set1.is_subset(&set2));  // true

    let set3 = BTreeSet::from([4, 5]);
    println!("不相交？{}", set1.is_disjoint(&set3));  // true
}
```

- **解释**：`is_subset` 检查包含。`is_disjoint` 无交集。

## 8. 高级：SplitOff、Append、Custom Ord

- SplitOff：分割树。

### 示例：SplitOff（分割扩展）
```rust
use std::collections::BTreeSet;

fn main() {
    let mut set = BTreeSet::from([1, 2, 3, 4]);
    let greater = set.split_off(&3);  // set {1,2}, greater {3,4}
    println!("greater: {:?}", greater);
}
```

- **解释**：`split_off` 返回 >= elem 的新 set。原子 O(log n)。

### 示例：Append（追加扩展）
```rust
use std::collections::BTreeSet;

fn main() {
    let mut set1 = BTreeSet::from([1, 2]);
    let mut set2 = BTreeSet::from([3, 4]);
    set1.append(&mut set2);  // set1 {1,2,3,4}, set2 空
}
```

- **解释**：`append` 移动 set2 到 set1，有序合并 O(n log n) 最坏。

### 示例：Custom Ord（逆序扩展）
```rust
use std::collections::BTreeSet;
use std::cmp::Reverse;

fn main() {
    let set: BTreeSet<Reverse<i32>> = (1..=5).map(Reverse).collect();
    println!("逆序: {:?}", set);  // {Reverse(5), ..., Reverse(1)}
}
```

- **解释**：`Reverse` 反转 Ord。扩展：自定义 Ord wrapper 复杂顺序。

## 9. 最佳实践和常见陷阱

- **Set 最佳**：用 range 范围；split_off 分区；drain_filter 清。
- **性能**：O(log n) 均衡；append 合并快于逐插。
- **错误**：panic 溢出，无 try_insert。
- **安全**：T Ord 正确；range mut 无 (Set 无 mut)。
- **跨平台**：cmp 一致。
- **测试**：loom 无，但 Ord fuzz。
- **资源**：drop 释放树；clear 不回收 Box。
- **常见扩展**：
  - 无序 elem：Ord impl 正确。
  - 平衡失调：树自平衡。
  - 未找到：contains bool。
  - 内存高：用 slab 节点池。

## 10. 练习建议

1. 编写有序去重：BTreeSet insert，contains 检查。
2. 实现区间集：BTreeSet<Interval> range 查询重叠。
3. 创建自定义 Ord：用 Reverse 逆序 set。
4. 处理大 Set：split_off 测试大树分割。
5. 基准：比较 BTreeSet insert vs HashSet insert 时间，用 criterion。
6. 与 iter：用 drain_filter 条件清 set。
7. 错误框架：mock Ord panic 测试 insert 恢复。
8. 高级 app：实现日志系统：BTreeSet<Timestamp> range 查询时间窗。
