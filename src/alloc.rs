use std::alloc::{alloc, dealloc, Layout};

pub fn malloc<T>(obj: T) -> *mut T where T: Sized, {
    let layout = Layout::new::<T>();
    let mut p_deap: *mut T = std::ptr::null_mut();
    unsafe {
        p_deap = alloc(layout) as *mut T;
        p_deap.write(obj);
    }
    return p_deap;
}
pub fn free<T>(p: *mut T) {
    let layout = Layout::new::<T>();
    unsafe {
        dealloc(p as *mut u8, layout);
    }
}
pub fn deref<'a, T>(p: *mut T) -> & 'a mut T where T: Sized, {
    unsafe {
        &mut *p
    }
}