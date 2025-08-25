# std::collections::HashMap 库教程

Rust 的 `std::collections::HashMap<K, V, S>` 类型是标准库 `std::collections` 模块中实现散列表（Hash Table）的核心组成部分，提供高效的键值对存储、查找、插入和删除操作，支持 O(1) 平均时间复杂度的动态映射，适用于唯一键的关联数据结构。它抽象了底层散列桶数组（使用 Vec<Bucket<K, V>> 的开放寻址或链式哈希变体，Rust 使用 SipHash 默认散列器以防哈希洪水攻击），确保跨平台兼容性和内存安全，并通过 `std::collections::hash_map::Entry` API、`std::collections::hash_map::OccupiedEntry`/`VacantEntry` 或运行时 panic（如容量溢出或无效散列）显式处理错误如分配失败、键不存在或散列冲突。`std::collections::HashMap` 强调 Rust 的所有权、借用和零成本抽象模型：HashMap 拥有键值，通过 insert/remove/get/get_mut/entry 等方法动态调整，支持泛型 K 的 Hash + Eq（键要求）、V 的任意类型和 S 的 BuildHasher（自定义散列器）；提供 capacity/reserve/shrink_to_fit 以控制内存使用；集成 Iterator/IntoIterator 以懒惰消费键/值/条目；支持 RawEntry API 以低级访问避免不必要散列计算。模块的设计优先高性能和灵活性，适用于缓存、配置映射和数据索引场景（对比 BTreeMap 的有序键），并作为 HashMap 的扩展变体支持自定义散列器如 RandomState 以安全默认。`std::collections::HashMap` 与 `std::hash`（Hash trait 和 BuildHasher）、`std::alloc`（自定义分配）、`std::iter`（迭代适配器）、`std::mem`（内存交换/forget）、`std::clone`（HashMap Clone 深拷贝）和 `std::ops`（Index 到 &V 但无 mut，以防无效化）深度集成，支持高级模式如 raw_entry 原子操作、drain_filter 条件排水和与 HashSet 的互转。

## 1. std::collections::HashMap 简介

- **导入和高级结构**：除了基本导入 `use std::collections::HashMap;`，高级用法可包括 `use std::collections::hash_map::{Entry, OccupiedEntry, VacantEntry, RawEntryMut};` 以访问 Entry API，以及 `use std::hash::{BuildHasher, RandomState};` 以自定义散列、`use std::alloc::Allocator;` 以自定义分配（alloc trait，1.36+）。模块的内部结构包括 HashMap 的 RawTable<Bucket<K, V>>（开放寻址哈希桶 Vec）、Hasher State S（默认 RandomState 以防 DoS）和 Entry 的枚举状态（Occupied/Vacant）。
  - **类型详解**：
    - `HashMap<K, V, S: BuildHasher = RandomState>`：散列表，支持 insert/remove/get/get_mut/entry/raw_entry/raw_entry_mut/len/is_empty/capacity/keys/values/iter/iter_mut/drain/drain_filter/retain/clear/reserve/shrink_to_fit/hasher 等；泛型 S 以自定义散列。
    - `Entry<'a, K, V>`：条目 API，支持 or_insert/or_insert_with/or_insert_with_key/and_modify/or_default/vacant/occupied 等原子操作。
    - `OccupiedEntry<'a, K, V>`/`VacantEntry<'a, K, V>`：占用/空闲条目，支持 get/get_mut/insert/remove/replace_entry 等。
    - `RawEntryMut<'a, K, V, S>`/`RawEntryBuilderMut<'a, K, V, S>`：低级 raw 条目，支持 or_insert/with/insert/remove 等避免额外散列。
    - `Iter<'a, K, V>`/`IterMut<'a, K, V>`/`Keys<'a, K>`/`Values<'a, V>`/`ValuesMut<'a, V>`/`Drain<'a, K, V>`：迭代器，支持 filter_map 以条件排水。
    - `RandomState`：默认构建器，随机种子防攻击。
  - **函数和方法扩展**：`HashMap::new` 创建、`HashMap::with_capacity` 预分配、`HashMap::with_hasher` 自定义 S、`HashMap::raw_entry_mut` 低级、`HashMap::leak` 'static 泄漏 (no, but drop empty)。
  - **宏**：无，但相关如 hashmap! {k=>v} (std::collections)。
