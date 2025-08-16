// 教学示例 - 允许未使用的代码
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// 第22章：过程宏深入解析
// 演示过程宏的概念和应用（模拟实现）

fn main() {
    println!("🦀 第22章：过程宏深入解析");
    println!("=====================================");
    
    // 1. 过程宏基本概念
    proc_macro_concepts_demo();
    
    // 2. 派生宏演示
    derive_macro_demo();
    
    // 3. 属性宏演示
    attribute_macro_demo();
    
    // 4. 函数式宏演示
    function_like_macro_demo();
    
    // 5. Debug trait 自动实现演示
    custom_debug_demo();
    
    // 6. Builder 模式演示
    builder_pattern_demo();
    
    // 7. 序列化宏演示
    serialization_demo();
    
    // 8. ORM 映射演示
    orm_mapping_demo();
    
    // 9. API 路由演示
    api_route_demo();
    
    // 10. 状态机演示
    state_machine_demo();
    
    // 11. 所有权分析演示
    ownership_analysis_demo();
    
    // 12. 内存安全检查演示
    memory_safety_demo();
    
    println!("\n🎉 第22章过程宏深入解析演示完成！");
    println!("📚 您已经了解了过程宏的强大功能");
    println!("💡 注意：实际的过程宏需要单独的 crate 和特殊配置");
}

// ============================================================================
// 1. 过程宏基本概念
// ============================================================================

fn proc_macro_concepts_demo() {
    println!("\n📍 1. 过程宏基本概念");
    println!("{}", "-".repeat(40));
    
    println!("🔧 过程宏的三种类型：");
    println!("   1. 派生宏 (Derive Macros): #[derive(MyDerive)]");
    println!("   2. 属性宏 (Attribute Macros): #[my_attribute]");
    println!("   3. 函数式宏 (Function-like Macros): my_macro!()");
    
    println!("\n💡 过程宏特点：");
    println!("   • 在编译时操作 TokenStream");
    println!("   • 可以生成任意复杂的代码");
    println!("   • 需要单独的 proc-macro crate");
    println!("   • 使用 syn、quote、proc-macro2 库");
    println!("   • 比声明宏更强大但更复杂");
    
    println!("\n🔧 Cargo.toml 配置：");
    println!("   [lib]");
    println!("   proc-macro = true");
    println!("");
    println!("   [dependencies]");
    println!("   proc-macro2 = \"1.0\"");
    println!("   quote = \"1.0\"");
    println!("   syn = {{ version = \"2.0\", features = [\"full\"] }}");
}

// ============================================================================
// 2. 派生宏演示
// ============================================================================

fn derive_macro_demo() {
    println!("\n📍 2. 派生宏演示");
    println!("{}", "-".repeat(40));
    
    // 模拟派生宏生成的代码
    #[derive(Debug)]
    struct MyStruct {
        name: String,
        value: i32,
    }
    
    impl MyStruct {
        pub fn hello(&self) {
            println!("   Hello from {}", self.name);
        }
    }
    
    println!("🔧 派生宏使用示例：");
    println!("   // 原始代码");
    println!("   #[derive(MyDerive)]");
    println!("   struct MyStruct {{");
    println!("       name: String,");
    println!("       value: i32,");
    println!("   }}");
    
    println!("\n🔧 生成的代码（概念）：");
    let my_struct = MyStruct {
        name: "TestStruct".to_string(),
        value: 42,
    };
    
    println!("   创建实例: {:?}", my_struct);
    my_struct.hello();
    
    println!("\n💡 派生宏用途：");
    println!("   • 自动实现 trait");
    println!("   • 生成辅助方法");
    println!("   • 代码模板化");
    println!("   • 减少样板代码");
}

// ============================================================================
// 3. 属性宏演示
// ============================================================================

fn attribute_macro_demo() {
    println!("\n📍 3. 属性宏演示");
    println!("{}", "-".repeat(40));
    
    // 模拟属性宏修饰的结构体
    struct AttributeStruct {
        data: String,
    }
    
    impl AttributeStruct {
        fn new(data: String) -> Self {
            Self { data }
        }
        
        fn with_attribute(&self) {
            println!("   这个结构体有属性: some_arg");
        }
    }
    
    println!("🔧 属性宏使用示例：");
    println!("   // 原始代码");
    println!("   #[my_attribute(some_arg)]");
    println!("   struct AttributeStruct {{");
    println!("       data: String,");
    println!("   }}");
    
    println!("\n🔧 属性宏效果：");
    let attr_struct = AttributeStruct::new("test data".to_string());
    attr_struct.with_attribute();
    println!("   数据: {}", attr_struct.data);
    
    println!("\n💡 属性宏用途：");
    println!("   • 修改现有代码");
    println!("   • 添加额外功能");
    println!("   • 配置生成");
    println!("   • AOP 编程");
}

