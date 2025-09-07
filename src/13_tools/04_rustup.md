# Rustup

Rustup 是 Rust 编程语言的工具链管理器，用于安装、更新和管理 Rust 版本、组件和目标平台。它支持丰富的命令行选项，允许开发者轻松切换工具链和配置环境。本教程基于官方文档和社区资源，提供超级扩展的指导，分为50个独立教程部分，每个部分聚焦一个关键命令或选项组合使用场景。每个教程包括：

- **描述**：命令的功能说明。
- **语法**：基本命令格式。
- **示例**：实际命令和预期效果（假设已安装 rustup）。
- **高级提示**：扩展用法或注意事项。

这些教程从基础开始，逐步深入，适合初学者到高级用户。安装 rustup 后，直接在终端运行 `rustup` 即可开始实验。注意：这些命令通常全局运行，不限于项目目录。

## 教程1: 获取帮助信息 (--help)
**描述**：显示 rustup 的所有命令和简要说明，帮助快速上手。  
**语法**：`rustup --help` 或 `rustup help <command>`。  
**示例**：  
`rustup --help`  
输出：列出所有子命令，如 `toolchain`、`target` 等。  
**高级提示**：`rustup help toolchain` 获取特定子命令的详细帮助。

## 教程2: 查看版本信息 (--version)
**描述**：打印当前 rustup 版本，便于检查兼容性。  
**语法**：`rustup --version` 或 `rustup version`。  
**示例**：  
`rustup --version`  
输出：`rustup 1.27.1 (2024-04-29)`（版本依安装而定）。  
**高级提示**：结合 `--verbose`：`rustup --version --verbose` 获取更多细节。

## 教程3: 安装工具链 (toolchain install)
**描述**：安装指定的 Rust 工具链版本。  
**语法**：`rustup toolchain install <toolchain>`。  
**示例**：  
`rustup toolchain install stable`  
安装稳定版工具链。  
**高级提示**：`rustup toolchain install nightly --allow-downgrade` 允许降级安装。

## 教程4: 列出工具链 (toolchain list)
**描述**：列出已安装的工具链。  
**语法**：`rustup toolchain list`。  
**示例**：  
`rustup toolchain list`  
输出：如 `stable-x86_64-unknown-linux-gnu (default)`。  
**高级提示**：`rustup toolchain list --verbose` 显示路径。

## 教程5: 更新工具链 (update)
**描述**：更新已安装的工具链到最新版本。  
**语法**：`rustup update`。  
**示例**：  
`rustup update`  
更新所有工具链。  
**高级提示**：`rustup update stable` 更新特定工具链。

## 教程6: 卸载工具链 (toolchain uninstall)
**描述**：移除指定的工具链。  
**语法**：`rustup toolchain uninstall <toolchain>`。  
**示例**：  
`rustup toolchain uninstall beta`  
移除 beta 版。  
**高级提示**：小心使用，避免移除默认工具链。

## 教程7: 设置默认工具链 (default)
**描述**：设置全局默认工具链。  
**语法**：`rustup default <toolchain>`。  
**示例**：  
`rustup default stable`  
将稳定版设为默认。  
**高级提示**：`rustup default nightly` 切换到 nightly。

## 教程8: 添加目标平台 (target add)
**描述**：为当前工具链添加交叉编译目标。  
**语法**：`rustup target add <target>`。  
**示例**：  
`rustup target add wasm32-unknown-unknown`  
添加 WebAssembly 目标。  
**高级提示**：`rustup target add --toolchain nightly` 指定工具链。

## 教程9: 移除目标平台 (target remove)
**描述**：移除指定的目标平台。  
**语法**：`rustup target remove <target>`。  
**示例**：  
`rustup target remove arm-unknown-linux-gnueabihf`  
移除 ARM 目标。  
**高级提示**：检查 `rustup target list` 先确认。

## 教程10: 列出目标平台 (target list)
**描述**：列出可用和已安装的目标平台。  
**语法**：`rustup target list`。  
**示例**：  
`rustup target list`  
输出：可用目标列表。  
**高级提示**：`rustup target list --installed` 只显示已安装。

## 教程11: 添加组件 (component add)
**描述**：为工具链添加组件，如 rustfmt。  
**语法**：`rustup component add <component>`。  
**示例**：  
`rustup component add clippy`  
添加 Clippy lint 工具。  
**高级提示**：`rustup component add --toolchain stable rust-src` 添加源代码。

## 教程12: 移除组件 (component remove)
**描述**：移除指定的组件。  
**语法**：`rustup component remove <component>`。  
**示例**：  
`rustup component remove rls`  
移除 RLS。  
**高级提示**：节省空间时使用。

## 教程13: 列出组件 (component list)
**描述**：列出可用和已安装的组件。  
**语法**：`rustup component list`。  
**示例**：  
`rustup component list`  
输出：组件列表。  
**高级提示**：`rustup component list --installed` 只显示已安装。

