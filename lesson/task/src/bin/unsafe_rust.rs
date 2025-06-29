// 第20章：Unsafe Rust
// 演示 Rust 中的 unsafe 代码块、原始指针、外部函数等

use std::slice;

fn main() {
    println!("⚠️ 第20章：Unsafe Rust");
    println!("=====================================");
    
    // 1. 原始指针
    raw_pointers();
    
    // 2. 不安全函数
    unsafe_functions();
    
    // 3. 访问可变静态变量
    static_variables();
    
    // 4. 实现不安全 trait
    unsafe_traits();
    
    // 5. 内存操作
    memory_operations();
    
    // 6. 实际应用示例
    practical_examples();
}

// ============================================================================
// 1. 原始指针
// ============================================================================

fn raw_pointers() {
    println!("\n🎯 1. 原始指针");
    println!("{}", "-".repeat(40));
    
    // 创建原始指针
    println!("  🔸 创建原始指针：");
    
    let mut num = 5;
    
    // 从引用创建原始指针
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    println!("    不可变原始指针: {:p}", r1);
    println!("    可变原始指针: {:p}", r2);
    
    // 解引用原始指针需要 unsafe
    unsafe {
        println!("    解引用 r1: {}", *r1);
        println!("    解引用 r2: {}", *r2);
        
        // 通过可变指针修改值
        *r2 = 10;
        println!("    修改后的值: {}", *r2);
    }
    
    // 从任意地址创建原始指针（危险！）
    println!("\n  🔸 从任意地址创建原始指针：");
    
    let address = 0x012345usize;
    let _r = address as *const i32;
    
    println!("    从地址 0x{:x} 创建指针: {:p}", address, _r);
    // 注意：不要解引用这个指针，会导致段错误！
    
    // 指针算术
    println!("\n  🔸 指针算术：");
    
    let arr = [1, 2, 3, 4, 5];
    let ptr = arr.as_ptr();
    
    unsafe {
        for i in 0..arr.len() {
            let element_ptr = ptr.add(i);
            println!("    元素 {}: {}", i, *element_ptr);
        }
    }
    
    // 空指针检查
    println!("\n  🔸 空指针检查：");
    
    let null_ptr: *const i32 = std::ptr::null();
    let non_null_ptr = &42 as *const i32;
    
    println!("    空指针是否为空: {}", null_ptr.is_null());
    println!("    非空指针是否为空: {}", non_null_ptr.is_null());
    
    // 指针比较
    println!("\n  🔸 指针比较：");
    
    let x = 42;
    let y = 42;
    let ptr_x = &x as *const i32;
    let ptr_y = &y as *const i32;
    let ptr_x2 = &x as *const i32;
    
    println!("    ptr_x == ptr_y: {}", ptr_x == ptr_y);
    println!("    ptr_x == ptr_x2: {}", ptr_x == ptr_x2);
    
    unsafe {
        println!("    *ptr_x == *ptr_y: {}", *ptr_x == *ptr_y);
    }
}

// ============================================================================
// 2. 不安全函数
// ============================================================================

unsafe fn dangerous_function() {
    println!("    这是一个不安全函数");
}

fn unsafe_functions() {
    println!("\n⚠️ 2. 不安全函数");
    println!("{}", "-".repeat(40));
    
    // 调用不安全函数
    println!("  🔸 调用不安全函数：");
    
    unsafe {
        dangerous_function();
    }
    
    // 在不安全函数中包装安全代码
    println!("\n  🔸 安全抽象：");
    
    fn split_at_mut_safe(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();
        
        assert!(mid <= len);
        
        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }
    
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut_safe(&mut v, 3);
    
    println!("    左半部分: {:?}", left);
    println!("    右半部分: {:?}", right);
    
    // 修改两个部分
    left[0] = 10;
    right[0] = 20;
    
    println!("    修改后的向量: {:?}", v);
    
    // 不安全的内存分配
    println!("\n  🔸 不安全的内存分配：");
    
    unsafe fn allocate_and_initialize(size: usize, value: i32) -> *mut i32 {
        let layout = std::alloc::Layout::array::<i32>(size).unwrap();
        let ptr = std::alloc::alloc(layout) as *mut i32;
        
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        
        for i in 0..size {
            ptr.add(i).write(value);
        }
        
        ptr
    }
    
    unsafe fn deallocate(ptr: *mut i32, size: usize) {
        let layout = std::alloc::Layout::array::<i32>(size).unwrap();
        std::alloc::dealloc(ptr as *mut u8, layout);
    }
    
    unsafe {
        let ptr = allocate_and_initialize(5, 42);
        
        println!("    分配的内存内容:");
        for i in 0..5 {
            println!("      位置 {}: {}", i, *ptr.add(i));
        }
        
        deallocate(ptr, 5);
        println!("    内存已释放");
    }
}

