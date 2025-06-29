// 第14章：文档与测试
// 演示 Rust 的文档注释、文档测试和单元测试

//! # 文档与测试示例 crate
//! 
//! 这个 crate 演示了如何在 Rust 中编写文档和测试。
//! 
//! ## 主要功能
//! 
//! - 文档注释的使用
//! - 文档测试的编写
//! - 单元测试的实现
//! - 集成测试的概念
//! 
//! ## 示例
//! 
//! ```
//! use docs_and_testing::Calculator;
//! 
//! let calc = Calculator::new();
//! assert_eq!(calc.add(2, 3), 5);
//! ```

fn main() {
    println!("📚 第14章：文档与测试");
    println!("=====================================");
    
    // 1. 文档注释演示
    documentation_demo();
    
    // 2. 文档测试演示
    doc_test_demo();
    
    // 3. 单元测试演示
    unit_test_demo();
    
    // 4. 测试组织演示
    test_organization_demo();
    
    // 5. 测试最佳实践
    testing_best_practices();
}

// ============================================================================
// 1. 文档注释演示
// ============================================================================

fn documentation_demo() {
    println!("\n📖 1. 文档注释演示");
    println!("{}", "-".repeat(40));
    
    let calc = Calculator::new();
    println!("🧮 创建计算器");
    
    let result = calc.add(5, 3);
    println!("5 + 3 = {}", result);
    
    let result = calc.divide(10.0, 2.0);
    match result {
        Ok(value) => println!("10.0 / 2.0 = {}", value),
        Err(e) => println!("错误: {}", e),
    }
    
    // 使用泛型函数
    let max_int = find_max(&[1, 5, 3, 9, 2]);
    println!("最大整数: {:?}", max_int);
    
    let max_char = find_max(&['a', 'z', 'm', 'b']);
    println!("最大字符: {:?}", max_char);
}

/// 一个简单的计算器结构体
/// 
/// `Calculator` 提供基本的数学运算功能。
/// 
/// # 示例
/// 
/// ```
/// # use docs_and_testing::Calculator;
/// let calc = Calculator::new();
/// assert_eq!(calc.add(2, 3), 5);
/// ```
/// 
/// # 注意
/// 
/// 这个计算器主要用于演示文档注释的使用。
#[derive(Debug)]
pub struct Calculator {
    /// 计算器的名称
    name: String,
}

impl Calculator {
    /// 创建一个新的计算器实例
    /// 
    /// # 示例
    /// 
    /// ```
    /// # use docs_and_testing::Calculator;
    /// let calc = Calculator::new();
    /// println!("计算器已创建");
    /// ```
    pub fn new() -> Self {
        Calculator {
            name: "基础计算器".to_string(),
        }
    }
    
    /// 将两个数字相加
    /// 
    /// # 参数
    /// 
    /// * `a` - 第一个加数
    /// * `b` - 第二个加数
    /// 
    /// # 返回值
    /// 
    /// 返回 `a` 和 `b` 的和
    /// 
    /// # 示例
    /// 
    /// ```
    /// # use docs_and_testing::Calculator;
    /// let calc = Calculator::new();
    /// let result = calc.add(2, 3);
    /// assert_eq!(result, 5);
    /// ```
    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
    
    /// 将两个数字相减
    /// 
    /// # 参数
    /// 
    /// * `a` - 被减数
    /// * `b` - 减数
    /// 
    /// # 返回值
    /// 
    /// 返回 `a` 减去 `b` 的结果
    /// 
    /// # 示例
    /// 
    /// ```
    /// # use docs_and_testing::Calculator;
    /// let calc = Calculator::new();
    /// assert_eq!(calc.subtract(5, 3), 2);
    /// assert_eq!(calc.subtract(3, 5), -2);
    /// ```
    pub fn subtract(&self, a: i32, b: i32) -> i32 {
        a - b
    }
    
    /// 将两个数字相乘
    /// 
    /// # 参数
    /// 
    /// * `a` - 第一个乘数
    /// * `b` - 第二个乘数
    /// 
    /// # 返回值
    /// 
    /// 返回 `a` 和 `b` 的乘积
    /// 
    /// # 示例
    /// 
    /// ```
    /// # use docs_and_testing::Calculator;
    /// let calc = Calculator::new();
    /// assert_eq!(calc.multiply(4, 5), 20);
    /// assert_eq!(calc.multiply(-2, 3), -6);
    /// ```
    pub fn multiply(&self, a: i32, b: i32) -> i32 {
        a * b
    }
    
