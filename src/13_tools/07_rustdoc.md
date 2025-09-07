# Rustdoc

Rustdoc 是 Rust 编程语言的文档生成工具，用于从代码中的文档注释生成 HTML 文档。它支持丰富的命令行选项，允许开发者控制输出格式、包含项、主题和扩展功能。本教程基于官方文档，提供超级扩展的指导，分为50个独立教程部分，每个部分聚焦一个关键命令行选项或组合使用场景。每个教程包括：

- **描述**：选项的功能说明。
- **语法**：基本命令格式。
- **示例**：实际命令和预期效果（假设有一个简单的 `lib.rs` 文件：`/// Hello crate\n pub fn hello() {}`）。
- **高级提示**：扩展用法或注意事项。

这些教程从基础开始，逐步深入，适合初学者到高级用户。安装 Rust 后，直接在终端运行 `rustdoc` 即可开始实验。注意：Rustdoc 通常通过 `cargo doc` 使用，但本教程聚焦纯 rustdoc 命令行。

## 教程1: 获取帮助信息 (-h / --help)
**描述**：显示 rustdoc 的所有命令行选项和简要说明，帮助快速上手。  
**语法**：`rustdoc -h` 或 `rustdoc --help`。  
**示例**：  
`rustdoc -h`  
输出：列出所有选项，如 `-o`、`--crate-name` 等。  
**高级提示**：结合 `--verbose` 查看更多细节。

## 教程2: 查看版本信息 (-V / --version)
**描述**：打印当前 rustdoc 版本，便于检查兼容性。  
**语法**：`rustdoc -V` 或 `rustdoc --version`。  
**示例**：  
`rustdoc -V`  
输出：`rustdoc 1.81.0 (eeb90cda0 2024-09-04)`（版本依安装而定）。  
**高级提示**：使用 `rustdoc --version --verbose` 获取更多细节。

## 教程3: 基本生成文档
**描述**：默认模式下，生成 crate 的 HTML 文档。  
**语法**：`rustdoc <file.rs>`。  
**示例**：  
`rustdoc lib.rs`  
生成 `doc/` 目录下的 HTML 文档。  
**高级提示**：用于库文件，生成 API 文档。

## 教程4: 指定输出目录 (-o / --out-dir)
**描述**：自定义输出目录。  
**语法**：`rustdoc -o <dir> <file.rs>`。  
**示例**：  
`rustdoc -o target/doc lib.rs`  
在 `target/doc/` 生成文档。  
**高级提示**：与 Cargo 的 target/doc 一致。

## 教程5: 指定 crate 名称 (--crate-name)
**描述**：自定义 crate 名称。  
**语法**：`rustdoc --crate-name <name> <file.rs>`。  
**示例**：  
`rustdoc --crate-name mycrate lib.rs`  
生成 mycrate 文档。  
**高级提示**：覆盖文件名推断。

## 教程6: 文档私有项 (--document-private-items)
**描述**：包括私有项在文档中。  
**语法**：`rustdoc --document-private-items <file.rs>`。  
**示例**：  
`rustdoc --document-private-items lib.rs`  
私有函数也生成文档。  
**高级提示**：私有项标记 🔒。

## 教程7: 指定 crate 版本 (--crate-version)
**描述**：添加版本信息到文档。  
**语法**：`rustdoc --crate-version <version> <file.rs>`。  
**示例**：  
`rustdoc --crate-version 1.0.0 lib.rs`  
侧边栏显示 "Version 1.0.0"。  
**高级提示**：用于版本控制。

## 教程8: 添加库搜索路径 (-L / --library-path)
**描述**：指定依赖库路径。  
**语法**：`rustdoc -L <path> <file.rs>`。  
**示例**：  
`rustdoc -L target/debug/deps lib.rs`  
从指定路径加载依赖。  
**高级提示**：多路径：`-L path1 -L path2`。

## 教程9: 指定外部依赖 (--extern)
**描述**：手动指定外部 crate 位置。  
**语法**：`rustdoc --extern <crate>=<path> <file.rs>`。  
**示例**：  
`rustdoc --extern serde=/path/to/serde.rlib lib.rs`  
链接 serde。  
**高级提示**：用于 no_std 或自定义依赖。

## 教程10: 条件编译 (--cfg)
**描述**：启用 cfg 标志。  
**语法**：`rustdoc --cfg <flag> <file.rs>`。  
**示例**：  
`rustdoc --cfg feature="foo" lib.rs`  
启用 foo 特性。  
**高级提示**：与 Cargo.toml features 结合。

