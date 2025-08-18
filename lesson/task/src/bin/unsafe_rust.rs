// 教学示例 - 允许未使用的代码
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// 第20章：Unsafe Rust
// 演示 Unsafe Rust 的强大功能和安全实践

use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use std::slice;

fn main() {
    println!("🦀 第20章：Unsafe Rust");
    println!("=====================================");
    
    // 1. Unsafe 的基本概念
    unsafe_basics_demo();
    
    // 2. 原始指针操作
    raw_pointers_demo();
    
    // 3. 不安全函数
    unsafe_functions_demo();
    
    // 4. 静态变量访问
    static_variables_demo();
    
    // 5. 创建安全抽象
    safe_abstractions_demo();
    
    // 6. 自定义智能指针
    custom_smart_pointer_demo();
    
    // 7. 高性能数据结构
    high_performance_structures_demo();
    
    // 8. 内存对齐和布局
    memory_layout_demo();
    
    // 9. RWO 权限分析
    rwo_permissions_demo();
    
    // 10. 安全实践
    safety_practices_demo();
    
    println!("\n🎉 第20章 Unsafe Rust 演示完成！");
    println!("📚 您已经了解了 Unsafe Rust 的强大功能");
    println!("⚠️  记住：能力越大，责任越大！");
}

// ============================================================================
// 1. Unsafe 的基本概念
// ============================================================================

fn unsafe_basics_demo() {
    println!("\n📍 1. Unsafe 的基本概念");
    println!("{}", "-".repeat(40));
    
    println!("🔧 Unsafe 的五大超能力：");
    println!("   1. 解引用原始指针");
    println!("   2. 调用不安全函数");
    println!("   3. 访问或修改可变静态变量");
    println!("   4. 实现不安全 trait");
    println!("   5. 访问联合体字段");
    
    // 基本的原始指针使用
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    println!("\n🔧 原始指针基础：");
    unsafe {
        println!("   r1 指向的值: {}", *r1);
        println!("   r2 指向的值: {}", *r2);
        
        // 修改值
        *r2 = 10;
        println!("   修改后的值: {}", *r2);
    }
    
    println!("\n💡 关键概念：");
    println!("   • Unsafe 块允许绕过编译器安全检查");
    println!("   • 原始指针可以在安全代码中创建");
    println!("   • 但只能在 unsafe 块中解引用");
    println!("   • 程序员需要手动保证内存安全");
}

// ============================================================================
// 2. 原始指针操作
// ============================================================================

fn raw_pointers_demo() {
    println!("\n📍 2. 原始指针操作");
    println!("{}", "-".repeat(40));
    
    let mut data = vec![1, 2, 3, 4, 5];
    let ptr = data.as_mut_ptr();
    let len = data.len();
    
    println!("🔧 原始指针遍历：");
    unsafe {
        for i in 0..len {
            let value = *ptr.add(i);
            println!("   索引 {}: 值 {}", i, value);
        }
        
        // 修改数据
        *ptr.add(0) = 10;
        *ptr.add(1) = 20;
    }
    
    println!("   修改后的向量: {:?}", data);
    
    // 从任意地址创建指针（危险！）
    let address = 0x012345usize;
    let _dangerous_ptr = address as *const i32;
    
    println!("\n🔧 指针算术：");
    let array = [1, 2, 3, 4, 5];
    let ptr = array.as_ptr();
    
    unsafe {
        println!("   第一个元素: {}", *ptr);
        println!("   第二个元素: {}", *ptr.add(1));
        println!("   第三个元素: {}", *ptr.offset(2));
    }
    
    println!("\n💡 原始指针特点：");
    println!("   • 不保证指向有效内存");
    println!("   • 不自动清理");
    println!("   • 允许别名和可变性");
    println!("   • 可以进行指针算术");
}

// ============================================================================
// 3. 不安全函数
// ============================================================================

unsafe fn dangerous_function() {
    println!("   这是一个不安全函数");
}

