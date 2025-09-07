# Cargo

Cargo 是 Rust 编程语言的包管理和构建工具，用于管理依赖、构建项目、运行测试等。它支持丰富的命令行选项，允许开发者精细控制项目生命周期。本教程基于官方文档和社区资源，提供超级扩展的指导，分为50个独立教程部分，每个部分聚焦一个关键命令或选项组合使用场景。每个教程包括：

- **描述**：命令的功能说明。
- **语法**：基本命令格式。
- **示例**：实际命令和预期效果（假设有一个简单的 Cargo 项目，如 `cargo new myproject` 创建的）。
- **高级提示**：扩展用法或注意事项。

这些教程从基础开始，逐步深入，适合初学者到高级用户。安装 Rust 后，直接在终端运行 `cargo` 即可开始实验。注意：这些命令通常在项目目录下运行。

## 教程1: 获取帮助信息 (--help)
**描述**：显示 Cargo 的所有命令和简要说明，帮助快速上手。  
**语法**：`cargo --help` 或 `cargo help <command>`。  
**示例**：  
`cargo --help`  
输出：列出所有命令，如 `build`、`run` 等。  
**高级提示**：`cargo help build` 获取特定命令的详细帮助。

## 教程2: 查看版本信息 (--version)
**描述**：打印当前 Cargo 版本，便于检查兼容性。  
**语法**：`cargo --version` 或 `cargo version`。  
**示例**：  
`cargo --version`  
输出：`cargo 1.81.0 (eeb90cda0 2024-09-04)`（版本依安装而定）。  
**高级提示**：结合 `--verbose`：`cargo --version --verbose` 获取更多细节。

## 教程3: 创建新项目 (new)
**描述**：创建一个新的 Cargo 项目，包括基本结构。  
**语法**：`cargo new <project-name>`。  
**示例**：  
`cargo new myproject`  
生成 `myproject/` 目录，包含 `Cargo.toml` 和 `src/main.rs`。  
**高级提示**：`cargo new myproject --lib` 创建库项目。

## 教程4: 初始化现有目录 (init)
**描述**：在当前目录初始化 Cargo 项目。  
**语法**：`cargo init`。  
**示例**：  
`cd existing_dir && cargo init`  
生成 `Cargo.toml` 和 `src/`。  
**高级提示**：`cargo init --name myproj` 指定项目名。

## 教程5: 构建项目 (build)
**描述**：编译项目生成可执行文件或库。  
**语法**：`cargo build`。  
**示例**：  
`cargo build`  
在 `target/debug/` 生成二进制文件。  
**高级提示**：`cargo build --bin mybin` 指定构建特定二进制。

## 教程6: 运行项目 (run)
**描述**：构建并运行项目的主二进制。  
**语法**：`cargo run`。  
**示例**：  
`cargo run`  
输出 "Hello, world!"（假设默认代码）。  
**高级提示**：`cargo run -- <args>` 传递参数，如 `cargo run -- hello`。

## 教程7: 测试项目 (test)
**描述**：运行项目的单元测试和集成测试。  
**语法**：`cargo test`。  
**示例**：  
`cargo test`  
运行所有测试并报告结果。  
**高级提示**：`cargo test my_test` 运行特定测试函数。

## 教程8: 检查项目 (check)
**描述**：检查代码是否能编译，而不生成输出。  
**语法**：`cargo check`。  
**示例**：  
`cargo check`  
快速验证语法和类型错误。  
**高级提示**：`cargo check --all-features` 检查所有特性。

## 教程9: 清理项目 (clean)
**描述**：移除构建生成的 artifacts。  
**语法**：`cargo clean`。  
**示例**：  
`cargo clean`  
删除 `target/` 目录。  
**高级提示**：`cargo clean --release` 只清理 release 构建。

## 教程10: 生成文档 (doc)
**描述**：构建项目的文档。  
**语法**：`cargo doc`。  
**示例**：  
`cargo doc`  
在 `target/doc/` 生成 HTML 文档。  
**高级提示**：`cargo doc --open` 自动打开浏览器查看。

## 教程11: 基准测试 (bench)
**描述**：运行项目的基准测试。  
**语法**：`cargo bench`。  
**示例**：  
`cargo bench`  
执行性能基准并报告。  
**高级提示**：`cargo bench --no-run` 只构建不运行。

## 教程12: 更新依赖 (update)
**描述**：更新 Cargo.lock 中的依赖版本。  
**语法**：`cargo update`。  
**示例**：  
`cargo update`  
更新所有依赖到最新兼容版本。  
**高级提示**：`cargo update -p rand` 更新特定包。

## 教程13: 搜索 crate (search)
**描述**：在 crates.io 搜索包。  
**语法**：`cargo search <crate>`。  
**示例**：  
`cargo search serde`  
列出相关 crate 和描述。  
**高级提示**：`cargo search serde --limit 5` 限制结果数。

