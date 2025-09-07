### Serde

`serde` 是 Rust 中一个功能强大、泛型的序列化和反序列化框架，用于高效地将 Rust 数据结构转换为各种格式（如 JSON、YAML、TOML 等），并反之。它支持 derive 宏自动生成实现，支持自定义行为，并与众多数据格式集成。Serde 的核心是 `Serialize` 和 `Deserialize` trait，允许数据结构与格式解耦。

#### 1. 安装 Serde
在你的 `Cargo.toml` 文件中添加依赖。推荐启用 `derive` 特性以使用宏自动生成 trait 实现。对于特定格式，如 JSON，需要额外添加相应 crate（如 `serde_json`）。

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }  # 启用 derive
serde_json = "1.0"  # 用于 JSON 支持
```

运行 `cargo build` 安装。Serde 支持 MSRV 1.56，支持 no-std 通过禁用默认特性。其他格式如 YAML（`serde_yaml`）、TOML（`toml`）等可类似添加。

#### 2. 基本用法
Serde 的核心是两个 trait：`Serialize`（序列化）和 `Deserialize`（反序列化）。使用 `#[derive(Serialize, Deserialize)]` 宏自动实现。

基本语法：

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };
    let serialized = serde_json::to_string(&point).unwrap();  // 序列化为 JSON 字符串
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();  // 反序列化
}
```

支持字符串、值（`Value`）、流等形式。

#### 3. 语义和实现细节
- **数据模型**：Serde 使用 Visitor 模式处理不同类型（如结构体、枚举、映射），确保高效和泛型。
- **序列化**：将 Rust 数据转换为格式（如 JSON 对象）。支持内置类型如 `String`、`Vec`、`HashMap`。
- **反序列化**：从格式解析回 Rust 数据。支持 `'de` 生命周期以处理借用数据。
- **属性**：使用 `#[serde(rename = "new_name")]`、`#[serde(skip)]` 等自定义字段名、跳过字段、默认值等。
- **错误处理**：函数返回 `Result<T, Error>`，错误类型如 `serde_json::Error` 支持链式原因。
- **性能**：零拷贝反序列化（借用数据），基准显示优于许多语言的类似库；堆分配最小化。

#### 4. 高级用法
- **自定义序列化**：实现 `Serialize` 和 `Deserialize` trait，使用 `Serializer` 和 `Deserializer` 接口。结合 Visitor 处理复杂逻辑。
- **枚举**：支持无标签、内部标签、外部标签、相邻标签变体。
- **泛型和 trait bound**：函数如 `fn serialize<T: Serialize>(t: &T)`。
- **多格式支持**：切换格式只需更换 crate（如 `serde_yaml::to_string`）。
- **流式处理**：使用 `Serializer`/`Deserializer` 处理大文件。
- **no-std**：禁用默认特性，支持嵌入式。
- **集成**：与 `reqwest`（网络）、`csv`（文件）、`tokio`（异步）等结合。

#### 5. 注意事项
- Derive 宏适合大多数场景，手动实现用于自定义格式或复杂逻辑。
- 确保类型实现 `Serialize`/`Deserialize`；自定义类型需实现 `FromStr` 或 Visitor。
- 性能开销低，但自定义 Visitor 可能增加复杂性；测试大输入。
- 避免在库中暴露 Serde 细节，使用具体错误。
- 与 `anyhow` 集成处理错误。

#### 6. 替代方案
- **bincode**：Serde 的二进制格式，适合性能关键场景。
- **prost** 或 **capnp**：协议缓冲或 Cap'n Proto，适合跨语言。
- **ron**：Rust 对象表示法，人类可读。
- **postcard**：高效二进制。
  Serde 被视为 Rust 序列化的标准。

#### 7. 20 个例子
以下是 20 个例子，从简单到复杂，覆盖结构体、枚举、自定义等。每个例子包括代码、输出（如果适用）和解释。假设已导入 `use serde::{Deserialize, Serialize};` 和 `use serde_json::{json, from_str, to_string};`。

##### 示例 1: 基本 Point 结构体
```rust
#[derive(Serialize, Deserialize, Debug)]
struct Point { x: i32, y: i32 }

fn main() {
    let point = Point { x: 1, y: 2 };
    let serialized = to_string(&point).unwrap();
    println!("{}", serialized);
    let deserialized: Point = from_str(&serialized).unwrap();
    println!("{:?}", deserialized);
}
```
输出：`{"x":1,"y":2}\nPoint { x: 1, y: 2 }`  
解释：基本序列化和反序列化。