// ============================================================================
// 4. 函数式宏演示
// ============================================================================

fn function_like_macro_demo() {
    println!("\n📍 4. 函数式宏演示");
    println!("{}", "-".repeat(40));
    
    // 模拟函数式宏的行为
    macro_rules! simulate_my_macro {
        ($($input:tt)*) => {
            {
                let input_str = stringify!($($input)*);
                println!("   宏输入: {}", input_str);
                $($input)*
            }
        };
    }
    
    println!("🔧 函数式宏使用示例：");
    println!("   // 原始代码");
    println!("   my_macro! {{");
    println!("       let x = 5;");
    println!("       println!(\"x = {{}}\", x);");
    println!("   }}");
    
    println!("\n🔧 函数式宏效果：");
    simulate_my_macro! {
        let x = 5;
        println!("   x = {}", x);
    }
    
    println!("\n💡 函数式宏用途：");
    println!("   • 自定义语法");
    println!("   • DSL 构建");
    println!("   • 复杂代码生成");
    println!("   • 编译时计算");
}

// ============================================================================
// 5. Debug trait 自动实现演示
// ============================================================================

fn custom_debug_demo() {
    println!("\n📍 5. Debug trait 自动实现演示");
    println!("{}", "-".repeat(40));
    
    // 模拟自定义 Debug 实现
    struct Person {
        name: String,
        age: u32,
    }
    
    impl std::fmt::Debug for Person {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Person")
                .field("name", &self.name)
                .field("age", &self.age)
                .finish()
        }
    }
    
    enum Color {
        Red,
        Blue,
        Rgb(u8, u8, u8),
    }
    
    impl std::fmt::Debug for Color {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Color::Red => f.debug_struct("Color::Red").finish(),
                Color::Blue => f.debug_struct("Color::Blue").finish(),
                Color::Rgb(r, g, b) => {
                    f.debug_tuple("Color::Rgb")
                        .field(r)
                        .field(g)
                        .field(b)
                        .finish()
                }
            }
        }
    }
    
    println!("🔧 自定义 Debug 演示：");
    println!("   // 原始代码");
    println!("   #[derive(CustomDebug)]");
    println!("   struct Person {{ name: String, age: u32 }}");
    
    println!("\n🔧 生成的 Debug 实现：");
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    println!("   Person: {:?}", person);
    
    let color = Color::Rgb(255, 0, 0);
    println!("   Color: {:?}", color);
    
    println!("\n💡 自定义 Debug 优势：");
    println!("   • 统一的格式");
    println!("   • 支持各种数据结构");
    println!("   • 自动处理所有字段");
    println!("   • 减少手工实现");
}

// ============================================================================
// 6. Builder 模式演示
// ============================================================================

fn builder_pattern_demo() {
    println!("\n📍 6. Builder 模式演示");
    println!("{}", "-".repeat(40));
    
    // 手动实现 Builder 模式来模拟宏生成的代码
    struct User {
        name: String,
        email: String,
        age: u32,
    }
    
    struct UserBuilder {
        name: Option<String>,
        email: Option<String>,
        age: Option<u32>,
    }
    
    impl User {
        fn builder() -> UserBuilder {
            UserBuilder {
                name: None,
                email: None,
                age: None,
            }
        }
    }
    
    impl UserBuilder {
        fn name(mut self, value: String) -> Self {
            self.name = Some(value);
            self
        }
        
        fn email(mut self, value: String) -> Self {
            self.email = Some(value);
            self
        }
        
        fn age(mut self, value: u32) -> Self {
            self.age = Some(value);
            self
        }
        
        fn build(self) -> Result<User, String> {
            Ok(User {
                name: self.name.ok_or_else(|| "字段 'name' 未设置".to_string())?,
                email: self.email.ok_or_else(|| "字段 'email' 未设置".to_string())?,
                age: self.age.ok_or_else(|| "字段 'age' 未设置".to_string())?,
            })
        }
    }
    
    println!("🔧 Builder 模式演示：");
    println!("   // 原始代码");
    println!("   #[derive(Builder)]");
    println!("   struct User {{");
    println!("       name: String,");
    println!("       email: String,");
    println!("       age: u32,");
    println!("   }}");
    
    println!("\n🔧 Builder 使用：");
    match User::builder()
        .name("Alice".to_string())
        .email("alice@example.com".to_string())
        .age(30)
        .build()
    {
        Ok(user) => {
            println!("   创建用户成功:");
            println!("     姓名: {}", user.name);
            println!("     邮箱: {}", user.email);
            println!("     年龄: {}", user.age);
        }
        Err(e) => println!("   创建用户失败: {}", e),
    }
    
    println!("\n💡 Builder 模式优势：");
    println!("   • 可选参数设置");
    println!("   • 链式调用");
    println!("   • 编译时检查");
    println!("   • 清晰的 API");
}