    /// 将两个数字相除
    /// 
    /// # 参数
    /// 
    /// * `a` - 被除数
    /// * `b` - 除数
    /// 
    /// # 返回值
    /// 
    /// 成功时返回 `Ok(result)`，除零时返回 `Err(error_message)`
    /// 
    /// # 错误
    /// 
    /// 当 `b` 为 0 时，函数返回错误。
    /// 
    /// # 示例
    /// 
    /// ```
    /// # use docs_and_testing::Calculator;
    /// let calc = Calculator::new();
    /// 
    /// // 正常除法
    /// assert_eq!(calc.divide(10.0, 2.0), Ok(5.0));
    /// 
    /// // 除零错误
    /// assert!(calc.divide(10.0, 0.0).is_err());
    /// ```
    pub fn divide(&self, a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("除数不能为零".to_string())
        } else {
            Ok(a / b)
        }
    }
    
    /// 计算数字的平方
    /// 
    /// # 参数
    /// 
    /// * `n` - 要计算平方的数字
    /// 
    /// # 返回值
    /// 
    /// 返回 `n` 的平方
    /// 
    /// # 示例
    /// 
    /// ```
    /// # use docs_and_testing::Calculator;
    /// let calc = Calculator::new();
    /// assert_eq!(calc.square(4), 16);
    /// assert_eq!(calc.square(-3), 9);
    /// ```
    /// 
    /// # 注意
    /// 
    /// 对于非常大的数字，可能会发生整数溢出。
    pub fn square(&self, n: i32) -> i32 {
        n * n
    }
}

/// 在切片中找到最大值
/// 
/// 这个函数使用泛型来处理任何实现了 `Ord` 和 `Copy` trait 的类型。
/// 
/// # 类型参数
/// 
/// * `T` - 必须实现 `Ord + Copy` 的类型
/// 
/// # 参数
/// 
/// * `list` - 要搜索的切片
/// 
/// # 返回值
/// 
/// 返回切片中的最大值，如果切片为空则返回 `None`
/// 
/// # 示例
/// 
/// ```
/// # use docs_and_testing::find_max;
/// let numbers = [1, 5, 3, 9, 2];
/// assert_eq!(find_max(&numbers), Some(&9));
/// 
/// let empty: [i32; 0] = [];
/// assert_eq!(find_max(&empty), None);
/// 
/// let chars = ['a', 'z', 'm'];
/// assert_eq!(find_max(&chars), Some(&'z'));
/// ```
/// 
/// # Panics
/// 
/// 这个函数不会 panic。
pub fn find_max<T: Ord + Copy>(list: &[T]) -> Option<&T> {
    list.iter().max()
}

// ============================================================================
// 2. 文档测试演示
// ============================================================================

fn doc_test_demo() {
    println!("\n🧪 2. 文档测试演示");
    println!("{}", "-".repeat(40));
    
    println!("📝 文档测试是嵌入在文档注释中的测试代码");
    println!("   运行 'cargo test' 会自动执行这些测试");
    println!("   文档测试确保文档中的示例代码始终有效");
    
    // 演示一些复杂的文档测试场景
    let temp = Temperature::new(25.0);
    println!("🌡️ 温度: {}°C = {}°F", temp.celsius(), temp.fahrenheit());
}

/// 表示温度的结构体
/// 
/// # 示例
/// 
/// ```
/// # use docs_and_testing::Temperature;
/// let temp = Temperature::new(0.0);
/// assert_eq!(temp.celsius(), 0.0);
/// assert_eq!(temp.fahrenheit(), 32.0);
/// ```
/// 
/// 也可以创建负温度：
/// 
/// ```
/// # use docs_and_testing::Temperature;
/// let cold = Temperature::new(-40.0);
/// assert_eq!(cold.celsius(), -40.0);
/// assert_eq!(cold.fahrenheit(), -40.0);  // -40°C = -40°F
/// ```
/// 
/// # 隐藏的文档测试
/// 
/// 有时我们需要在文档测试中包含设置代码，但不想在文档中显示：
/// 
/// ```
/// # use docs_and_testing::Temperature;
/// # // 这是隐藏的设置代码
/// # let room_temp = 20.0;
/// let temp = Temperature::new(room_temp);
/// assert!(temp.celsius() > 0.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Temperature {
    celsius: f64,
}

impl Temperature {
    /// 创建新的温度实例
    /// 
    /// # 参数
    /// 
    /// * `celsius` - 摄氏度温度
    /// 
    /// # 示例
    /// 
    /// ```
    /// # use docs_and_testing::Temperature;
    /// let freezing = Temperature::new(0.0);
    /// let boiling = Temperature::new(100.0);
    /// ```
    pub fn new(celsius: f64) -> Self {
        Temperature { celsius }
    }
    
