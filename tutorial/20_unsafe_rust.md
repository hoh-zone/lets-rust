# 第20章：Unsafe Rust

Unsafe Rust 允许我们绕过编译器的安全检查，获得更多控制权，但也需要程序员承担更多责任。

## 20.1 Unsafe 的核心概念与作用

### Unsafe 的基本概念

```rust
fn main() {
    let mut num = 5;
    
    // 创建原始指针
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    // 在 unsafe 块中解引用原始指针
    unsafe {
        println!("r1 指向的值: {}", *r1);
        println!("r2 指向的值: {}", *r2);
        
        // 修改值
        *r2 = 10;
        println!("修改后的值: {}", *r2);
    }
}
```

### Unsafe 的超能力

```rust
fn main() {
    // 1. 解引用原始指针
    let x = 5;
    let raw = &x as *const i32;
    unsafe {
        println!("解引用原始指针: {}", *raw);
    }
    
    // 2. 调用 unsafe 函数
    unsafe {
        dangerous_function();
    }
    
    // 3. 访问或修改可变静态变量
    unsafe {
        COUNTER += 1;
        println!("计数器: {}", COUNTER);
    }
    
    // 4. 实现 unsafe trait
    // (在下面的例子中展示)
}

unsafe fn dangerous_function() {
    println!("这是一个 unsafe 函数");
}

static mut COUNTER: usize = 0;
```

### 原始指针详解

```rust
fn main() {
    let mut num = 5;
    
    // 从引用创建原始指针
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    // 从任意内存地址创建原始指针（危险！）
    let address = 0x012345usize;
    let r3 = address as *const i32;
    
    // 原始指针可以在安全代码中创建
    println!("r1: {:p}", r1);
    println!("r2: {:p}", r2);
    println!("r3: {:p}", r3);
    
    // 但只能在 unsafe 块中解引用
    unsafe {
        println!("r1 的值: {}", *r1);
        
        // 修改通过可变原始指针
        *r2 = 10;
        println!("修改后的值: {}", num);
        
        // 解引用无效地址会导致未定义行为
        // println!("r3 的值: {}", *r3); // 危险！可能崩溃
    }
}
```

### 创建安全的抽象

```rust
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    
    assert!(mid <= len);
    
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut v, 3);
    
    println!("左半部分: {:?}", left);
    println!("右半部分: {:?}", right);
    
    left[0] = 10;
    right[0] = 20;
    
    println!("修改后的向量: {:?}", v);
}
```

## 20.2 Unsafe 的常见应用场景

### 与 C 代码互操作

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("C 语言的 abs(-3): {}", abs(-3));
    }
}

// 定义供 C 调用的函数
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("这个函数被 C 代码调用！");
}
```

### 实现自定义智能指针

```rust
use std::ops::Deref;

struct MyBox<T> {
    ptr: *mut T,
}

impl<T> MyBox<T> {
    fn new(value: T) -> MyBox<T> {
        let ptr = Box::into_raw(Box::new(value));
        MyBox { ptr }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.ptr));
        }
    }
}

fn main() {
    let x = MyBox::new(5);
    println!("自定义智能指针的值: {}", *x);
}
```

### 高性能数据结构

```rust
use std::ptr;
use std::alloc::{alloc, dealloc, Layout};

struct RawVec<T> {
    ptr: *mut T,
    cap: usize,
}

impl<T> RawVec<T> {
    fn new() -> Self {
        assert!(std::mem::size_of::<T>() != 0, "不支持零大小类型");
        RawVec {
            ptr: std::ptr::NonNull::dangling().as_ptr(),
            cap: 0,
        }
    }
    
    fn with_capacity(cap: usize) -> Self {
        if cap == 0 {
            Self::new()
        } else {
            let layout = Layout::array::<T>(cap).unwrap();
            let ptr = unsafe { alloc(layout) } as *mut T;
            if ptr.is_null() {
                panic!("内存分配失败");
            }
            RawVec { ptr, cap }
        }
    }
    
