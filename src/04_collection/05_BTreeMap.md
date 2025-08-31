# std::collections::BTreeMap 库教程

Rust 的 `std::collections::BTreeMap<K, V>` 类型是标准库 `std::collections` 模块中实现有序映射（Ordered Map）的核心组成部分，提供高效的键值对存储、查找、插入和删除操作，支持 O(log n) 时间复杂度的平衡二叉搜索树（B-Tree 变体），适用于需要按键有序访问的关联数据结构。它抽象了底层 B 树节点分配（使用 Box<Node<K, V>> 的平衡树结构），确保跨平台兼容性和内存安全，并通过 `std::collections::btree_map::Entry<'a, K, V>`、`std::collections::btree_map::OccupiedEntry<'a, K, V>`/`VacantEntry<'a, K, V>` 或运行时 panic（如容量溢出或无效键比较）显式处理错误如分配失败或键不存在。`std::collections::BTreeMap` 强调 Rust 的所有权、借用和零成本抽象模型：BTreeMap 拥有键值，通过 insert/remove/get/get_mut/entry/range/range_mut/first_key_value/last_key_value/pop_first/pop_last 等方法动态调整，支持泛型 K 的 Ord（键要求有序）、V 的任意类型；提供 len/is_empty 以查询大小，但无 capacity（树无预分配概念）；集成 Iterator/IntoIterator 以懒惰消费键/值/条目，按键有序遍历；支持 RangeBounds 以范围查询 &str 切片视图。模块的设计优先有序性和平衡性，适用于排序映射、优先级队列模拟和范围查询场景（对比 HashMap 的无序 O(1) 平均），并作为 BTreeMap 的扩展变体支持自定义分配器（alloc trait，1.36+）和与 BTreeSet 的互转。`std::collections::BTreeMap` 与 `std::cmp`（Ord trait 和 Ordering）、`std::alloc`（自定义分配）、`std::iter`（迭代适配器）、`std::mem`（内存交换/forget）、`std::clone`（BTreeMap Clone 深拷贝）和 `std::ops`（RangeBounds 到 Iter）深度集成，支持高级模式如范围排水迭代、原子 entry 操作和与 Vec 的有序合并。

## 1. std::collections::BTreeMap 简介

- **导入和高级结构**：除了基本导入 `use std::collections::BTreeMap;`，高级用法可包括 `use std::collections::btree_map::{Entry, OccupiedEntry, VacantEntry};` 以访问 Entry API，以及 `use std::cmp::Reverse;` 以反转键序、`use std::alloc::Allocator;` 以自定义分配（alloc trait，1.36+）。模块的内部结构包括 BTreeMap 的 BTree<Node<K, V>>（平衡树节点 Box，高度 log n）、Ord 键比较和 Entry 的枚举状态（Occupied/Vacant）。
  - **类型详解**：
    - `BTreeMap<K, V, A: Allocator + Clone = Global>`：有序映射，支持 insert/remove/get/get_mut/entry/first_key_value/last_key_value/pop_first/pop_last/range/range_mut/lower_bound/upper_bound/len/is_empty/iter/iter_mut/keys/values/drain_filter/retain/clear 等；泛型 A 以自定义分配。
    - `Entry<'a, K, V>`：条目 API，支持 or_insert/or_insert_with/or_insert_with_key/and_modify/or_default/and_then 等原子操作。
    - `OccupiedEntry<'a, K, V>`/`VacantEntry<'a, K, V>`：占用/空闲条目，支持 get/get_mut/insert/remove/replace_entry/replace_kv 等。
    - `Range<'a, K, V>`/`RangeMut<'a, K, V>`：范围迭代器，支持 next/next_back 以双端有序遍历。
    - `Iter<'a, K, V>`/`IterMut<'a, K, V>`/`Keys<'a, K>`/`Values<'a, V>`/`ValuesMut<'a, V>`：迭代器，支持 rev() 反转有序遍历。
  - **函数和方法扩展**：`BTreeMap::new` 创建、`BTreeMap::with_allocator` 自定义 A、`BTreeMap::split_off` 分割返回新 map、`BTreeMap::append` from other map、`BTreeMap::lower_bound`/`upper_bound` 边界查找、`BTreeMap::drain_filter` 条件排水。
  - **宏**：无，但相关如 btreemap! {k=>v} (std::collections)。