## 教程11: 检查 cfg (--check-cfg)
**描述**：检查 cfg 值。  
**语法**：`rustdoc --check-cfg <expr> <file.rs>`。  
**示例**：  
`rustdoc --check-cfg 'cfg(my_cfg, values("foo"))' lib.rs`  
检查 my_cfg 值。  
**高级提示**：确保 cfg 有效性。

## 教程12: 代码生成选项 (-C / --codegen)
**描述**：传递 rustc 代码生成选项。  
**语法**：`rustdoc -C <option> <file.rs>`。  
**示例**：  
`rustdoc -C target_feature=+avx lib.rs`  
启用 AVX。  
**高级提示**：用于特定目标优化。

## 教程13: 指定 Rust 版本 (--edition)
**描述**：选择 Rust edition。  
**语法**：`rustdoc --edition <year> <file.rs>`。  
**示例**：  
`rustdoc --edition 2021 lib.rs`  
使用 2021 edition。  
**高级提示**：默认 2015。

## 教程14: 交叉文档目标 (--target)
**描述**：指定目标平台。  
**语法**：`rustdoc --target <triple> <file.rs>`。  
**示例**：  
`rustdoc --target x86_64-unknown-linux-gnu lib.rs`  
针对 Linux x64。  
**高级提示**：需安装目标。

## 教程15: 限制 lint (--cap-lints)
**描述**：设置 lint 上限。  
**语法**：`rustdoc --cap-lints <level> <file.rs>`。  
**示例**：  
`rustdoc --cap-lints warn lib.rs`  
lint 最多 warn。  
**高级提示**：用于子 crate。

## 教程16: 错误格式 (--error-format)
**描述**：自定义错误输出格式。  
**语法**：`rustdoc --error-format <format> <file.rs>`。  
**示例**：  
`rustdoc --error-format json lib.rs`  
JSON 输出。  
**高级提示**：用于 IDE。

## 教程17: JSON 输出 (--json)
**描述**：启用 JSON 输出。  
**语法**：`rustdoc --json <kinds> <file.rs>`。  
**示例**：  
`rustdoc --json diagnostic-rendered-ansi lib.rs`  
JSON 诊断。  
**高级提示**：多种类：`--json artifacts,diagnostic-short`。

## 教程18: 颜色控制 (--color)
**描述**：控制输出颜色。  
**语法**：`rustdoc --color <mode> <file.rs>`。  
**示例**：  
`rustdoc --color always lib.rs`  
始终颜色。  
**高级提示**：模式：auto, always, never。

## 教程19: 输出类型 (--emit)
**描述**：控制生成文件类型。  
**语法**：`rustdoc --emit <types> <file.rs>`。  
**示例**：  
`rustdoc --emit metadata lib.rs`  
生成元数据。  
**高级提示**：类型：unversioned-shared, link, dep-info 等。

## 教程20: 外部 HTML 根 URL (--extern-html-root-url)
**描述**：设置外部 crate HTML 根 URL。  
**语法**：`rustdoc --extern-html-root-url <crate>=<url> <file.rs>`。  
**示例**：  
`rustdoc --extern-html-root-url std=https://doc.rust-lang.org/std lib.rs`  
链接 std 文档。  
**高级提示**：用于交叉链接。

## 教程21: 外部 HTML 根优先 (--extern-html-root-takes-precedence)
**描述**：外部根 URL 优先本地。  
**语法**：`rustdoc --extern-html-root-takes-precedence <file.rs>`。  
**示例**：  
`rustdoc --extern-html-root-takes-precedence lib.rs`  
优先 URL 链接。  
**高级提示**：避免本地依赖文档。

## 教程22: Markdown CSS (--markdown-css)
**描述**：添加 Markdown CSS 文件链接。  
**语法**：`rustdoc --markdown-css <file> <file.rs>`。  
**示例**：  
`rustdoc --markdown-css style.css lib.rs`  
添加 CSS 到 Markdown 渲染。  
**高级提示**：多文件：多次使用。

## 教程23: 无 Markdown TOC (--markdown-no-toc)
**描述**：禁用 Markdown TOC 生成。  
**语法**：`rustdoc --markdown-no-toc <file.rs>`。  
**示例**：  
`rustdoc --markdown-no-toc lib.rs`  
无目录。  
**高级提示**：用于自定义 TOC。