    /// 获取摄氏度温度
    /// 
    /// ```
    /// # use docs_and_testing::Temperature;
    /// let temp = Temperature::new(25.0);
    /// assert_eq!(temp.celsius(), 25.0);
    /// ```
    pub fn celsius(&self) -> f64 {
        self.celsius
    }
    
    /// 获取华氏度温度
    /// 
    /// 使用公式：F = C × 9/5 + 32
    /// 
    /// ```
    /// # use docs_and_testing::Temperature;
    /// let temp = Temperature::new(0.0);
    /// assert_eq!(temp.fahrenheit(), 32.0);
    /// 
    /// let temp = Temperature::new(100.0);
    /// assert_eq!(temp.fahrenheit(), 212.0);
    /// ```
    pub fn fahrenheit(&self) -> f64 {
        self.celsius * 9.0 / 5.0 + 32.0
    }
}

// ============================================================================
// 3. 单元测试演示
// ============================================================================

fn unit_test_demo() {
    println!("\n🔬 3. 单元测试演示");
    println!("{}", "-".repeat(40));
    
    println!("🧪 单元测试通常放在 #[cfg(test)] 模块中");
    println!("   使用 #[test] 属性标记测试函数");
    println!("   运行 'cargo test' 执行所有测试");
    
    // 演示测试辅助函数
    let result = add_two(3);
    println!("add_two(3) = {}", result);
    
    let greeting = greeting("世界");
    println!("问候: {}", greeting);
}

/// 将数字加 2
/// 
/// # 示例
/// 
/// ```
/// # use docs_and_testing::add_two;
/// assert_eq!(add_two(3), 5);
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}

/// 生成问候语
/// 
/// # 示例
/// 
/// ```
/// # use docs_and_testing::greeting;
/// assert_eq!(greeting("世界"), "你好, 世界!");
/// ```
pub fn greeting(name: &str) -> String {
    format!("你好, {}!", name)
}

/// 检查一个数是否为偶数
/// 
/// # 示例
/// 
/// ```
/// # use docs_and_testing::is_even;
/// assert!(is_even(4));
/// assert!(!is_even(3));
/// ```
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

// 单元测试模块
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculator_add() {
        let calc = Calculator::new();
        assert_eq!(calc.add(2, 3), 5);
        assert_eq!(calc.add(-1, 1), 0);
        assert_eq!(calc.add(0, 0), 0);
    }
    
    #[test]
    fn test_calculator_subtract() {
        let calc = Calculator::new();
        assert_eq!(calc.subtract(5, 3), 2);
        assert_eq!(calc.subtract(3, 5), -2);
        assert_eq!(calc.subtract(0, 0), 0);
    }
    
    #[test]
    fn test_calculator_multiply() {
        let calc = Calculator::new();
        assert_eq!(calc.multiply(4, 5), 20);
        assert_eq!(calc.multiply(-2, 3), -6);
        assert_eq!(calc.multiply(0, 100), 0);
    }
    
    #[test]
    fn test_calculator_divide() {
        let calc = Calculator::new();
        
        // 正常除法
        assert_eq!(calc.divide(10.0, 2.0), Ok(5.0));
        assert_eq!(calc.divide(7.0, 2.0), Ok(3.5));
        
        // 除零错误
        assert!(calc.divide(10.0, 0.0).is_err());
        
        // 检查错误消息
        match calc.divide(5.0, 0.0) {
            Err(msg) => assert_eq!(msg, "除数不能为零"),
            Ok(_) => panic!("应该返回错误"),
        }
    }
    
    #[test]
    fn test_calculator_square() {
        let calc = Calculator::new();
        assert_eq!(calc.square(4), 16);
        assert_eq!(calc.square(-3), 9);
        assert_eq!(calc.square(0), 0);
    }
    
    #[test]
    fn test_find_max() {
        // 测试整数
        let numbers = [1, 5, 3, 9, 2];
        assert_eq!(find_max(&numbers), Some(&9));
        
        // 测试空切片
        let empty: [i32; 0] = [];
        assert_eq!(find_max(&empty), None);
        
        // 测试单个元素
        let single = [42];
        assert_eq!(find_max(&single), Some(&42));
        
        // 测试字符
        let chars = ['a', 'z', 'm', 'b'];
        assert_eq!(find_max(&chars), Some(&'z'));
    }
    
    #[test]
    fn test_temperature() {
        let temp = Temperature::new(0.0);
        assert_eq!(temp.celsius(), 0.0);
        assert_eq!(temp.fahrenheit(), 32.0);
        
        let temp = Temperature::new(100.0);
        assert_eq!(temp.celsius(), 100.0);
        assert_eq!(temp.fahrenheit(), 212.0);
        
        // 测试 -40 度（摄氏度和华氏度相等的点）
        let temp = Temperature::new(-40.0);
        assert_eq!(temp.celsius(), -40.0);
        assert_eq!(temp.fahrenheit(), -40.0);
    }
    
    #[test]
    fn test_add_two() {
        assert_eq!(add_two(3), 5);
        assert_eq!(add_two(0), 2);
        assert_eq!(add_two(-2), 0);
    }
    
    #[test]
    fn test_greeting() {
        assert_eq!(greeting("世界"), "你好, 世界!");
        assert_eq!(greeting("Rust"), "你好, Rust!");
        assert_eq!(greeting(""), "你好, !");
    }
    
    #[test]
    fn test_is_even() {
        assert!(is_even(4));
        assert!(is_even(0));
        assert!(is_even(-2));
        
        assert!(!is_even(3));
        assert!(!is_even(1));
        assert!(!is_even(-1));
    }
    
    // 测试应该 panic 的情况
    #[test]
    #[should_panic]
    fn test_panic_example() {
        panic!("这个测试应该 panic");
    }
    
    // 测试应该 panic 并包含特定消息
    #[test]
    #[should_panic(expected = "特定错误")]
    fn test_panic_with_message() {
        panic!("这是特定错误消息");
    }
    
    // 使用 Result 的测试
    #[test]
    fn test_with_result() -> Result<(), String> {
        let calc = Calculator::new();
        
        if calc.add(2, 3) == 5 {
            Ok(())
        } else {
            Err("计算错误".to_string())
        }
    }
    
    // 忽略的测试
    #[test]
    #[ignore]
    fn expensive_test() {
        // 这个测试需要很长时间运行
        // 使用 cargo test -- --ignored 来运行被忽略的测试
        println!("运行昂贵的测试...");
    }
}

