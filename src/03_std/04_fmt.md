# std::fmt 模块

Rust 的 `std::fmt` 模块提供了格式化和打印字符串的工具，包括 trait（如 Display 和 Debug）和宏（如 format! 和 write!）。它是 Rust 打印和字符串操作的核心，用于自定义类型的格式化输出。`std::fmt` 强调 trait 实现，确保类型安全和灵活性，而不依赖运行时反射。模块支持占位符、精度、对齐等高级格式化选项。


## 1. std::fmt 简介

- **导入**：`use std::fmt;`
- **主要组件**：
    - **Trait**：Display（用户友好打印）、Debug（调试打印）、Binary/Octal/Hex（进制格式）、Pointer（指针）。
    - **Formatter**：核心类型，用于 write! 等宏的底层。
    - **宏**：format!（创建 String）、write!（写入 Formatter）、writeln!（写入并换行）、print! / println! / eprint! / eprintln!（标准输出/错误）。
- **占位符语法**：`{}`（默认）、`{:?}`（Debug）、`{:#?}`（美化 Debug）、`{:width$}`（宽度）、`{:.precision$}`（精度）、`{:>}`（右对齐）等。
- **优势**：编译时检查、零运行时开销、可扩展自定义类型。

## 2. Display 和 Debug Trait

- **Display**：用于用户可见的字符串表示，实现 `fmt` 方法。
- **Debug**：用于调试，通常用 `#[derive(Debug)]` 自动实现。

### 示例：实现 Display 和 Debug
```rust
use std::fmt;

#[derive(Debug)]  // 自动实现 Debug
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 3, y: 4 };
    println!("Display: {}", p);   // 输出: Point(3, 4)
    println!("Debug: {:?}", p);   // 输出: Point { x: 3, y: 4 }
    println!("美化 Debug: {:#?}", p);  // 输出多行: Point {\n    x: 3,\n    y: 4,\n}
}
```

- **解释**：`impl Display` 自定义字符串。`write!` 使用 Formatter 写入。`fmt::Result` 是 Ok(()) 或 Err(fmt::Error)。Debug 常用于日志，Display 用于用户输出。

## 3. 格式化宏

宏简化字符串构建。

### 示例：format! 和 write!
```rust
use std::fmt;

fn main() {
    let name = "Rust";
    let version = 1.80;
    let s = format!("{} 版本: {:.2}", name, version);  // "Rust 版本: 1.80"
    println!("{}", s);

    let mut writer = String::new();
    write!(&mut writer, "整数: {:05}", 42).unwrap();  // "整数: 00042" (填充0到5位)
    writeln!(&mut writer, "\n十六进制: {:x}", 255).unwrap();  // "\n十六进制: ff"
    println!("{}", writer);
}
```

- **解释**：`format!` 返回 String。`write!` 写入任何实现 Write 的类型（如 String 或文件）。占位符：`{:05}`（0填充5位）、`{:.2}`（2位小数）、`{:x}`（小写十六进制）。unwrap 处理错误（罕见于字符串）。

### 示例：print! 系列
```rust
fn main() {
    print!("无换行 ");
    println!("有换行");

    eprint!("错误无换行 ");
    eprintln!("错误有换行");
}
```

- **解释**：`print! / println!` 到 stdout，`eprint! / eprintln!` 到 stderr。用于 CLI 输出。

## 4. 高级格式化选项

支持对齐、填充、精度和标志。

### 示例：格式化选项
```rust
fn main() {
    println!("右对齐: {:>10}", "test");  // "      test" (宽度10，右对齐)
    println!("居中: {:^10}", "test");     // "   test   " (居中)
    println!("填充: {:*<10}", "test");    // "test******" (*填充，左对齐)
    println!("精度: {:.3}", 3.141592);    // "3.142" (3位小数)
    println!("正号: {:+}", 42);           // "+42"
    println!("二进制: {:b}", 10);         // "1010"
}
```

- **解释**：`{:>10}`（右对齐宽度10）。标志：`+`（符号）、`#`（前缀如 0x）、`0`（0填充）。结合如 `{:08x}`（0填充8位十六进制）。

## 5. 其他 Trait：Binary, Pointer 等

用于特定格式。

### 示例：Binary 和 Pointer
```rust
use std::fmt;

struct Data(u8);

impl fmt::Binary for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;
        write!(f, "{:08b}", val)  // 8位二进制
    }
}

fn main() {
    let d = Data(170);
    println!("二进制: {:b}", d);  // 10101010

    let ptr: *const i32 = &42;
    println!("指针: {:p}", ptr);  // 如 0x7ffc0e0a1234
}
```

- **解释**：实现 Binary 自定义二进制格式。Pointer 用于打印内存地址（{:p}）。

## 6. Formatter 高级使用

Formatter 提供低级控制。

### 示例：自定义 Formatter
```rust
use std::fmt::{self, Formatter, Write};

fn format_complex(f: &mut Formatter<'_>, real: f64, imag: f64) -> fmt::Result {
    if imag >= 0.0 {
        write!(f, "{} + {}i", real, imag)
    } else {
        write!(f, "{} - {}i", real, -imag)
    }
}

struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        format_complex(f, self.real, self.imag)
    }
}

fn main() {
    let c = Complex { real: 3.0, imag: -4.0 };
    println!("{}", c);  // 3 - 4i
}
```

- **解释**：Formatter 有方法如 pad（填充）、precision（获取精度）。用于复杂逻辑。

## 7. 最佳实践和常见陷阱

- **优先 derive**：用 `#[derive(Debug)]` 自动 Debug，避免手动 impl。
- **Display vs Debug**：Display 用于最终输出，Debug 用于开发/日志。
- **错误处理**：fmt::Result 通常 Ok，但自定义时检查 write! 返回。
- **性能**：format! 分配 String，避免循环中；用 write! 到缓冲。
- **常见错误**：
    - 未实现 trait：打印时编译错误（添加 impl 或 derive）。
    - 格式不匹配：如 {:?} 于非 Debug 类型（实现 Debug）。
    - 生命周期：Formatter 的 '_ 是 elided 生命周期。
- **与 serde**：复杂序列化用外部 crate，但 fmt 适合简单打印。
- **国际化**：fmt 无内置 i18n，用 crate 如 fluent。

## 练习建议
1. 为自定义 enum 实现 Display，处理不同变体。
2. 用 format! 创建 JSON-like 字符串，包含数组和对象。
3. 实现 Binary 为位字段 struct，打印二进制表示。

