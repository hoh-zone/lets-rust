### VS Code 中安装 Rust 插件

Visual Studio Code (VS Code) 是 Rust 开发的流行编辑器，通过 rust-analyzer 插件提供智能补全、语法高亮、代码导航和调试支持。以下是详细步骤，确保你已安装 Rust（参考之前的安装指南）。如果尚未安装 VS Code，可从官网 https://code.visualstudio.com 下载。

#### 前提条件
- 已安装 Rust 工具链（使用 rustup），并确保 `rustc` 和 `cargo` 在 PATH 中可用。运行 `rustc --version` 验证。
- 重启终端和 VS Code 以应用环境变量变化。

#### 安装 rust-analyzer 插件
1. 打开 VS Code。
2. 按快捷键打开扩展视图：Mac 使用 ⇧⌘X，Windows/Linux 使用 Ctrl+Shift+X。
3. 在搜索框中输入 "rust-analyzer"。
4. 选择官方的 rust-analyzer 扩展（发布版），点击 "Install" 安装。
5. 安装后，重启 VS Code 以激活插件。

#### 配置插件（可选，但推荐）
- rust-analyzer 会自动检测 Rust 工具链，无需额外配置。但你可以自定义：
    - 启用/禁用内嵌提示（Inlay Hints）：在设置中搜索 "Editor > Inlay Hints: Enabled" 并调整。
    - 启用 Clippy  linting：打开设置（Ctrl+,），搜索 "Rust-analyzer > Check: Command"，将其改为 "clippy"（默认是 "check"）。
    - 自定义语义高亮：在 settings.json 中添加：
      ```
      {
        "editor.semanticTokenColorCustomizations": {
          "rules": {
            "*.mutable": {
              "fontStyle": ""
            }
          }
        }
      }
      ```
- 如果遇到问题，查看插件文档：https://rust-analyzer.github.io。

#### 编写和运行 Rust 代码
以 Hello World 示例说明：

1. **创建项目**：
    - 打开终端（在 VS Code 中按 Ctrl+Shift+` 或 ⌃⇧`）。
    - 运行 `cargo new hello_world` 创建新项目。这会生成 `hello_world` 目录，包含 `Cargo.toml` 和 `src/main.rs`。
    - 进入目录：`cd hello_world`。
    - 在 VS Code 中打开项目：运行 `code .`（或手动打开文件夹）。

2. **编写代码**：
    - 打开 `src/main.rs`，默认代码已是 Hello World：
      ```rust
      fn main() {
          println!("Hello, world!");
      }
      ```
    - rust-analyzer 会提供智能提示：如类型推断、文档悬停（hover）、自动补全（按 Ctrl+Space 触发）。代码导航：F12 跳转定义，Shift+F12 查看引用。
    - 保存文件，插件会实时 linting 和高亮（例如，可变变量下划线）。

3. **构建和运行**：
    - 在终端运行 `cargo build` 编译项目（生成 `target/debug` 目录）。
    - 运行 `cargo run` 执行程序，输出 "Hello, world!"。
    - 或者直接运行可执行文件：`./target/debug/hello_world`（Windows 为 `.\target\debug\hello_world.exe`）。
    - VS Code 支持调试：按 F5 或在 Run 视图中启动。

#### 故障排除和额外提示
- 如果 IntelliSense 不工作：确保 Workspace Trust 已启用（VS Code 会提示），或信任父文件夹。
- 更新 Rust：运行 `rustup update`。
- 访问本地文档：运行 `rustup doc`。
- 额外功能：支持语义高亮、调用层次（Shift+Alt+H）、符号导航（Ctrl+Shift+O）。如果性能问题，检查系统资源或禁用不必要扩展。



以下是关于 RustRover 的安装和使用指南。RustRover 是 JetBrains 公司开发的专为 Rust 编程语言设计的集成开发环境 (IDE)，它提供了代码补全、调试、测试等功能，帮助开发者高效编写 Rust 代码。

### 前提条件
在使用 RustRover 之前，强烈推荐安装 Rust 工具链。如果未安装，RustRover 在创建项目时可以帮助下载，但手动安装更可靠：
1. 访问 https://www.rust-lang.org/tools/install。
2. 下载并运行 rustup 安装程序（Windows/macOS/Linux 通用）。
3. 运行命令 `rustup --version` 验证安装成功。
   RustRover 会自动检测 Rust 工具链的位置。

### 安装步骤
RustRover 支持免费试用（非商业用途），可以从官方下载页面获取：https://www.jetbrains.com/rust/download/。

安装方式有两种：使用 JetBrains Toolbox App（推荐，便于管理多个 IDE）或独立安装。下面分操作系统说明。

