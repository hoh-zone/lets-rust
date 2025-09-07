# rust-analyzer

rust-analyzer 是 Rust 编程语言的官方 LSP（Language Server Protocol）实现，用于为代码编辑器提供智能功能，如代码补全、诊断、跳转定义、悬停提示等。它是 RLS（Rust Language Server）的继任者，性能更高、功能更丰富，专为 IDE 和编辑器集成设计。本教程基于官方文档和社区资源，提供从安装到高级配置的完整指导。 教程分为多个部分，适合初学者到高级用户。当前日期为 2025 年 9 月 7 日，信息基于最新可用文档。

## 1. 介绍
rust-analyzer 是一个模块化的 Rust 编译器前端，专注于实时语义分析。它通过 LSP 协议与编辑器通信，支持 VS Code、Neovim、Vim、Emacs 等流行编辑器。主要优势包括：
- **高性能**：增量分析，支持大型项目。
- **准确性**：基于 Rust 编译器前端，提供精确诊断。
- **功能丰富**：代码补全、错误检查、重构、文档生成等。
- **开源**：MIT/Apache 双许可，社区活跃。

rust-analyzer 需要 Rust 标准库源代码（rust-src），并依赖编辑器的 LSP 客户端。

## 2. 安装
安装 rust-analyzer 需要获取二进制文件和 rust-src 组件。以下是不同平台的方法。

### 前置要求
- 已安装 Rust（通过 rustup）。
- 安装 rust-src：`rustup component add rust-src`（rust-analyzer 会自动尝试安装，但手动确保兼容最新稳定版）。
- 注意：仅支持最新稳定版 std 库源代码。旧工具链可能导致问题，建议更新到最新 stable。

### 方法 1: 通过 rustup（推荐，适用于所有平台）
```bash
rustup component add rust-analyzer
```
- 这会安装最新版本的 rust-analyzer 到 `~/.cargo/bin/`。
- 验证：`rust-analyzer --version`。
- 适用于 Windows、macOS、Linux。

### 方法 2: 手动下载二进制（适用于非 VS Code 编辑器）
从 GitHub Releases 下载：https://github.com/rust-analyzer/releases/latest
1. 选择适合平台的文件，例如：
    - Linux x86_64: `rust-analyzer-x86_64-unknown-linux-gnu.gz`
    - macOS arm64: `rust-analyzer-aarch64-apple-darwin.gz`
    - Windows: `rust-analyzer-x86_64-pc-windows-msvc.zip`
2. 下载并解压：
    - Linux/macOS 示例：
      ```bash
      mkdir -p ~/.local/bin
      curl -L https://github.com/rust-analyzer/releases/latest/download/rust-analyzer-x86_64-unknown-linux-gnu.gz | gunzip -c - > ~/.local/bin/rust-analyzer
      chmod +x ~/.local/bin/rust-analyzer
      ```
    - Windows：解压 zip 文件，重命名为 `rust-analyzer.exe`，添加到 PATH。
3. 确保 `~/.local/bin`（或等效路径）在 `$PATH` 中。
- 其他平台：使用 Homebrew（macOS）：`brew install rust-analyzer`；Arch Linux：`pacman -S rust-analyzer`；Gentoo：启用 `rust-analyzer` use flag。

### 方法 3: 从源代码构建
```bash
git clone https://github.com/rust-lang/rust-analyzer.git
cd rust-analyzer
cargo xtask install --server
```
- 需要最新 stable Rust 工具链。
- 适用于开发或自定义版本。

### 故障排除（安装）
- 二进制未找到：检查 PATH，确保编辑器从 shell 启动（Unix 上修改 .desktop 文件设置环境）。
- Windows：安装 Microsoft Visual C++ Redistributable。
- 版本过旧：每周更新 rust-analyzer 以匹配 Rust 更新。

## 3. 编辑器配置
rust-analyzer 通过 LSP 集成编辑器。VS Code 扩展内置二进制，其他编辑器需手动配置。

