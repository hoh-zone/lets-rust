# Rust std::iter 模块教程

Rust 的 `std::iter` 模块是标准库中处理迭代器的核心部分，提供 `Iterator` trait 和各种适配器、实用工具，用于懒惰地处理序列数据。迭代器允许链式操作（如 map、filter），避免中间集合分配，提高效率和表达力。`std::iter` 强调零成本抽象：编译时展开，运行时无开销。它与集合（如 Vec、HashMap）和范围（Range）集成，支持函数式编程风格。模块包括 trait 定义、适配器函数（如 `once`、`empty`）和扩展方法。

## 1. std::iter 简介

- **导入和基本结构**：通常用 `use std::iter;` 或指定如 `use std::iter::{Iterator, FromIterator};`。模块分为 trait、函数和适配器三大类。
    - **Trait 概述**：
        - `Iterator`：核心 trait，定义 `Item` 类型和 `next()` 方法，返回 `Option<Self::Item>`。支持 `size_hint` 以优化分配。
        - `DoubleEndedIterator`：扩展 `Iterator`，添加 `next_back()` 以支持反向迭代（如 rev()）。
        - `ExactSizeIterator`：提供精确 `len()`，用于预分配（如 Vec 的 iter）。
        - `Extend`：从迭代器扩展集合。
        - `FromIterator`：从迭代器创建集合（如 collect()）。
    - **函数**：`empty`（空迭代器）、`once`（单元素）、`repeat`（无限重复）、`successors`（生成序列）。
    - **适配器**：方法如 `map`、`filter`、`take`、`skip`、`chain`、`zip`、`enumerate`、`flatten`、`fuse`、`peekable`、`scan`、`cycle`、`step_by` 等，返回新迭代器（懒惰）。
- **设计哲学**：迭代器是懒惰的，只在消费（如 for、collect）时计算；支持融合优化（链式方法合并循环）。错误通过 Option/Result 处理，无 panic。
- **跨平台注意**：纯 Rust，无 OS 依赖；但与文件/网络迭代结合时考虑平台 I/O 差异。
- **性能基础**：零开销，但长链可能增加编译时间；用 `fold`/`reduce` 避免中间 Vec。
- **常见用例**：数据处理管道、集合转换、无限序列（如生成器）、与闭包结合函数式代码。
- **扩展概念**：迭代器适配器是零成本的；Rust 优化如循环展开。与 rayon crate 集成并行（par_iter）。

## 2. Iterator Trait 和基本使用

`Iterator` 是所有迭代器的基础。任何实现它的类型都可以用 for 循环或适配器。

### 示例：基本迭代（Vec 示例）
```rust
use std::iter::Iterator;

fn main() {
    let v = vec![1, 2, 3];
    let mut iter = v.iter();  // &i32 迭代器

    while let Some(item) = iter.next() {
        println!("项: {}", item);
    }
}
```

- **解释**：`next()` 返回 Option<&i32>；耗尽返回 None。`iter()` 返回借用迭代器。性能：直接栈访问，快于 into_iter()（移动）。

### 示例：自定义 Iterator（计数器扩展）
```rust
use std::iter::Iterator;

#[derive(Debug)]
struct Fibonacci {
    curr: u64,
    next: u64,
    limit: u64,
}

impl Fibonacci {
    fn new(limit: u64) -> Self {
        Fibonacci { curr: 0, next: 1, limit }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr > self.limit {
            return None;
        }
        let current = self.curr;
        self.curr = self.next;
        self.next = current + self.next;
        Some(current)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // 粗略估计
        let remaining = ((self.limit - self.curr) / self.next + 1) as usize;
        (remaining, Some(remaining))
    }
}

fn main() {
    let fib = Fibonacci::new(100);
    let seq: Vec<u64> = fib.collect();
    println!("序列: {:?}", seq);  // [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89]

    // 扩展：使用 size_hint
    println!("大小提示: {:?}", fib.size_hint());
}
```

- **解释**：实现 `next()` 生成序列。`type Item` 定义输出类型。`size_hint` 返回 (下界, 上界 Option)，帮助 collect 预分配。陷阱：无限迭代器 size_hint (0, None)，避免 collect() OOM。扩展：用 `fused` 确保耗尽后 None 一致。

### 示例：DoubleEndedIterator（反向迭代扩展）
```rust
use std::iter::{DoubleEndedIterator, Iterator};

fn main() {
    let v = vec![1, 2, 3, 4];
    let mut iter = v.iter();

    println!("前: {:?}", iter.next());     // Some(1)
    println!("后: {:?}", iter.next_back()); // Some(4)
    println!("前: {:?}", iter.next());     // Some(2)
    println!("后: {:?}", iter.next_back()); // Some(3)
}
```

- **解释**：`next_back()` 从末尾取。用于 Vec/Range 等双端结构。性能：Vec O(1)，但链表可能 O(n)。扩展：用 `rev()` 反转单端迭代器为双端。

## 3. 消费迭代器

消费器执行迭代，如 `collect`、`sum`、`any`、`fold`。