    fn grow(&mut self, new_cap: usize) {
        if new_cap <= self.cap {
            return;
        }
        
        let new_layout = Layout::array::<T>(new_cap).unwrap();
        
        let new_ptr = if self.cap == 0 {
            unsafe { alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                std::alloc::realloc(self.ptr as *mut u8, old_layout, new_layout.size())
            }
        } as *mut T;
        
        if new_ptr.is_null() {
            panic!("内存分配失败");
        }
        
        self.ptr = new_ptr;
        self.cap = new_cap;
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}

struct FastVec<T> {
    buf: RawVec<T>,
    len: usize,
}

impl<T> FastVec<T> {
    fn new() -> Self {
        FastVec {
            buf: RawVec::new(),
            len: 0,
        }
    }
    
    fn push(&mut self, elem: T) {
        if self.len == self.buf.cap {
            let new_cap = if self.buf.cap == 0 { 1 } else { 2 * self.buf.cap };
            self.buf.grow(new_cap);
        }
        
        unsafe {
            ptr::write(self.buf.ptr.add(self.len), elem);
        }
        self.len += 1;
    }
    
    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                Some(ptr::read(self.buf.ptr.add(self.len)))
            }
        }
    }
    
    fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe {
                Some(&*self.buf.ptr.add(index))
            }
        } else {
            None
        }
    }
}

impl<T> Drop for FastVec<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

fn main() {
    let mut v = FastVec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    
    println!("元素 0: {:?}", v.get(0));
    println!("弹出: {:?}", v.pop());
    println!("元素 0: {:?}", v.get(0));
}
```

### 实现链表

```rust
use std::ptr;

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

struct LinkedList<T> {
    head: *mut Node<T>,
    len: usize,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: ptr::null_mut(),
            len: 0,
        }
    }
    
    fn push_front(&mut self, data: T) {
        let new_node = Box::into_raw(Box::new(Node {
            data,
            next: self.head,
        }));
        
        self.head = new_node;
        self.len += 1;
    }
    
    fn pop_front(&mut self) -> Option<T> {
        if self.head.is_null() {
            None
        } else {
            unsafe {
                let head = Box::from_raw(self.head);
                self.head = head.next;
                self.len -= 1;
                Some(head.data)
            }
        }
    }
    
    fn peek_front(&self) -> Option<&T> {
        if self.head.is_null() {
            None
        } else {
            unsafe {
                Some(&(*self.head).data)
            }
        }
    }
    
    fn len(&self) -> usize {
        self.len
    }
    
    fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

impl<T> Iterator for LinkedList<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.pop_front()
    }
}

fn main() {
    let mut list = LinkedList::new();
    
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    
    println!("长度: {}", list.len());
    println!("前端元素: {:?}", list.peek_front());
    
    for value in list {
        println!("迭代器输出: {}", value);
    }
}
```

### 内存映射文件

```rust
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::slice;

struct MemoryMappedFile {
    ptr: *mut u8,
    len: usize,
}

impl MemoryMappedFile {
    fn new(file: &File) -> std::io::Result<Self> {
        let len = file.metadata()?.len() as usize;
        
        if len == 0 {
            return Ok(MemoryMappedFile {
                ptr: ptr::null_mut(),
                len: 0,
            });
        }
        
        unsafe {
            let ptr = libc::mmap(
                ptr::null_mut(),
                len,
                libc::PROT_READ,
                libc::MAP_PRIVATE,
                file.as_raw_fd(),
                0,
            ) as *mut u8;
            
            if ptr == libc::MAP_FAILED as *mut u8 {
                return Err(std::io::Error::last_os_error());
            }
            
            Ok(MemoryMappedFile { ptr, len })
        }
    }
    
    fn as_slice(&self) -> &[u8] {
        if self.ptr.is_null() {
            &[]
        } else {
            unsafe { slice::from_raw_parts(self.ptr, self.len) }
        }
    }
}

