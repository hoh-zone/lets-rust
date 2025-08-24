# Rust Slice 教程

Rust 中的 slice（切片）是一种引用集合中连续元素的视图，而不拥有这些元素。它类似于数组或向量的子视图，使用 `&[T]` 表示不可变切片，`&mut [T]` 表示可变切片。Slice 是借用的一部分，遵守借用规则，确保内存安全。Slice 常用于字符串、数组和向量，帮助避免不必要的拷贝，提高效率。

本教程从基础开始，逐步深入，包含代码示例和解释。假设你已熟悉 Rust 的所有权和借用（如 & 和 &mut）。每个示例后，我会解释关键点。如果你有 Rust 环境，可以复制代码运行测试。教程基于 Rust 1.80+（截至 2025 年，slice 核心未变，但有性能优化）。

## 1. Slice 简介

- **什么是 slice？**：Slice 是对数据序列的引用视图，指向连续内存块。不拥有数据，只借用。长度在运行时确定。
- **语法**：`&[T]`（不可变）、`&mut [T]`（可变）。T 是元素类型。
- **优势**：零拷贝访问子集；函数参数通用（如接受 &[i32] 而非 Vec<i32> 或 [i32; N]）。
- **与数组/向量的关系**：数组是固定大小，向量是动态。Slice 可以从两者创建。
- **字符串 slice**：`&str` 是 &[u8] 的特殊形式，处理 UTF-8。

### 示例：基本 slice
```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];  // 数组
    let slice = &arr[1..4];     // 创建 slice: &arr[1], &arr[2], &arr[3]

    println!("{:?}", slice);    // 输出: [2, 3, 4]
}
```

- **解释**：`[start..end]` 是半开区间（包括 start，不包括 end）。`&arr[..]` 是全切片。Slice 借用 arr，借用规则适用。

## 2. 创建 Slice

Slice 通过借用和范围运算符创建。

- **范围语法**：
    - `[start..end]`：从 start 到 end-1。
    - `[..end]`：从 0 到 end-1。
    - `[start..]`：从 start 到结束。
    - `[..]`：整个集合。
- **从向量/数组**：直接 &vec[start..end]。
- **边界检查**：运行时检查，如果越界 panic!（安全）。

### 示例：各种创建方式
```rust
fn main() {
    let vec = vec![10, 20, 30, 40, 50];
    
    let full = &vec[..];      // 全切片: [10, 20, 30, 40, 50]
    let first_three = &vec[0..3];  // [10, 20, 30]
    let last_two = &vec[3..];     // [40, 50]
    
    println!("{:?}", first_three);
}
```

- **解释**：Vec 和数组都支持。Slice 的 len() 返回元素数，get(i) 返回 Option<&T>（安全访问）。

### 可变 slice
```rust
fn main() {
    let mut vec = vec![1, 2, 3];
    let slice = &mut vec[1..3];  // 可变借用
    
    slice[0] = 20;  // 修改 vec[1]
    println!("{:?}", vec);  // 输出: [1, 20, 3]
}
```

- **解释**：可变 slice 允许修改元素，但遵守独占借用规则。

## 3. 字符串 Slice (&str)

字符串 slice 是常见的，处理 String 或 str。

### 示例：字符串 slice
```rust
fn first_word(s: &str) -> &str {  // 接受 &str（通用）
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);  // &String 隐式转为 &str
    
    println!("{}", word);  // 输出: hello
    
    // s.clear();  // 错误！word 借用期间不能修改 s
}
```

- **解释**：`&str` 是 UTF-8 安全的。as_bytes() 转为 &[u8]。切片索引必须在字符边界（否则 panic!）。用 chars() 或 bytes() 迭代以避免。

## 4. 函数参数中的 Slice

Slice 使函数更通用，不依赖具体集合类型。

### 示例：求和函数
```rust
fn sum_slice(nums: &[i32]) -> i32 {
    let mut sum = 0;
    for &num in nums {
        sum += num;
    }
    sum
}

fn main() {
    let arr = [1, 2, 3];
    let vec = vec![4, 5, 6];
    
    println!("{}", sum_slice(&arr));  // 6
    println!("{}", sum_slice(&vec));  // 15
}
```

- **解释**：`&[i32]` 接受数组或向量的借用。迭代用 for &item（解引用）。

## 5. 多维 Slice

Slice 可以是多维的，如 &[[T]]。

### 示例：矩阵 slice
```rust
fn main() {
    let matrix = vec![vec![1, 2], vec![3, 4]];
    let row = &matrix[0][..];  // &[i32]: [1, 2]
    
    println!("{:?}", row);
}
```

- **解释**：嵌套借用。复杂时考虑扁平化或专用 crate。

## 6. 高级主题：Unsafe 和 Split

- **Split 方法**：如 split_at() 分割 slice。
  ```rust
  fn main() {
      let arr = [1, 2, 3, 4];
      let (left, right) = arr.split_at(2);  // left: &[1,2], right: &[3,4]
  }
  ```
- **Unsafe slice**：在 unsafe 块中，可以创建原始指针，但避免，除非必要。
- **Deref 到 slice**：Vec 和 String 实现 Deref<Target=[T]>，所以 &Vec<T> 可隐式转为 &[T]。

## 7. 最佳实践和常见陷阱

- **安全访问**：用 get(i) 而非 [i]，避免 panic!。
- **避免修改借用**：借用 slice 时，不能修改底层集合（借用 checker 防止）。
- **UTF-8 安全**：字符串 slice 时，用 char_indices() 处理多字节字符。
- **性能**：Slice 是零成本视图，无分配。
- **常见错误**：
    - 索引越界：运行时 panic!（用 if let Some(v) = slice.get(i)）。
    - 非字符边界切片：如 &s[0..1] 如果 s 是多字节（panic!）。
    - 借用冲突：如借用 slice 同时 push 到 vec（用临时变量或重组代码）。
- **与生命周期**：复杂函数需生命周期注解（如 fn foo<'a>(s: &'a [T])）。

## 练习建议
1. 编写函数，接收 &[u8]，返回最大元素的 &u8。
2. 实现一个反转字符串 slice 的函数（不修改原字符串）。
3. 从 Vec<Vec<i32>> 创建子矩阵 slice，并求和。