// ============================================================================
// 3. 访问可变静态变量
// ============================================================================

static mut COUNTER: usize = 0;

fn static_variables() {
    println!("\n📊 3. 访问可变静态变量");
    println!("{}", "-".repeat(40));
    
    // 访问可变静态变量
    println!("  🔸 访问可变静态变量：");
    
    unsafe {
        COUNTER += 1;
        println!("    计数器值: {}", COUNTER);
        
        COUNTER += 5;
        println!("    计数器值: {}", COUNTER);
    }
    
    // 函数中的静态变量
    println!("\n  🔸 函数中的静态变量：");
    
    fn increment_counter() -> usize {
        unsafe {
            COUNTER += 1;
            COUNTER
        }
    }
    
    for i in 1..=3 {
        let count = increment_counter();
        println!("    第 {} 次调用，计数器: {}", i, count);
    }
    
    // 静态变量的线程安全问题
    println!("\n  🔸 静态变量的线程安全：");
    
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    static ATOMIC_COUNTER: AtomicUsize = AtomicUsize::new(0);
    
    fn safe_increment() -> usize {
        ATOMIC_COUNTER.fetch_add(1, Ordering::SeqCst)
    }
    
    for i in 1..=3 {
        let count = safe_increment();
        println!("    安全计数器第 {} 次: {}", i, count);
    }
    
    // 延迟初始化的静态变量
    println!("\n  🔸 延迟初始化：");
    
    use std::sync::Once;
    
    static INIT: Once = Once::new();
    static mut GLOBAL_DATA: Option<Vec<i32>> = None;
    
    fn get_global_data() -> &'static Vec<i32> {
        unsafe {
            INIT.call_once(|| {
                GLOBAL_DATA = Some(vec![1, 2, 3, 4, 5]);
            });
            GLOBAL_DATA.as_ref().unwrap()
        }
    }
    
    let data1 = get_global_data();
    let data2 = get_global_data();
    
    println!("    全局数据1: {:?}", data1);
    println!("    全局数据2: {:?}", data2);
    println!("    两次获取的是同一个对象: {}", std::ptr::eq(data1, data2));
}

// ============================================================================
// 4. 实现不安全 trait
// ============================================================================

unsafe trait UnsafeTrait {
    fn unsafe_method(&self);
}

struct MyStruct {
    data: i32,
}

unsafe impl UnsafeTrait for MyStruct {
    fn unsafe_method(&self) {
        println!("    执行不安全方法，数据: {}", self.data);
    }
}

fn unsafe_traits() {
    println!("\n🚨 4. 实现不安全 trait");
    println!("{}", "-".repeat(40));
    
    // 使用不安全 trait
    println!("  🔸 使用不安全 trait：");
    
    let my_struct = MyStruct { data: 42 };
    my_struct.unsafe_method();
    
    // Send 和 Sync trait
    println!("\n  🔸 Send 和 Sync trait：");
    
    use std::rc::Rc;
    
    struct MyBox<T> {
        data: *mut T,
    }
    
    impl<T> MyBox<T> {
        fn new(data: T) -> Self {
            let boxed = Box::new(data);
            MyBox {
                data: Box::into_raw(boxed),
            }
        }
        
        fn get(&self) -> &T {
            unsafe { &*self.data }
        }
    }
    
    impl<T> Drop for MyBox<T> {
        fn drop(&mut self) {
            unsafe {
                let _ = Box::from_raw(self.data);
            }
        }
    }
    
    // 手动实现 Send 和 Sync（通常不推荐）
    unsafe impl<T: Send> Send for MyBox<T> {}
    unsafe impl<T: Sync> Sync for MyBox<T> {}
    
    let my_box = MyBox::new(42);
    println!("    MyBox 中的数据: {}", my_box.get());
    
    // 验证 Send 和 Sync
    fn is_send<T: Send>() -> &'static str { "Send" }
    fn is_sync<T: Sync>() -> &'static str { "Sync" }
    
    println!("    MyBox<i32> 是 {}", is_send::<MyBox<i32>>());
    println!("    MyBox<i32> 是 {}", is_sync::<MyBox<i32>>());
    
    // Rc 不是 Send 或 Sync
    // println!("    Rc<i32> 是 {}", is_send::<Rc<i32>>()); // 编译错误
    println!("    Rc<i32> 不是 Send 或 Sync");
}

