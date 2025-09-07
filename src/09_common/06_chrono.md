### Chrono

`chrono` 是 Rust 中一个功能全面的日期和时间库，旨在成为标准 `time` 库的超集。它支持处理日期、时间、时区、持续时间，并严格遵守 ISO 8601 标准。Chrono 默认提供时区感知的 `DateTime` 类型，同时有无时区的朴素类型（如 `NaiveDateTime`），支持解析、格式化、算术运算，并处理无效或歧义结果（如闰秒或 DST 转换）。它高效、空间优化，并可与 Serde 等集成序列化。

#### 1. 安装 Chrono
在你的 `Cargo.toml` 文件中添加依赖。最新版本为 0.4.38（发布于 2024 年 12 月 9 日）。 推荐启用可选特性，如 `serde` 用于序列化、`clock` 用于本地时区、`unstable-locales` 用于本地化。

```toml
[dependencies]
chrono = "0.4.38"  # 基本版本
```

启用特性示例：

```toml
chrono = { version = "0.4.38", features = ["serde", "clock", "unstable-locales"] }
```

对于完整时区支持（如 IANA 时区），添加伴侣 crate 如 `chrono-tz`：

```toml
chrono-tz = "0.9"  # 或最新
```

运行 `cargo build` 安装。Chrono 支持 MSRV 1.61.0，并可通过禁用默认特性支持 no-std 环境。

#### 2. 基本用法
Chrono 的核心类型包括 `DateTime<Tz>`（时区感知日期时间）、`NaiveDateTime`（无时区）和 `Duration`（持续时间）。使用 `Utc::now()` 或 `Local::now()` 获取当前时间。

基本语法：

```rust
use chrono::{DateTime, Utc, Local};

fn main() {
    let utc_now: DateTime<Utc> = Utc::now();
    let local_now: DateTime<Local> = Local::now();
    println!("UTC: {}", utc_now);
    println!("Local: {}", local_now);
}
```

解析和格式化使用 `parse_from_str` 和 `format` 方法，支持 `strftime` 风格的格式化字符串。

#### 3. 语义和实现细节
- **DateTime**：结合日期和时间，支持时区。操作可能返回 `Option` 或 `MappedLocalTime` 以处理无效结果（如 DST 间隙）。
- **Naive 类型**：无时区，用于低级构建块。支持闰秒表示，但不完全处理。
- **Duration/TimeDelta**：表示精确持续时间（秒 + 纳秒），不支持名义单位（如月）。算术运算如 `dt + delta`。
- **时区**：`Utc` 最高效；`FixedOffset` 用于固定偏移；`Local` 使用系统时区（需 `clock` 特性）。
- **解析/格式化**：使用 `%Y-%m-%d %H:%M:%S` 等占位符。支持本地化（需 `unstable-locales`）。
- **错误处理**：函数返回 `Result` 或 `Option`，如解析失败返回 `ParseError`。
- **性能**：合理高效，格式化性能在 0.4.38 中提升约 20%。 时区数据不内置以减少二进制大小，使用 `chrono-tz` 加载。

#### 4. 高级用法
- **时区转换**：使用 `with_timezone` 方法，如 `dt.with_timezone(&Utc)`。
- **算术**：使用 `TimeDelta::try_days(1)` 等构建持续时间，支持加减。
- **本地化**：使用 `format_localized` 和 `Locale`。
- **序列化**：与 Serde 集成，使用 `#[serde(with = "chrono::serde::ts_seconds")]` 等。
- **异步/多线程**：类型实现 `Send + Sync`，适合 Tokio 等。避免多线程修改 `TZ` 环境变量。
- **分布式时区**：结合 `chrono-tz` 支持 IANA 时区，如 `Tz::Europe__London`。
- **集成**：与 Diesel（数据库）、PyO3（Python 互操作）、Reqwest（网络时间戳）等结合。

#### 5. 注意事项
- 时区数据不内置：默认仅支持 UTC 和固定偏移；使用 `chrono-tz` 或 `tzfile` 获取完整支持。
- 无效日期：如 2 月 30 日，返回 `None`。
- 闰秒：表示但不完全支持；本地时区查询可能忽略。
- 环境变量：0.4.20 后不再使用 `localtime_r`，改用 Rust 代码查询 `TZ`，避免多线程问题。
- 性能开销：`Local` 涉及系统调用；大型日期范围操作可能慢。
- 弃用：旧类型如 `Date`、`MAX_DATE` 已弃用，转用 `NaiveDate` 等。
- 与其他库冲突：如 Arrow，可能函数名冲突。

#### 6. 替代方案
- **std::time**：标准库，提供基本 `Instant`、`SystemTime`、`Duration`，但无时区或高级日期支持。
- **time crate**：类似 Chrono，但更注重性能和 no-std；支持格式化和偏移。
- **chrono-tz**：Chrono 的扩展，用于完整 IANA 时区。
- **bincode** 或 **serde**：用于序列化时间，但依赖 Chrono。
  Chrono 被视为 Rust 日期时间的标准选择。

#### 7. 20 个例子
以下是 20 个例子，从简单到复杂，覆盖基本到高级场景。每个例子包括代码、输出（如果适用）和解释。假设已导入 `use chrono::{DateTime, Duration, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta, Utc};`。对于时区示例，可能需 `chrono-tz`。

##### 示例 1: 获取当前 UTC 时间
```rust
fn main() {
    let now: DateTime<Utc> = Utc::now();
    println!("{}", now);
}
```
输出：`2025-09-03 00:00:00 UTC`（实际取决于时间）  
解释：基本获取当前时间。

##### 示例 2: 获取本地时间
```rust
fn main() {
    let now: DateTime<Local> = Local::now();
    println!("{}", now);
}
```
解释：使用系统本地时区。