### VS Code
1. 安装扩展：搜索 "rust-analyzer"（由 rust-lang 发布），安装。
2. 扩展自动下载并使用 rust-analyzer。
3. 配置（.vscode/settings.json）：
   ```json
   {
     "rust-analyzer.server.extraEnv": {
       "RUSTUP_TOOLCHAIN": "stable"  // 覆盖工具链
     }
   }
   ```
4. 启用功能：如 inlay hints（设置中搜索 "rust-analyzer.inlayHints"）。
- 快捷键：Ctrl+Shift+P > "rust-analyzer: Show RA Version" 检查版本。
- 高级：启用 proc-macro 支持和 build scripts（默认启用）。

### Neovim (0.5+)
1. 使用 nvim-lspconfig 插件：
    - 在 init.lua 或 plugins 中添加：
      ```lua
      require'lspconfig'.rust_analyzer.setup({
        on_attach = function(client, bufnr)
          vim.lsp.inlay_hint.enable(true, { bufnr = bufnr })  // Neovim 0.10+ inlay hints
        end,
        settings = {
          ["rust-analyzer"] = {
            imports = {
              granularity = { group = "module" },
              prefix = "self"
            },
            cargo = { buildScripts = { enable = true } },
            procMacro = { enable = true }
          }
        }
      })
      ```
2. 安装 mason.nvim 或类似插件自动管理 LSP。
3. 补全：使用 nvim-cmp 插件集成。
- 故障排除：确保 rust-analyzer 在 PATH 中。

### Vim
1. 使用 coc.nvim（需要 Node.js）：
    - 安装 coc.nvim，然后 `:CocInstall coc-rust-analyzer`。
2. 或 LanguageClient-neovim：
   ```vim
   let g:LanguageClient_serverCommands = {
     \ 'rust': ['rust-analyzer'],
   \ }
   ```
3. 或 ALE：
   ```vim
   let g:ale_linters = {'rust': ['analyzer']}
   ```
4. 或 vim-lsp：
   ```vim
   if executable('rust-analyzer')
     au User lsp_setup call lsp#register_server({
       \ 'name': 'Rust Language Server',
       \ 'cmd': {server_info->['rust-analyzer']},
       \ 'whitelist': ['rust'],
       \ 'initialization_options': {
       \   'cargo': { 'buildScripts': { 'enable': v:true } },
       \   'procMacro': { 'enable': v:true }
       \ },
     \ })
   endif
   ```
- 补全和诊断通过插件提供。

### Emacs
1. 使用 Eglot（Emacs 29+ 内置）：
   ```elisp
   (add-hook 'rust-mode-hook 'eglot-ensure)
   ```
    - 配置 clippy：
      ```elisp
      (add-to-list 'eglot-server-programs
                   '((rust-ts-mode rust-mode) .
                     ("rust-analyzer" :initializationOptions (:check (:command "clippy")))))
      ```
2. 或 LSP Mode：
   ```elisp
   (add-hook 'rust-mode-hook 'lsp-deferred)
   ```
    - 安装 lsp-mode 和 lsp-ui、dap-mode。
- 高级：eglot-x 扩展支持 rust-analyzer 特定功能。

### 其他编辑器
- JetBrains Fleet：添加 settings.json 配置 rust-analyzer。
- Helix：内置支持，配置 lsp.rust-analyzer。
- 参考官方手册：https://rust-analyzer.github.io/book/other_editors.html

## 4. 关键功能
rust-analyzer 提供丰富的 LSP 功能。

### 诊断 (Diagnostics)
- 实时错误检查，包括类型错误、借用检查。
- 实验诊断：启用 `diagnostics.experimental.enable = true`。
- 禁用特定诊断：`diagnostics.disabled = ["clippy::needless_return"]`。

### 代码补全 (Completion)
- 自动导入：启用 `completion.autoimport.enable = true`，分组导入（std、外部、当前 crate）。
- 魔法补全：如 `if` 自动加括号，后缀补全 `expr.if`。
- 自定义片段：`completion.snippets.custom` 定义如 "Ok" postfix。
- 限制：`completion.limit = 100`。

