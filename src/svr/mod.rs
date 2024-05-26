use crate::alloc;

pub trait base {
    fn new(name: String) -> Self;
    fn begin(&mut self);
    fn run(&mut self);
    fn end(&mut self);
    fn shutdown(&mut self);
    fn name(&self) -> String;
}

pub fn create<T>(name: String) -> *mut T where T: base{
    let p = alloc::malloc(T::new(name));
    alloc::deref(p).begin();
    return p;
}