##### 示例 2: Address 结构体
```rust
#[derive(Serialize, Deserialize)]
struct Address { street: String, city: String }

fn main() {
    let address = Address { street: "10 Downing Street".to_owned(), city: "London".to_owned() };
    let j = to_string(&address).unwrap();
    println!("{}", j);
}
```
输出：`{"street":"10 Downing Street","city":"London"}`  
解释：字符串字段序列化。

##### 示例 3: Person 结构体（类型化）
```rust
#[derive(Serialize, Deserialize)]
struct Person { name: String, age: u8, phones: Vec<String> }

fn main() {
    let data = r#"{ "name": "John Doe", "age": 43, "phones": ["+44 1234567", "+44 2345678"] }"#;
    let p: Person = from_str(data).unwrap();
    println!("Name: {}", p.name);
}
```
输出：`Name: John Doe`  
解释：反序列化到自定义结构体。

##### 示例 4: 无类型 JSON 值
```rust
use serde_json::Value;

fn main() {
    let data = r#"{ "name": "John Doe", "age": 43, "phones": ["+44 1234567", "+44 2345678"] }"#;
    let v: Value = from_str(data).unwrap();
    println!("Name: {}", v["name"]);
}
```
输出：`Name: "John Doe"`  
解释：处理动态 JSON。

##### 示例 5: json! 宏构建
```rust
fn main() {
    let john = json!({ "name": "John Doe", "age": 43, "phones": ["+44 1234567"] });
    println!("{}", john);
}
```
输出：`{"age":43,"name":"John Doe","phones":["+44 1234567"]}`  
解释：手动构建 JSON 值。

##### 示例 6: 单元结构体
```rust
#[derive(Serialize, Deserialize, Debug)]
struct U;

fn main() {
    let u = U;
    let s = to_string(&u).unwrap();
    println!("{}", s);
    let d: U = from_str("null").unwrap();
    println!("{:?}", d);
}
```
输出：`null\nU`  
解释：序列化为 null。

##### 示例 7: 元组结构体
```rust
#[derive(Serialize, Deserialize, Debug)]
struct T(u8, f64, bool, String);

fn main() {
    let t = T(10, 3.14159, true, "Hello".to_owned());
    let s = to_string(&t).unwrap();
    println!("{}", s);
    let d: T = from_str(&s).unwrap();
    println!("{:?}", d);
}
```
输出：`[10,3.14159,true,"Hello"]\nT(10, 3.14159, true, "Hello")`  
解释：序列化为数组。

##### 示例 8: Newtype 结构体
```rust
#[derive(Serialize, Deserialize, Debug)]
struct N(i32);

fn main() {
    let n = N(10);
    let s = to_string(&n).unwrap();
    println!("{}", s);
    let d: N = from_str("10").unwrap();
    println!("{:?}", d);
}
```
输出：`10\nN(10)`  
解释：序列化为内部值。

##### 示例 9: 带名字段的结构体
```rust
#[derive(Serialize, Deserialize, Debug)]
struct C { a: i32, b: f64, c: bool, d: String }

fn main() {
    let c = C { a: 10, b: 3.14159, c: true, d: "Hello".to_owned() };
    let s = to_string(&c).unwrap();
    println!("{}", s);
    let d: C = from_str(&s).unwrap();
    println!("{:?}", d);
}
```
输出：`{"a":10,"b":3.14159,"c":true,"d":"Hello"}\nC { a: 10, b: 3.14159, c: true, d: "Hello" }`  
解释：序列化为对象。

##### 示例 10: MyStruct 基本
```rust
#[derive(Deserialize, Serialize)]
struct MyStruct { message: String }

fn main() {
    let json = json!({"message": "Hello world!"});
    let my_struct: MyStruct = from_str(&json.to_string()).unwrap();
    println!("{}", to_string(&my_struct).unwrap());
}
```
输出：`{"message":"Hello world!"}`  
解释：简单结构体。