- **设计哲学扩展**：`std::collections::HashMap` 遵循 " robin hood hashing" 开放寻址以减冲突（负载因子 0.9），RandomState 安全默认；Entry API 原子减查找；raw_entry 优避免双散列；shrink_to 回收内存。HashMap 是 Send + Sync 如果 K/V/S 是，允许线程转移；无内置 ordered (用 indexmap)。
- **跨平台详解**：散列用 SipHash 一致；分配用 malloc (Unix)/HeapAlloc (Windows)；测试差异用 CI，焦点大 Map 分配失败于低内存 OS 和散列种子随机。
- **性能详析**：insert/lookup amortized O(1)，最坏 O(n) 冲突；reserve O(n) rehash；raw_entry O(1) 查找；大 K/V memmove 慢。基准用 criterion，profile 用 heaptrack 内存高峰和 hashbrown 比较。
- **常见用例扩展**：缓存（TTL HashMap）、配置键值（CLI 解析）、索引映射（数据库查询）、游戏状态（实体 ID 到对象）、测试数据 mock。
- **超级扩展概念**：与 std::hash::Hasher 集成自定义（如 FxHasher 快）；与 std::panic::catch_unwind 安全 drop 大 Map；错误 panic 于溢出；与 hashbrown::HashMap 高性能 no_std 替代；高吞吐用 dashmap::HashMap 并发；与 tracing::field HashMap 日志；历史：从 1.0 HashMap 到 1.56 raw_entry 优化。

## 2. 创建 HashMap：HashMap::new 和 with_capacity

`HashMap::new` 是入口，`with_capacity` 预分配。

### 示例：基本 HashMap 创建（空和初始化扩展）
```rust
use std::collections::HashMap;

fn main() {
    let map: HashMap<i32, String> = HashMap::new();
    println!("空: len {}, cap {}", map.len(), map.capacity());  // 0, 0

    let map2 = HashMap::from([(1, "a".to_string()), (2, "b".to_string())]);
    println!("from: {:?}", map2);
}
```

- **解释**：`new` 零桶。`from` 从数组/iter。性能：from 预计算 cap。

### 示例：With Capacity 和 Hasher（预分配自定义扩展）
```rust
use std::collections::HashMap;
use std::hash::RandomState;

fn main() {
    let map = HashMap::with_capacity(10);
    println!("cap: {}", map.capacity());  // >=10

    let hasher = RandomState::new();
    let map_custom = HashMap::with_hasher(hasher);
}
```

- **解释**：`with_capacity` 预桶避免 rehash。`with_hasher` 自定义 S。扩展：用 BuildHasherDefault<FxHasher> 快散列。

### 示例：From Iter 高级（链式构建扩展）
```rust
use std::collections::HashMap;

fn main() {
    let map = (1..=5).map(|i| (i, i.to_string())).collect::<HashMap<_, _>>();
    println!("collect: {:?}", map);
}
```

- **解释**：`collect` 从 (K,V) iter 构建。扩展：用 extend 追加 iter。

## 3. 操作 HashMap：Insert、Remove、Get

操作调整映射。

### 示例：Insert 和 Remove（添加移除扩展）
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    let old = map.insert(1, "a");
    println!("old: {:?}", old);  // None

    let removed = map.remove(&1);
    println!("removed: {:?}", removed);  // Some("a")
}
```

- **解释**：`insert` 返回旧 V Option。`remove` 返回 V Option。性能：O(1) 平均。

### 示例：Get 和 GetMut（访问扩展）
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::from([(1, "a".to_string())]);
    if let Some(val) = map.get(&1) {
        println!("get: {}", val);
    }

    if let Some(mut_val) = map.get_mut(&1) {
        mut_val.push('b');
    }
    println!("mut: {:?}", map.get(&1));
}
```

- **解释**：`get` &V Option。`get_mut` &mut V Option。扩展：use raw_entry 避免借用 K。

### 示例：Entry API（原子操作扩展）
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.entry(1).or_insert("default".to_string());
    map.entry(1).and_modify(|v| *v += " modified");
    println!("entry: {:?}", map.get(&1));  // Some("default modified")
}
```

- **解释**：`entry` 返回 Entry。`or_insert` 如果空插入。`and_modify` 修改存在。性能：单散列。

### 示例：RawEntry API（低级扩展）
```rust
use std::collections::HashMap;
use std::collections::hash_map::RawEntryMut;

fn main() {
    let mut map = HashMap::new();
    let hasher = map.hasher();
    match map.raw_entry_mut().from_key_hashed_nocheck(0, &1) {
        RawEntryMut::Occupied(mut o) => { o.insert("val".to_string()); }
        RawEntryMut::Vacant(v) => { v.insert(1, "val".to_string()); }
    }
}
```

- **解释**：`raw_entry_mut` 避免键借用/散列。`from_key_hashed_nocheck` 用预计算 hash。扩展：用 build_hasher 自定义。

## 4. 迭代和访问：Iter、Keys、Values

迭代返回借用。

### 示例：Iter 和 MutIter（借用扩展）
```rust
use std::collections::HashMap;