// ============================================================================
// 5. 内存操作
// ============================================================================

fn memory_operations() {
    println!("\n🧠 5. 内存操作");
    println!("{}", "-".repeat(40));
    
    // 内存复制
    println!("  🔸 内存复制：");
    
    let src = [1, 2, 3, 4, 5];
    let mut dst = [0; 5];
    
    unsafe {
        std::ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), src.len());
    }
    
    println!("    源数组: {:?}", src);
    println!("    目标数组: {:?}", dst);
    
    // 内存交换
    println!("\n  🔸 内存交换：");
    
    let mut x = 42;
    let mut y = 24;
    
    println!("    交换前: x = {}, y = {}", x, y);
    
    unsafe {
        std::ptr::swap(&mut x, &mut y);
    }
    
    println!("    交换后: x = {}, y = {}", x, y);
    
    // 内存替换
    println!("\n  🔸 内存替换：");
    
    let mut data = vec![1, 2, 3];
    let new_data = vec![4, 5, 6, 7];
    
    println!("    替换前: {:?}", data);
    
    let old_data = std::mem::replace(&mut data, new_data);
    
    println!("    替换后: {:?}", data);
    println!("    旧数据: {:?}", old_data);
    
    // 未初始化内存
    println!("\n  🔸 未初始化内存：");
    
    use std::mem::MaybeUninit;
    
    let mut uninit_array: [MaybeUninit<i32>; 5] = unsafe {
        MaybeUninit::uninit().assume_init()
    };
    
    // 初始化数组
    for (i, elem) in uninit_array.iter_mut().enumerate() {
        elem.write(i as i32 * 10);
    }
    
    // 转换为已初始化的数组
    let init_array: [i32; 5] = unsafe {
        std::mem::transmute(uninit_array)
    };
    
    println!("    初始化的数组: {:?}", init_array);
    
    // 内存对齐
    println!("\n  🔸 内存对齐：");
    
    #[repr(C)]
    struct AlignedStruct {
        a: u8,
        b: u32,
        c: u16,
    }
    
    println!("    AlignedStruct 大小: {}", std::mem::size_of::<AlignedStruct>());
    println!("    AlignedStruct 对齐: {}", std::mem::align_of::<AlignedStruct>());
    
    #[repr(packed)]
    struct PackedStruct {
        a: u8,
        b: u32,
        c: u16,
    }
    
    println!("    PackedStruct 大小: {}", std::mem::size_of::<PackedStruct>());
    println!("    PackedStruct 对齐: {}", std::mem::align_of::<PackedStruct>());
    
    // 类型转换
    println!("\n  🔸 类型转换：");
    
    let x: f64 = 42.0;
    let y: u64 = unsafe { std::mem::transmute(x) };
    
    println!("    f64 值: {}", x);
    println!("    转换为 u64: 0x{:x}", y);
    
    let z: f64 = unsafe { std::mem::transmute(y) };
    println!("    转换回 f64: {}", z);
}

// ============================================================================
// 6. 实际应用示例
// ============================================================================

fn practical_examples() {
    println!("\n🚀 6. 实际应用示例");
    println!("{}", "-".repeat(40));
    
    // 自定义智能指针
    custom_smart_pointer();
    
    // 内存池分配器
    memory_pool_allocator();
    
    // 高性能数据结构
    high_performance_data_structure();
}

fn custom_smart_pointer() {
    println!("\n  🔸 自定义智能指针：");
    
    struct UniquePtr<T> {
        ptr: *mut T,
    }
    
    impl<T> UniquePtr<T> {
        fn new(value: T) -> Self {
            let boxed = Box::new(value);
            UniquePtr {
                ptr: Box::into_raw(boxed),
            }
        }
        
        fn get(&self) -> &T {
            unsafe { &*self.ptr }
        }
        
        fn get_mut(&mut self) -> &mut T {
            unsafe { &mut *self.ptr }
        }
        
        fn into_inner(mut self) -> T {
            let value = unsafe { std::ptr::read(self.ptr) };
            self.ptr = std::ptr::null_mut(); // 防止 drop 时重复释放
            std::mem::forget(self); // 防止 drop
            value
        }
    }
    
    impl<T> Drop for UniquePtr<T> {
        fn drop(&mut self) {
            unsafe {
                let _ = Box::from_raw(self.ptr);
            }
        }
    }
    
    unsafe impl<T: Send> Send for UniquePtr<T> {}
    unsafe impl<T: Sync> Sync for UniquePtr<T> {}
    
    let mut ptr = UniquePtr::new(String::from("Hello, World!"));
    println!("    智能指针内容: {}", ptr.get());
    
    ptr.get_mut().push_str(" - Modified");
    println!("    修改后内容: {}", ptr.get());
    
    let value = ptr.into_inner();
    println!("    提取的值: {}", value);
}

