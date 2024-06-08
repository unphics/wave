
pub fn malloc<T>(obj: T) -> *mut T where T: Sized {
    let layout = std::alloc::Layout::new::<T>();
    let mut ptr_deap: *mut T = std::ptr::null_mut();
    unsafe {
        ptr_deap = std::alloc::alloc(layout) as *mut T;
        ptr_deap.write(obj);
    }
    return ptr_deap;
}

pub fn free<T>(ptr_deap: *mut T) {
    let layout = std::alloc::Layout::new::<T>();
    unsafe {
        std::alloc::dealloc(ptr_deap as *mut u8, layout);
    }
}

pub fn deref<'a, T>(ptr: *mut T) ->&'a mut T where T: Sized {
    unsafe{&mut*ptr}
}