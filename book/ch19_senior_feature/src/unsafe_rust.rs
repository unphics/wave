use core::slice;

static HELLO_WORLD: &str = "hello, world";
static mut COUNTER: u32 = 0;

pub fn unsafe_rust() {
    println!("Hello, world!");

    // 1. 解引用裸指针需要unsafe

    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    // println!("r1: {}", *r1); // 报错: 解引用裸指针是不安全的
    // println!("r2: {}", *r2); // 报错: 解引用裸指针是不安全的
    unsafe {
        println!("r1: {}", *r1); // 报错: 解引用裸指针是不安全的
        println!("r2: {}", *r2); // 报错: 解引用裸指针是不安全的
    }

    let addr = 0x12345usize;
    let r = addr as *const i32;
    unsafe {
        println!("r: {}", *r);
    }

    // 2. unsafe 函数

    // dangerous(); // unsafe fn必须在unsafe作用域(unsafe block)中调用
    unsafe {
        dangerous();
    }

    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let r: &mut[i32] = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    // split_at_mut 是标准库中的, 接收一个切片, 并从指定位置分割成俩切片
    // 这个标准库中的函数其实是使用了不安全的代码
    assert_eq!(a, &mut[1, 2, 3]);
    assert_eq!(b, &mut[4, 5, 6]);

    // 3. extern block 中可以调用ffi, 任何extern函数都是不安全的
    // extern: 简化创建和使用外部函数接口(ffi)的过程
    // ffi: foreign function interface 外部函数接口, 他允许一种编程语言定义函数, 并让其他编程语言能调用这些函数
    // abi: application binary interface 应用二进制接口, 定义函数在汇编层的调用方式
    // 参见下方 call_from_c
    

    // 4. 读写静态变量
    println!("name is {}", HELLO_WORLD);
    // 静态变量可以是可变的, 但是读写可变的静态变量是不安全的
    add_to_count(3);
    unsafe {
        println!("static var counter is {}", COUNTER);
    }

    // 5. 当某个trait中至少一个方法有编译器无法校验的不安全的因素时, 这个trait是不安全的
}

unsafe fn dangerous() {}

fn split_at_mut(slice: &mut[i32], mid: usize) ->(&mut[i32], &mut[i32]) {
    let len = slice.len();
    assert!(mid <= len);
    // (&mut slice[.. mid], &mut slice[mid ..])
    // 这里是切开了切片的两个不同部分, 这两个部分并不交叉, 但是rust无法理解这件事

    let ptr = slice.as_mut_ptr();

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// 此函数在编译和链接后可以被c语言访问
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("just called a rust fn from c");
}

fn add_to_count(value: u32) {
    unsafe {
        COUNTER += value;
    }
}

unsafe trait  foo {
    
}

unsafe impl foo for i32 {
    
}