### 示例：collect 和 FromIterator（基本消费）
```rust
use std::iter::FromIterator;

fn main() {
    let v: Vec<i32> = (1..5).collect();
    println!("收集: {:?}", v);  // [1, 2, 3, 4]

    let s = String::from_iter(['h', 'e', 'l', 'l', 'o']);
    println!("字符串: {}", s);  // "hello"
}
```

- **解释**：`collect` 用 FromIterator 创建集合。泛型 `<Vec<_>>` 指定类型。性能：用 size_hint 预分配，避免重分配。

### 示例：fold 和 reduce（累积扩展）
```rust
fn main() {
    let sum = (1..=5).fold(0, |acc, x| acc + x);
    println!("fold 总和: {}", sum);  // 15

    let max = (1..=5).reduce(|acc, x| if x > acc { x } else { acc });
    println!("reduce 最大: {:?}", max);  // Some(5)

    // 扩展：处理空迭代器
    let empty_max = vec![] as Vec<i32>;
    println!("空 reduce: {:?}", empty_max.into_iter().reduce(|a, b| a.max(b)));  // None
}
```

- **解释**：`fold` 用初始值累积；`reduce` 用第一个元素作为初始，无元素返回 None。陷阱：空迭代器 reduce None，避免 unwrap。扩展：用 `try_fold` 处理 Result，早返回错误。

### 示例：any、all、find（谓词消费扩展）
```rust
fn main() {
    let has_even = (1..10).any(|x| x % 2 == 0);
    println!("有偶数？{}", has_even);  // true

    let all_positive = (1..10).all(|x| x > 0);
    println!("全正？{}", all_positive);  // true

    let found = (1..10).find(|&x| x > 5);
    println!("找到 >5: {:?}", found);  // Some(6)

    // 扩展：position 和 rposition（反向）
    let pos = (1..10).position(|x| x == 5);
    println!("位置: {:?}", pos);  // Some(4)

    let rpos = (1..10).rposition(|x| x == 5);
    println!("反向位置: {:?}", rpos);  // Some(4)
}
```

- **解释**：`any`/`all` 短路（早停）；`find` 返回第一个匹配 Option。`position` 返回索引。性能：短路优化大序列。扩展：用 `find_map` 结合 map 和 find。

## 4. 迭代器适配器

适配器返回新迭代器，懒惰链式。

### 示例：map 和 filter（基本转换）
```rust
fn main() {
    let doubled: Vec<i32> = (1..5).map(|x| x * 2).collect();
    println!("map: {:?}", doubled);  // [2, 4, 6, 8]

    let evens: Vec<i32> = (1..10).filter(|&x| x % 2 == 0).collect();
    println!("filter: {:?}", evens);  // [2, 4, 6, 8]
}
```

- **解释**：`map` 转换每个 Item；`filter` 保留 true 的。闭包捕获借用。

### 示例：chain、zip、enumerate（组合扩展）
```rust
fn main() {
    let chained: Vec<i32> = (1..3).chain(4..6).collect();
    println!("chain: {:?}", chained);  // [1, 2, 4, 5]

    let zipped: Vec<(i32, char)> = (1..4).zip('a'..='c').collect();
    println!("zip: {:?}", zipped);  // [(1, 'a'), (2, 'b'), (3, 'c')]

    let enumerated: Vec<(usize, i32)> = (10..13).enumerate().collect();
    println!("enumerate: {:?}", enumerated);  // [(0, 10), (1, 11), (2, 12)]

    // 扩展：多 zip 和 chain
    let multi_zip: Vec<(i32, char, bool)> = (1..4).zip('a'..).zip([true, false, true]).map(|((a, b), c)| (a, b, c)).collect();
    println!("多 zip: {:?}", multi_zip);
}
```

- **解释**：`chain` 连接；`zip` 并行，最短结束；`enumerate` 添加索引。性能：链长编译优化融合单循环。

### 示例：take、skip、step_by（限制扩展）
```rust
fn main() {
    let taken: Vec<i32> = (1..).take(5).collect();  // [1, 2, 3, 4, 5]
    println!("take: {:?}", taken);

    let skipped: Vec<i32> = (1..10).skip(3).collect();  // [4, 5, 6, 7, 8, 9]
    println!("skip: {:?}", skipped);

    let stepped: Vec<i32> = (1..10).step_by(2).collect();  // [1, 3, 5, 7, 9]
    println!("step_by: {:?}", stepped);

    // 扩展：无限迭代器安全
    let infinite = std::iter::repeat(42).take(3).collect::<Vec<_>>();
    println!("repeat take: {:?}", infinite);  // [42, 42, 42]
}
```

- **解释**：`take` 限制数量；`skip` 跳过前 n；`step_by` 每步跳跃。陷阱：无限如 (1..) 无 take 会挂起。

### 示例：flatten 和 flat_map（嵌套扩展）
```rust
fn main() {
    let nested = vec![vec![1, 2], vec![3, 4]];
    let flat: Vec<i32> = nested.into_iter().flatten().collect();
    println!("flatten: {:?}", flat);  // [1, 2, 3, 4]

    let flat_mapped: Vec<char> = (1..4).flat_map(|x| x.to_string().chars()).collect();
    println!("flat_map: {:?}", flat_mapped);  // ['1', '2', '3']
}
```