fn memory_pool_allocator() {
    println!("\n  🔸 内存池分配器：");
    
    struct MemoryPool {
        memory: *mut u8,
        size: usize,
        offset: usize,
    }
    
    impl MemoryPool {
        fn new(size: usize) -> Self {
            unsafe {
                let layout = std::alloc::Layout::from_size_align(size, 8).unwrap();
                let memory = std::alloc::alloc(layout);
                if memory.is_null() {
                    std::alloc::handle_alloc_error(layout);
                }
                
                MemoryPool {
                    memory,
                    size,
                    offset: 0,
                }
            }
        }
        
        fn allocate<T>(&mut self) -> Option<*mut T> {
            let size = std::mem::size_of::<T>();
            let align = std::mem::align_of::<T>();
            
            // 对齐偏移
            let aligned_offset = (self.offset + align - 1) & !(align - 1);
            
            if aligned_offset + size <= self.size {
                let ptr = unsafe { self.memory.add(aligned_offset) as *mut T };
                self.offset = aligned_offset + size;
                Some(ptr)
            } else {
                None
            }
        }
        
        fn reset(&mut self) {
            self.offset = 0;
        }
        
        fn used(&self) -> usize {
            self.offset
        }
        
        fn remaining(&self) -> usize {
            self.size - self.offset
        }
    }
    
    impl Drop for MemoryPool {
        fn drop(&mut self) {
            unsafe {
                let layout = std::alloc::Layout::from_size_align(self.size, 8).unwrap();
                std::alloc::dealloc(self.memory, layout);
            }
        }
    }
    
    let mut pool = MemoryPool::new(1024);
    println!("    创建内存池，大小: {} 字节", pool.size);
    
    // 分配一些内存
    if let Some(ptr) = pool.allocate::<i32>() {
        unsafe {
            *ptr = 42;
            println!("    分配 i32，值: {}", *ptr);
        }
    }
    
    if let Some(ptr) = pool.allocate::<f64>() {
        unsafe {
            *ptr = 3.14159;
            println!("    分配 f64，值: {}", *ptr);
        }
    }
    
    println!("    已使用: {} 字节", pool.used());
    println!("    剩余: {} 字节", pool.remaining());
    
    pool.reset();
    println!("    重置后已使用: {} 字节", pool.used());
}

fn high_performance_data_structure() {
    println!("\n  🔸 高性能数据结构：");
    
    // 无锁栈（简化版）
    use std::sync::atomic::{AtomicPtr, Ordering};
    
    struct Node<T> {
        data: T,
        next: *mut Node<T>,
    }
    
    struct LockFreeStack<T> {
        head: AtomicPtr<Node<T>>,
    }
    
    impl<T> LockFreeStack<T> {
        fn new() -> Self {
            LockFreeStack {
                head: AtomicPtr::new(std::ptr::null_mut()),
            }
        }
        
        fn push(&self, data: T) {
            let new_node = Box::into_raw(Box::new(Node {
                data,
                next: std::ptr::null_mut(),
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
                    Err(_) => continue,
                }
            }
        }
        
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
                        let data = unsafe { Box::from_raw(head).data };
                        return Some(data);
                    }
                    Err(_) => continue,
                }
            }
        }
    }
    
    impl<T> Drop for LockFreeStack<T> {
        fn drop(&mut self) {
            while self.pop().is_some() {}
        }
    }
    
    unsafe impl<T: Send> Send for LockFreeStack<T> {}
    unsafe impl<T: Send> Sync for LockFreeStack<T> {}
    
    let stack = LockFreeStack::new();
    
    // 推入一些数据
    for i in 1..=5 {
        stack.push(i);
        println!("    推入: {}", i);
    }
    
    // 弹出数据
    while let Some(value) = stack.pop() {
        println!("    弹出: {}", value);
    }
    
    println!("    无锁栈演示完成");
} 