## 教程14: 安装 crate (install)
**描述**：从 crates.io 安装二进制 crate。  
**语法**：`cargo install <crate>`。  
**示例**：  
`cargo install ripgrep`  
安装 ripgrep 命令行工具。  
**高级提示**：`cargo install --path .` 从本地路径安装。

## 教程15: 卸载 crate (uninstall)
**描述**：卸载已安装的二进制 crate。  
**语法**：`cargo uninstall <crate>`。  
**示例**：  
`cargo uninstall ripgrep`  
移除 ripgrep。  
**高级提示**：`cargo uninstall --root /path` 指定安装根目录。

## 教程16: 添加依赖 (add)
**描述**：向 Cargo.toml 添加依赖。  
**语法**：`cargo add <crate>`。  
**示例**：  
`cargo add serde`  
添加 serde 到 [dependencies]。  
**高级提示**：`cargo add serde --features derive` 添加特性。

## 教程17: 移除依赖 (remove / rm)
**描述**：从 Cargo.toml 移除依赖。  
**语法**：`cargo remove <crate>` 或 `cargo rm <crate>`。  
**示例**：  
`cargo remove serde`  
移除 serde 依赖。  
**高级提示**：`cargo remove --dev rand` 移除开发依赖。

## 教程18: 格式化代码 (fmt)
**描述**：使用 rustfmt 格式化源代码。  
**语法**：`cargo fmt`。  
**示例**：  
`cargo fmt`  
格式化所有 .rs 文件。  
**高级提示**：`cargo fmt --check` 检查而不修改。

## 教程19: 修复警告 (fix)
**描述**：自动修复 rustc 报告的 lint 警告。  
**语法**：`cargo fix`。  
**示例**：  
`cargo fix`  
应用修复到代码。  
**高级提示**：`cargo fix --edition` 迁移到新 edition。

## 教程20: 显示依赖树 (tree)
**描述**：可视化依赖图。  
**语法**：`cargo tree`。  
**示例**：  
`cargo tree`  
输出依赖树结构。  
**高级提示**：`cargo tree -d` 显示重复依赖。

## 教程21: 元数据输出 (metadata)
**描述**：打印项目元数据为 JSON。  
**语法**：`cargo metadata`。  
**示例**：  
`cargo metadata`  
输出包信息 JSON。  
**高级提示**：`cargo metadata --format-version 1` 指定格式。

## 教程22: 发布包 (publish)
**描述**：上传包到 crates.io。  
**语法**：`cargo publish`。  
**示例**：  
`cargo publish`  
发布当前包。  
**高级提示**：`cargo publish --dry-run` 模拟发布。

## 教程23: 打包项目 (package)
**描述**：组装项目为 tarball。  
**语法**：`cargo package`。  
**示例**：  
`cargo package`  
生成 .crate 文件。  
**高级提示**：`cargo package --list` 列出包含文件。

## 教程24: 登录 registry (login)
**描述**：登录 crates.io。  
**语法**：`cargo login`。  
**示例**：  
`cargo login <token>`  
保存 API token。  
**高级提示**：`cargo login --registry myreg` 指定 registry。

## 教程25: 登出 registry (logout)
**描述**：登出 crates.io。  
**语法**：`cargo logout`。  
**示例**：  
`cargo logout`  
移除 token。  
**高级提示**：`cargo logout --registry myreg` 指定 registry。

## 教程26: 生成锁文件 (generate-lockfile)
**描述**：生成 Cargo.lock 文件。  
**语法**：`cargo generate-lockfile`。  
**示例**：  
`cargo generate-lockfile`  
创建或更新锁文件。  
**高级提示**：用于 CI，确保一致性。

## 教程27: 读取 manifest (read-manifest)
**描述**：打印 Cargo.toml 为 JSON。  
**语法**：`cargo read-manifest`。  
**示例**：  
`cargo read-manifest`  
输出 manifest JSON。  
**高级提示**：用于脚本集成。

## 教程28: 验证项目 (verify-project)
**描述**：检查 crate manifest 的正确性。  
**语法**：`cargo verify-project`。  
**示例**：  
`cargo verify-project`  
报告错误。  
**高级提示**：在发布前运行。

## 教程29: 拉取 crate (yank)
**描述**：从索引移除已推送的 crate。  
**语法**：`cargo yank <crate>@<version>`。  
**示例**：  
`cargo yank mycrate@0.1.0`  
yank 指定版本。  
**高级提示**：`cargo yank --undo` 撤销 yank。

## 教程30: 传递 rustc 选项 (rustc)
**描述**：编译包并传递额外选项给 rustc。  
**语法**：`cargo rustc -- <rustc_flags>`。  
**示例**：  
`cargo rustc -- -C opt-level=3`  
自定义优化。  
**高级提示**：用于调试 compiler flags。

## 教程31: 传递 rustdoc 选项 (rustdoc)
**描述**：构建文档并传递自定义 flags。  
**语法**：`cargo rustdoc -- <rustdoc_flags>`。  
**示例**：  
`cargo rustdoc -- --all-features`  
文档所有特性。  
**高级提示**：结合 `--open` 查看。