fn unsafe_functions_demo() {
    println!("\n📍 3. 不安全函数");
    println!("{}", "-".repeat(40));
    
    println!("🔧 调用不安全函数：");
    unsafe {
        dangerous_function();
    }
    
    // 创建安全包装器
    fn safe_wrapper(data: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        unsafe {
            split_at_mut_unsafe(data, mid)
        }
    }
    
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = safe_wrapper(&mut v, 3);
    
    println!("   左半部分: {:?}", left);
    println!("   右半部分: {:?}", right);
    
    left[0] = 10;
    right[0] = 20;
    
    println!("   修改后的向量: {:?}", v);
    
    println!("\n💡 不安全函数原则：");
    println!("   • 在安全抽象中包装不安全代码");
    println!("   • 确保函数的前置条件");
    println!("   • 提供安全的公共 API");
}

unsafe fn split_at_mut_unsafe(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    
    assert!(mid <= len);
    
    (
        slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.add(mid), len - mid),
    )
}

// ============================================================================
// 4. 静态变量访问
// ============================================================================

static mut COUNTER: usize = 0;

fn static_variables_demo() {
    println!("\n📍 4. 静态变量访问");
    println!("{}", "-".repeat(40));
    
    println!("🔧 访问可变静态变量：");
    unsafe {
        COUNTER += 1;
        println!("    计数器值: {}", COUNTER);
        COUNTER += 1;
        println!("    计数器值: {}", COUNTER);
    }
    
    // 初始化静态变量
    static mut GLOBAL_DATA: Option<String> = None;
    
    unsafe {
        if GLOBAL_DATA.is_none() {
            GLOBAL_DATA = Some(String::from("全局数据"));
        }
        
        if let Some(ref data) = GLOBAL_DATA {
            println!("    全局数据: {}", data);
        }
    }
    
    println!("\n💡 静态变量注意事项：");
    println!("   • 可变静态变量访问总是不安全的");
    println!("   • 可能存在数据竞争");
    println!("   • 考虑使用原子类型或锁");
    println!("   • 不可变静态变量是安全的");
}

// ============================================================================
// 5. 创建安全抽象
// ============================================================================

struct SafeVec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

impl<T> SafeVec<T> {
    fn new() -> Self {
        SafeVec {
            ptr: ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }
    
    fn with_capacity(cap: usize) -> Self {
        if cap == 0 {
            return Self::new();
        }
        
        let layout = Layout::array::<T>(cap).unwrap();
        let ptr = unsafe { alloc(layout) } as *mut T;
        
        if ptr.is_null() {
            panic!("内存分配失败");
        }
        
        SafeVec { ptr, len: 0, cap }
    }
    
    fn push(&mut self, value: T) {
        if self.len >= self.cap {
            self.grow();
        }
        
        unsafe {
            ptr::write(self.ptr.add(self.len), value);
        }
        self.len += 1;
    }
    
    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                Some(ptr::read(self.ptr.add(self.len)))
            }
        }
    }
    
    fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe { Some(&*self.ptr.add(index)) }
        } else {
            None
        }
    }
    
    fn len(&self) -> usize {
        self.len
    }
    
    fn grow(&mut self) {
        let new_cap = if self.cap == 0 { 1 } else { 2 * self.cap };
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

impl<T> Drop for SafeVec<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            // 析构所有元素
            while let Some(_) = self.pop() {}
            
            // 释放内存
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}

fn safe_abstractions_demo() {
    println!("\n📍 5. 创建安全抽象");
    println!("{}", "-".repeat(40));
    
    println!("🔧 自定义安全向量：");
    let mut vec = SafeVec::new();
    
    vec.push(1);
    vec.push(2);
    vec.push(3);
    
    println!("   向量长度: {}", vec.len());
    for i in 0..vec.len() {
        if let Some(value) = vec.get(i) {
            println!("   元素 {}: {}", i, value);
        }
    }
    
    println!("   弹出元素: {:?}", vec.pop());
    println!("   弹出后长度: {}", vec.len());
    
    println!("\n💡 安全抽象原则：");
    println!("   • 在安全接口下隐藏不安全代码");
    println!("   • 维护数据结构的不变量");
    println!("   • 提供内存安全保证");
    println!("   • 正确实现 Drop trait");
}

// ============================================================================
// 6. 自定义智能指针
// ============================================================================

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

