# Rustup 配置中国源教程

Rustup 是 Rust 官方的工具链管理器，用于安装、更新和管理 Rust 版本、组件和目标平台。在中国大陆，由于网络限制，默认的官方源（rust-lang.org）下载速度较慢，可能导致安装或更新失败。为此，可以配置国内镜像源（如清华大学 TUNA、RsProxy、中国科学技术大学 USTC 等）来加速下载。这些镜像会代理官方源的内容，通常速度更快、更稳定。

**注意**：镜像源可能不完整（如 nightly 版本只保留一段时间），如果下载失败，可临时切换回官方源（设置 `RUSTUP_DIST_SERVER=` 为空）。配置后，首次更新可能触发 sha256 校验失败，需要运行 `rustup self update` 修复。 以下教程基于 2025 年 9 月 7 日的最新信息，适用于 Linux/macOS/Windows（Windows 使用 PowerShell 或 CMD）。

## 1. 为什么配置中国源？
- **加速下载**：官方源下载工具链（如 stable、nightly）可能需数小时，镜像源可缩短至几分钟。
- **稳定性**：避免网络波动导致的超时。
- **适用场景**：安装 Rust、更新工具链、添加组件（如 clippy、rustfmt）。
- **Cargo 相关**：rustup 配置主要针对工具链，Cargo（依赖管理）需单独配置镜像（见第 5 节）。

## 2. 流行中国镜像源比较
以下是常见可靠的 rustup 镜像源（基于社区推荐和官方镜像站）。选择时优先 RsProxy（全面、稳定）或 TUNA（清华大学，学术友好）。

| 镜像源       | RUSTUP_DIST_SERVER                  | RUSTUP_UPDATE_ROOT                  | 优点                          | 缺点/警告                     | 官网/文档                          |
|--------------|-------------------------------------|-------------------------------------|-------------------------------|-------------------------------|------------------------------------|
| **RsProxy** (推荐) | https://rsproxy.cn                 | https://rsproxy.cn/rustup          | 全面、速度快、支持所有通道    | 无明显缺点                   | https://rsproxy.cn/                |
| **TUNA (清华大学)** | https://mirrors.tuna.tsinghua.edu.cn/rustup | https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup | 学术镜像，稳定                | nightly 只保留一段时间      | https://mirrors.tuna.tsinghua.edu.cn/help/rustup/ |
| **USTC (中国科学技术大学)** | https://mirrors.ustc.edu.cn/rust-static | https://mirrors.ustc.edu.cn/rust-static/rustup | 全面，学术友好                | 更新稍慢                     | https://mirrors.ustc.edu.cn/help/rustup.html |
| **阿里云**   | https://mirrors.aliyun.com/rust-static | https://mirrors.aliyun.com/rust-static/rustup | 商业镜像，速度快              | 可能有延迟                   | https://developer.aliyun.com/mirror/rustup |
| **字节跳动** | https://mirrors.bytedance.com/rust-static | https://mirrors.bytedance.com/rust-static/rustup | 国内企业镜像，稳定            | 较新，可能不全               | https://mirrors.bytedance.com/     |

- **选择建议**：初次使用 RsProxy；学术项目用 TUNA/USTC；如果镜像失效，fallback 到官方（注释环境变量）。

## 3. 配置步骤
配置通过设置环境变量 `RUSTUP_DIST_SERVER`（工具链下载源）和 `RUSTUP_UPDATE_ROOT`（rustup 更新源）。这些变量会影响 `rustup install`、`rustup update` 等命令。

### 步骤 1: 选择镜像源并设置环境变量
以 RsProxy 为例（其他源替换 URL）。

#### Linux/macOS (Bash/Zsh)
1. 编辑 shell 配置文件（`~/.bashrc` 或 `~/.zshrc`）：
   ```bash
   nano ~/.bashrc  # 或 vim ~/.zshrc
   ```
2. 添加以下行：
   ```bash
   export RUSTUP_DIST_SERVER="https://rsproxy.cn"
   export RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
   ```