#### Windows
**使用 Toolbox App：**
1. 从 https://www.jetbrains.com/toolbox/app/ 下载 .exe 安装程序。
2. 运行安装程序并按照向导操作。
3. 打开 Toolbox App，搜索并安装 RustRover（可选择特定版本）。
4. 使用 JetBrains 账户登录激活。

**独立安装：**
1. 从 https://www.jetbrains.com/rust/download/ 下载 .exe 安装程序。
2. 运行安装程序，按照向导配置选项（如创建桌面快捷方式、添加 PATH）。
3. 通过开始菜单或桌面快捷方式启动 RustRover。

#### macOS
**使用 Toolbox App：**
1. 从 https://www.jetbrains.com/toolbox/app/ 下载 .dmg 文件。
2. 挂载镜像并将 JetBrains Toolbox 拖到 Applications 文件夹。
3. 打开 Toolbox App，搜索并安装 RustRover。
4. 使用 JetBrains 账户登录激活。

**独立安装：**
1. 从 https://www.jetbrains.com/rust/download/ 下载 .dmg 文件。
2. 挂载镜像并将 RustRover 拖到 Applications 文件夹。
3. 通过 Applications、Launchpad 或 Spotlight 启动。

#### Linux
**使用 Toolbox App：**
1. 从 https://www.jetbrains.com/toolbox/app/ 下载 .tar.gz 文件。
2. 解压并运行可执行文件：`tar -xzf jetbrains-toolbox-<version>.tar.gz && cd jetbrains-toolbox-<version> && ./jetbrains-toolbox`。
3. Toolbox App 会自动安装到主目录，打开后搜索并安装 RustRover。

**独立安装：**
1. 从 https://www.jetbrains.com/rust/download/ 下载 .tar.gz 文件。
2. 解压到支持执行的目录，例如 `sudo tar -xzf RustRover.tar.gz -C /opt`。
3. 运行解压目录下的 `rustrover.sh` 脚本启动。
4. 可选：通过 Tools | Create Desktop Entry 创建桌面快捷方式。

**Snap 包安装（适用于 Ubuntu 等）：**
1. 确保 snapd 已安装：`sudo apt update && sudo apt install snapd`。
2. 安装：`sudo snap install rustrover --classic`。
3. 运行：`rustrover`。

安装后，首次运行时会提示配置主题、插件等，按照默认设置即可。

### 使用指南
以下是 RustRover 的基本使用步骤，基于官方快速入门指南。

#### 1. 创建或打开项目
- **新建 Cargo 项目：**
    1. 启动 RustRover，点击欢迎界面上的 "New Project" 或 File | New | Project。
    2. 选择 Rust，指定项目路径和名称。
    3. 确认 Rust 工具链位置（如果未安装，可下载 rustup）。
    4. 选择模板（如 Binary 或 Library），点击 Create。
- **打开本地项目：**
    1. File | Open，选择包含 Cargo.toml 的目录，点击 Open。
    2. 选择 "Open as project"。
- **从 VCS 克隆：**
    1. File | New | Project from Version Control，输入 GitHub 等仓库 URL，点击 Clone。

#### 2. 编写和分析代码
- 使用语法高亮、代码补全（Ctrl+Space）和内联提示。
- 查看宏展开：Alt+Enter。
- 快速文档：Ctrl+Q。
- 代码检查：通过 Problems 工具窗口（View | Tool Windows | Problems）查看问题。
- 格式化代码：Ctrl+Alt+L（或启用 Rustfmt 在设置中：Ctrl+Alt+S > Rust | Rustfmt）。

#### 3. 构建和运行
- 在 Cargo 工具窗口（View | Tool Windows | Cargo）双击目标运行。
- 或在代码行号旁点击绿色箭头，选择 Run。
- 使用工具栏中的运行配置：选择配置，点击 Run (Shift+F10)。

#### 4. 调试
- 设置断点：点击代码行号旁。
- 启动调试：代码行号旁点击虫子图标，选择 Debug。
- 在调试会话中，使用步进（F8）、变量监视和内存视图。

#### 5. 测试
- 点击测试函数旁绿色箭头，选择 Run。
- 查看结果在 Run 工具窗口。
- 运行覆盖率：选择 Run with Coverage，结果在 Coverage 工具窗口。

#### 6. 其他功能
- 版本控制：集成 Git，支持克隆、提交、推送（VCS 菜单）。
- 插件安装：File | Settings | Plugins，搜索并安装（如 Rustfmt）。
- 键盘快捷键：学习默认快捷键，或自定义（Ctrl+Alt+S > Keymap）。
- 分享代码：选中代码，右键 > Rust | Share in Playground，生成 GitHub Gist。

如果遇到问题，可以参考官方文档：https://www.jetbrains.com/help/rust/。首次使用时，IDE 会引导你学习基本功能。享受 Rust 开发！