impl Drop for MemoryMappedFile {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                libc::munmap(self.ptr as *mut libc::c_void, self.len);
            }
        }
    }
}

// 注意：这个例子需要 libc crate
// [dependencies]
// libc = "0.2"

fn main() {
    // 示例代码，实际使用时需要处理文件打开等操作
    println!("内存映射文件示例（需要完整的错误处理）");
}
```

## 20.3 Unsafe 使用的最佳实践与风险管理

### 最小化 Unsafe 代码

```rust
// 好的做法：将 unsafe 包装在安全接口中
mod safe_wrapper {
    use std::ptr;
    
    pub struct SafeArray<T> {
        ptr: *mut T,
        len: usize,
        cap: usize,
    }
    
    impl<T> SafeArray<T> {
        pub fn new() -> Self {
            SafeArray {
                ptr: ptr::null_mut(),
                len: 0,
                cap: 0,
            }
        }
        
        pub fn with_capacity(cap: usize) -> Self {
            if cap == 0 {
                return Self::new();
            }
            
            let layout = std::alloc::Layout::array::<T>(cap).unwrap();
            let ptr = unsafe { std::alloc::alloc(layout) } as *mut T;
            
            if ptr.is_null() {
                panic!("内存分配失败");
            }
            
            SafeArray { ptr, len: 0, cap }
        }
        
        pub fn push(&mut self, value: T) {
            if self.len >= self.cap {
                panic!("数组已满");
            }
            
            unsafe {
                ptr::write(self.ptr.add(self.len), value);
            }
            self.len += 1;
        }
        
        pub fn get(&self, index: usize) -> Option<&T> {
            if index < self.len {
                unsafe { Some(&*self.ptr.add(index)) }
            } else {
                None
            }
        }
        
        pub fn len(&self) -> usize {
            self.len
        }
    }
    
    impl<T> Drop for SafeArray<T> {
        fn drop(&mut self) {
            if !self.ptr.is_null() {
                // 先析构所有元素
                for i in 0..self.len {
                    unsafe {
                        ptr::drop_in_place(self.ptr.add(i));
                    }
                }
                
                // 释放内存
                let layout = std::alloc::Layout::array::<T>(self.cap).unwrap();
                unsafe {
                    std::alloc::dealloc(self.ptr as *mut u8, layout);
                }
            }
        }
    }
}

fn main() {
    let mut arr = safe_wrapper::SafeArray::with_capacity(10);
    arr.push(1);
    arr.push(2);
    arr.push(3);
    
    for i in 0..arr.len() {
        println!("元素 {}: {:?}", i, arr.get(i));
    }
}
```

### 详尽的文档和测试

```rust
/// 一个使用 unsafe 的快速字符串构建器
/// 
/// # Safety
/// 
/// 这个结构体使用原始指针直接操作内存，用户必须确保：
/// 1. 不要在多线程环境中共享可变引用
/// 2. 不要手动释放内部指针指向的内存
/// 3. 确保所有添加的字符串都是有效的 UTF-8
pub struct FastStringBuilder {
    ptr: *mut u8,
    len: usize,
    cap: usize,
}

impl FastStringBuilder {
    /// 创建一个新的字符串构建器
    /// 
    /// # Examples
    /// 
    /// ```
    /// let builder = FastStringBuilder::new();
    /// assert_eq!(builder.len(), 0);
    /// ```
    pub fn new() -> Self {
        FastStringBuilder {
            ptr: std::ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }
    
    /// 添加一个字符串到构建器
    /// 
    /// # Safety
    /// 
    /// 调用者必须确保 `s` 是有效的 UTF-8 字符串
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut builder = FastStringBuilder::new();
    /// builder.push_str("Hello");
    /// builder.push_str(" World");
    /// assert_eq!(builder.as_str(), "Hello World");
    /// ```
    pub fn push_str(&mut self, s: &str) {
        let bytes = s.as_bytes();
        self.ensure_capacity(self.len + bytes.len());
        
        unsafe {
            std::ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                self.ptr.add(self.len),
                bytes.len(),
            );
        }
        
        self.len += bytes.len();
    }
    
