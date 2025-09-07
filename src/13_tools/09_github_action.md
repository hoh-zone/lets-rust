# Rust GitHub Actions

GitHub Actions 是 GitHub 提供的 CI/CD（持续集成/持续交付）平台，用于自动化 Rust 项目的构建、测试、发布和部署。它支持 YAML 配置的工作流（workflow），可以触发于 push、pull request 等事件。本教程基于官方文档和社区最佳实践，提供从基础到高级的完整指导，适用于 Rust 库或应用开发。教程假设你有基本的 YAML 和 Rust 知识。 当前日期为 2025 年 9 月 7 日，信息基于最新可用资源。

## 1. 介绍
GitHub Actions 允许你在仓库中定义 `.github/workflows/` 目录下的 YAML 文件，这些文件描述自动化任务。Rust 项目常用 Actions 来：
- **构建（Build）**：运行 `cargo build`。
- **测试（Test）**：运行 `cargo test`，支持多工具链（如 stable、beta、nightly）。
- **代码检查（Lint）**：集成 `clippy` 和 `rustfmt`。
- **缓存（Cache）**：加速依赖和构建。
- **发布（Publish）**：上传到 crates.io 或 Docker Hub。
- **部署（Deploy）**：如使用 Shuttle 或其他平台。

优势：免费（公共仓库无限分钟，私有仓库有限制）、集成 GitHub、支持矩阵构建（多平台/工具链）。 入门：仓库 > Actions > New workflow > 搜索 "Rust" 模板。

## 2. 前置要求
- GitHub 仓库（公共或私有）。
- Rust 项目：包含 `Cargo.toml` 和源代码。
- 安装 rustup 和 Cargo。
- 可选：crates.io 账户（发布用），GitHub Secrets（存储 token，如 CRATES_TOKEN）。
- 权限：仓库需启用 Actions（Settings > Actions > General > Allow all actions）。

验证：本地运行 `cargo build` 和 `cargo test` 确保项目正常。

## 3. 基本 CI 工作流：构建和测试
创建第一个工作流：`.github/workflows/ci.yml`。

### 步骤
1. 在仓库根目录创建 `.github/workflows/` 文件夹。
2. 添加 `ci.yml`：
   ```yaml
   name: Rust CI  # 工作流名称

   on:  # 触发事件
     push:
       branches: [ "main" ]
     pull_request:
       branches: [ "main" ]

   env:  # 全局环境变量
     CARGO_TERM_COLOR: always  # 启用彩色输出

   jobs:  # 作业定义
     build_and_test:
       name: Rust project  # 作业名称
       runs-on: ubuntu-latest  # 运行器（GitHub 托管的 Ubuntu）
       steps:
         - name: Checkout code  # 检出仓库
           uses: actions/checkout@v4  # 使用官方 checkout action

         - name: Install Rust  # 安装 Rust 工具链
           uses: dtolnay/rust-toolchain@stable  # 使用 dtolnay 的 action（推荐，自动安装 stable）
           with:
             components: rustfmt, clippy  # 添加 rustfmt 和 clippy

         - name: Cache dependencies  # 缓存 Cargo 依赖（可选，但推荐）
           uses: actions/cache@v4
           with:
             key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
             path: |
               ~/.cargo/registry
               ~/.cargo/git
               target

         - name: Build  # 构建项目
           run: cargo build --verbose  # --verbose 显示详细输出

         - name: Run tests  # 运行测试
           run: cargo test --verbose
   ```
3. 提交并推送：`git add . && git commit -m "Add CI workflow" && git push`。
4. 查看：仓库 > Actions > 运行的工作流，检查日志。

### 解释
- **on**：触发于 push 或 PR 到 main 分支。
- **jobs**：单个作业 `build_and_test`，在 Ubuntu 上运行。
- **steps**：序列步骤，包括检出、安装 Rust、缓存、构建、测试。
- **uses**：调用复用 Actions（如 checkout、rust-toolchain）。
- **run**：执行 shell 命令（默认 bash）。

最佳实践：使用矩阵（matrix）测试多工具链：
```yaml
strategy:
  matrix:
    toolchain: [stable, beta, nightly]  # 测试多个版本
steps:
  - uses: dtolnay/rust-toolchain@master
    with:
      toolchain: ${{ matrix.toolchain }}
```
这会并行运行三个作业，失败任何一职则整体失败。

## 4. 集成代码检查：Clippy 和 Rustfmt
添加 lint 步骤，确保代码质量。

更新 `ci.yml`：
```yaml
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - run: cargo fmt --all -- --check  # 检查格式，不修改
      - run: cargo clippy --all-targets --all-features -- -D warnings  # 拒绝警告
```

- `cargo fmt --check`：验证格式一致性。
- `cargo clippy -D warnings`：将警告视为错误。
- 最佳实践：添加 `continue-on-error: true` 到非关键步骤，避免阻塞。

## 5. 缓存优化
Rust 构建慢？使用缓存加速。

上述示例已包含 Cargo 缓存。高级：使用 sccache（编译缓存）：
```yaml
- name: Install sccache
  uses: mozilla-actions/sccache-action@v0.0.7  # Mozilla 的 sccache action

env:
  SCCACHE_GHA_ENABLED: true
  RUSTC_WRAPPER: sccache  # 包装 rustc
```
- 支持 S3 等后端，减少重复编译。
- 提示：缓存键基于 `Cargo.lock` hash，确保依赖变更时失效。