// ============================================================================
// 7. 序列化宏演示
// ============================================================================

fn serialization_demo() {
    println!("\n📍 7. 序列化宏演示");
    println!("{}", "-".repeat(40));
    
    // 模拟序列化宏生成的代码
    struct Product {
        name: String,
        price: f64,
        in_stock: bool,
    }
    
    impl Product {
        fn serialize(&self) -> String {
            format!(
                "Product {{name:{:?},price:{:?},in_stock:{:?},}}",
                self.name, self.price, self.in_stock
            )
        }
    }
    
    println!("🔧 序列化宏演示：");
    println!("   // 原始代码");
    println!("   #[derive(SimpleSerialize)]");
    println!("   struct Product {{");
    println!("       name: String,");
    println!("       price: f64,");
    println!("       in_stock: bool,");
    println!("   }}");
    
    println!("\n🔧 序列化结果：");
    let product = Product {
        name: "Laptop".to_string(),
        price: 999.99,
        in_stock: true,
    };
    
    println!("   原始对象:");
    println!("     名称: {}", product.name);
    println!("     价格: {}", product.price);
    println!("     库存: {}", product.in_stock);
    
    println!("   序列化结果:");
    println!("     {}", product.serialize());
    
    println!("\n💡 序列化宏用途：");
    println!("   • 自动序列化实现");
    println!("   • 多种格式支持");
    println!("   • 自定义序列化规则");
    println!("   • 性能优化");
}

// ============================================================================
// 8. ORM 映射演示
// ============================================================================

fn orm_mapping_demo() {
    println!("\n📍 8. ORM 映射演示");
    println!("{}", "-".repeat(40));
    
    // 模拟 ORM 宏生成的代码
    struct Book {
        id: i32,
        title: String,
        author: String,
    }
    
    impl Book {
        fn table_name() -> &'static str {
            "books"
        }
        
        fn field_mappings() -> Vec<(&'static str, &'static str)> {
            vec![
                ("id", "id"),
                ("title", "title"),
                ("author", "author"),
            ]
        }
        
        fn select_all_sql() -> String {
            "SELECT id, title, author FROM books".to_string()
        }
        
        fn insert_sql() -> String {
            "INSERT INTO books (id, title, author) VALUES (?, ?, ?)".to_string()
        }
        
        fn find_by_id_sql() -> String {
            "SELECT id, title, author FROM books WHERE id = ?".to_string()
        }
    }
    
    println!("🔧 ORM 映射演示：");
    println!("   // 原始代码");
    println!("   #[derive(Model)]");
    println!("   #[table(name = \"books\")]");
    println!("   struct Book {{");
    println!("       id: i32,");
    println!("       #[column(name = \"title\")]");
    println!("       title: String,");
    println!("       author: String,");
    println!("   }}");
    
    println!("\n🔧 生成的 SQL 方法：");
    println!("   表名: {}", Book::table_name());
    println!("   字段映射: {:?}", Book::field_mappings());
    println!("   查询所有: {}", Book::select_all_sql());
    println!("   插入语句: {}", Book::insert_sql());
    println!("   按 ID 查询: {}", Book::find_by_id_sql());
    
    println!("\n💡 ORM 映射优势：");
    println!("   • 自动 SQL 生成");
    println!("   • 类型安全");
    println!("   • 字段映射");
    println!("   • 减少样板代码");
}

// ============================================================================
// 9. API 路由演示
// ============================================================================

