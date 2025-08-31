# std::collections::HashSet 库教程

Rust 的 `std::collections::HashSet<T, S>` 类型是标准库 `std::collections` 模块中实现散列集合（Hash Set）的核心组成部分，提供高效的唯一元素存储、查找、插入和删除操作，支持 O(1) 平均时间复杂度的动态集合，适用于去重和成员检查的场景。它抽象了底层散列桶数组（使用 Vec<Bucket<T>> 的开放寻址或链式哈希变体，Rust 使用 SipHash 默认散列器以防哈希洪水攻击），确保跨平台兼容性和内存安全，并通过 `std::collections::hash_set::Drain<'a, T>`、`std::collections::hash_set::Iter<'a, T>` 或运行时 panic（如容量溢出或无效散列）显式处理错误如分配失败或元素不存在。`std::collections::HashSet` 强调 Rust 的所有权、借用和零成本抽象模型：HashSet 拥有元素，通过 insert/remove/contains/len/is_empty/iter/drain/drain_filter/retain/clear/reserve/shrink_to_fit 等方法动态调整，支持泛型 T 的 Hash + Eq（元素要求）和 S 的 BuildHasher（自定义散列器）；集成 Iterator/IntoIterator 以懒惰消费元素；支持 intersection/union/difference/symmetric_difference 以集合运算返回迭代器。模块的设计优先高性能和灵活性，适用于唯一集、缓存键和成员测试场景（对比 BTreeSet 的有序 O(log n)），并作为 HashSet 的扩展变体支持自定义散列器如 RandomState 以安全默认。`std::collections::HashSet` 与 `std::hash`（Hash trait 和 BuildHasher）、`std::alloc`（自定义分配）、`std::iter`（迭代适配器）、`std::mem`（内存交换/forget）、`std::clone`（HashSet Clone 深拷贝）和 `std::ops`（无 Index，以防无效化）深度集成，支持高级模式如 drain_filter 条件排水、retain 就地过滤和与 HashMap 的互转。

## 1. std::collections::HashSet 简介

- **导入和高级结构**：除了基本导入 `use std::collections::HashSet;`，高级用法可包括 `use std::collections::hash_set::{Drain, Iter, IntoIter};` 以访问迭代器变体，以及 `use std::hash::{BuildHasher, RandomState};` 以自定义散列、`use std::alloc::Allocator;` 以自定义分配（alloc trait，1.36+）。模块的内部结构包括 HashSet 的 RawTable<Bucket<T>>（开放寻址哈希桶 Vec，与 HashMap 共享）、Hasher State S（默认 RandomState 以防 DoS）和 Iter 的桶遍历状态机。
  - **类型详解**：
    - `HashSet<T, S: BuildHasher = RandomState>`：散列集合，支持 insert/remove/contains/len/is_empty/capacity/iter/drain/drain_filter/retain/clear/reserve/shrink_to_fit/hasher/union/intersection/difference/symmetric_difference/is_subset/is_superset/is_disjoint 等；泛型 S 以自定义散列。
    - `Drain<'a, T>`：排水迭代器，支持 filter_map 以条件排水。
    - `Iter<'a, T>`：借用迭代器，支持 cloned 以值复制。
    - `IntoIter<T>`：消耗迭代器，支持 as_slice (no, but drain)。
    - `Intersection<'a, T, S>`/`Union<'a, T, S>`/`Difference<'a, T, S>`/`SymmetricDifference<'a, T, S>`：集合运算迭代器，支持 size_hint 以预估大小。
    - `RandomState`：默认构建器，随机种子防攻击。
  - **函数和方法扩展**：`HashSet::new` 创建、`HashSet::with_capacity` 预分配、`HashSet::with_hasher` 自定义 S、`HashSet::get` &T Option (1.76+，contains 替代)、`HashSet::leak` 'static 泄漏 (no, but drop empty)。
  - **宏**：无，但相关如 hashset! {v} (std::collections proposal)。