## 教程32: 定位项目 (locate-project)
**描述**：打印 Cargo.toml 的位置 JSON。  
**语法**：`cargo locate-project`。  
**示例**：  
`cargo locate-project`  
输出路径。  
**高级提示**：`cargo locate-project --workspace` 处理 workspace。

## 教程33: 别名 b (build)
**描述**：`cargo build` 的别名。  
**语法**：`cargo b`。  
**示例**：  
`cargo b`  
快速构建。  
**高级提示**：结合其他 flags 如 `cargo b --release`。

## 教程34: 别名 d (doc)
**描述**：`cargo doc` 的别名。  
**语法**：`cargo d`。  
**示例**：  
`cargo d`  
生成文档。  
**高级提示**：`cargo d --open`。

## 教程35: 别名 t (test)
**描述**：`cargo test` 的别名。  
**语法**：`cargo t`。  
**示例**：  
`cargo t`  
运行测试。  
**高级提示**：`cargo t -- --nocapture` 显示输出。

## 教程36: 发布模式 (--release)
**描述**：以优化模式构建。  
**语法**：`cargo build --release`。  
**示例**：  
`cargo build --release`  
在 `target/release/` 生成优化二进制。  
**高级提示**：用于生产部署。

## 教程37: 指定目标 (--target)
**描述**：交叉编译到特定平台。  
**语法**：`cargo build --target <triple>`。  
**示例**：  
`cargo build --target x86_64-unknown-linux-gnu`  
针对 Linux x64。  
**高级提示**：需安装目标 toolchain。

## 教程38: 详细输出 (--verbose)
**描述**：启用详细日志。  
**语法**：`cargo build --verbose`。  
**示例**：  
`cargo build -v`  
显示详细构建过程。  
**高级提示**：`-vv` 更详细。

## 教程39: 安静模式 (--quiet)
**描述**：抑制非错误输出。  
**语法**：`cargo build --quiet`。  
**示例**：  
`cargo build -q`  
安静构建。  
**高级提示**：用于脚本。

## 教程40: 启用特性 (--features)
**描述**：激活特定特性。  
**语法**：`cargo build --features <feat1,feat2>`。  
**示例**：  
`cargo build --features "json"`  
启用 json 特性。  
**高级提示**：`--all-features` 启用所有。

## 教程41: 无默认特性 (--no-default-features)
**描述**：禁用默认特性。  
**语法**：`cargo build --no-default-features`。  
**示例**：  
`cargo build --no-default-features`  
最小构建。  
**高级提示**：结合 `--features` 指定。

## 教程42: 指定 profile (--profile)
**描述**：使用自定义构建 profile。  
**语法**：`cargo build --profile <name>`。  
**示例**：  
`cargo build --profile release-lto`  
使用自定义 profile。  
**高级提示**：在 Cargo.toml 定义 profile。

## 教程43: 离线模式 (--offline)
**描述**：不访问网络。  
**语法**：`cargo build --offline`。  
**示例**：  
`cargo build --offline`  
使用本地缓存。  
**高级提示**：结合 `--frozen` 锁定依赖。

## 教程44: 锁定模式 (--locked)
**描述**：要求 Cargo.lock 存在且不更新。  
**语法**：`cargo build --locked`。  
**示例**：  
`cargo build --locked`  
确保可重现构建。  
**高级提示**：用于 CI/CD。

## 教程45: 指定 manifest (--manifest-path)
**描述**：使用自定义 Cargo.toml 路径。  
**语法**：`cargo build --manifest-path <path>`。  
**示例**：  
`cargo build --manifest-path sub/Cargo.toml`  
构建子项目。  
**高级提示**：用于 monorepo。

## 教程46: 工作空间模式 (--workspace)
**描述**：操作整个 workspace。  
**语法**：`cargo build --workspace`。  
**示例**：  
`cargo build --workspace`  
构建所有成员。  
**高级提示**：`--workspace --exclude <pkg>` 排除特定包。

## 教程47: 指定包 (--package / -p)
**描述**：操作特定包。  
**语法**：`cargo build -p <pkg>`。  
**示例**：  
`cargo build -p mylib`  
只构建 mylib。  
**高级提示**：多包：`-p pkg1 -p pkg2`。

## 教程48: 作业数 (--jobs / -j)
**描述**：设置并行作业数。  
**语法**：`cargo build -j <n>`。  
**示例**：  
`cargo build -j 4`  
使用 4 个核心。  
**高级提示**：` -j 1` 序列化构建调试。

## 教程49: 颜色控制 (--color)
**描述**：控制输出颜色。  
**语法**：`cargo build --color <always|never|auto>`。  
**示例**：  
`cargo build --color always`  
始终使用颜色。  
**高级提示**：默认 auto。

## 教程50: 计时输出 (--timings)
**描述**：输出编译计时信息。  
**语法**：`cargo build --timings`。  
**示例**：  
`cargo build --timings`  
生成 HTML 报告。  
**高级提示**：`--timings=html,json` 多格式输出。