fn api_route_demo() {
    println!("\n📍 9. API 路由演示");
    println!("{}", "-".repeat(40));
    
    // 模拟路由系统
    struct Route {
        path: String,
        method: String,
        handler: String,
    }
    
    struct Router {
        routes: Vec<Route>,
    }
    
    impl Router {
        fn new() -> Self {
            Self { routes: Vec::new() }
        }
        
        fn route(&mut self, path: &str, method: &str, handler: &str) {
            self.routes.push(Route {
                path: path.to_string(),
                method: method.to_string(),
                handler: handler.to_string(),
            });
        }
        
        fn print_routes(&self) {
            for route in &self.routes {
                println!("     {} {} -> {}", route.method, route.path, route.handler);
            }
        }
    }
    
    // 模拟处理函数
    fn get_users() -> String {
        "获取用户列表".to_string()
    }
    
    fn create_user() -> String {
        "创建新用户".to_string()
    }
    
    // 模拟宏生成的注册函数
    fn register_get_users(router: &mut Router) {
        router.route("/users", "GET", "get_users");
    }
    
    fn register_create_user(router: &mut Router) {
        router.route("/users", "POST", "create_user");
    }
    
    println!("🔧 API 路由演示：");
    println!("   // 原始代码");
    println!("   #[route(method = \"GET\", path = \"/users\")]");
    println!("   fn get_users() -> String {{ ... }}");
    println!("");
    println!("   #[route(method = \"POST\", path = \"/users\")]");
    println!("   fn create_user() -> String {{ ... }}");
    
    println!("\n🔧 路由注册：");
    let mut router = Router::new();
    register_get_users(&mut router);
    register_create_user(&mut router);
    
    println!("   注册的路由:");
    router.print_routes();
    
    println!("\n🔧 处理函数调用：");
    println!("   GET /users: {}", get_users());
    println!("   POST /users: {}", create_user());
    
    println!("\n💡 API 路由宏优势：");
    println!("   • 声明式路由定义");
    println!("   • 自动注册");
    println!("   • 类型安全");
    println!("   • 减少配置代码");
}

// ============================================================================
// 10. 状态机演示
// ============================================================================

fn state_machine_demo() {
    println!("\n📍 10. 状态机演示");
    println!("{}", "-".repeat(40));
    
    // 手动实现状态机来模拟宏生成的代码
    #[derive(Debug, Clone, PartialEq)]
    enum TrafficLightState {
        Red,
        Yellow,
        Green,
    }
    
    struct TrafficLight {
        state: TrafficLightState,
    }
    
    impl TrafficLight {
        fn new() -> Self {
            Self {
                state: TrafficLightState::Red,
            }
        }
        
        fn current_state(&self) -> &TrafficLightState {
            &self.state
        }
        
        fn next(&mut self) -> Result<(), String> {
            match self.state {
                TrafficLightState::Red => {
                    self.state = TrafficLightState::Green;
                    Ok(())
                }
                TrafficLightState::Green => {
                    self.state = TrafficLightState::Yellow;
                    Ok(())
                }
                TrafficLightState::Yellow => {
                    self.state = TrafficLightState::Red;
                    Ok(())
                }
            }
        }
        
        fn emergency_stop(&mut self) -> Result<(), String> {
            self.state = TrafficLightState::Red;
            Ok(())
        }
    }
    
    println!("🔧 状态机演示：");
    println!("   // 原始代码");
    println!("   state_machine! {{");
    println!("       struct TrafficLight {{");
    println!("           states: {{ Red, Yellow, Green }}");
    println!("           transitions: {{");
    println!("               Red -> Green on next,");
    println!("               Green -> Yellow on next,");
    println!("               Yellow -> Red on next,");
    println!("           }}");
    println!("       }}");
    println!("   }}");
    
    println!("\n🔧 状态机操作：");
    let mut light = TrafficLight::new();
    println!("   初始状态: {:?}", light.current_state());
    
    light.next().unwrap();
    println!("   下一状态: {:?}", light.current_state());
    
    light.next().unwrap();
    println!("   下一状态: {:?}", light.current_state());
    
    light.emergency_stop().unwrap();
    println!("   紧急停止: {:?}", light.current_state());
    
    println!("\n💡 状态机宏优势：");
    println!("   • 声明式状态定义");
    println!("   • 自动转换验证");
    println!("   • 类型安全");
    println!("   • 清晰的状态逻辑");
}

// ============================================================================
// 11. 所有权分析演示
// ============================================================================