fn custom_smart_pointer_demo() {
    println!("\n📍 6. 自定义智能指针");
    println!("{}", "-".repeat(40));
    
    println!("🔧 自定义 Box 类型：");
    let x = MyBox::new(5);
    println!("   MyBox 的值: {}", *x);
    
    let y = MyBox::new(String::from("Hello"));
    println!("   MyBox 字符串: {}", *y);
    
    // 自动解引用
    println!("   字符串长度: {}", y.len());
    
    println!("\n💡 智能指针要点：");
    println!("   • 实现 Deref trait 提供解引用");
    println!("   • 实现 Drop trait 自动清理");
    println!("   • 提供 RAII 语义");
    println!("   • 确保内存安全");
}

// ============================================================================
// 7. 高性能数据结构
// ============================================================================

struct FastStack<T> {
    data: Vec<T>,
    top: usize,
}

impl<T> FastStack<T> {
    fn new() -> Self {
        FastStack {
            data: Vec::new(),
            top: 0,
        }
    }
    
    fn with_capacity(cap: usize) -> Self {
        FastStack {
            data: Vec::with_capacity(cap),
            top: 0,
        }
    }
    
    fn push(&mut self, value: T) {
        if self.top >= self.data.len() {
            self.data.push(value);
        } else {
            unsafe {
                // 直接写入已分配的内存
                ptr::write(self.data.as_mut_ptr().add(self.top), value);
            }
        }
        self.top += 1;
    }
    
    fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            None
        } else {
            self.top -= 1;
            if self.top < self.data.len() {
                unsafe {
                    Some(ptr::read(self.data.as_ptr().add(self.top)))
                }
            } else {
                self.data.pop()
            }
        }
    }
    
    fn peek(&self) -> Option<&T> {
        if self.top == 0 {
            None
        } else {
            self.data.get(self.top - 1)
        }
    }
    
    fn len(&self) -> usize {
        self.top
    }
}

fn high_performance_structures_demo() {
    println!("\n📍 7. 高性能数据结构");
    println!("{}", "-".repeat(40));
    
    println!("🔧 高性能栈：");
    let mut stack = FastStack::with_capacity(100);
    
    for i in 0..5 {
        stack.push(i);
        println!("   推入: {}", i);
    }
    
    println!("   栈长度: {}", stack.len());
    println!("   栈顶元素: {:?}", stack.peek());
    
    while let Some(value) = stack.pop() {
        println!("   弹出: {}", value);
    }
    
    println!("\n💡 高性能技巧：");
    println!("   • 预分配内存避免重复分配");
    println!("   • 使用原始指针减少边界检查");
    println!("   • 直接内存操作提高性能");
    println!("   • 保持数据结构的完整性");
}

// ============================================================================
// 8. 内存对齐和布局
// ============================================================================

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

fn memory_layout_demo() {
    println!("\n📍 8. 内存对齐和布局");
    println!("{}", "-".repeat(40));
    
    use std::mem;
    
    println!("🔧 内存布局分析：");
    println!("   C 兼容结构体:");
    println!("     大小: {} 字节", mem::size_of::<CCompatible>());
    println!("     对齐: {} 字节", mem::align_of::<CCompatible>());
    
    println!("   紧凑结构体:");
    println!("     大小: {} 字节", mem::size_of::<PackedStruct>());
    println!("     对齐: {} 字节", mem::align_of::<PackedStruct>());
    
    // 字段偏移量分析
    let c_struct = CCompatible { a: 1, b: 2, c: 3 };
    let base_ptr = &c_struct as *const _ as usize;
    let a_ptr = &c_struct.a as *const _ as usize;
    let b_ptr = &c_struct.b as *const _ as usize;
    let c_ptr = &c_struct.c as *const _ as usize;
    
    println!("\n🔧 字段偏移量：");
    println!("   字段 a 偏移: {} 字节", a_ptr - base_ptr);
    println!("   字段 b 偏移: {} 字节", b_ptr - base_ptr);
    println!("   字段 c 偏移: {} 字节", c_ptr - base_ptr);
    
    // 类型转换示例
    let bytes: [u8; mem::size_of::<CCompatible>()] = unsafe {
        mem::transmute(c_struct)
    };
    println!("   字节表示: {:?}", &bytes[..4]);
    
    println!("\n💡 内存布局要点：");
    println!("   • #[repr(C)] 确保 C 兼容布局");
    println!("   • #[repr(packed)] 消除填充");
    println!("   • 对齐影响性能和兼容性");
    println!("   • transmute 进行底层类型转换");
}