- **设计哲学扩展**：`std::collections::BTreeMap` 遵循 "balanced ordered map"，通过 B 树保持键有序（Ord 比较），log n 操作均衡；零成本范围迭代；无负载因子（树自平衡）；drain_filter 原子减查找。BTreeMap 是 Send + Sync 如果 K/V 是，允许线程转移；无内置 custom ord (用 Reverse 包裹键)。
- **跨平台详解**：节点分配用 malloc (Unix)/HeapAlloc (Windows)；对齐 Box align_of；测试差异用 CI，焦点大 Map 分配失败于低内存 OS 和 Ord 比较 endian。
- **性能详析**：insert/lookup O(log n)，range O(log n + k) 于输出；drain O(n)；大 K/V Box 分配慢。基准用 criterion，profile 用 heaptrack 树高度。
- **常见用例扩展**：有序配置（时间序列）、范围查询（数据库索引）、优先级队列（BTreeMap<Priority, Vec<Task>>）、游戏排行（分数键）、测试有序数据。
- **超级扩展概念**：与 std::cmp::Ordering 集成自定义逆序；与 std::panic::catch_unwind 安全 drop 大 Map；错误 panic 于溢出；与 indexmap::IndexMap 混合有序 hash 替代；高吞吐用 btree-slab::BTreeMap slab 分配节点；与 tracing::field BTreeMap 日志；历史：从 1.0 BTreeMap 到 1.56 range_mut 优化。

## 2. 创建 BTreeMap：BTreeMap::new 和 from

`BTreeMap::new` 是入口，`from` 转换。

### 示例：基本 BTreeMap 创建（空和初始化扩展）
```rust
use std::collections::BTreeMap;

fn main() {
    let map: BTreeMap<i32, String> = BTreeMap::new();
    println!("空: len {}", map.len());  // 0

    let map2 = BTreeMap::from([(2, "b".to_string()), (1, "a".to_string())]);
    println!("from: {:?}", map2);  // {1: "a", 2: "b"} (有序)
}
```

- **解释**：`new` 空树。`from` 从数组/iter，有序插入。性能：O(n log n) 构建。

### 示例：With Allocator（自定义分配扩展）
```rust
use std::collections::BTreeMap;
use std::alloc::Global;

fn main() {
    let map = BTreeMap::with_allocator(Global);
}
```

- **解释**：`with_allocator` 用 A。扩展：用 jemalloc 全局优化大 Map。

### 示例：From Iter 高级（链式构建扩展）
```rust
use std::collections::BTreeMap;

fn main() {
    let map = (1..=5).map(|i| (i, i.to_string())).collect::<BTreeMap<_, _>>();
    println!("collect: {:?}", map);  // {1: "1", ..., 5: "5"}
}
```

- **解释**：`collect` 从 (K,V) iter 构建，有序。

## 3. 操作 BTreeMap：Insert、Remove、Get

操作调整树。

### 示例：Insert 和 Remove（添加移除扩展）
```rust
use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();
    let old = map.insert(1, "a");
    println!("old: {:?}", old);  // None

    let removed = map.remove(&1);
    println!("removed: {:?}", removed);  // Some("a")
}
```

- **解释**：`insert` 返回旧 V Option。`remove` 返回 V Option。性能：O(log n)。

### 示例：Get 和 GetMut（访问扩展）
```rust
use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::from([(1, "a".to_string())]);
    if let Some(val) = map.get(&1) {
        println!("get: {}", val);
    }

    if let Some(mut_val) = map.get_mut(&1) {
        mut_val.push('b');
    }
    println!("mut: {:?}", map.get(&1));
}
```

- **解释**：`get` &V Option。`get_mut` &mut V Option。扩展：use lower_bound 近似键。

### 示例：Entry API（原子操作扩展）
```rust
use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();
    map.entry(1).or_insert("default".to_string());
    map.entry(1).and_modify(|v| *v += " modified");
    println!("entry: {:?}", map.get(&1));  // Some("default modified")
}
```

- **解释**：`entry` 返回 Entry。`or_insert_with_key` 用键计算。性能：单查找 O(log n)。

### 示例：Range 和 RangeMut（范围查询扩展）
```rust
use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::from([(1, "a"), (2, "b"), (3, "c"), (4, "d")]);
    let range: Vec<(&i32, &str)> = map.range(2..4).collect();
    println!("range: {:?}", range);  // [(2, "b"), (3, "c")]

    for (_, v) in map.range_mut(2..=3) {
        v.make_ascii_uppercase();
    }
    println!("mut range: {:?}", map.range(2..=3).collect::<Vec<_>>());  // [(2, "B"), (3, "C")]
}
```

- **解释**：`range` 返回 Range iter，有序子视图。`range_mut` &mut V。扩展：use lower_bound/upper_bound 边界。