- **设计哲学扩展**：`std::collections::HashSet` 遵循 " robin hood hashing" 开放寻址以减冲突（负载因子 0.9），RandomState 安全默认；drain_filter 原子减查找；运算迭代器懒惰无分配；shrink_to 回收内存。HashSet 是 Send + Sync 如果 T 是，允许线程转移；无内置 ordered (用 indexset::IndexSet)。
- **跨平台详解**：散列用 SipHash 一致；分配用 malloc (Unix)/HeapAlloc (Windows)；测试差异用 CI，焦点大 Set 分配失败于低内存 OS 和散列种子随机。
- **性能详析**：insert/contains amortized O(1)，最坏 O(n) 冲突；drain O(n)；大 T memmove 慢。基准用 criterion，profile 用 heaptrack 内存高峰和 hashbrown 比较。
- **常见用例扩展**：去重集（输入验证）、权限检查（用户角色）、缓存键（唯一 ID）、游戏唯一实体、测试成员 mock。
- **超级扩展概念**：与 std::hash::Hasher 集成自定义（如 FxHasher 快）；与 std::panic::catch_unwind 安全 drop 大 Set；错误 panic 于溢出；与 hashbrown::HashSet 高性能 no_std 替代；高吞吐用 dashset::HashSet 并发；与 tracing::field HashSet 日志；历史：从 1.0 HashSet 到 1.56 drain_filter 优化。

## 2. 创建 HashSet：HashSet::new 和 with_capacity

`HashSet::new` 是入口，`with_capacity` 预分配。

### 示例：基本 HashSet 创建（空和初始化扩展）
```rust
use std::collections::HashSet;

fn main() {
    let set: HashSet<i32> = HashSet::new();
    println!("空: len {}, cap {}", set.len(), set.capacity());  // 0, 0

    let set2 = HashSet::from([1, 2, 3]);
    println!("from: {:?}", set2);
}
```

- **解释**：`new` 零桶。`from` 从数组/iter，去重插入。性能：from 预计算 cap。

### 示例：With Capacity 和 Hasher（预分配自定义扩展）
```rust
use std::collections::HashSet;
use std::hash::RandomState;

fn main() {
    let set = HashSet::with_capacity(10);
    println!("cap: {}", set.capacity());  // >=10

    let hasher = RandomState::new();
    let set_custom = HashSet::with_hasher(hasher);
}
```

- **解释**：`with_capacity` 预桶避免 rehash。`with_hasher` 自定义 S。扩展：用 BuildHasherDefault<FxHasher> 快确定性。

### 示例：From Iter 高级（链式构建扩展）
```rust
use std::collections::HashSet;

fn main() {
    let set = (1..=5).collect::<HashSet<_>>();
    println!("collect: {:?}", set);  // {1,2,3,4,5} (无序)
}
```

- **解释**：`collect` 从 iter 构建，去重。

## 3. 操作 HashSet：Insert、Remove、Contains

操作调整集合。

### 示例：Insert 和 Remove（添加移除扩展）
```rust
use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    let new = set.insert(1);
    println!("new: {}", new);  // true

    let removed = set.remove(&1);
    println!("removed: {}", removed);  // true
}
```

- **解释**：`insert` 返回 bool（是否新）。`remove` 返回 bool（是否存在）。性能：O(1) 平均。

### 示例：Contains 和 Get (1.76+)（检查扩展）
```rust
use std::collections::HashSet;

fn main() {
    let set = HashSet::from([1, 2]);
    println!("包含 1？{}", set.contains(&1));  // true

    // 1.76+ get &T Option
    println!("get: {:?}", set.get(&1));  // Some(&1)
}
```

- **解释**：`contains` bool 检查。`get` &T Option。扩展：use raw_entry (HashMap like, future Set)。

## 4. 集合运算：Union、Intersection

运算返回迭代器。

### 示例：Union 和 Intersection（运算扩展）
```rust
use std::collections::HashSet;

fn main() {
    let set1 = HashSet::from([1, 2, 3]);
    let set2 = HashSet::from([3, 4, 5]);

    let union: HashSet<&i32> = set1.union(&set2).cloned().collect();
    println!("union: {:?}", union);  // {1,2,3,4,5}

    let intersection: HashSet<&i32> = set1.intersection(&set2).cloned().collect();
    println!("intersection: {:?}", intersection);  // {3}
}
```