    fn ensure_capacity(&mut self, needed: usize) {
        if needed <= self.cap {
            return;
        }
        
        let new_cap = if self.cap == 0 {
            8
        } else {
            self.cap * 2
        }.max(needed);
        
        let new_ptr = if self.cap == 0 {
            unsafe { std::alloc::alloc(std::alloc::Layout::array::<u8>(new_cap).unwrap()) }
        } else {
            unsafe {
                std::alloc::realloc(
                    self.ptr,
                    std::alloc::Layout::array::<u8>(self.cap).unwrap(),
                    new_cap,
                )
            }
        };
        
        if new_ptr.is_null() {
            panic!("内存分配失败");
        }
        
        self.ptr = new_ptr;
        self.cap = new_cap;
    }
    
    /// 返回构建的字符串
    /// 
    /// # Safety
    /// 
    /// 这个方法假设内部缓冲区包含有效的 UTF-8 数据
    pub fn as_str(&self) -> &str {
        if self.len == 0 {
            ""
        } else {
            unsafe {
                let slice = std::slice::from_raw_parts(self.ptr, self.len);
                std::str::from_utf8_unchecked(slice)
            }
        }
    }
    
    pub fn len(&self) -> usize {
        self.len
    }
}

impl Drop for FastStringBuilder {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                std::alloc::dealloc(
                    self.ptr,
                    std::alloc::Layout::array::<u8>(self.cap).unwrap(),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_builder() {
        let builder = FastStringBuilder::new();
        assert_eq!(builder.len(), 0);
        assert_eq!(builder.as_str(), "");
    }
    
    #[test]
    fn test_push_str() {
        let mut builder = FastStringBuilder::new();
        builder.push_str("Hello");
        assert_eq!(builder.as_str(), "Hello");
        
        builder.push_str(" World");
        assert_eq!(builder.as_str(), "Hello World");
    }
    
    #[test]
    fn test_capacity_growth() {
        let mut builder = FastStringBuilder::new();
        
        // 添加足够多的字符串来触发容量增长
        for i in 0..100 {
            builder.push_str(&format!("String {}", i));
        }
        
        assert!(builder.len() > 0);
    }
}

fn main() {
    let mut builder = FastStringBuilder::new();
    builder.push_str("Hello");
    builder.push_str(" ");
    builder.push_str("Unsafe");
    builder.push_str(" Rust!");
    
    println!("构建的字符串: {}", builder.as_str());
}
```

### 使用工具检查 Unsafe 代码

```rust
// 使用 Miri 检测未定义行为
// cargo +nightly miri run

fn main() {
    // 这个例子展示如何编写可以通过 Miri 检查的代码
    
    let mut v = vec![1, 2, 3, 4, 5];
    let ptr = v.as_mut_ptr();
    
    // 安全的原始指针操作
    unsafe {
        // 在有效范围内访问
        for i in 0..v.len() {
            let value = *ptr.add(i);
            println!("元素 {}: {}", i, value);
        }
        
        // 修改元素
        *ptr.add(0) = 10;
    }
    
    println!("修改后的向量: {:?}", v);
}

// 使用地址清理器的示例配置
// RUSTFLAGS="-Z sanitizer=address" cargo run --target x86_64-unknown-linux-gnu
```

## RWO 权限分析

### Unsafe 对权限系统的影响

```rust
use std::ptr;

fn main() {
    let mut data = vec![1, 2, 3, 4, 5];
    
    // R: 通过原始指针读取
    let read_ptr = data.as_ptr();
    unsafe {
        for i in 0..data.len() {
            let value = *read_ptr.add(i);
            println!("读取索引 {}: {}", i, value);
        }
    }
    
    // W: 通过原始指针写入
    let write_ptr = data.as_mut_ptr();
    unsafe {
        *write_ptr.add(0) = 10;
        *write_ptr.add(1) = 20;
    }
    
    // O: 原始指针不拥有数据，但可以转移所有权
    let owned_ptr = Box::into_raw(Box::new(42));
    unsafe {
        println!("拥有的值: {}", *owned_ptr);
        // 必须手动释放
        drop(Box::from_raw(owned_ptr));
    }
    
    println!("修改后的数据: {:?}", data);
}
```

### 绕过借用检查器的危险

```rust
use std::cell::UnsafeCell;

struct DangerousContainer<T> {
    data: UnsafeCell<T>,
}

impl<T> DangerousContainer<T> {
    fn new(value: T) -> Self {
        DangerousContainer {
            data: UnsafeCell::new(value),
        }
    }
    
