### tracing

`tracing` 是 Rust 中一个强大的框架，用于在程序中收集结构化、基于事件的诊断信息。它特别适用于异步系统（如 Tokio），通过 spans（跨度）、events（事件）和 subscribers（订阅者）来记录结构化的数据，包括时间性和因果关系。不同于传统的日志记录，`tracing` 允许添加上下文、字段和层次结构，支持分布式追踪、指标和日志。`tracing` 的设计目标是高性能、低开销，并与 OpenTelemetry 等标准集成。
s
#### 1. 安装 tracing
在你的 `Cargo.toml` 文件中添加依赖。推荐同时添加 `tracing-subscriber` 用于实际收集数据。对于 JSON 输出或 OpenTelemetry 支持，可启用相应特性。

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt"] }  # 用于订阅者和格式化
```

对于 OpenTelemetry 集成：

```toml
tracing-opentelemetry = "0.25"
opentelemetry = "0.25"
```

运行 `cargo build` 安装。`tracing` 支持 MSRV 1.63+，并可通过禁用 "std" 和 "alloc" 特性支持 no-std 环境。

#### 2. 基本用法
`tracing` 的核心是 `Span`（跨度）和 `Event`（事件）。使用宏如 `span!`、`event!` 和 `#[instrument]` 添加仪器化。

基本语法：

```rust
use tracing::{info, span, Level};

fn main() {
    tracing_subscriber::fmt::init();  // 初始化订阅者
    let span = span!(Level::INFO, "my_span");
    let _enter = span.enter();  // 进入跨度
    info!("Hello from tracing!");
}
```

`#[instrument]` 属性可自动为函数添加跨度：

```rust
use tracing::instrument;

#[instrument]
fn my_function(arg: i32) {
    info!("Inside function with arg: {}", arg);
}
```

事件使用 `event!` 或级别宏如 `info!` 记录。

#### 3. 语义和实现细节
- **Spans**：表示时间段，支持嵌套。进入/退出通过 RAII 守卫记录。
- **Events**：表示瞬间，支持字段和级别（TRACE, DEBUG, INFO, WARN, ERROR）。
- **Subscribers**：实现 `Subscriber` trait，收集数据。过滤通过元数据（如级别、目标）优化性能，避免不必要的构造。
- **字段**：键值对，使用 `field_name = value` 语法，支持 `?`（Debug）和 `%`（Display）格式化。
- **错误处理**：事件可记录错误，使用 `error!` 或字段。
- **性能**：低开销，过滤在元数据级别；基准显示在异步任务中高效。

#### 4. 高级用法
- **自定义订阅者**：使用 `tracing-subscriber` 的层（如 `FmtLayer`、`FilterLayer`）构建。
- **异步支持**：在 Tokio 中使用 `#[instrument]` 跨 `await` 点。
- **分布式追踪**：与 `tracing-opentelemetry` 集成，导出到 Jaeger 或 Zipkin。
- **日志兼容**：启用 "log" 特性，将事件转换为日志记录；使用 `tracing-log` 反向转换。
- **过滤**：使用环境变量如 `RUST_LOG=info` 或 `EnvFilter`。
- **多线程**：跨度上下文通过 `Dispatch` 传播，支持线程本地。
- **集成**：与 `axum`、`actix`、`tokio` 等结合，用于 Web 和异步。

#### 5. 注意事项
- 库应仅依赖 `tracing`，不设置全局订阅者；可执行文件设置 `set_global_default`。
- 在异步代码中，避免跨 `await` 持有守卫，使用 `#[instrument]`。
- 性能开销低，但大量字段可能增加；使用过滤优化。
- 与 `log` 兼容，但 `tracing` 更结构化。
- 测试中，可使用 `tracing-test` 断言输出。

#### 6. 替代方案
- **log**：简单日志，但缺乏结构化和跨度。
- **slog**：结构化日志，类似但较旧。
- **fern** 或 **env_logger**：简单配置，但无追踪。
- **opentelemetry**：直接用于分布式，但 `tracing` 更易集成。
  `tracing` 被视为 Rust 异步追踪的标准。

#### 7. 20 个例子
以下是 20 个例子，从简单到复杂，覆盖基本到高级场景。每个例子包括代码、预期输出（如果适用）和解释。假设已导入 `use tracing::{debug, error, info, instrument, span, trace, warn, Level};` 和初始化订阅者 `tracing_subscriber::fmt::init();`。

##### 示例 1: 基本事件记录
```rust
fn main() {
    tracing_subscriber::fmt::init();
    info!("Hello, tracing!");
}
```
输出：`[timestamp] INFO main: Hello, tracing!`  
解释：简单信息事件。

##### 示例 2: 跨度创建
```rust
fn main() {
    tracing_subscriber::fmt::init();
    let span = span!(Level::INFO, "my_span");
    let _enter = span.enter();
    info!("Inside span");
}
```
输出：进入/退出跨度日志。  
解释：手动进入跨度。