##### 示例 3: 手动构建 NaiveDate
```rust
fn main() {
    let date = NaiveDate::from_ymd_opt(2025, 9, 3).unwrap();
    println!("{}", date);
}
```
输出：`2025-09-03`  
解释：无时区日期。

##### 示例 4: 构建 NaiveTime
```rust
fn main() {
    let time = NaiveTime::from_hms_opt(12, 34, 56).unwrap();
    println!("{}", time);
}
```
输出：`12:34:56`  
解释：无时区时间。

##### 示例 5: 构建 NaiveDateTime
```rust
fn main() {
    let dt = NaiveDateTime::from_timestamp_opt(0, 0).unwrap();
    println!("{}", dt);
}
```
输出：`1970-01-01 00:00:00`  
解释：从 Unix 时间戳。

##### 示例 6: 固定偏移时区
```rust
fn main() {
    let offset = FixedOffset::east_opt(5 * 3600).unwrap();
    let dt = Utc::now().with_timezone(&offset);
    println!("{}", dt);
}
```
解释：UTC+05:00 时区。

##### 示例 7: 解析字符串
```rust
fn main() {
    let dt: DateTime<Utc> = DateTime::parse_from_rfc3339("2025-09-03T12:00:00Z").unwrap();
    println!("{}", dt);
}
```
输出：`2025-09-03 12:00:00 UTC`  
解释：RFC 3339 格式。

##### 示例 8: 自定义格式化
```rust
fn main() {
    let now = Utc::now();
    let formatted = now.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("{}", formatted);
}
```
解释：strftime 风格。

##### 示例 9: 持续时间加法
```rust
fn main() {
    let now = Utc::now();
    let delta = TimeDelta::try_hours(1).unwrap();
    let later = now + delta;
    println!("Later: {}", later);
}
```
解释：添加 1 小时。

##### 示例 10: 计算持续时间
```rust
fn main() {
    let dt1 = Utc.with_ymd_and_hms(2025, 9, 3, 0, 0, 0).unwrap();
    let dt2 = Utc.with_ymd_and_hms(2025, 9, 4, 0, 0, 0).unwrap();
    let duration: Duration = dt2 - dt1;
    println!("Days: {}", duration.num_days());
}
```
输出：`Days: 1`  
解释：日期差。

##### 示例 11: 时区转换
```rust
use chrono_tz::Tz;

fn main() {
    let now = Utc::now();
    let london = now.with_timezone(&Tz::Europe__London);
    println!("London: {}", london);
}
```
解释：需 `chrono-tz`。

##### 示例 12: 周几计算
```rust
fn main() {
    let date = NaiveDate::from_ymd_opt(2025, 9, 3).unwrap();
    println!("Weekday: {:?}", date.weekday());
}
```
输出：`Weekday: Wed`  
解释：获取星期。

##### 示例 13: 闰年检查
```rust
fn main() {
    let date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
    println!("Leap year: {}", date.year() % 4 == 0 && (date.year() % 100 != 0 || date.year() % 400 == 0));
}
```
输出：`Leap year: true`  
解释：手动检查。

##### 示例 14: Unix 时间戳
```rust
fn main() {
    let now = Utc::now();
    let timestamp = now.timestamp();
    let from_ts: DateTime<Utc> = DateTime::from_timestamp(timestamp, 0).unwrap();
    println!("From TS: {}", from_ts);
}
```
解释：时间戳转换。

##### 示例 15: 本地化格式
```rust
use chrono::Locale;

fn main() {
    let now = Utc::now();
    let formatted = now.format_localized("%A %e %B %Y", Locale::fr_FR).to_string();
    println!("{}", formatted);
}
```
解释：需 `unstable-locales`。

##### 示例 16: 日期迭代
```rust
fn main() {
    let start = NaiveDate::from_ymd_opt(2025, 9, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(2025, 9, 5).unwrap();
    let mut date = start;
    while date <= end {
        println!("{}", date);
        date = date.succ_opt().unwrap();
    }
}
```
解释：日期循环。

##### 示例 17: 持续时间分解
```rust
fn main() {
    let delta = TimeDelta::try_days(10).unwrap() + TimeDelta::try_hours(5).unwrap();
    println!("Hours: {}", delta.num_hours());
}
```
输出：`Hours: 245`  
解释：单位转换。

##### 示例 18: 测试用 DateTime
```rust
fn main() {
    let dt = Utc.with_ymd_and_hms(2025, 9, 3, 12, 0, 0).unwrap();
    println!("{}", dt);
}
```
解释：固定时间用于测试。

##### 示例 19: 时区歧义处理
```rust
use chrono::offset::LocalResult;

fn main() {
    let offset = FixedOffset::east_opt(3600).unwrap();
    let naive = NaiveDateTime::new(NaiveDate::from_ymd_opt(2025, 3, 30).unwrap(), NaiveTime::from_hms_opt(2, 30, 0).unwrap());
    let result = offset.from_local_datetime(&naive);
    match result {
        LocalResult::Single(dt) => println!("Single: {}", dt),
        LocalResult::Ambiguous(dt1, dt2) => println!("Ambiguous: {} or {}", dt1, dt2),
        LocalResult::None => println!("None"),
    }
}
```
解释：处理 DST 转换。

##### 示例 20: Serde 集成
```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Event {
    #[serde(with = "chrono::serde::ts_seconds")]
    timestamp: DateTime<Utc>,
}

fn main() {
    let event = Event { timestamp: Utc::now() };
    let json = serde_json::to_string(&event).unwrap();
    println!("{}", json);
}
```
解释：序列化时间戳（需 `serde` 特性）。