    // 危险：可能违反借用规则
    fn get_multiple_mut(&self) -> (&mut T, &mut T) {
        unsafe {
            let ptr = self.data.get();
            (&mut *ptr, &mut *ptr) // 返回两个可变引用到同一个数据！
        }
    }
    
    // 安全的替代方案
    fn get_mut(&self) -> &mut T {
        unsafe { &mut *self.data.get() }
    }
    
    fn get(&self) -> &T {
        unsafe { &*self.data.get() }
    }
}

// 这是一个错误的用法示例
fn demonstrate_danger() {
    let container = DangerousContainer::new(vec![1, 2, 3]);
    
    // 这会创建别名的可变引用，违反了 Rust 的借用规则
    let (ref1, ref2) = container.get_multiple_mut();
    
    // 同时通过两个引用修改同一个数据是未定义行为
    ref1.push(4);
    ref2.push(5); // 危险！
    
    println!("结果: {:?}", container.get());
}

// 正确的用法
fn demonstrate_safety() {
    let container = DangerousContainer::new(vec![1, 2, 3]);
    
    // 一次只使用一个可变引用
    container.get_mut().push(4);
    container.get_mut().push(5);
    
    println!("安全结果: {:?}", container.get());
}

fn main() {
    // demonstrate_danger(); // 不要运行这个！
    demonstrate_safety();
}
```

### 手动管理生命周期

```rust
use std::ptr;
use std::marker::PhantomData;

struct RawSlice<'a, T> {
    ptr: *const T,
    len: usize,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> RawSlice<'a, T> {
    // O: 从切片创建原始切片（借用生命周期）
    fn from_slice(slice: &'a [T]) -> Self {
        RawSlice {
            ptr: slice.as_ptr(),
            len: slice.len(),
            _marker: PhantomData,
        }
    }
    
    // 危险：可以创建任意生命周期的原始切片
    unsafe fn from_raw_parts(ptr: *const T, len: usize) -> Self {
        RawSlice {
            ptr,
            len,
            _marker: PhantomData,
        }
    }
    
    // R: 获取元素的引用
    fn get(&self, index: usize) -> Option<&'a T> {
        if index < self.len {
            unsafe { Some(&*self.ptr.add(index)) }
        } else {
            None
        }
    }
    
    // 转换回安全切片
    fn as_slice(&self) -> &'a [T] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
    
    fn len(&self) -> usize {
        self.len
    }
}

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    
    // 安全的创建方式
    let raw_slice = RawSlice::from_slice(&data);
    
    println!("长度: {}", raw_slice.len());
    for i in 0..raw_slice.len() {
        if let Some(value) = raw_slice.get(i) {
            println!("索引 {}: {}", i, value);
        }
    }
    
    // 转换回安全切片
    let safe_slice = raw_slice.as_slice();
    println!("安全切片: {:?}", safe_slice);
}
```

### 原子操作与 Unsafe

```rust
use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;

struct LockFreeStack<T> {
    head: AtomicPtr<Node<T>>,
}

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