fn ownership_analysis_demo() {
    println!("\n📍 11. 所有权分析演示");
    println!("{}", "-".repeat(40));
    
    // 模拟所有权分析宏的功能
    fn analyze_function(name: &str, params: &[&str], return_type: &str) {
        println!("   函数 {} 的所有权分析:", name);
        for (i, param) in params.iter().enumerate() {
            println!("     参数 {}: {}", i, param);
        }
        println!("     返回: {}", return_type);
    }
    
    // 模拟被分析的函数
    fn take_ownership(_data: Vec<i32>) -> String {
        "所有权转移".to_string()
    }
    
    fn borrow_data(_data: &Vec<i32>) -> &str {
        "借用引用"
    }
    
    fn borrow_mut(_data: &mut Vec<i32>) {
        // 可变借用
    }
    
    println!("🔧 所有权分析演示：");
    println!("   // 原始代码");
    println!("   #[analyze_ownership]");
    println!("   fn take_ownership(data: Vec<i32>) -> String {{ ... }}");
    println!("");
    println!("   #[analyze_ownership]");
    println!("   fn borrow_data(data: &Vec<i32>) -> &str {{ ... }}");
    
    println!("\n🔧 分析结果：");
    analyze_function(
        "take_ownership",
        &["参数 0: 所有权转移"],
        "返回: 所有权转移",
    );
    
    analyze_function(
        "borrow_data",
        &["参数 0: 不可变借用"],
        "返回: 借用引用",
    );
    
    analyze_function(
        "borrow_mut",
        &["参数 0: 可变借用"],
        "返回: ()",
    );
    
    println!("\n🔧 实际函数调用：");
    let data = vec![1, 2, 3];
    println!("   原始数据: {:?}", data);
    
    let result = take_ownership(data);
    println!("   函数结果: {}", result);
    // data 在这里不再可用
    
    let data2 = vec![4, 5, 6];
    let borrowed_result = borrow_data(&data2);
    println!("   借用结果: {}", borrowed_result);
    println!("   原始数据仍可用: {:?}", data2);
    
    println!("\n💡 RWO 权限分析：");
    println!("   • R (Read): 不可变借用 &T");
    println!("   • W (Write): 可变借用 &mut T");
    println!("   • O (Own): 所有权转移 T");
    println!("   • 编译时验证内存安全");
}

// ============================================================================
// 12. 内存安全检查演示
// ============================================================================

fn memory_safety_demo() {
    println!("\n📍 12. 内存安全检查演示");
    println!("{}", "-".repeat(40));
    
    // 模拟内存安全宏生成的代码
    struct SafeData {
        value: i32,
        text: String,
    }
    
    impl SafeData {
        // 安全的获取方法
        fn safe_get_value(&self) -> Option<&i32> {
            Some(&self.value)
        }
        
        fn safe_get_text(&self) -> Option<&String> {
            Some(&self.text)
        }
        
        // 安全的设置方法
        fn safe_set_value(&mut self, value: i32) -> Result<i32, String> {
            let old_value = std::mem::replace(&mut self.value, value);
            Ok(old_value)
        }
        
        fn safe_set_text(&mut self, text: String) -> Result<String, String> {
            let old_text = std::mem::replace(&mut self.text, text);
            Ok(old_text)
        }
        
        // 内存安全检查
        fn memory_check(&self) -> bool {
            // 基本的内存有效性检查
            true
        }
        
        // 安全的克隆
        fn safe_clone(&self) -> Result<Self, String> {
            Ok(SafeData {
                value: self.value,
                text: self.text.clone(),
            })
        }
        
        // 安全的销毁
        fn safe_destroy(self) -> Result<(), String> {
            drop(self);
            Ok(())
        }
    }
    
    println!("🔧 内存安全检查演示：");
    println!("   // 原始代码");
    println!("   #[derive(MemorySafe)]");
    println!("   struct SafeData {{");
    println!("       value: i32,");
    println!("       text: String,");
    println!("   }}");
    
    println!("\n🔧 安全操作演示：");
    let mut data = SafeData {
        value: 42,
        text: "Hello".to_string(),
    };
    
    // 安全获取
    if let Some(value) = data.safe_get_value() {
        println!("   安全获取 value: {}", value);
    }
    
    if let Some(text) = data.safe_get_text() {
        println!("   安全获取 text: {}", text);
    }
    
    // 安全设置
    match data.safe_set_value(100) {
        Ok(old_value) => println!("   安全设置 value: {} -> 100", old_value),
        Err(e) => println!("   设置失败: {}", e),
    }
    
    // 内存检查
    if data.memory_check() {
        println!("   内存检查: 通过");
    }
    
    // 安全克隆
    match data.safe_clone() {
        Ok(_cloned) => println!("   安全克隆: 成功"),
        Err(e) => println!("   克隆失败: {}", e),
    }
    
    // 安全销毁
    match data.safe_destroy() {
        Ok(()) => println!("   安全销毁: 成功"),
        Err(e) => println!("   销毁失败: {}", e),
    }
    
    println!("\n💡 内存安全宏优势：");
    println!("   • 自动生成安全访问器");
    println!("   • 运行时检查");
    println!("   • 错误处理");
    println!("   • 内存泄漏防护");
} 