- **解释**：`union` 返回 &T iter 并集。`cloned` 转 T。扩展：difference/symmetric_difference 差/对称差。

### 示例：IsSubset 和 IsDisjoint（检查扩展）
```rust
use std::collections::HashSet;

fn main() {
    let set1 = HashSet::from([1, 2]);
    let set2 = HashSet::from([1, 2, 3]);
    println!("子集？{}", set1.is_subset(&set2));  // true

    let set3 = HashSet::from([4, 5]);
    println!("不相交？{}", set1.is_disjoint(&set3));  // true
}
```

- **解释**：`is_subset` 检查包含。`is_disjoint` 无交集。

## 4. 迭代：Iter、Drain

迭代返回借用。

### 示例：Drain Filter（条件排水扩展）
```rust
use std::collections::HashSet;

fn main() {
    let mut set = HashSet::from([1, 2, 3, 4]);
    let drained: HashSet<i32> = set.drain_filter(|&x| x % 2 == 0).collect();
    println!("drained: {:?}", drained);  // {2,4}
    println!("set: {:?}", set);  // {1,3}
}
```

- **解释**：`drain_filter` 条件移除返回 T iter。

### 示例：Retain（就地保留扩展）
```rust
use std::collections::HashSet;

fn main() {
    let mut set = HashSet::from([1, 2, 3, 4]);
    set.retain(|&x| x % 2 == 0);
    println!("retain: {:?}", set);  // {2,4}
}
```

- **解释**：`retain` 条件保留，移除 false。

## 5. 容量管理：Reserve、Shrink

控制桶数。

### 示例：Reserve 和 Shrink（管理扩展）
```rust
use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    set.reserve(10);  // 桶 >=10
    set.extend(1..=5);
    set.shrink_to_fit();  // 桶~5
    println!("cap: {}", set.capacity());
}
```

- **解释**：`reserve` 确保 cap >= len + add。`shrink_to_fit` 最小化。

### 示例：Load Factor 分析（性能扩展）
```rust
use std::collections::HashSet;

fn main() {
    let mut set = HashSet::with_capacity(100);
    // 负载因子 len / cap ~0.9 最坏 rehash
}
```

- **解释**：负载 >0.9 rehash O(n)。优化：reserve 超预期 len。

## 6. 高级：Custom Hasher、Alloc

- Custom Hasher：防攻击。

### 示例：Custom Hasher（扩展）
```rust
use std::collections::HashSet;
use std::hash::RandomState;

fn main() {
    let hasher = RandomState::new();
    let set: HashSet<i32, RandomState> = HashSet::with_hasher(hasher);
}
```

- **解释**：`with_hasher` 自定义。扩展：用 FxHasher (fxhash crate) 快确定性。

### 示例：With Allocator（分配器扩展）
```rust
use std::collections::HashSet;
use std::alloc::Global;

fn main() {
    let set = HashSet::with_allocator(Global);
}
```

- **解释**：`with_allocator` 用 A (future full support)。

## 7. 最佳实践和常见陷阱

- **Set 最佳**：with_capacity 预分配；retain 就地过滤；drain_filter 条件清。
- **性能**：自定义 hasher 减冲突；shrink 回收。
- **错误**：panic 溢出，用 try_reserve (HashMap like)。
- **安全**：T Hash+Eq 正确。
- **跨平台**：hash 一致。
- **测试**：miri UB；fuzz insert/remove。
- **资源**：drop 释放；clear 不缩 cap。
- **常见扩展**：
  - 冲突：custom hasher。
  - OOM：reserve 处理。
  - 无效 T：Hash impl 正确。
  - rehash 慢：reserve 避免。

## 8. 练习建议

1. 编写去重：HashSet insert，contains 检查。
2. 实现 LRU set：HashSet + LinkedList 双结构。
3. 创建自定义 hasher：FxHash 基准比较。
4. 处理大 Set：reserve 测试 OOM 恢复。
5. 基准：比较 insert vs BTreeSet insert 时间，用 criterion。
6. 与 iter：用 drain_filter 条件清 set。
7. 错误框架：mock alloc fail 测试 reserve。
8. 高级 app：实现唯一 ID 集：HashSet<Id> contains 查询。