## 教程14: 设置目录覆盖 (override set)
**描述**：为当前目录设置工具链覆盖。  
**语法**：`rustup override set <toolchain>`。  
**示例**：  
`rustup override set nightly`  
当前目录使用 nightly。  
**高级提示**：用于项目特定版本。

## 教程15: 移除目录覆盖 (override unset)
**描述**：移除当前目录的工具链覆盖。  
**语法**：`rustup override unset`。  
**示例**：  
`rustup override unset`  
恢复全局默认。  
**高级提示**：`rustup override list` 检查覆盖。

## 教程16: 运行命令 (run)
**描述**：使用指定工具链运行命令。  
**语法**：`rustup run <toolchain> <command>`。  
**示例**：  
`rustup run stable cargo build`  
使用 stable 运行 cargo。  
**高级提示**：临时切换工具链。

## 教程17: 查找二进制路径 (which)
**描述**：显示指定二进制的路径。  
**语法**：`rustup which <command>`。  
**示例**：  
`rustup which rustc`  
输出 rustc 路径。  
**高级提示**：`rustup which --toolchain nightly rustc` 指定工具链。

## 教程18: 显示信息 (show)
**描述**：显示当前工具链和覆盖信息。  
**语法**：`rustup show`。  
**示例**：  
`rustup show`  
输出活跃工具链。  
**高级提示**：`rustup show active-toolchain` 只显示活跃。

## 教程19: 打开文档 (doc)
**描述**：打开 Rust 文档。  
**语法**：`rustup doc`。  
**示例**：  
`rustup doc`  
在浏览器打开 std 文档。  
**高级提示**：`rustup doc --book` 打开 The Book。

## 教程20: 打开手册 (man)
**描述**：显示 Rust 工具的手册页。  
**语法**：`rustup man <command>`。  
**示例**：  
`rustup man rustc`  
显示 rustc 手册。  
**高级提示**：需安装 man 组件。

## 教程21: 自更新 (self update)
**描述**：更新 rustup 本身。  
**语法**：`rustup self update`。  
**示例**：  
`rustup self update`  
更新到最新 rustup。  
**高级提示**：定期运行保持最新。

## 教程22: 自卸载 (self uninstall)
**描述**：卸载 rustup 和所有工具链。  
**语法**：`rustup self uninstall`。  
**示例**：  
`rustup self uninstall`  
移除一切。  
**高级提示**：确认前备份。

## 教程23: 遥测启用 (telemetry enable)
**描述**：启用 rustup 遥测。  
**语法**：`rustup telemetry enable`。  
**示例**：  
`rustup telemetry enable`  
开始收集数据。  
**高级提示**：用于贡献社区。

## 教程24: 遥测禁用 (telemetry disable)
**描述**：禁用 rustup 遥测。  
**语法**：`rustup telemetry disable`。  
**示例**：  
`rustup telemetry disable`  
停止收集。  
**高级提示**：隐私优先。

## 教程25: 转储测试 (dump-testament)
**描述**：转储 rustup 测试文件。  
**语法**：`rustup dump-testament`。  
**示例**：  
`rustup dump-testament`  
输出 JSON 测试。  
**高级提示**：开发使用。

## 教程26: 详细模式 (--verbose / -v)
**描述**：启用详细输出。  
**语法**：`rustup -v <command>`。  
**示例**：  
`rustup -v update`  
显示详细更新过程。  
**高级提示**：调试问题时使用。

## 教程27: 安静模式 (--quiet / -q)
**描述**：抑制非错误输出。  
**语法**：`rustup -q <command>`。  
**示例**：  
`rustup -q install stable`  
安静安装。  
**高级提示**：脚本中使用。

## 教程28: 指定配置文件 (--config)
**描述**：使用自定义配置文件。  
**语法**：`rustup --config <file> <command>`。  
**示例**：  
`rustup --config custom.toml install stable`  
使用自定义 config。  
**高级提示**：自定义设置。

## 教程29: 工具链链接 (toolchain link)
**描述**：链接自定义工具链。  
**语法**：`rustup toolchain link <name> <path>`。  
**示例**：  
`rustup toolchain link custom /path/to/rust`  
链接自定义构建。  
**高级提示**：用于自编译 Rust。

## 教程30: 工具链移除 (toolchain remove)
**描述**：别名卸载工具链。  
**语法**：`rustup toolchain remove <toolchain>`。  
**示例**：  
`rustup toolchain remove stable`  
移除稳定版。  
**高级提示**：与 uninstall 同。

## 教程31: 目标安装 (target install)
**描述**：别名添加目标。  
**语法**：`rustup target install <target>`。  
**示例**：  
`rustup target install x86_64-apple-darwin`  
安装目标。  
**高级提示**：交叉编译。

## 教程32: 组件安装 (component install)
**描述**：别名添加组件。  
**语法**：`rustup component install <component>`。  
**示例**：  
`rustup component install rustfmt`  
安装格式化工具。  
**高级提示**：IDE 集成。

