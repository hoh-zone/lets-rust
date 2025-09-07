# Docker 与 Rust

Docker 是一个开源的容器化平台，用于打包、部署和运行应用程序，而 Rust 是一种高效、安全的系统编程语言，常用于构建高性能应用。将 Rust 与 Docker 结合，可以实现跨平台部署、隔离环境和高效构建，尤其适合微服务、Web 服务或命令行工具。本教程基于官方文档和社区最佳实践，提供从基础到高级的指导，适用于初学者和开发者。教程假设你有基本的 Rust 和命令行知识。 当前日期为 2025 年 9 月 7 日，信息基于最新资源。

## 1. 介绍
- **为什么用 Docker 与 Rust？**：Rust 应用编译后是静态二进制，便于 Docker 打包。Docker 提供一致的环境，避免“本地运行正常、生产环境出错”。好处包括：小镜像大小（优化后几 MB）、快速部署、多平台支持（Linux、Windows、macOS）。常见场景：Web API、CLI 工具、游戏服务器。
- **挑战**：Rust 构建慢（依赖编译），Docker 镜像可能大。解决方案：多阶段构建和缓存。

## 2. 前置要求
- **安装 Rust**：通过 rustup（`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`）。验证：`rustc --version`。
- **安装 Docker**：下载 Docker Desktop（Windows/macOS）或 Docker Engine（Linux）。验证：`docker --version`。
- **创建 Rust 项目**：`cargo new hello-rust`。编辑 `src/main.rs`：
  ```rust
  fn main() {
      println!("Hello, Docker and Rust!");
  }
  ```
- **其他工具**：可选 Git（版本控制）、VS Code（编辑 Dockerfile）。

## 3. 基本 Dockerfile：Hello World
开始一个简单镜像。

1. 在项目根目录创建 `Dockerfile`：
   ```dockerfile
   # 基础镜像：Rust 官方镜像（包含工具链）
   FROM rust:1.85 as builder

   # 设置工作目录
   WORKDIR /app

   # 复制 Cargo.toml 和 Cargo.lock（依赖文件）
   COPY Cargo.toml Cargo.lock ./

   # 构建依赖（空构建以缓存层）
   RUN cargo build --release

   # 复制源代码
   COPY src ./src

   # 构建应用
   RUN cargo build --release

   # 运行阶段：使用最小镜像
   FROM debian:bookworm-slim

   # 复制二进制
   COPY --from=builder /app/target/release/hello-rust /usr/local/bin/

   # 运行命令
   CMD ["hello-rust"]
   ```
    - **解释**：使用多阶段构建（builder 和运行阶段）。第一阶段编译，第二阶段只复制二进制，减少镜像大小（从 GB 到 MB）。

2. 构建镜像：`docker build -t hello-rust .`（`-t` 指定标签）。
    - 时间：首次可能慢（下载 Rust 镜像），后续快。

3. 运行容器：`docker run --rm hello-rust`。输出："Hello, Docker and Rust!"。
    - `--rm`：运行后删除容器。

## 4. 多阶段构建与优化
Rust 构建慢，优化 Dockerfile 以利用 Docker 缓存。

### 4.1 添加 .dockerignore
创建 `.dockerignore`（类似 .gitignore）：
```
target
.git
README.md
```
- **好处**：忽略不必要文件，减少上下文大小，加速构建。

### 4.2 缓存依赖
优化 Dockerfile：
```dockerfile
FROM rust:1.85 as builder
WORKDIR /app

# 复制依赖文件并构建空项目（缓存依赖层）
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
RUN rm -f target/release/deps/hello_rust*

# 复制源代码并重新构建
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/hello-rust /usr/local/bin/
CMD ["hello-rust"]
```
- **解释**：先构建依赖（不变时缓存），源代码变更只重新编译应用。 构建时间：依赖缓存后，源代码变更只需几秒。

### 4.3 使用 sccache 缓存编译
为进一步加速，使用 sccache（Rust 编译缓存）：
```dockerfile
FROM rust:1.85 AS base
RUN cargo install sccache --locked
ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache

FROM base AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/hello-rust /usr/local/bin/
CMD ["hello-rust"]
```
- **安装**：`cargo install sccache`。
- **好处**：缓存单个 crate 编译，依赖变更时只重编译变更部分。

### 4.4 使用 cargo-chef
cargo-chef 生成依赖“菜谱”以优化缓存：
1. 安装：`cargo install cargo-chef`。
2. 更新 Dockerfile：
   ```dockerfile
   FROM rust:1.85 AS planner
   WORKDIR /app
   COPY . .
   RUN cargo chef prepare --recipe-path recipe.json

   FROM rust:1.81 AS cacher
   WORKDIR /app
   COPY --from=planner /app/recipe.json recipe.json
   RUN cargo chef cook --release --recipe-path recipe.json

   FROM rust:1.81 AS builder
   WORKDIR /app
   COPY . .
   COPY --from=cacher /app/target target
   COPY --from=cacher $CARGO_HOME/registry $CARGO_HOME/registry
   RUN cargo build --release

   FROM debian:bookworm-slim
   COPY --from=builder /app/target/release/hello-rust /usr/local/bin/
   CMD ["hello-rust"]
   ```
- **好处**：精确缓存依赖，源代码变更不重下依赖。

## 5. 运行与配置
- **环境变量**：在 Dockerfile 添加 `ENV KEY=value`，或运行时 `-e KEY=value`。
- **端口暴露**：Web 应用用 `EXPOSE 8080`，运行时 `-p 8080:8080`。
- **卷挂载**：持久化数据 `-v /host/path:/container/path`。
- **多容器**：使用 Docker Compose（compose.yaml）：
  ```yaml
  services:
    app:
      build: .
      ports: ["8080:8080"]
      volumes: ["/data:/app/data"]
  ```
  运行：`docker compose up`。

## 6. 最佳实践（2025 更新）
- **最小镜像**：用 `FROM scratch` 或 `alpine`（需 musl 目标：`rustup target add x86_64-unknown-linux-musl`）。
- **安全**：运行非 root 用户（`USER appuser`），扫描镜像（`trivy image myimage`）。
- **多平台**：用 `docker buildx` 构建（`docker buildx create --use`）。
- **健康检查**：`HEALTHCHECK CMD curl -f http://localhost/health || exit 1`。
- **避免最新标签**：用具体版本如 `rust:1.85`。
- **CI/CD**：GitHub Actions 示例（.github/workflows/ci.yml）：
  ```yaml
  name: Rust Docker CI
  on: [push]
  jobs:
    build:
      runs-on: ubuntu-latest
      steps:
        - uses: actions/checkout@v4
        - uses: docker/setup-buildx-action@v3
        - uses: docker/build-push-action@v6
          with:
            push: false
            tags: hello-rust:latest
  ```


## 7. 高级主题
- **Web 服务**：用 Axum 或 Rocket，暴露端口。
- **数据库集成**：用 Docker Compose 连接 Postgres。
- **推送镜像**：`docker push user/image:tag` 到 Docker Hub。
- **优化大小**：用 UPX 压缩二进制（可选，但小心兼容性）。

## 8. 故障排除
- **构建失败**：检查 Dockerfile 缩进；用 `--no-cache` 重建。
- **慢构建**：启用 BuildKit（`DOCKER_BUILDKIT=1`）。
- **权限问题**：用 `--user` 指定 UID/GID。
- **网络错误**：用 `--network=host`。