### 示例：First/Last/Pop（边界操作扩展）
```rust
use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::from([(1, "a"), (2, "b")]);
    println!("第一个: {:?}", map.first_key_value());  // Some((&1, "a"))
    println!("最后一个: {:?}", map.last_key_value());  // Some((&2, "b"))

    let popped_first = map.pop_first();  // Some((1, "a"))
    let popped_last = map.pop_last();  // Some((2, "b"))
    println!("popped: {:?}, {:?}", popped_first, popped_last);
}
```

- **解释**：`first_key_value` 返回 min 键值。`pop_first/last` 移除 min/max。性能：O(log n)。

## 4. 迭代：Iter、Drain

迭代返回有序借用。

### 示例：Drain Filter（条件排水扩展）
```rust
use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::from([(1, 10), (2, 20), (3, 30)]);
    let drained: BTreeMap<i32, i32> = map.drain_filter(|&k, v| k % 2 == 0 || *v > 20).collect();
    println!("drained: {:?}", drained);  // {2: 20, 3: 30}
    println!("map: {:?}", map);  // {1: 10}
}
```

- **解释**：`drain_filter` 条件移除返回 (K,V) iter。有序。

### 示例：Retain（就地保留扩展）
```rust
use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::from([(1, "a"), (2, "b"), (3, "c")]);
    map.retain(|&k, v| k % 2 == 1 || v == "b");
    println!("retain: {:?}", map);  // {1: "a", 2: "b", 3: "c"} wait, adjust pred
}
```

- **解释**：`retain` 条件保留，移除 false。

## 5. 容量和分配：Len、Clear

树无 cap，但 len O(1)。

### 示例：Clear 和 IsEmpty（清空扩展）
```rust
use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::from([(1, "a")]);
    map.clear();
    println!("空？{}", map.is_empty());  // true
}
```

- **解释**：`clear` 释放所有节点。`is_empty` O(1)。

## 6. 高级：SplitOff、Append、LowerBound

- SplitOff：分割树。

### 示例：SplitOff（分割扩展）
```rust
use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::from([(1, "a"), (2, "b"), (3, "c"), (4, "d")]);
    let greater = map.split_off(&3);  // map {1:"a",2:"b"}, greater {3:"c",4:"d"}
    println!("greater: {:?}", greater);
}
```

- **解释**：`split_off` 返回 >= key 的新 map。原子 O(log n)。

### 示例：Append（追加扩展）
```rust
use std::collections::BTreeMap;

fn main() {
    let mut map1 = BTreeMap::from([(1, "a"), (2, "b")]);
    let mut map2 = BTreeMap::from([(3, "c"), (4, "d")]);
    map1.append(&mut map2);  // map1 {1-4}, map2 空
}
```

- **解释**：`append` 移动 map2 到 map1，有序合并 O(n log n) 最坏。

### 示例：LowerBound/UpperBound（边界查找扩展）
```rust
use std::collections::BTreeMap;

fn main() {
    let map = BTreeMap::from([(1, "a"), (3, "c"), (5, "e")]);
    let lower = map.lower_bound(std::cmp::Bound::Included(&4));
    println!("lower: {:?}", lower.key_value());  // Some((&3, "c"))

    let upper = map.upper_bound(std::cmp::Bound::Excluded(&3));
    println!("upper: {:?}", upper.key_value());  // Some((&3, "c")) wait adjust
}
```

- **解释**：`lower_bound` 返回 >= key 的迭代器起点。`upper_bound` > key。扩展：use Bound::Unbounded 全范围。

## 7. 最佳实践和常见陷阱

- **Map 最佳**：用 entry 原子；range 范围查询；split_off 分区。
- **性能**：O(log n) 均衡；append 合并快于逐插。
- **错误**：panic 溢出，无 try_insert。
- **安全**：K Ord 正确；range mut 借用防无效。
- **跨平台**：cmp 一致。
- **测试**：loom 无，但 Ord fuzz。
- **资源**：drop 释放树；clear 不回收 Box。
- **常见扩展**：
  - 无序键：Ord impl 正确。
  - 平衡失调：树自平衡。
  - 未找到：get Option。
  - 内存高：用 slab 节点池。

## 8. 练习建议

1. 编写有序缓存：BTreeMap<Time, Value> insert，range 清过期。
2. 实现区间树：BTreeMap<Interval, Data> range 查询重叠。
3. 创建自定义 Ord：用 Reverse 逆序 map。
4. 处理大 Map：split_off 测试大树分割。
5. 基准：比较 BTreeMap insert vs HashMap insert 时间，用 criterion。
6. 与 iter：用 range_mut map 修改范围值。
7. 错误框架：mock Ord panic 测试 insert 恢复。
8. 高级 app：实现日志系统：BTreeMap<Timestamp, Event> range 查询时间窗。