## 教程24: HTML 头部 (--html-in-header)
**描述**：添加 HTML 文件到头部。  
**语法**：`rustdoc --html-in-header <file> <file.rs>`。  
**示例**：  
`rustdoc --html-in-header header.html lib.rs`  
插入 <head> 内容。  
**高级提示**：自定义元标签。

## 教程25: HTML 前内容 (--html-before-content)
**描述**：添加 HTML 到内容前。  
**语法**：`rustdoc --html-before-content <file> <file.rs>`。  
**示例**：  
`rustdoc --html-before-content intro.html lib.rs`  
添加介绍。  
**高级提示**：用于自定义页面。

## 教程26: HTML 后内容 (--html-after-content)
**描述**：添加 HTML 到内容后。  
**语法**：`rustdoc --html-after-content <file> <file.rs>`。  
**示例**：  
`rustdoc --html-after-content footer.html lib.rs`  
添加页脚。  
**高级提示**：版权信息。

## 教程27: 扩展 CSS (--extend-css)
**描述**：扩展默认 CSS。  
**语法**：`rustdoc --extend-css <file> <file.rs>`。  
**示例**：  
`rustdoc --extend-css extra.css lib.rs`  
追加 CSS 规则。  
**高级提示**：覆盖主题。

## 教程28: 启用索引页 (--enable-index-page)
**描述**：生成 crates 索引页。  
**语法**：`rustdoc --enable-index-page <file.rs>`。  
**示例**：  
`rustdoc --enable-index-page lib.rs`  
生成 index.html。  
**高级提示**：多 crate 文档。

## 教程29: 指定索引页 (--index-page)
**描述**：自定义索引页。  
**语法**：`rustdoc --index-page <file.md> <file.rs>`。  
**示例**：  
`rustdoc --index-page index.md lib.rs`  
使用 Markdown 生成索引。  
**高级提示**：与 --enable-index-page 结合。

## 教程30: 静态根路径 (--static-root-path)
**描述**：设置静态文件根路径。  
**语法**：`rustdoc --static-root-path <path> <file.rs>`。  
**示例**：  
`rustdoc --static-root-path /static/ lib.rs`  
静态文件链接到 /static/。  
**高级提示**：托管文档时使用。

## 教程31: 持久化 doctests (--persist-doctests)
**描述**：保存 doctest 二进制。  
**语法**：`rustdoc --persist-doctests <dir> <file.rs>`。  
**示例**：  
`rustdoc --persist-doctests target/doctest lib.rs`  
保存测试二进制。  
**高级提示**：调试 doctests。

## 教程32: 显示覆盖率 (--show-coverage)
**描述**：显示文档覆盖率。  
**语法**：`rustdoc --show-coverage <file.rs>`。  
**示例**：  
`rustdoc --show-coverage lib.rs`  
报告未文档项。  
**高级提示**：改进文档完整性。

## 教程33: 启用压缩 (--enable-minification)
**描述**：启用 HTML/JS 压缩。  
**语法**：`rustdoc --enable-minification <file.rs>`。  
**示例**：  
`rustdoc --enable-minification lib.rs`  
减小文件大小。  
**高级提示**：默认启用。

## 教程34: 禁用压缩 (--disable-minification)
**描述**：禁用压缩。  
**语法**：`rustdoc --disable-minification <file.rs>`。  
**示例**：  
`rustdoc --disable-minification lib.rs`  
保持原始格式。  
**高级提示**：调试 JS 时使用。

## 教程35: 指定主题 (--theme)
**描述**：添加自定义主题 CSS。  
**语法**：`rustdoc --theme <file.css> <file.rs>`。  
**示例**：  
`rustdoc --theme dark.css lib.rs`  
使用 dark 主题。  
**高级提示**：多主题：多次使用。

## 教程36: 检查主题 (--check-theme)
**描述**：检查主题文件有效性。  
**语法**：`rustdoc --check-theme <file.css>`。  
**示例**：  
`rustdoc --check-theme dark.css`  
报告缺失规则。  
**高级提示**：开发主题时使用。

## 教程37: 默认主题 (--default-theme)
**描述**：设置默认主题。  
**语法**：`rustdoc --default-theme <name> <file.rs>`。  
**示例**：  
`rustdoc --default-theme ayu lib.rs`  
默认 ayu 主题。  
**高级提示**：覆盖 light。