##### 示例 11: 自定义序列化 Color
```rust
use serde::ser::{Serialize, Serializer, SerializeStruct};

struct Color { r: u8, g: u8, b: u8 }

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("Color", 3)?;
        state.serialize_field("r", &self.r)?;
        state.serialize_field("g", &self.g)?;
        state.serialize_field("b", &self.b)?;
        state.end()
    }
}

fn main() {
    let color = Color { r: 255, g: 0, b: 0 };
    println!("{}", to_string(&color).unwrap());
}
```
输出：`{"r":255,"g":0,"b":0}`  
解释：手动实现 Serialize。

##### 示例 12: 自定义 Visitor 反序列化
```rust
use std::fmt;
use serde::de::{self, Visitor, Deserialize, Deserializer};

struct MyStruct { message: String }

struct MessageVisitor;

impl<'de> Visitor<'de> for MessageVisitor {
    type Value = String;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A message")
    }
    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> { Ok(value) }
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> { Ok(value.to_owned()) }
    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E> { Ok(value.to_string()) }
}

impl<'de> Deserialize<'de> for MyStruct {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let message = deserializer.deserialize_string(MessageVisitor)?;
        Ok(Self { message })
    }
}

fn main() {
    let data = "42";
    let s: MyStruct = from_str(data).unwrap();
    println!("{}", s.message);
}
```
输出：`42`  
解释：自定义 Visitor 处理多种类型。

##### 示例 13: 枚举变体（外部标签）
```rust
#[derive(Serialize, Deserialize, Debug)]
enum Shape { Circle { radius: f64 }, Rectangle { width: f64, height: f64 } }

fn main() {
    let circle = Shape::Circle { radius: 5.0 };
    let s = to_string(&circle).unwrap();
    println!("{}", s);
    let d: Shape = from_str(&s).unwrap();
    println!("{:?}", d);
}
```
输出：`{"Circle":{"radius":5.0}}\nCircle { radius: 5.0 }`  
解释：枚举序列化带标签。

##### 示例 14: 可选字段
```rust
#[derive(Serialize, Deserialize, Debug)]
struct Opt { value: Option<i32> }

fn main() {
    let opt = Opt { value: Some(42) };
    println!("{}", to_string(&opt).unwrap());
    let opt_none = Opt { value: None };
    println!("{}", to_string(&opt_none).unwrap());
}
```
输出：`{"value":42}\n{"value":null}`  
解释：Option 处理 null。

##### 示例 15: 重名字段
```rust
#[derive(Serialize, Deserialize, Debug)]
struct Renamed { #[serde(rename = "new_name")] old_name: String }

fn main() {
    let r = Renamed { old_name: "test".to_owned() };
    println!("{}", to_string(&r).unwrap());
}
```
输出：`{"new_name":"test"}`  
解释：使用属性自定义字段名。

##### 示例 16: 跳过字段
```rust
#[derive(Serialize, Deserialize, Debug)]
struct Skip { value: i32, #[serde(skip)] ignored: String }

fn main() {
    let s = Skip { value: 42, ignored: "skip".to_owned() };
    println!("{}", to_string(&s).unwrap());
}
```
输出：`{"value":42}`  
解释：忽略字段序列化。

##### 示例 17: 默认值
```rust
#[derive(Serialize, Deserialize, Debug)]
struct DefaultVal { #[serde(default)] optional: Option<String> }

fn main() {
    let data = "{}";
    let d: DefaultVal = from_str(data).unwrap();
    println!("{:?}", d.optional);
}
```
输出：`None`  
解释：缺失字段使用默认。

##### 示例 18: 泛型函数
```rust
fn serialize_to_json<T: Serialize>(t: &T) -> String {
    to_string(t).unwrap()
}

fn main() {
    let point = Point { x: 1, y: 2 };
    println!("{}", serialize_to_json(&point));
}
```
输出：`{"x":1,"y":2}`  
解释：泛型序列化。

##### 示例 19: 文件读写（模拟）
```rust
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let point = Point { x: 1, y: 2 };
    let mut file = File::create("point.json").unwrap();
    file.write_all(to_string(&point).unwrap().as_bytes()).unwrap();

    let mut file = File::open("point.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let p: Point = from_str(&data).unwrap();
    println!("{:?}", p);
}
```
解释：序列化到文件。

##### 示例 20: 枚举无标签
```rust
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Untagged { A(i32), B(String) }

fn main() {
    let a = Untagged::A(42);
    println!("{}", to_string(&a).unwrap());
    let b: Untagged = from_str("\"text\"").unwrap();
    println!("{:?}", b);
}
```
输出：`42\nB("text")`  
解释：无标签枚举变体。
