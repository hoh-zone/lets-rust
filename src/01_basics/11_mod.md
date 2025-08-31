以下是关于 Rust 编程语言中 `mod` 语法（模块系统）的教程。
Rust 的模块系统用于组织代码、管理作用域和隐私性，确保代码的可读性和复用性。模块形成一个树状结构，以 crate（包）根为起点。

### 1. 定义模块（Defining Modules）
`mod` 关键字用于声明一个模块。模块可以内联定义（在同一文件中用 `{}` 包围代码）或在单独文件中定义。默认情况下，模块及其内部项是私有的（private），外部无法访问。

- **内联定义**：直接在 `mod` 后用 `{}` 写模块代码。
- **单独文件定义**：声明 `mod module_name;`（无 `{}`），Rust 会自动在特定文件中查找代码：
    - 对于 crate 根文件（如 `src/main.rs` 或 `src/lib.rs`）：查找 `src/module_name.rs` 或 `src/module_name/mod.rs`。
    - 对于子模块：类似地，在父模块目录下查找。

**示例（内联定义）**：
```rust
// src/main.rs
mod garden {  // 内联模块
    fn plant() {
        println!("种植蔬菜");
    }
}

fn main() {
    garden::plant();  // 在同一文件中可访问
}
```

**示例（单独文件）**：
- 在 `src/main.rs` 中：`mod garden;`
- 在 `src/garden.rs` 中：
  ```rust
  fn plant() {
      println!("种植蔬菜");
  }
  ```
  然后在 `main` 中调用 `garden::plant();`。

注意：模块树类似于文件系统目录树，帮助组织大型项目。

### 2. 模块树结构（Module Tree Structure）
模块形成层次结构，隐式根模块为 `crate`。子模块嵌套在父模块中，兄弟模块在同一父级定义。

**示例模块树**：
```
crate
 └── garden
     ├── vegetables
     │   └── asparagus
     └── fruits
```

**代码表示**：
```rust
// src/lib.rs
mod garden {
    pub mod vegetables {
        pub fn asparagus() {}
    }
    pub mod fruits {}
}
```

### 3. 路径引用项（Paths for Referring to Items）
路径用于访问模块中的项（如函数、结构体）。路径可以是绝对路径（从 `crate` 开始）或相对路径（从当前模块开始）。

- **绝对路径**：以 `crate::` 开头。
- **相对路径**：使用模块名、`self`（当前模块）或 `super`（父模块）。

**示例**：
```rust
// src/lib.rs
mod garden {
    pub mod vegetables {
        pub struct Asparagus {}
    }
}

fn absolute_path() {
    let _ = crate::garden::vegetables::Asparagus {};  // 绝对路径
}

mod example {
    fn relative_path() {
        let _ = super::garden::vegetables::Asparagus {};  // 使用 super 访问父级
        let _ = self::local_item();  // self 访问当前模块
    }

    fn local_item() {}
}
```

注意：路径使用 `::` 分隔，类似于命名空间。

### 4. pub 关键字：控制可见性（Visibility）
默认所有项私有。使用 `pub` 使模块、函数、结构体等公开可见：
- `pub mod`：公开模块。
- `pub fn`、`pub struct` 等：公开项。
- 私有项只能在当前模块或子模块中使用。

**示例**：
```rust
// src/lib.rs
pub mod garden {  // 公开模块
    pub fn plant() {  // 公开函数
        println!("种植");
    }

    fn water() {}  // 私有函数，仅 garden 内部可用
}
```

外部 crate 可访问 `garden::plant()`，但不能访问 `water()`。

### 5. use 关键字：导入路径（Importing）
`use` 创建路径别名，简化长路径的使用。作用域限于当前块或模块。

- 支持绝对或相对路径。
- 可导入单个项、模块或使用 `*` 通配符（不推荐滥用）。
- 支持重命名：`use path as alias;`。

**示例**：
```rust
// src/main.rs
mod garden {
    pub mod vegetables {
        pub struct Asparagus {}
    }
}

use crate::garden::vegetables::Asparagus;  // 导入

fn main() {
    let _ = Asparagus {};  // 直接使用别名
}

use crate::garden::vegetables as veg;  // 重命名模块
let _ = veg::Asparagus {};
```

### 6. super、self 和重新导出（Re-exporting）
- **super**：访问父模块，类似于文件系统的 `..`。
- **self**：显式引用当前模块。
- **重新导出**：在 `use` 中使用 `pub use` 将导入项公开导出给外部。

**示例（super 和 self）**：
```rust
// src/lib.rs
mod garden {
    fn parent_item() {}

    mod vegetables {
        fn use_super() {
            super::parent_item();  // 访问父模块
        }

        fn use_self() {
            self::local_item();
        }

        fn local_item() {}
    }
}
```

**示例（重新导出）**：
```rust
// src/lib.rs
mod garden {
    pub mod vegetables {
        pub struct Asparagus {}
    }
}

pub use crate::garden::vegetables::Asparagus;  // 重新导出

// 外部可直接用 Asparagus {} 而非 garden::vegetables::Asparagus
```

### 注意事项
- **隐私规则**：父模块不能访问子模块私有项，但子模块可访问祖先模块所有项。
- **文件组织**：大型项目使用目录结构，如 `src/garden/vegetables.rs`。
- **crate 和 binary/lib**：在 binary crate（main.rs）中，模块用于内部组织；在 lib crate 中，用于公开 API。
- **最佳实践**：使用 `pub` 仅公开必要项；避免循环依赖；结合 Cargo 管理多 crate 项目。
- **错误处理**：路径错误（如未找到模块）会在编译时报错。

这些是 Rust `mod` 语法的基础，建议通过 `cargo new` 创建项目测试示例。