## 6. 多平台和交叉编译
测试多目标（如 x86_64、ARM）：
```yaml
jobs:
  cross-compile:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        target: [x86_64-unknown-linux-gnu, aarch64-unknown-linux-gnu]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      - run: cargo build --target ${{ matrix.target }} --release
```
- 安装目标：`rustup target add <triple>`。
- 最佳实践：仅构建 release 模式，上传 artifacts：
  ```yaml
  - name: Upload binary
    uses: actions/upload-artifact@v4
    with:
      name: binary-${{ matrix.target }}
      path: target/${{ matrix.target }}/release/myapp
  ```


## 7. 发布到 crates.io
自动化发布：仅在 main 分支 tag 时触发。

创建 `.github/workflows/release.yml`：
```yaml
name: Release to crates.io

on:
  push:
    tags: [ "v*" ]  # 触发于 v1.0.0 等 tag

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - run: cargo build --release
      - name: Login to crates.io
        run: cargo login ${{ secrets.CRATES_TOKEN }}  # Secrets 中存储 token
      - name: Publish
        run: cargo publish
```

- 获取 token：crates.io > Account Settings > API access tokens > New token（full access）。
- 添加 Secret：仓库 > Settings > Secrets and variables > Actions > New repository secret (CRATES_TOKEN)。
- 最佳实践：使用 `cargo publish --dry-run` 测试；添加变更日志。

## 8. 依赖管理：Dependabot
自动化依赖更新。

创建 `.github/dependabot.yml`：
```yaml
version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"  # 每周检查
    ignore:
      - dependency-name: "semver"  # 忽略特定依赖
      - dependency-name: "crates-io"
    open-pull-requests-limit: 10  # 最多 10 个 PR
```

- Dependabot 会创建 PR 更新 Cargo.toml 和 Cargo.lock。
- 最佳实践：结合 CI 运行测试，确保更新安全。

## 9. 高级发布：使用 release-plz
自动化发布笔记和 crates.io 上传。

1. 添加依赖：`cargo add release-plz --build`。
2. 创建工作流 `.github/workflows/release-plz.yml`：
   ```yaml
   name: Release

   on:
     push:
       branches: [ "main" ]

   permissions:
     contents: write  # 发布笔记
     pull-requests: write

   jobs:
     release:
       runs-on: ubuntu-latest
       steps:
         - uses: actions/checkout@v4
           with:
             fetch-depth: 0  # 获取历史
         - uses: dtolnay/rust-toolchain@stable
         - uses: release-plz/release-plz-action@v0.5
           with:
             version: semver
             sign: false  # 如需签名，配置 GPG
   ```
- release-plz 生成 PR 包含发布笔记和版本变更，合并后自动发布。
- 最佳实践：用于库项目，集成 changelog。

## 10. Docker 集成：构建和推送镜像
对于应用，构建 Docker 镜像。

更新 `ci.yml` 添加 Docker 步骤：
```yaml
- name: Build Docker image
  run: docker build -t myapp:latest .

- name: Login to Docker Hub
  uses: docker/login-action@v3
  with:
    username: ${{ secrets.DOCKER_USERNAME }}
    password: ${{ secrets.DOCKER_PASSWORD }}

- name: Push to Docker Hub
  run: docker push myapp:latest
```

- Dockerfile 示例：
  ```dockerfile
  FROM rust:1.81 as builder
  WORKDIR /usr/src/app
  COPY . .
  RUN cargo build --release

  FROM debian:bookworm-slim
  COPY --from=builder /usr/src/app/target/release/myapp /usr/local/bin/
  CMD ["myapp"]
  ```
- 最佳实践：使用多阶段构建，缓存 Docker 层。

## 11. 优化和最佳实践
- **速度**：使用 sccache 和缓存；并行作业（matrix）；避免 verbose 在生产。
- **安全**：使用 OIDC 认证（而非 token）登录 registry；扫描依赖（cargo-audit）。
- **成本**：公共仓库免费；私有限 2000 分钟/月，使用 self-hosted runners。
- **多平台**：测试 Windows/macOS：`runs-on: windows-latest` 或 `macos-latest`。
- **通知**：集成 Slack/Discord 使用 actions/slack-notify。
- **矩阵排除**：`fail-fast: false` 允许部分失败继续。
- **环境**：使用 `environment` 保护部署步骤。
- **秘密管理**：避免硬编码，使用 GitHub Secrets。
- **监控**：查看 Actions > 运行日志；设置 badge 在 README：`![CI](https://github.com/user/repo/actions/workflows/ci.yml/badge.svg)`。
- **常见错误**：YAML 缩进；工具链版本不匹配；权限不足（添加 `permissions: { contents: read }`）。

## 12. 故障排除
- **工作流失败**：检查日志，常见：缺少组件（添加 clippy 到 rust-toolchain）；缓存失效（清除 key）。
- **权限错误**：添加 `permissions` 到 workflow。
- **慢构建**：启用缓存，升级到 GitHub-hosted larger runners。
- **测试失败**：本地复现；使用 `cargo nextest` 加速测试（`cargo install cargo-nextest`）。
- **发布失败**：验证 token 权限；检查 Cargo.toml metadata（如 license）。