### 导航 (Navigation)
- Goto Definition/Declaration/Implementation/Type Definition。
- Find All References：包括宏展开和构造函数。
- Workspace Symbol：模糊搜索符号，使用 `#` 搜索类型。
- File Structure：文件大纲和面包屑导航。
- Matching Brace：跳转匹配括号。

### 悬停和提示 (Hover & Inlay Hints)
- Hover：显示类型、文档；启用 `hover.documentation.enable = true`。
- Inlay Hints：变量类型、参数提示；`inlayHints.typeHints.enable = true`，最大长度 `inlayHints.maxLength = 25`。
- 关闭提示：`inlayHints.closingEntAngleBrackets.enable = false`。

### 高级功能
- 宏展开：递归展开宏。
- 结构化搜索替换 (SSR)：使用通配符匹配 AST 节点。
- 查看语法树、HIR/MIR、内存布局、crate 图（需 dot 工具）。
- Run/Debug：建议运行测试/二进制，Peek Related Tests。
- 重命名 (Rename)：重命名项及其引用。
- 解释函数/常量：评估值。
- Join Lines/Move Item：智能加入行/移动项。
- On Enter/Typing：自动缩进、添加分号。
- 语义高亮：标记类型和修饰符。

## 5. 高级配置
配置通过 LSP 初始化选项（JSON 对象），键忽略 `rust-analyzer.` 前缀。文件位置依编辑器而定（如 VS Code 的 settings.json）。

### 配置位置
- VS Code：settings.json 中的 "rust-analyzer" 对象。
- Neovim：lspconfig settings。
- 全局：rust-analyzer.toml（实验性）。
- 验证：设置 `RA_LOG=rust_analyzer=info` 查看日志。

### 关键配置选项
- **Cargo**：`cargo.buildScripts.enable = true`（构建脚本），`cargo.features = ["foo"]`（特性）。
- **Check**：`check.command = "clippy"`（检查命令）。
- **Completion**：`completion.postfix.enable = true`（后缀补全）。
- **Diagnostics**：`diagnostics.enable = false`（禁用）。
- **Hover**：`hover.actions.enable = true`。
- **Inlay Hints**：`inlayHints.lifetimeElisionHints.enable = "skip_trivial"`。
- **Proc Macro**：`procMacro.enable = true`（宏支持）。
- **性能**：`numThreads = 8`（线程数），`cachePriming.enable = true`。

示例（VS Code settings.json）：
```json
{
  "rust-analyzer": {
    "cargo": {
      "buildScripts": { "enable": true }
    },
    "procMacro": { "enable": true },
    "inlayHints": {
      "typeHints": { "enable": true }
    }
  }
}
```

## 6. 故障排除
常见问题及解决：

1. **版本过旧**：运行 "rust-analyzer: Show RA Version" 检查，每周更新。
2. **崩溃/日志**：查看 Output > Rust Analyzer Language Server，设置 `RA_LOG=info` 增加细节。启用 LSP 日志：`rust-analyzer: Toggle LSP Logs`。
3. **项目加载失败**：检查状态栏错误，运行 `RA_LOG=project_model=debug`。使用 `rust-analyzer analysis-stats .` 批量检查。
4. **工具链问题**：确保使用最新 stable，设置 `RUSTUP_TOOLCHAIN=stable`。
5. **二进制未找到**：确认 PATH，编辑器从 shell 启动。
6. **报告问题**：提供最小示例、版本、`analysis-stats` 输出，到 Rust 论坛（IDEs 类别）或 Zulip WG。
7. **慢速**：禁用实验诊断，减少线程数。

## 7. 高级主题
- **贡献**：阅读 CONTRIBUTING.md，修改源代码运行 `cargo xtask install`。
- **安全/隐私**：rust-analyzer 不收集数据，详见 https://rust-analyzer.github.io/book/security.html。
- **自定义**：使用 `ra_ap_rust_analyzer` crate 程序化使用。

通过本教程，你可以快速上手 rust-analyzer，提升 Rust 开发效率。更多详情参考官方手册：https://rust-analyzer.github.io/book/