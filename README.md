# lets rust
lets rust 让我们移动到`rust`编程语言

## 如何参与
- 复制 `Rustacean` 目录下的 `00000` 并且用自己的 `github username` 命名复制的目录
-> 注意是 `copy`  不是 `rename`
- 在 `readme.md` 里面填写个人信息 后面的任务模板暂时不管
- 提交PR
- 必须加入社区群发出PR 才会被合并 (防止女巫和自动化攻击)


## 社区 & 问答
- [lets rust TG](https://t.me/letsrust)
- QQ群: 827637782

## 如何完成任务并获取奖励
- 在自己`github username`目录的 readme.md 对应的任务列表填写完成信息
- 提交PR 如果只完成 task 1 用 `完成 task 1` 取名 完成多个 用 `完成 task 1 2 3` 等取名


## 奖励明细 请仔细阅读要求

| 任务                                 | 名称         | 积分 | 说明            |
|:-----------------------------------|------------|:--:|:--------------|
| [task 1](challenge/001.install.md) | hello Rust | *  | 完成rust和开发环境安装 |


## 课程大纲

### 1. Rust语言基础
1. 变量与常量
2. 数据类型
3. 函数
4. 程序控制流

### 2. 程序与内存管理
1. 程序的基本执⾏流程
2. 栈与堆
3. 指针类型
4. 函数调用


### 3. 所有权机制
1. 目的与核心思想
2. 所有权规则
3. 验证规则

### 4. 借用机制
1. 引用与借用规则
2. 验证借用规则
3. 切片
4. 悬垂引用


### 5. 结构体
1. 定义与使用
2. 关联方法与函数
3. 内存布局

### 6. 常用类型解析
1. Vector
2. String
3. 类型比较: String / str / &str、[T;N] / [T]  / &[T])


## 7. 枚举
1. 定义与使用
2. 用 match 操作枚举
3. Option 及常用方法
4. 内存布局

## 8. 泛型与特征
1. 泛型数据类型
2. 特征
3. 用特征约束泛型

## 9. 生命周期与标注
1. 目的与核心思想
2. 变量⽣命周期
3. ⽣命周期标注及规则
4. ⽣命周期示例代码解析

## 10. 特征对象及其应用
1. 特征对象概述
2. 特征对象的创建与使用
3. 特征对象的应用场景
4. 特征对象的深度解析


## 11. 常用的特征解析
1. 常用的特征（Trait)介绍
2. 特征实现与应用场景


## 12. 错误处理
1. `panic! 宏` 与 不可恢复错误
2. `Result 类型` 与 可恢复错误
3. 错误传播（ ? 操作符）
4. ⾃定义错误类型


## 13. 项目管理
1. crate 与模块的基本概念
3. 模块的访问控制
4. 模块的文件组织


## 14. 文档与测试
1. 注释与文档
2. 单元测试
3. 文档测试
4. 集成测试

## 15. 闭包
1. 闭包的定义与基本语法
2. 闭包的常见使用场景
3. 对闭包的深度解析

## 16. 迭代器
1. 迭代器的概念与类型
2. 常见的迭代器方法
3. 自定义迭代器
4. 消费型与适配型迭代器

## 17. 智能指针
1. 智能指针概述：什么是智能指针
2. Deref 特证 与 Drop 特征 的解析
3. 使用 Box<T> 管理堆上的数据

## 18. 常见智能指针及其应用
1. Rc<T>：引用计数智能指针
2. Weak<T>：避免循环引用的弱引用
3. Arc<T>：原子引用计数智能指针
4. RefCell<T>：内部可变性与运行时借用检查


## 19. 多线程与并发
1. 并发与并行
2. 多线程及同步机制
3. Send 与 Sync 解析

## 20. Unsafe Rust
1. unsafe 的核心概念与作用
2. unsafe 的常见应用场景
3. unsafe 使用的最佳实践与风险管理

## 21. 宏
1. 宏的定义与特点
2. Rust中常见宏的应用
3. 编写与使用声明宏

## 22. 过程宏深入解析
1. 过程宏的基本概念
2. 编写和使用过程宏
3. 过程宏的高级应用


