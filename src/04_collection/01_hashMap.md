# Rust HashMap 教程

Rust 中的 `HashMap<K, V>` 是标准库 `std::collections` 中的哈希映射类型，用于存储键值对。它使用哈希函数将键映射到值，支持快速查找、插入和删除。`HashMap` 要求键类型 `K` 实现 `Hash` 和 `Eq` trait，值 `V` 可以是任意类型。`HashMap` 是无序的（不像 `BTreeMap` 有序），适合需要 O(1) 平均操作的场景。Rust 的 HashMap 是安全的，默认使用 SipHash 以防哈希洪水攻击。

本教程从基础开始，逐步深入，包含代码示例和解释。假设你已熟悉 Rust 的基本语法（如所有权、借用）和集合（如 Vec）。每个示例后，我会解释关键点。如果你有 Rust 环境，可以复制代码运行测试。教程基于 Rust 1.80+（截至 2025 年，HashMap 核心未变，但有性能优化如更快的哈希构建器）。

## 1. HashMap 简介

- **导入**：`use std::collections::HashMap;`
- **特点**：
    - 动态大小：可以增长或收缩。
    - 键唯一：重复键会覆盖旧值。
    - 无序：迭代顺序不保证。
    - 所有权：插入时，键和值的所有权转移到 HashMap。
- **与 Vec 的比较**：Vec 用索引，HashMap 用任意键。

### 示例：基本创建
```rust
use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);  // 输出: {"Yellow": 50, "Blue": 10}（顺序可能不同）
}
```

- **解释**：`HashMap::new()` 创建空映射。`insert` 添加键值对，键是 String（拥有所有权），值是 i32。打印用 `{:?}` 因为 HashMap 实现 Debug。

## 2. 创建和插入

- **new()**：空 HashMap。
- **with_capacity(n)**：预分配容量，提高性能。
- **insert(k, v)**：插入或更新，返回旧值（Option<V>）。
- **从迭代器创建**：用 `collect()` 从 (K, V) 元组。

### 示例：多种创建方式
```rust
fn main() {
    // 从 new 和 insert
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);

    // 从元组迭代器
    let teams = vec![("Blue", 10), ("Yellow", 50)];
    let scores: HashMap<_, _> = teams.into_iter().collect();
    println!("{:?}", scores);

    // 更新现有键
    let old = map.insert("one", 11);  // 返回 Some(1)
    println!("旧值: {:?}", old);
}
```

- **解释**：`collect()` 推断类型，用 `_` 占位。insert 返回旧值，便于检查是否存在。

### Entry API：高效插入
```rust
fn main() {
    let mut map = HashMap::new();
    map.entry("one").or_insert(1);  // 如果不存在，插入 1
    map.entry("one").or_insert(2);  // 已存在，不插入

    if let Some(value) = map.entry("two").or_insert_with(|| 2 * 2) {  // 懒惰插入
        *value += 1;  // 修改现有值
    }
    println!("{:?}", map);  // {"one": 1, "two": 5}
}
```

- **解释**：`entry` 返回 Entry（Occupied 或 Vacant）。`or_insert` 插入默认，`or_insert_with` 用闭包懒惰计算。适合避免不必要计算。

## 3. 访问和更新

- **get(&k)**：返回 Option<&V>（借用）。
- **get_mut(&k)**：返回 Option<&mut V>（可变借用）。
- **contains_key(&k)**：检查键是否存在。
- **len()**：元素数量。
- **is_empty()**：是否空。

### 示例：访问值
```rust
fn main() {
    let mut map = HashMap::new();
    map.insert("key", 42);

    if let Some(&value) = map.get("key") {
        println!("值: {}", value);  // 输出: 值: 42
    }

    if let Some(value_mut) = map.get_mut("key") {
        *value_mut += 1;
    }
    println!("{:?}", map.get("key"));  // Some(43)
}
```

- **解释**：get 返回引用 Option。解引用用 `&value`（Copy 类型）或 `value`（非 Copy）。get_mut 允许修改。

## 4. 删除和清空

- **remove(&k)**：删除键，返回 Option<V>。
- **clear()**：清空所有元素。
- **remove_entry(&k)**：返回 Option<(K, V)>。

### 示例：删除
```rust
fn main() {
    let mut map = HashMap::new();
    map.insert("key", 42);
    let removed = map.remove("key");  // Some(42)
    println!("移除: {:?}", removed);

    map.insert("another", 100);
    map.clear();
    println!("空？{}", map.is_empty());  // true
}
```

- **解释**：remove 返回拥有值的所有权。clear 保留容量。

## 5. 迭代 HashMap

- **iter()**：&(&K, &V)
- **iter_mut()**：&(&K, &mut V)
- **into_iter()**：(K, V)，消耗 HashMap
- **keys() / values()**：迭代键或值。

### 示例：迭代
```rust
fn main() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);

    for (key, value) in &map {  // 借用迭代
        println!("{}: {}", key, value);
    }

    for (key, value) in map.iter_mut() {
        *value += 10;
    }

    let sum: i32 = map.values().sum();  // 迭代值
    println!("总和: {}", sum);  // 23
}
```

- **解释**：for 循环解构 (&K, &V)。顺序不保证。values() 返回 &V 迭代器。

## 6. 高级主题：自定义哈希和容量

- **HashBuilder**：自定义哈希，如 `RandomState`。
- **capacity() / reserve(n)**：管理容量。
- **drain()**：消耗并返回迭代器。

### 示例：容量管理
```rust
use std::collections::hash_map::RandomState;
use std::hash::BuildHasher;

fn main() {
    let s = RandomState::new();
    let mut map: HashMap<i32, i32, RandomState> = HashMap::with_hasher(s);
    map.reserve(10);  // 预分配

    for i in 0..5 {
        map.insert(i, i * 2);
    }
    println!("容量: {}", map.capacity());  // >= 10
}
```

- **解释**：with_hasher 自定义哈希（安全考虑）。reserve 避免频繁重分配。

## 7. 最佳实践和常见陷阱

- **键类型**：确保 K 实现 Hash + Eq（如 String、i32）。自定义类型用 #[derive(Hash, Eq, PartialEq)]。
- **性能**：平均 O(1)，但最坏 O(n)（哈希碰撞）。用 with_capacity 优化。
- **借用**：get 时，键用 &k（借用）。插入 String 时，用 String::from 或 ToOwned。
- **常见错误**：
    - 键不唯一：insert 覆盖，无错误。
    - 借用冲突：迭代时不能插入（用 collect 到 Vec 或 clone 键）。
    - 非 Hash 类型：编译错误（添加 derive）。
    - 所有权：插入后，原始变量失效（用 clone 如果需要）。
- **与 BTreeMap**：用有序键时考虑 BTreeMap（有序，但 O(log n)）。
- **标准库方法**：探索 extend、merge_into 等。

## 练习建议
1. 编写函数，统计字符串中单词频率，用 HashMap<String, u32>。
2. 用 Entry API 实现“如果键不存在，插入默认列表并追加值”。
3. 从两个 HashMap 合并，处理冲突键（如求和值）。