3. 保存并重新加载：
   ```bash
   source ~/.bashrc  # 或 source ~/.zshrc
   ```
4. 验证：
   ```bash
   echo $RUSTUP_DIST_SERVER  # 应输出 https://rsproxy.cn
   ```

#### Windows (PowerShell)
1. 编辑 PowerShell 配置文件：
   ```powershell
   notepad $PROFILE  # 如果不存在，运行 New-Item -Path $PROFILE -Type File -Force
   ```
2. 添加：
   ```powershell
   $env:RUSTUP_DIST_SERVER="https://rsproxy.cn"
   $env:RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
   ```
3. 保存并重启 PowerShell，或运行：
   ```powershell
   . $PROFILE
   ```
4. 验证：
   ```powershell
   echo $env:RUSTUP_DIST_SERVER
   ```

#### Windows (CMD)
- 在系统环境变量中添加（搜索“环境变量” > 编辑系统环境变量 > 新建）：
    - 变量名：RUSTUP_DIST_SERVER，值：https://rsproxy.cn
    - 变量名：RUSTUP_UPDATE_ROOT，值：https://rsproxy.cn/rustup
- 重启 CMD 生效。

#### Fish Shell (macOS/Linux)
编辑 `~/.config/fish/config.fish`：
```fish
set -x RUSTUP_DIST_SERVER https://rsproxy.cn
set -x RUSTUP_UPDATE_ROOT https://rsproxy.cn/rustup
```
重新加载：`source ~/.config/fish/config.fish`。

### 步骤 2: 安装或更新 Rust
- **首次安装**：使用镜像安装脚本（RsProxy 示例）：
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://rsproxy.cn/rustup-init.sh | sh
  ```
    - 对于 TUNA 等其他源，使用默认 `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`，但环境变量已设置会自动使用镜像。
- **更新现有安装**：
  ```bash
  rustup self update
  rustup update stable  # 更新 stable 通道
  ```
- **添加组件**（如 clippy）：
  ```bash
  rustup component add clippy rustfmt
  ```
- **临时使用镜像**（不永久配置）：
  ```bash
  RUSTUP_DIST_SERVER=https://rsproxy.cn rustup install nightly
  ```

### 步骤 3: 验证配置
1. 检查工具链下载：
   ```bash
   rustup toolchain list  # 应快速响应
   ```
2. 更新并观察日志：
   ```bash
   rustup update --verbose  # 查看下载源
   ```
3. 测试速度：安装一个新工具链，如 `rustup toolchain install beta`，观察是否更快。
- 如果失败：临时禁用镜像 `RUSTUP_DIST_SERVER= rustup update`，然后重试配置。

## 4. 常见问题与故障排除
- **下载失败/校验错误**：镜像不完整，fallback 到官方：`RUSTUP_DIST_SERVER= rustup self update`。然后重新设置变量。
- **环境变量无效**：确保 shell 重新加载（重启终端）；Windows 检查系统 vs 用户变量。
- **Nightly 版本缺失**：镜像只同步部分 nightly，使用官方或切换源。
- **代理/防火墙**：如果企业网络，使用 VPN 或额外代理（如 `export https_proxy=http://proxy:port`）。
- **恢复官方源**：注释或删除环境变量，运行 `source ~/.bashrc`。
- **Cargo 慢**：rustup 配置不影响 Cargo，单独配置（见下一节）。

## 5. Cargo 配置中国源（推荐补充）
Rustup 只管工具链，Cargo（crates.io 依赖）需单独镜像。编辑 `~/.cargo/config.toml`（创建如果不存在）：
```toml
[source.crates-io]
replace-with = 'rsproxy-sparse'
[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"
[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"
[net]
git-fetch-with-cli = true
```
- RsProxy：使用 sparse 协议加速（仅下载需要的 crate）。
- 验证：`cargo new test && cd test && cargo build`（应更快）。
- 切换回官方：删除 [source] 部分。

配置后，你的 Rust 开发环境将显著加速。如果镜像变更，检查官网更新。更多细节参考清华大学镜像帮助或 RsProxy 站点。