// ============================================================================
// 4. 测试组织演示
// ============================================================================

fn test_organization_demo() {
    println!("\n📁 4. 测试组织演示");
    println!("{}", "-".repeat(40));
    
    println!("🏗️ Rust 测试组织结构：");
    println!("  📦 src/");
    println!("  ├── 📄 lib.rs           # 库根文件");
    println!("  ├── 📄 main.rs          # 二进制根文件");
    println!("  └── 📁 modules/         # 模块文件");
    println!("      ├── 📄 mod.rs");
    println!("      └── 📄 calculator.rs");
    println!("  📦 tests/               # 集成测试目录");
    println!("  ├── 📄 integration_test.rs");
    println!("  ├── 📄 common/");
    println!("  │   └── 📄 mod.rs       # 测试辅助模块");
    println!("  └── 📄 another_test.rs");
    
    println!("\n🧪 测试类型：");
    println!("  • 单元测试：测试单个模块或函数");
    println!("  • 集成测试：测试库的公共 API");
    println!("  • 文档测试：测试文档中的示例代码");
    
    println!("\n⚙️ 测试运行选项：");
    println!("  • cargo test              # 运行所有测试");
    println!("  • cargo test unit_test    # 运行名称包含 'unit_test' 的测试");
    println!("  • cargo test -- --ignored # 运行被忽略的测试");
    println!("  • cargo test -- --nocapture # 显示 println! 输出");
}

// ============================================================================
// 5. 测试最佳实践
// ============================================================================

fn testing_best_practices() {
    println!("\n🎯 5. 测试最佳实践");
    println!("{}", "-".repeat(40));
    
    println!("✅ 测试最佳实践：");
    println!("  1. 每个函数都应该有测试");
    println!("  2. 测试边界条件和错误情况");
    println!("  3. 使用描述性的测试名称");
    println!("  4. 保持测试简单和独立");
    println!("  5. 使用断言宏：assert!, assert_eq!, assert_ne!");
    println!("  6. 测试应该快速且可重复");
    println!("  7. 使用文档测试确保示例代码有效");
    
    println!("\n🔍 断言宏示例：");
    
    // 演示不同的断言宏
    let x = 5;
    let y = 10;
    
    // 这些在正常运行时不会执行，只是演示语法
    println!("  assert!(x < y)           # 断言条件为真");
    println!("  assert_eq!(x + 5, y)     # 断言两值相等");
    println!("  assert_ne!(x, y)         # 断言两值不等");
    
    println!("\n📊 测试覆盖率：");
    println!("  • 使用 cargo-tarpaulin 等工具检查测试覆盖率");
    println!("  • 目标：至少 80% 的代码覆盖率");
    println!("  • 重要：覆盖率不是唯一指标，质量同样重要");
    
    println!("\n🚀 持续集成：");
    println!("  • 在 CI/CD 管道中自动运行测试");
    println!("  • 使用 GitHub Actions、GitLab CI 等");
    println!("  • 测试多个 Rust 版本和平台");
} 