fn main() {
    let map = HashMap::from([(1, "a"), (2, "b")]);
    let sum_len: usize = map.iter().map(|(_, v)| v.len()).sum();
    println!("sum_len: {}", sum_len);

    let mut map_mut = map;
    map_mut.iter_mut().for_each(|(_, v)| v.make_ascii_uppercase());
}
```

- **解释**：`iter` (&K, &V)，`iter_mut` (&K, &mut V)。扩展：use drain 消耗迭代。

### 示例：Keys 和 Values（专用迭代扩展）
```rust
use std::collections::HashMap;

fn main() {
    let map = HashMap::from([(1, "a"), (2, "b")]);
    let keys: Vec<&i32> = map.keys().cloned().collect();
    println!("keys: {:?}", keys);

    let mut values_mut = map.values_mut();
    if let Some(v) = values_mut.next() {
        v.make_ascii_uppercase();
    }
}
```

- **解释**：`keys` &K iter。`values_mut` &mut V iter。扩展：use drain_filter 条件消耗。

## 4. 高级：Drain、RawEntry、Hasher

- Drain：移除迭代。

### 示例：Drain Filter（条件排水扩展）
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::from([(1, 10), (2, 20), (3, 30)]);
    let drained: HashMap<i32, i32> = map.drain_filter(|&k, v| k % 2 == 0 || *v > 20).collect();
    println!("drained: {:?}", drained);  // (2,20), (3,30)
    println!("map: {:?}", map);  // (1,10)
}
```

- **解释**：`drain_filter` 条件移除返回 (K,V) iter。扩展：use retain 就地保留。

### 示例：RawEntry Mut（低级插入扩展）
```rust
use std::collections::HashMap;
use std::collections::hash_map::RawEntryMut;

fn main() {
    let mut map = HashMap::new();
    let hash = map.hasher().hash_one(&1);
    match map.raw_entry_mut().from_key_hashed_nocheck(hash, &1) {
        RawEntryMut::Occupied(o) => println!("存在: {}", o.get()),
        RawEntryMut::Vacant(v) => { v.insert(1, "val"); }
    }
}
```

- **解释**：`raw_entry_mut` 低级，避免 K 借用。`from_key_hashed_nocheck` 用预 hash。扩展：build_raw_entry 用于复杂。

### 示例：Custom Hasher（防攻击扩展）
```rust
use std::collections::HashMap;
use std::hash::{BuildHasher, RandomState};

fn main() {
    let hasher = RandomState::new();
    let map: HashMap<i32, String, RandomState> = HashMap::with_hasher(hasher);
}
```

- **解释**：`with_hasher` 自定义。扩展：用 FxHasher (fxhash crate) 快确定性。

## 5. 容量管理：Reserve、Shrink

控制桶数。

### 示例：Reserve 和 Shrink（管理扩展）
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.reserve(10);  // 桶 >=10
    map.extend((1..=5).map(|i| (i, i.to_string())));
    map.shrink_to_fit();  // 桶~5
    println!("cap: {}", map.capacity());
}
```

- **解释**：`reserve` 确保 cap >= len + add。`shrink_to_fit` 最小化。

### 示例：Load Factor 分析（性能扩展）
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::with_capacity(100);
    // 负载因子 len / cap ~0.9 最坏 rehash
}
```

- **解释**：负载 >0.9 rehash O(n)。优化：reserve 超预期 len。

## 6. 迭代：Iter、Drain

迭代返回借用。

### 示例：Drain（消耗扩展）
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::from([(1, "a"), (2, "b")]);
    let drained: Vec<(i32, String)> = map.drain().collect();
    println!("drained: {:?}", drained);
    println!("map 空: {}", map.is_empty());
}
```

- **解释**：`drain` (K,V) iter 清 map。扩展：drain_filter 条件。

## 7. 最佳实践和常见陷阱

- **Map 最佳**：with_capacity 预分配；entry 原子操作；raw_entry 低级优化。
- **性能**：自定义 hasher 减冲突；shrink 回收。
- **错误**：panic 溢出，用 try_reserve。
- **安全**：K Hash+Eq 正确；raw 避免双散列。
- **跨平台**：hash 一致。
- **测试**：miri UB；fuzz insert/remove。
- **资源**：drop 释放；clear 不缩 cap。
- **常见扩展**：
  - 冲突：custom hasher。
  - OOM：try_reserve 处理。
  - 无效 K：Hash impl 正确。
  - rehash 慢：reserve 避免。

## 8. 练习建议

1. 编写缓存：HashMap<K, V> insert，entry or_insert。
2. 实现 LRU：HashMap + LinkedList 双结构。
3. 创建自定义 hasher：FxHash 基准比较。
4. 处理大 Map：try_reserve 测试 OOM 恢复。
5. 基准：比较 insert vs raw_entry 时间，用 criterion。
6. 与 iter：用 drain_filter 条件清 map。
7. 错误框架：mock alloc fail 测试 try_reserve。
8. 高级 app：实现 DB 索引：HashMap<Key, Vec<Value>> query。