impl<T> LockFreeStack<T> {
    fn new() -> Self {
        LockFreeStack {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }
    
    // O: 推入元素（获取节点所有权）
    fn push(&self, data: T) {
        let new_node = Box::into_raw(Box::new(Node {
            data,
            next: ptr::null_mut(),
        }));
        
        loop {
            let head = self.head.load(Ordering::Acquire);
            
            unsafe {
                (*new_node).next = head;
            }
            
            match self.head.compare_exchange_weak(
                head,
                new_node,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(_) => continue, // 重试
            }
        }
    }
    
    // O: 弹出元素（转移节点所有权）
    fn pop(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            
            if head.is_null() {
                return None;
            }
            
            let next = unsafe { (*head).next };
            
            match self.head.compare_exchange_weak(
                head,
                next,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    unsafe {
                        let head_box = Box::from_raw(head);
                        return Some(head_box.data);
                    }
                }
                Err(_) => continue, // 重试
            }
        }
    }
    
    // R: 查看顶部元素
    fn peek(&self) -> Option<&T> {
        let head = self.head.load(Ordering::Acquire);
        if head.is_null() {
            None
        } else {
            unsafe { Some(&(*head).data) }
        }
    }
}

impl<T> Drop for LockFreeStack<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

fn main() {
    let stack = LockFreeStack::new();
    
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    println!("顶部元素: {:?}", stack.peek());
    
    while let Some(value) = stack.pop() {
        println!("弹出: {}", value);
    }
}
```

### 内存对齐和布局控制

```rust
use std::mem;

#[repr(C)]
struct CCompatible {
    a: u8,
    b: u32,
    c: u16,
}

#[repr(packed)]
struct PackedStruct {
    a: u8,
    b: u32,
    c: u16,
}

#[repr(align(16))]
struct AlignedStruct {
    data: [u8; 8],
}

fn main() {
    // 内存布局分析
    println!("C 兼容结构体:");
    println!("  大小: {}", mem::size_of::<CCompatible>());
    println!("  对齐: {}", mem::align_of::<CCompatible>());
    
    println!("紧凑结构体:");
    println!("  大小: {}", mem::size_of::<PackedStruct>());
    println!("  对齐: {}", mem::align_of::<PackedStruct>());
    
    println!("对齐结构体:");
    println!("  大小: {}", mem::size_of::<AlignedStruct>());
    println!("  对齐: {}", mem::align_of::<AlignedStruct>());
    
    // 字段偏移量
    let c_struct = CCompatible { a: 1, b: 2, c: 3 };
    let base_ptr = &c_struct as *const _ as usize;
    let a_ptr = &c_struct.a as *const _ as usize;
    let b_ptr = &c_struct.b as *const _ as usize;
    let c_ptr = &c_struct.c as *const _ as usize;
    
    println!("字段偏移量:");
    println!("  a: {}", a_ptr - base_ptr);
    println!("  b: {}", b_ptr - base_ptr);
    println!("  c: {}", c_ptr - base_ptr);
    
    // 使用 unsafe 进行类型转换
    let bytes: [u8; mem::size_of::<CCompatible>()] = unsafe {
        mem::transmute(c_struct)
    };
    println!("字节表示: {:?}", bytes);
}
```

## 小结

本章深入学习了 Unsafe Rust：

1. **Unsafe 核心概念**：
   - 解引用原始指针
   - 调用 unsafe 函数
   - 访问可变静态变量
   - 实现 unsafe trait
   - 创建安全抽象

2. **常见应用场景**：
   - FFI（外部函数接口）
   - 高性能数据结构
   - 自定义智能指针
   - 系统级编程
   - 内存映射文件

3. **最佳实践**：
   - 最小化 unsafe 代码
   - 详尽的文档和测试
   - 使用工具检查
   - 创建安全的 API 封装

4. **RWO 权限分析**：
   - **R**：原始指针可以绕过借用检查读取
   - **W**：可以通过原始指针进行不安全的写入
   - **O**：手动管理所有权，需要正确的分配和释放
   - Unsafe 代码需要程序员手动确保内存安全
   - 违反借用规则可能导致未定义行为

Unsafe Rust 是一把双刃剑，提供了强大的控制能力，但也要求程序员承担更多责任。下一章我们将学习宏。 