## 教程33: 覆盖列表 (override list)
**描述**：列出所有覆盖。  
**语法**：`rustup override list`。  
**示例**：  
`rustup override list`  
显示目录覆盖。  
**高级提示**：管理多项目。

## 教程34: 显示首页 (show home)
**描述**：显示 Rust 安装目录。  
**语法**：`rustup show home`。  
**示例**：  
`rustup show home`  
输出 ~/.rustup。  
**高级提示**：自定义 RUSTUP_HOME。

## 教程35: 显示工具链 (show toolchain)
**描述**：显示活跃工具链。  
**语法**：`rustup show toolchain`。  
**示例**：  
`rustup show toolchain`  
输出当前工具链。  
**高级提示**：脚本中使用。

## 教程36: 文档主题 (doc --std)
**描述**：打开标准库文档。  
**语法**：`rustup doc --std`。  
**示例**：  
`rustup doc --std`  
打开 std 文档。  
**高级提示**：离线浏览。

## 教程37: 自升级 (self upgrade)
**描述**：升级 rustup。  
**语法**：`rustup self upgrade`。  
**示例**：  
`rustup self upgrade`  
升级到最新。  
**高级提示**：与 update 同。

## 教程38: 遥测分析 (telemetry analyze)
**描述**：分析遥测数据。  
**语法**：`rustup telemetry analyze`。  
**示例**：  
`rustup telemetry analyze`  
显示统计。  
**高级提示**：开发工具。

## 教程39: 安装 nightly (toolchain install nightly)
**描述**：安装 nightly 工具链。  
**语法**：`rustup toolchain install nightly`。  
**示例**：  
`rustup toolchain install nightly`  
安装实验版。  
**高级提示**：测试新特性。

## 教程40: 更新所有 (update --force)
**描述**：强制更新工具链。  
**语法**：`rustup update --force`。  
**示例**：  
`rustup update --force`  
强制刷新。  
**高级提示**：解决缓存问题。

## 教程41: 组件 rust-analyzer (component add rust-analyzer)
**描述**：添加 rust-analyzer 组件。  
**语法**：`rustup component add rust-analyzer`。  
**示例**：  
`rustup component add rust-analyzer`  
用于 LSP。  
**高级提示**：VS Code 集成。

## 教程42: 目标 wasm (target add wasm32-wasi)
**描述**：添加 WASI 目标。  
**语法**：`rustup target add wasm32-wasi`。  
**示例**：  
`rustup target add wasm32-wasi`  
WebAssembly 系统接口。  
**高级提示**：Web 开发。

## 教程43: 运行 rustc (run stable rustc)
**描述**：运行特定 rustc。  
**语法**：`rustup run stable rustc hello.rs`。  
**示例**：  
`rustup run stable rustc hello.rs`  
编译文件。  
**高级提示**：测试版本差异。

## 教程44: 显示 profile (show profile)
**描述**：显示当前 profile。  
**语法**：`rustup show profile`。  
**示例**：  
`rustup show profile`  
输出 minimal/default 等。  
**高级提示**：自定义安装。

## 教程45: 设置 profile (set profile)
**描述**：设置默认安装 profile。  
**语法**：`rustup set profile <profile>`。  
**示例**：  
`rustup set profile minimal`  
最小安装。  
**高级提示**：节省空间。

## 教程46: 代理设置 (proxy)
**描述**：设置代理工具链。  
**语法**：`rustup proxy <toolchain>`。  
**示例**：  
`rustup proxy stable`  
代理执行。  
**高级提示**：高级使用。

## 教程47: 环境变量 (RUSTUP_TOOLCHAIN)
**描述**：使用环境变量指定工具链。  
**语法**：`RUSTUP_TOOLCHAIN=stable cargo build`。  
**示例**：  
`RUSTUP_TOOLCHAIN=stable cargo build`  
临时指定。  
**高级提示**：脚本自动化。

## 教程48: 自定义镜像 (RUSTUP_DIST_SERVER)
**描述**：设置自定义分发服务器。  
**语法**：`export RUSTUP_DIST_SERVER=https://mirror`。  
**示例**：  
`export RUSTUP_DIST_SERVER=https://mirror.rust-lang.org`  
自定义镜像。  
**高级提示**：加速下载。

## 教程49: 禁用更新检查 (RUSTUP_UPDATE_ROOT)
**描述**：自定义更新根。  
**语法**：`export RUSTUP_UPDATE_ROOT=https://custom`。  
**示例**：  
`export RUSTUP_UPDATE_ROOT=https://custom`  
自定义更新。  
**高级提示**：企业环境。

## 教程50: 离线安装 (--no-self-update)
**描述**：安装时禁用自更新。  
**语法**：`rustup toolchain install stable --no-self-update`。  
**示例**：  
`rustup toolchain install stable --no-self-update`  
避免更新 rustup。  
**高级提示**：离线场景。
