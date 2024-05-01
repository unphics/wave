use std::ops::{Deref, DerefMut};
struct Base {
    id: u32,
    name: String,
}
impl Base {
    fn watch(&self) {
        println!("base: id = {}, name = {}", self.id, self.name);
    }
}
struct Derive {
    base: Base,
    num: u32,
}
impl Derive {
    fn new() -> Derive {
        Derive{
            base: Base{id: 1111, name: "qqq".to_string()},
            num: 3333
        }
    }
}
impl Deref for Derive {
    type Target = Base;
    fn deref<'a>(&'a self) -> &'a Base {
        &self.base
    }
}
impl DerefMut for Derive {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Base {
        &mut self.base
    }
}
fn main() {
    let mut obj = Derive::new();
    obj.watch();
    obj.id = 222;
    obj.watch();
    println!("obj.num: {}", obj.num);
    obj.num = 4444;
    println!("obj.num: {}", obj.num);
}