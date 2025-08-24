# Rust Axum 教程

Axum 是 Rust 生态中一个高效、模块化的 web 应用框架，由 Tokio 团队维护。它构建在 Tokio（异步运行时）、Tower（服务和中间件框架）和 Hyper（HTTP 库）之上，强调人体工程学（ergonomics）、类型安全和性能。Axum 不依赖宏来定义路由，而是使用类型安全的 API，支持中间件复用，并无缝集成 Tokio 的异步模型。 截至 2025 年 8 月，Axum 的最新版本是 0.8.x 系列（0.8.0 于 2025 年 1 月 1 日发布，0.8.2 于 4 月 30 日发布但随后被撤回）。 这个版本引入了更多优化，如改进的错误处理模型、增强的提取器支持，以及对 Tokio 新特性的更好集成。 Axum 适用于构建 REST API、WebSockets 服务或微服务，尤其适合需要高并发和低延迟的场景。

本教程将从基础到高级逐步讲解 Axum 的使用，包括安装、路由、处理程序、提取器、中间件、错误处理、状态管理和 WebSockets 等。假设你已安装 Rust（通过 `rustup`），并使用 `cargo` 创建项目（如 `cargo new axum-app`）。所有示例基于 Axum 0.8.x，可复制到 `src/main.rs` 中，使用 `cargo run` 执行。教程将包含详细解释、多个代码示例和最佳实践，以帮助你构建生产级应用。

## 1. 安装与依赖

在 `Cargo.toml` 中添加核心依赖：

```toml
[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }  # 启用 Tokio 的完整特性，包括宏和多线程运行时
```

- **Axum**：提供路由、提取器和响应工具。
- **Tokio**：异步运行时，必须启用 `"full"` 以支持宏（如 `#[tokio::main]`）和网络功能。

可选依赖扩展功能：
- `tower-http = { version = "0.5", features = ["trace", "cors", "compression"] }`：添加追踪、CORS 和压缩中间件。
- `serde = { version = "1", features = ["derive"] }` 和 `serde_json = "1"`：处理 JSON 序列化。
- `axum-extra = "0.9"`：额外提取器和实用工具（Axum 0.8 兼容）。

Axum 支持多种 feature flags 来控制依赖：
- `"http1"`（默认）：启用 HTTP/1 支持。
- `"http2"`：启用 HTTP/2。
- `"json"`（默认）：启用 JSON 提取器。
- `"ws"`：启用 WebSockets。
- `"tracing"`（默认）：集成 tracing 日志。

运行 `cargo build` 安装。注意：Axum 0.8 修复了 0.7 中的一些性能问题，如更快的路由匹配。

## 2. 基础：Hello World 与服务器启动

从一个简单服务器开始，了解 Axum 的核心流程：构建路由、定义处理程序、启动服务器。

### 示例代码：基本 Hello World
```rust
use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // 构建应用路由
    let app = Router::new().route("/", get(handler));

    // 绑定本地地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("服务器监听: {}", addr);

    // 启动服务器（使用 hyper 作为底层）
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> &'static str {
    "Hello, World! 从 Axum 0.8"
}
```

- **解释**：`Router::new()` 创建路由表。`route()` 方法指定路径和 HTTP 方法（这里是 GET）。处理程序是异步函数，返回实现 `IntoResponse` 的类型（如字符串）。`axum::serve()` 在 0.8 中优化了启动过程，支持自定义监听器。
- **运行**：`cargo run`，访问 `http://localhost:3000/`。
- **扩展**：添加健康检查路由：`.route("/health", get(|| async { "OK" }))`。

### 高级启动：使用共享状态
在 main 中引入状态：
```rust
use std::sync::Arc;
use axum::extract::State;

#[derive(Clone)]
struct AppState { counter: Arc<std::sync::atomic::AtomicU32> }

let state = AppState { counter: Arc::new(std::sync::atomic::AtomicU32::new(0)) };
let app = Router::new().route("/", get(handler)).with_state(state);

async fn handler(State(state): State<AppState>) -> String {
    let count = state.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    format!("访问次数: {}", count)
}
```
这展示了状态共享，适用于计数器或数据库连接。