// ============================================================================
// 9. RWO 权限分析
// ============================================================================

fn rwo_permissions_demo() {
    println!("\n📍 9. RWO 权限分析");
    println!("{}", "-".repeat(40));
    
    let mut data = vec![1, 2, 3, 4, 5];
    
    println!("🔧 R (Read) - 原始指针读取：");
    let read_ptr = data.as_ptr();
    unsafe {
        for i in 0..data.len() {
            let value = *read_ptr.add(i);
            println!("   读取索引 {}: {}", i, value);
        }
    }
    
    println!("\n🔧 W (Write) - 原始指针写入：");
    let write_ptr = data.as_mut_ptr();
    unsafe {
        *write_ptr.add(0) = 10;
        *write_ptr.add(1) = 20;
    }
    println!("   修改后的数据: {:?}", data);
    
    println!("\n🔧 O (Own) - 所有权管理：");
    let owned_ptr = Box::into_raw(Box::new(42));
    unsafe {
        println!("   拥有的值: {}", *owned_ptr);
        // 必须手动释放
        let _box = Box::from_raw(owned_ptr);
        println!("   所有权已转回 Box 进行释放");
    }
    
    println!("\n⚠️  Unsafe 权限风险：");
    println!("   • 可以绕过借用检查器");
    println!("   • 可能创建悬空指针");
    println!("   • 可能违反别名规则");
    println!("   • 需要手动保证内存安全");
    
    // 演示潜在危险
    println!("\n🔧 潜在危险示例（已安全处理）：");
    let mut vec = vec![1, 2, 3];
    let ptr1 = vec.as_mut_ptr();
    let ptr2 = vec.as_mut_ptr();
    
    unsafe {
        // 通过不同指针访问同一内存位置
        println!("   通过 ptr1 读取: {}", *ptr1);
        println!("   通过 ptr2 读取: {}", *ptr2);
        // 注意：同时使用两个可变指针是危险的
    }
}

// ============================================================================
// 10. 安全实践
// ============================================================================

fn safety_practices_demo() {
    println!("\n📍 10. 安全实践");
    println!("{}", "-".repeat(40));
    
    println!("🔧 安全实践原则：");
    println!("   1. 最小化 Unsafe 代码");
    println!("   2. 在安全抽象中封装");
    println!("   3. 详尽的文档和测试");
    println!("   4. 使用工具检查");
    
    // 示例：安全的缓冲区操作
    fn safe_buffer_copy(src: &[u8], dst: &mut [u8]) -> usize {
        let copy_len = src.len().min(dst.len());
        
        if copy_len > 0 {
            unsafe {
                ptr::copy_nonoverlapping(
                    src.as_ptr(),
                    dst.as_mut_ptr(),
                    copy_len
                );
            }
        }
        
        copy_len
    }
    
    let source = b"Hello, Unsafe Rust!";
    let mut destination = [0u8; 10];
    
    let copied = safe_buffer_copy(source, &mut destination);
    println!("   复制了 {} 字节", copied);
    println!("   目标缓冲区: {:?}", &destination[..copied]);
    
    println!("\n🛠️  推荐工具：");
    println!("   • Miri: 检测未定义行为");
    println!("   • AddressSanitizer: 内存错误检测");
    println!("   • Valgrind: 内存调试");
    println!("   • 静态分析工具");
    
    println!("\n📚 最佳实践建议：");
    println!("   • 优先使用安全 Rust");
    println!("   • 将 unsafe 封装在安全 API 中");
    println!("   • 编写全面的单元测试");
    println!("   • 使用 #[cfg(test)] 进行额外检查");
    println!("   • 定期审查 unsafe 代码");
} 