- **解释**：`flatten` 展平一层嵌套；`flat_map` 结合 map 和 flatten。扩展：多层用 chain flatten。

## 5. 高级适配器：Peekable、Scan、Cycle

### 示例：Peekable（预览扩展）
```rust
use std::iter::Peekable;

fn main() {
    let mut iter = (1..5).peekable();
    println!("预览: {:?}", iter.peek());  // Some(1)
    println!("下一个: {:?}", iter.next());  // Some(1)
}
```

- **解释**：`peekable` 添加 `peek` 预览下一个而不消费。用于解析器。

### 示例：Scan（状态累积扩展）
```rust
fn main() {
    let scanned: Vec<i32> = (1..5).scan(0, |state, x| {
        *state += x;
        Some(*state)
    }).collect();
    println!("scan: {:?}", scanned);  // [1, 3, 6, 10]
}
```

- **解释**：`scan` 维护状态，返回 Option（None 早停）。类似 fold 但产生中间值。

### 示例：Cycle（循环无限扩展）
```rust
fn main() {
    let cycled: Vec<i32> = (1..4).cycle().take(10).collect();
    println!("cycle: {:?}", cycled);  // [1, 2, 3, 1, 2, 3, 1, 2, 3, 1]
}
```

- **解释**：`cycle` 无限重复；用 take 限制。陷阱：无限制 collect 挂起/OOM。

## 6. 函数和实用工具

- `once`：单元素。
- `successors`：基于函数生成。

### 示例：once 和 empty
```rust
use std::iter;

fn main() {
    let single: Vec<i32> = iter::once(42).collect();
    println!("once: {:?}", single);  // [42]

    let empty_vec: Vec<i32> = iter::empty().collect();
    println!("empty: {:?}", empty_vec);  // []
}
```

- **解释**：`once` 用于单项；`empty` 用于空序列。扩展：与 chain 组合动态列表。

### 示例：successors（生成序列扩展）
```rust
use std::iter;

fn main() {
    let powers_of_two: Vec<u32> = iter::successors(Some(1u32), |&n| Some(n * 2))
        .take_while(|&n| n < 100)
        .collect();
    println!("2 的幂: {:?}", powers_of_two);  // [1, 2, 4, 8, 16, 32, 64]
}
```

- **解释**：`successors` 从初始生成，直到 None。`take_while` 条件停止。扩展：用于递归序列如树遍历。

## 7. 高级主题：Extend、FromIterator 和 融合

- `Extend`：从迭代器添加元素。
- `FromIterator`：实现 collect。

### 示例：Extend 扩展集合
```rust
use std::iter::Extend;

fn main() {
    let mut v = vec![1, 2];
    v.extend(3..6);
    println!("extend: {:?}", v);  // [1, 2, 3, 4, 5]
}
```

- **解释**：`extend` 消费迭代器添加。性能：用 size_hint 预分配。

### 示例：自定义 FromIterator（扩展集合）
```rust
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let set: HashSet<i32> = FromIterator::from_iter(1..4);
    println!("set: {:?}", set);  // {1, 2, 3} (无序)
}
```

- **解释**：`from_iter` 用 FromIterator 创建。扩展：实现自定义集合。

## 8. 最佳实践和常见陷阱

- **懒惰优先**：链适配器避免中间集合；如 filter.map.collect 而非两个 collect。
- **性能最佳实践**：用 fold/reduce 代替 collect+loop；长链用 turbofish 指定类型减编译时间。
- **错误陷阱**：无限迭代器（如 cycle）无 take 挂起；空 reduce None，避免 unwrap。
- **安全性**：闭包捕获借用检查；无限生成防 OOM 用 take_while。
- **跨平台扩展**：无依赖，但文件迭代考虑 OS 编码。
- **测试扩展**：用 once/empty 测试边界；mock Iterator 测试函数。
- **与并行**：用 rayon::iter 测试 par_map 等。
- **资源管理**：迭代器 drop 时释放资源，但显式消费好。
- **常见错误扩展**：
    - 类型推断失败：用 ::<Vec<_>> 指定 collect。
    - 借用冲突：迭代时修改集合，用 collect 克隆。
    - 非 Sized：存储迭代器用 Box<dyn Iterator>。
    - 融合失败：复杂链不优化，拆分简单链。

## 练习建议
1. 编写管道：从 Vec  filter 偶数，map 加倍，fold 求和。
2. 实现自定义 Iterator：生成素数，用 successors 和 filter。
3. 创建嵌套 flatten：处理 Vec<Vec<Vec<i32>>> 多层展平。
4. 处理大序列：用 scan 累积状态，take_while 限制。
5. 基准测试：比较 chain vs 两个 collect 大 Vec 时间，用 Instant。
6. 与 io 集成：用 lines() map 解析日志，collect 到 HashMap。