## 3. 路由（Routing）

Axum 的路由系统基于类型安全的 `Router`，支持链式定义、嵌套和方法过滤。

### 基本语法与示例
```rust
use axum::{routing::{get, post, put, delete}, Router};

let app = Router::new()
    .route("/", get(root))
    .route("/users", post(create_user).get(list_users))
    .route("/users/:id", get(get_user).put(update_user).delete(delete_user));

async fn root() -> &'static str { "欢迎" }
async fn create_user() -> &'static str { "用户创建" }
// 类似其他处理程序...
```

- **路径参数**：使用 `/:id` 捕获动态部分。
- **嵌套路由**：分组路由以共享中间件。
  ```rust
  let api = Router::new()
      .route("/v1/users", get(list_users))
      .route("/v1/posts", get(list_posts));
  let app = Router::new().nest("/api", api);
  ```
- **方法过滤**：`.route("/path", get(handler).with(MethodFilter::POST, other_handler))`。
- **注意**：路由按添加顺序匹配；使用 `merge()` 组合多个 Router。

### 高级路由：Fallback 与 MatchedPath
添加回退处理程序：
```rust
let app = Router::new()
    .route("/", get(root))
    .fallback(not_found);

async fn not_found() -> (axum::http::StatusCode, &'static str) {
    (axum::http::StatusCode::NOT_FOUND, "页面未找到")
}
```
使用 `MatchedPath` 提取器记录路径。

## 4. 处理程序（Handlers）

处理程序是异步函数，接收提取器并返回响应。Axum 0.8 增强了响应生成，减少 boilerplate。

### 示例：多种返回类型
- 字符串：`async fn simple() -> String { "文本".to_string() }`
- JSON：
  ```rust
  use axum::Json;
  use serde::Serialize;

  #[derive(Serialize)]
  struct User { id: u32, name: String }

  async fn json_response() -> Json<User> {
      Json(User { id: 1, name: "Alice".to_string() })
  }
  ```
- 状态码与头：`(StatusCode::CREATED, [("Location", "/users/1")], "创建成功")`
- 流式响应：使用 `axum::body::StreamBody`。

### 高级处理：异步操作
集成 Tokio 的 async：
```rust
async fn delay_response() -> String {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    "延迟响应".to_string()
}
```
这展示了 Axum 与 Tokio 的无缝集成。

## 5. 提取器（Extractors）

提取器从请求中解析数据，实现 `FromRequest` 或 `FromRequestParts`。

### 常见提取器示例
- **Path**：
  ```rust
  use axum::extract::Path;

  async fn path_extract(Path(id): Path<u32>) -> String {
      format!("ID: {}", id)
  }
  ```
- **Query**：
  ```rust
  use axum::extract::Query;
  use std::collections::HashMap;

  async fn query_extract(Query(params): Query<HashMap<String, String>>) -> String {
      format!("参数: {:?}", params)
  }
  ```
- **Json** 与 **Form**：
  ```rust
  use axum::extract::{Json, Form};
  use serde::Deserialize;

  #[derive(Deserialize)]
  struct Payload { name: String }

  async fn json_extract(Json(payload): Json<Payload>) -> String {
      format!("名称: {}", payload.name)
  }

  async fn form_extract(Form(payload): Form<Payload>) -> String {
      format!("表单: {}", payload.name)
  }
  ```
- **State**：共享应用状态（推荐优于 Extension）。
- **Multipart**：处理文件上传（启用 `"multipart"` feature）。
  ```rust
  use axum::extract::Multipart;

  async fn upload(mut multipart: Multipart) -> String {
      while let Some(field) = multipart.next_field().await.unwrap() {
          let name = field.name().unwrap().to_string();
          let data = field.bytes().await.unwrap();
          println!("文件 {} 大小: {}", name, data.len());
      }
      "上传完成".to_string()
  }
  ```

- **自定义提取器**：实现 `FromRequest` trait 以扩展功能。

## 6. 中间件（Middleware）

Axum 利用 Tower 的中间件系统，支持日志、认证等。