## 教程38: 运行工具 (--runtool)
**描述**：指定运行工具。  
**语法**：`rustdoc --runtool <tool> <file.rs>`。  
**示例**：  
`rustdoc --runtool custom-tool lib.rs`  
使用自定义工具。  
**高级提示**：扩展处理。

## 教程39: 运行工具参数 (--runtool-arg)
**描述**：传递参数给 runtool。  
**语法**：`rustdoc --runtool-arg <arg> <file.rs>`。  
**示例**：  
`rustdoc --runtool-arg --flag lib.rs`  
传递标志。  
**高级提示**：多次使用多参数。

## 教程40: 测试构建器 (--test-builder)
**描述**：指定测试构建器。  
**语法**：`rustdoc --test-builder <builder> <file.rs>`。  
**示例**：  
`rustdoc --test-builder custom-builder lib.rs`  
自定义测试。  
**高级提示**：用于 doctests。

## 教程41: 测试运行目录 (--test-run-directory)
**描述**：设置测试运行目录。  
**语法**：`rustdoc --test-run-directory <dir> <file.rs>`。  
**示例**：  
`rustdoc --test-run-directory target/test lib.rs`  
在指定目录运行测试。  
**高级提示**：结合 --persist-doctests。

## 教程42: 刮取示例输出 (--scrape-examples-output-path)
**描述**：刮取示例输出路径。  
**语法**：`rustdoc --scrape-examples-output-path <path> <file.rs>`。  
**示例**：  
`rustdoc --scrape-examples-output-path examples.json lib.rs`  
生成 JSON 示例。  
**高级提示**：用于交互文档。

## 教程43: 刮取示例目标 crate (--scrape-examples-target-crate)
**描述**：指定刮取目标 crate。  
**语法**：`rustdoc --scrape-examples-target-crate <crate> <file.rs>`。  
**示例**：  
`rustdoc --scrape-examples-target-crate mycrate lib.rs`  
刮取 mycrate 示例。  
**高级提示**：多 crate 项目。

## 教程44: 不稳定选项 (-Z / --unstable-options)
**描述**：启用不稳定选项（nightly）。  
**语法**：`rustdoc -Z unstable-options <file.rs>`。  
**示例**：  
`rustdoc -Z unstable-options lib.rs`  
启用实验功能。  
**高级提示**：需 nightly 工具链。

## 教程45: 与 Cargo doc 集成 (cargo doc)
**描述**：通过 Cargo 生成文档。  
**语法**：`cargo doc`。  
**示例**：  
`cargo doc`  
生成项目文档。  
**高级提示**：`cargo doc --open` 打开浏览器。

## 教程46: 文档 Markdown 文件
**描述**：生成 Markdown 文档。  
**语法**：`rustdoc README.md --markdown-css style.css`。  
**示例**：  
`rustdoc README.md`  
生成 HTML 从 Markdown。  
**高级提示**：用于 README 渲染。

## 教程47: 运行 doctests (cargo test --doc)
**描述**：测试文档示例。  
**语法**：`cargo test --doc`。  
**示例**：  
`cargo test --doc`  
运行 /// 示例代码。  
**高级提示**：确保文档正确。

## 教程48: 自定义扩展 (--extend-css 与 --theme)
**描述**：组合自定义 CSS 和主题。  
**语法**：`rustdoc --extend-css extra.css --theme dark.css lib.rs`。  
**示例**：  
`rustdoc --extend-css extra.css --theme dark.css lib.rs`  
自定义外观。  
**高级提示**：主题开发。

## 教程49: 多文件文档 (--html-in-header 与 --html-after-content)
**描述**：添加多个 HTML 片段。  
**语法**：`rustdoc --html-in-header head.html --html-after-content foot.html lib.rs`。  
**示例**：  
`rustdoc --html-in-header head.html --html-after-content foot.html lib.rs`  
自定义布局。  
**高级提示**：品牌化文档。

## 教程50: 不稳定刮取示例 (-Z scrape-examples)
**描述**：刮取示例（不稳定）。  
**语法**：`rustdoc -Z unstable-options --scrape-examples-output-path out.json lib.rs`。  
**示例**：  
`rustdoc -Z unstable-options --scrape-examples-output-path out.json lib.rs`  
生成示例 JSON。  
**高级提示**：nightly 功能，增强交互。