##### 示例 3: instrument 属性
```rust
#[instrument]
fn my_fn(arg: i32) {
    info!("Arg: {}", arg);
}

fn main() {
    tracing_subscriber::fmt::init();
    my_fn(42);
}
```
输出：自动跨度日志。  
解释：函数自动仪器化。

##### 示例 4: 级别宏
```rust
fn main() {
    tracing_subscriber::fmt::init();
    trace!("Trace level");
    debug!("Debug level");
    info!("Info level");
    warn!("Warn level");
    error!("Error level");
}
```
解释：不同级别事件。

##### 示例 5: 添加字段
```rust
fn main() {
    tracing_subscriber::fmt::init();
    let user = "ferris";
    info!(user, age = 42, "User info");
}
```
输出：带字段日志。  
解释：键值字段。

##### 示例 6: 格式化字段
```rust
#[derive(Debug)]
struct Point { x: i32, y: i32 }

fn main() {
    tracing_subscriber::fmt::init();
    let p = Point { x: 1, y: 2 };
    info!(?p, "Debug point");
    info!(%p.x, "Display x");
}
```
解释：`?` 和 `%` 格式化。

##### 示例 7: 父子跨度
```rust
fn main() {
    tracing_subscriber::fmt::init();
    let parent = span!(Level::INFO, "parent");
    let _p = parent.enter();
    let child = span!(Level::DEBUG, "child");
    let _c = child.enter();
    info!("In child");
}
```
解释：嵌套跨度。

##### 示例 8: 异步跨度
```rust
use tokio::runtime::Runtime;

#[instrument]
async fn async_fn() {
    info!("Async inside");
}

fn main() {
    tracing_subscriber::fmt::init();
    let rt = Runtime::new().unwrap();
    rt.block_on(async_fn());
}
```
解释：跨 `await` 点。

##### 示例 9: 错误记录
```rust
fn main() {
    tracing_subscriber::fmt::init();
    let err = std::io::Error::new(std::io::ErrorKind::Other, "test error");
    error!(error = %err, "An error occurred");
}
```
解释：记录错误。

##### 示例 10: 自定义订阅者
```rust
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
    info!("Custom subscriber");
}
```
解释：构建订阅者。

##### 示例 11: JSON 输出
```rust
use tracing_subscriber::{fmt, prelude::*, registry};
use tracing_subscriber::fmt::format::Json;

fn main() {
    registry().with(fmt::layer().json()).init();
    info!("JSON event");
}
```
输出：JSON 格式日志。  
解释：JSON 格式化。

##### 示例 12: 过滤事件
```rust
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

fn main() {
    registry().with(fmt::layer()).with(EnvFilter::new("info")).init();
    trace!("Filtered out");
    info!("Visible");
}
```
解释：级别过滤。

##### 示例 13: 多线程
```rust
use std::thread;

fn main() {
    tracing_subscriber::fmt::init();
    let handle = thread::spawn(|| {
        info!("In thread");
    });
    handle.join().unwrap();
    info!("Main thread");
}
```
解释：跨线程日志。

##### 示例 14: OpenTelemetry 集成
```rust
use opentelemetry::sdk::trace::Tracer;
use tracing_opentelemetry::layer;
use tracing_subscriber::prelude::*;

fn main() {
    let tracer = opentelemetry::sdk::trace::Tracer::default();
    tracing_subscriber::registry().with(layer().with_tracer(tracer)).init();
    info!("OTel event");
}
```
解释：导出到 OTel。

##### 示例 15: 自定义字段
```rust
fn main() {
    tracing_subscriber::fmt::init();
    let value = 42;
    info!(custom_field = value, "With custom");
}
```
解释：动态字段。

##### 示例 16: 测试中追踪
```rust
#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test() {
        info!("Test event");
        assert!(logs_contain("Test event"));
    }
}
```
解释：使用 `tracing-test` 断言。

##### 示例 17: 目标覆盖
```rust
fn main() {
    tracing_subscriber::fmt::init();
    info!(target: "custom_target", "Targeted event");
}
```
解释：自定义目标。

##### 示例 18: in_scope 包装
```rust
fn main() {
    tracing_subscriber::fmt::init();
    let result = tracing::info_span!("compute").in_scope(|| 1 + 2);
    info!("Result: {}", result);
}
```
解释：包装外部代码。

##### 示例 19: 性能指标
```rust
#[instrument]
fn expensive() {
    std::thread::sleep(std::time::Duration::from_millis(100));
}

fn main() {
    tracing_subscriber::fmt::init();
    expensive();
}
```
解释：追踪耗时操作。

##### 示例 20: log 兼容
```rust
use log::info as log_info;

fn main() {
    tracing_subscriber::fmt::init();
    log_info!("Log compatible");
}
```
解释：与 `log` 集成（需启用特性）。
