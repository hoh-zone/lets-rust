# 编程语言完整教程

这是一份详细的 Rust 编程语言中文教程，从基础到高级，
深入浅出地讲解 Rust 的核心概念和实战技巧。

## 目录

### 基础篇
- [基础篇]()
  - [介绍](01_basics/01_intro.md) 
  - [安装Rust](01_basics/02_install.md) 
  - [安装IDE](01_basics/03_rust_ide.md) 
  - [变量.方法](01_basics/04_var_fun.md) 
  - [条件语句](01_basics/05_condition.md) 
  - [结构体](01_basics/06_struct.md) 
  - [enum](01_basics/07_enum.md) 
  - [注释](01_basics/08_comments.md) 
  - [循环语句](01_basics/09_loop.md)
  - [打印方法](01_basics/10_print.md)
  - [模块](01_basics/11_mod.md)
### 高级篇
- [高级篇]()
  - [错误处理](02_advanced/01_error.md)
  - [impl语法](02_advanced/02_impl.md)
  - [所有权](02_advanced/03_ownership.md)
  - [借用](02_advanced/04_borrow.md)
  - [切片](02_advanced/05_slice.md)
  - [泛型](02_advanced/06_generics.md)
  - [trait](02_advanced/07_trait.md)
  - [生命周期](02_advanced/08_lifetimes.md)
  - [闭包](02_advanced/09_closures.md)
  - [迭代器](02_advanced/10_iterator.md)
### 标准库
- [标准库]()
  - [option](03_std/01_option.md)
  - [result](03_std/02_result.md)
  - [env](03_std/03_env.md)
  - [fmt](03_std/04_fmt.md)
  - [fs](03_std/05_fs.md)
  - [io](03_std/06_io.md)
  - [iter](03_std/07_iter.md)
  - [net](03_std/08_net.md)
  - [os](03_std/09_os.md)
  - [path](03_std/10_path.md)
  - [process](03_std/11_process.md)
  - [str](03_std/12_str.md)
  - [time](03_std/13_time.md)
  - [Box](03_std/14_Box.md)

### 常用集合
- [集合]()
  - [Vec](04_collection/01_Vec.md)
  - [VecDeque](04_collection/02_VecDeque.md)
  - [LikedList](04_collection/03_LinkedList.md)
  - [HashMap](04_collection/04_HashMap.md)
  - [BTreeMap](04_collection/05_BTreeMap.md)
  - [HashSet](04_collection/06_HashSet.md)
  - [BTreeSet](04_collection/07_BTreeSet.md)
  - [BinaryHeap](04_collection/08_BinaryHeap.md)


### 并发
- [并发]()
  - [线程](05_concurrency/01_thread.md)
  - [channel](05_concurrency/02_channels.md)
  - [mutex](05_concurrency/03_mutex.md)
  - [RC](05_concurrency/04_rc.md)
  - [ARC](05_concurrency/05_arc.md)

### 常用trait
- [常用trait]()
  - [Debug](06_trait/00_Debug.md)
  - [Display](06_trait/00_Display.md)
  - [Default](06_trait/00_Default.md)
  - [Error](06_trait/00_Error.md)
  - [From](06_trait/01_From.md)
  - [Into](06_trait/01_Into.md)
  - [FromStr](06_trait/01_FromStr.md)
  - [TryFrom](06_trait/01_TryFrom.md)
  - [TryInto](06_trait/01_TryInto.md)
  - [ToString](06_trait/01_ToString.md)
  - [AsMut](06_trait/02_AsMut.md)
  - [AsRef](06_trait/02_AsRef.md)
  - [Borrow](06_trait/02_Borrow.md)
  - [BorrowMut](06_trait/02_BorrowMut.md)
  - [Deref](06_trait/02_Deref.md)
  - [DerefMut](06_trait/02_DerefMut.md)
  - [ToOwned](06_trait/02_ToOwned.md)
  - [Clone](06_trait/03_Clone.md)
  - [Copy](06_trait/03_Copy.md)
  - [Send](06_trait/03_Send.md)
  - [Sized](06_trait/04_Sized.md)
  - [Drop](06_trait/06_Drop.md)
  - [Sync](06_trait/07_Sync.md)
  - [Unpin](06_trait/08_Unpin.md)
  - [Fn](06_trait/09_Fn.md)
  - [FnMut](06_trait/09_FnMut.md)
  - [FnOnce](06_trait/09_FnOnce.md)
  - [Eq](06_trait/11_Eq.md)
  - [PartialEq](06_trait/11_PartialEq.md)
  - [Ord](06_trait/11_Ord.md)
  - [PartialOrd](06_trait/11_PartialOrd.md)
  - [Hash](06_trait/11_Hash.md)

### 宏
- [宏macro]()
  - [宏简介](07_macro/01_intro.md)
  - [声明宏](07_macro/02_macro_rules.md)
  - [过程宏](07_macro/03_proc-macro.md)
  - [属性派生宏](07_macro/04_proc_macro_attribute.md)
  - [自定义派生宏](07_macro/05_proc_macro_derive.md)

### 异步编程
- [异步编程]()
  - [异步介绍](08_async/00_intro.md)
  - [future](08_async/01_future.md)
  - [async/await](08_async/02_async_await.md)
  - [AsyncFn](08_async/03_AsyncFn.md)

### 常用库
- [常用库]()
  - [serde](09_common/01_serde.md)
  - [clap](09_common/02_clap.md)
  - [anyhow](09_common/03_anyhow.md)
  - [tracing](09_common/04_tracing.md)
  - [lazy_static](09_common/05_lazy_static.md)
  - [chrono](09_common/06_chrono.md)
  - [once_cell](09_common/07_once_cell.md)

### 后端框架 axum
- [axum]()
  - [axum](10_axum/01_intro.md)

### 异步运行时Tokio
- [Tokio]()
  - [tokio](11_tokio/readme.md)