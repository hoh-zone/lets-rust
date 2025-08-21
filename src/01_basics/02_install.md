### 如何安装 Rust

Rust 的官方安装工具是 rustup，它可以管理 Rust 的版本和工具链。以下是基于官方指南的安装步骤，分平台说明。安装后，重启终端或注销重新登录以确保环境变量生效。

#### Unix-like 系统（Linux/macOS，包括 WSL）
1. 打开终端。
2. 运行以下命令：
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. 按照屏幕提示操作（通常选择默认安装）。
4. 验证安装：运行 `rustc --version`。如果失败，重启终端。

Rust 工具（如 rustc、cargo、rustup）会安装在 `~/.cargo/bin` 目录下，并自动添加到 PATH 中。

#### Windows
1. 从 https://win.rustup.rs 下载 rustup-init.exe。
2. 运行下载的文件。
3. 按照屏幕提示操作（可能需要安装 MSVC build tools for Visual Studio 2013 或更高版本）。
4. 验证安装：运行 `rustc --version`。如果失败，重启命令提示符。

Rust 工具会安装在 `%USERPROFILE%\.cargo\bin` 目录下，并添加到 PATH 中。

#### 其他安装方法
如果 rustup 不适用，可以参考官方的其他安装方式：https://forge.rust-lang.org/infra/other-installation-methods.html。

安装完成后，运行 `cargo --version` 来确认 Cargo（Rust 的构建工具和包管理器）已就绪。

### 如何运行 Hello World

安装 Rust 后，使用 Cargo 创建并运行一个简单的 Hello World 程序。以下是步骤：

1. 打开终端或命令提示符。
2. 创建新项目：
   ```
   cargo new hello-rust
   ```
   这会生成一个名为 `hello-rust` 的目录，包含 `Cargo.toml`（项目元数据文件）和 `src/main.rs`（主代码文件，默认已包含 Hello World 代码）。

3. 进入项目目录：
   ```
   cd hello-rust
   ```

4. 编译并运行程序：
   ```
   cargo run
   ```
   输出类似于：
   ```
   Compiling hello-rust v0.1.0 (/path/to/hello-rust)
   Finished dev [unoptimized + debuginfo] target(s) in X.XXs
   Running `target/debug/hello-rust`
   Hello, world!
   ```

如果想手动编写代码，可以编辑 `src/main.rs`：
```rust
fn main() {
    println!("Hello, world!");
}
```
然后再次运行 `cargo run`。

这些步骤适用于所有平台。如果遇到问题，检查官方文档或运行 `rustup update` 更新 Rust。