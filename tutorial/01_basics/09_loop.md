### 1. loop 循环：无限循环
`loop` 用于创建无限循环，直到显式使用 `break` 退出。这在需要持续运行直到特定条件时有用，如游戏循环或服务器监听。

- **语法**：
  ```rust
  fn main() {
      loop {
          println!("无限循环！");
          break;  // 退出循环
      }
  }
  ```

- **条件退出**：结合 `if` 和 `break` 使用。
  示例：
  ```rust
  fn main() {
      let mut counter = 0;

      loop {
          counter += 1;

          if counter == 10 {
              break;
          }
      }

      println!("计数器达到: {}", counter);
  }
  ```

- **从循环返回值**：`break` 可以携带值，使 `loop` 成为表达式。
  示例：
  ```rust
  fn main() {
      let mut counter = 0;

      let result = loop {
          counter += 1;

          if counter == 10 {
              break counter * 2;
          }
      };

      println!("结果: {}", result);  // 输出 20
  }
  ```
  注意：这类似于其他语言的 `do-while`，但更灵活。

- **continue**：跳过当前迭代，继续下一次。
  示例：
  ```rust
  loop {
      // 一些代码
      if 条件 {
          continue;  // 跳过剩余代码
      }
      // 其他代码
  }
  ```

### 2. while 循环：条件循环
`while` 在条件为真时执行循环体，常用于不确定迭代次数的情况。

- **语法**：
  示例：
  ```rust
  fn main() {
      let mut number = 3;

      while number != 0 {
          println!("{}!", number);
          number -= 1;
      }

      println!("发射！");
  }
  ```

- **与数组结合**：避免手动索引，使用 `while` 处理可变条件。
  示例：
  ```rust
  fn main() {
      let a = [10, 20, 30, 40, 50];
      let mut index = 0;

      while index < 5 {
          println!("值: {}", a[index]);
          index += 1;
      }
  }
  ```
  注意：手动索引可能导致越界（panic），推荐使用 `for` 代替。

- **break 和 continue**：同样适用，`break` 退出循环，`continue` 跳到下一次检查条件。

### 3. for 循环：迭代循环
`for` 用于遍历集合（如数组、范围），是最安全的循环，避免索引错误。Rust 的 `for` 使用迭代器（iterator）。

- **语法**：遍历范围或集合。
  示例（范围）：
  ```rust
  fn main() {
      for number in (1..4).rev() {  // 3, 2, 1（rev() 反转）
          println!("{}!", number);
      }
      println!("发射！");
  }
  ```

- **遍历数组或集合**：
  示例：
  ```rust
  fn main() {
      let a = [10, 20, 30, 40, 50];

      for element in a {
          println!("值: {}", element);
      }
  }
  ```
  注意：`in` 后是借用集合，避免所有权转移。如果需要修改，使用 `&mut` 或迭代器方法。

- **枚举索引**：使用 `.iter().enumerate()` 获取索引和值。
  示例：
  ```rust
  fn main() {
      let a = [10, 20, 30];

      for (index, value) in a.iter().enumerate() {
          println!("索引 {} 的值: {}", index, value);
      }
  }
  ```

- **break 和 continue**：同样支持，但 `for` 不能像 `loop` 那样直接从 `break` 返回值。

### 4. 嵌套循环和循环标签（Loop Labels）
当循环嵌套时，`break` 或 `continue` 默认影响最内层循环。使用标签（以 `'label:` 开头）控制外层循环。

- **语法**：
  示例：
  ```rust
  fn main() {
      let mut count = 0;
      'counting_up: loop {
          println!("count = {}", count);
          let mut remaining = 10;

          loop {
              println!("remaining = {}", remaining);
              if remaining == 9 {
                  break;  // 退出内层
              }
              if count == 2 {
                  break 'counting_up;  // 退出外层
              }
              remaining -= 1;
          }

          count += 1;
      }
      println!("结束 count = {}", count);
  }
  ```
  注意：标签以单引号开头，后跟冒号。适用于所有循环类型。

### 注意事项
- **安全性**：Rust 编译器确保循环不会导致未定义行为，如越界访问。优先使用 `for` 遍历集合。
- **性能**：循环是零成本抽象，不会引入额外开销。
- **无限循环**：`loop` 可用于故意无限运行，但需小心避免死循环。
- **与所有权交互**：循环中借用或移动值时，遵循借用规则（如不可变借用）。
- **实践**：这些示例可在 `main.rs` 中测试，使用 `cargo run` 执行。结合条件语句（如 `if`）增强灵活性。

