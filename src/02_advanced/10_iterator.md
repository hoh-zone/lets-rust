# 迭代器教程

Rust 的迭代器（iterators）是处理序列数据的强大工具，允许你以懒惰（lazy）方式遍历集合，而不立即计算所有元素。这提高了效率，尤其在链式操作中。迭代器实现了 `Iterator` trait，提供 `next()` 方法返回 `Option<Item>`。Rust 标准库中的许多类型如 Vec、HashMap、Range 等都支持迭代器。迭代器是零成本抽象，编译时优化。

本教程从基础开始，逐步深入，包含代码示例和解释。假设你已熟悉 Rust 的集合（如 Vec）和借用。每个示例后，我会解释关键点。如果你有 Rust 环境，可以复制代码运行测试。教程基于 Rust 1.80+（截至 2025 年，迭代器核心未变，但有性能改进如更高效的适配器）。

## 1. 迭代器简介

- **什么是迭代器？**：迭代器是一个可迭代的对象，提供逐个访问元素的方式。核心是 `Iterator` trait：
  ```rust
  trait Iterator {
      type Item;
      fn next(&mut self) -> Option<Self::Item>;
  }
  ```
- **懒惰性**：迭代器不预计算值，只在消费时计算（如 for 循环中）。
- **类型**：
    - `iter()`：不可变借用 (&Item)。
    - `iter_mut()`：可变借用 (&mut Item)。
    - `into_iter()`：消耗所有权 (Item)。
- **优势**：链式方法调用、函数式编程风格、高效过滤/转换。

### 示例：基本迭代
```rust
fn main() {
    let v = vec![1, 2, 3];
    let mut iter = v.iter();  // &i32 的迭代器

    println!("{:?}", iter.next());  // Some(1)
    println!("{:?}", iter.next());  // Some(2)
    println!("{:?}", iter.next());  // Some(3)
    println!("{:?}", iter.next());  // None
}
```

- **解释**：`next()` 消费元素，返回 Option。迭代器耗尽后返回 None。`iter()` 借用向量，不消耗它。

## 2. 消费迭代器

消费器（consumers）如 sum、collect 会遍历整个迭代器。

### 示例：for 循环和 sum
```rust
fn main() {
    let v = vec![1, 2, 3];

    // for 循环消费 iter()
    for &num in v.iter() {
        println!("{}", num);
    }

    // sum 消费
    let total: i32 = v.iter().sum();
    println!("总和: {}", total);  // 输出: 总和: 6

    // v 仍有效，因为 iter() 是借用
    println!("{:?}", v);
}
```

- **解释**：for 隐式调用 next()。sum() 要求 Item: Sum。其他消费器：max、min、count、any、all 等。

### 示例：collect
```rust
fn main() {
    let v = vec![1, 2, 3];
    let collected: Vec<_> = v.iter().map(|&x| x * 2).collect();
    println!("{:?}", collected);  // [2, 4, 6]
}
```

- **解释**：collect() 收集到新集合。类型用 turbofish 如 `collect::<Vec<_>>()` 如果推断失败。

## 3. 迭代器适配器

适配器（adaptors）转换迭代器，返回新迭代器（懒惰）。

- **常见适配器**：
    - map：转换每个元素。
    - filter：过滤元素。
    - take/skip：限制数量。
    - chain：连接迭代器。
    - enumerate：添加索引。
    - zip：并行迭代。

### 示例：链式适配器
```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let result: Vec<_> = v.iter()
        .map(|&x| x * 2)          // [2, 4, 6, 8, 10]
        .filter(|&x| x > 5)       // [6, 8, 10]
        .take(2)                  // [6, 8]
        .collect();

    println!("{:?}", result);  // [6, 8]
}
```

- **解释**：链式调用懒惰，只在 collect() 时执行。map 接收闭包，filter 返回 bool。

### 示例：enumerate 和 zip
```rust
fn main() {
    let v = vec!["a", "b", "c"];

    for (i, &item) in v.iter().enumerate() {
        println!("{}: {}", i, item);  // 0: a, 1: b, 2: c
    }

    let v2 = vec![1, 2, 3];
    for (&s, &n) in v.iter().zip(v2.iter()) {
        println!("{} {}", s, n);  // a 1, b 2, c 3
    }
}
```

- **解释**：enumerate() 返回 (usize, Item)。zip() 以最短迭代器结束。

## 4. 可变迭代和 IntoIterator

- `iter_mut()`：修改元素。
- `into_iter()`：消耗集合，转移所有权。

### 示例：可变迭代
```rust
fn main() {
    let mut v = vec![1, 2, 3];

    for num in v.iter_mut() {
        *num *= 2;  // 修改借用
    }
    println!("{:?}", v);  // [2, 4, 6]

    let consumed: Vec<_> = v.into_iter().map(|x| x + 1).collect();
    println!("{:?}", consumed);  // [3, 5, 7]
    // v 已消耗，无法使用
}
```

- **解释**：iter_mut() 返回 &mut Item，需要解引用修改。into_iter() 后，v 失效。

## 5. 自定义迭代器

实现 Iterator trait 创建自定义迭代器。

### 示例：自定义计数器
```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let sum: u32 = Counter::new().sum();
    println!("总和: {}", sum);  // 15 (1+2+3+4+5)
}
```

- **解释**：实现 next()。可与其他适配器链用。

## 6. 高级主题：DoubleEndedIterator 和 ExactSizeIterator

- **DoubleEndedIterator**：支持 rev()（反向迭代），如 rev()。
- **ExactSizeIterator**：提供 len() 和 is_empty()。
- **并行迭代**：用 rayon crate（如 par_iter()）。
- **错误处理**：try_fold、try_collect 处理 Result。

### 示例：反向迭代
```rust
fn main() {
    let v = vec![1, 2, 3];
    let rev: Vec<_> = v.iter().rev().cloned().collect();
    println!("{:?}", rev);  // [3, 2, 1]
}
```

- **解释**：rev() 反转。cloned() 因为 iter() 是 &i32，collect 到 i32。

## 7. 最佳实践和常见陷阱

- **懒惰优先**：链适配器，避免中间集合。
- **选择正确迭代**：用 iter() 保持所有权，into_iter() 当消耗 OK。
- **性能**：迭代器高效，但过多链可能影响可读性（拆分）。
- **常见错误**：
    - 借用冲突：迭代时修改集合（用 collect() 到新 Vec）。
    - 类型推断失败：显式注解如 |x: &i32| 或 turbofish。
    - 非 Sized 类型：迭代器大小未知，用 Box<dyn Iterator> 如果需存储。
    - 无限迭代器：如 (0..)，用 take() 限制。
- **标准库示例**：lines() 于文件、bytes() 于字符串。
- **与闭包**：适配器用闭包，捕获需注意借用。

## 练习建议
1. 用迭代器过滤 Vec，只保留奇数，然后 map 加倍，collect 到新 Vec。
2. 实现自定义迭代器，生成斐波那契序列的前 n 项。
3. 用 zip 和 enumerate 处理两个 Vec，打印索引和配对值。