### 示例：内置中间件
使用 `tower-http`：
```rust
use tower_http::{trace::TraceLayer, cors::CorsLayer};

let app = Router::new()
    .route("/", get(root))
    .layer(TraceLayer::new_for_http())
    .layer(CorsLayer::permissive());
```

### 自定义中间件
```rust
use axum::{middleware::Next, extract::Request, response::Response};

async fn auth(req: Request, next: Next) -> Result<Response, StatusCode> {
    if let Some(auth) = req.headers().get("Authorization") {
        // 验证...
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

let app = Router::new().route_layer(middleware::from_fn(auth));
```
- **层级**：`.layer()` 添加到所有路由；`.route_layer()` 只针对特定路由。

### 高级：压缩与超时
添加 `CompressionLayer` 和 `TimeoutLayer` 以优化性能。

## 7. 错误处理

Axum 提供简单、可预测的错误模型，所有错误必须处理。

### 示例
- 返回错误：处理程序返回 `Result<T, E>`，其中 E 实现 `IntoResponse`。
  ```rust
  type AppResult<T> = Result<T, AppError>;

  enum AppError { NotFound, Internal }

  impl IntoResponse for AppError {
      fn into_response(self) -> Response {
          match self {
              AppError::NotFound => (StatusCode::NOT_FOUND, "未找到").into_response(),
              AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "错误").into_response(),
          }
      }
  }

  async fn handler() -> AppResult<&'static str> {
      Err(AppError::NotFound)
  }
  ```
- 全局错误处理：使用 `handle_error` 在 Router 中。

## 8. 状态管理

使用 `State` 提取器共享数据，如数据库池。

### 示例
```rust
use sqlx::PgPool;  // 假设使用 sqlx

let pool = PgPool::connect("postgres://...").await.unwrap();
let app = Router::new().route("/", get(handler)).with_state(pool.clone());

async fn handler(State(pool): State<PgPool>) -> String {
    // 使用 pool 查询...
    "数据库连接".to_string()
}
```
- **最佳**：使用 `Arc` 包装非 Clone 类型。

## 9. WebSockets 支持

启用 `"ws"` feature 处理实时通信。

### 示例
```rust
use axum::extract::WebSocketUpgrade;
use axum::ws::{WebSocket, Message};

async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg { msg } else { return; };
        if socket.send(Message::Text("回音".to_string())).await.is_err() { return; }
    }
}
```
路由：`.route("/ws", get(ws_handler))`。

## 10. 常见错误与最佳实践

扩展表格：

| 问题 | 原因 | 解决方案 |
|------|------|----------|
| 类型不匹配 | 返回未实现 `IntoResponse` | 使用 `Json`、`StatusCode` 等 |
| 缺少 Tokio 特性 | 未启用 `"full"` | 更新 `Cargo.toml` |
| 路由冲突 | 路径重叠 | 调整顺序或嵌套 |
| 性能瓶颈 | 过多中间件 | 基准测试，只添加必要 |
| 状态未共享 | 未使用 `Arc` | 包装共享数据 |
| 错误未处理 | 未实现 `IntoResponse` | 定义自定义错误类型 |
| WebSocket 失败 | 未启用 `"ws"` | 添加 feature flag |
| 提取器失败 | 请求格式错误 | 添加验证层 |

- **最佳实践**：使用 `State` 而非 `Extension` 以提高类型安全；集成 `tracing` 日志；测试使用 `axum::test`；避免宏，保持模块化；定期检查 Axum 更新以利用新优化。

## 11. 练习与高级主题

1. 构建 REST API：用户 CRUD 操作，使用数据库（如 sqlx）。
2. 添加认证中间件：JWT 支持。
3. 实现 WebSocket 聊天室。
4. 集成 gRPC：使用 tonic 与 Axum 结合。
5. 性能优化：添加缓存中间件。

高级主题：探索 Axum 与 tonic 的集成（共享 Router）；使用 `axum-test` 进行单元测试。

通过这个扩展教程，你能构建复杂的 web 应用。参考官方文档以获取更多细节。 如需代码调试，提供反馈！