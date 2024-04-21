

fn main() {
    println!("Hello, world!");
    // 1. 闭包
    let add_fn = |a, b| {
        a + b
    };
    println!("add: {}", add_fn(1, 3));

    // 3. 闭包捕获上下文
    let x = 3;
    let equal = move |z| {
        z == x
    };
    let  y = 3;
    assert!(equal(y));

}
// 2. 闭包缓存器
struct Cacher<T> where T: Fn(u32) -> u32 {
    calc: T,
    value: Option<u32>,
}
impl<T> Cacher<T> where T: Fn(u32) -> u32  {
    fn new(calc: T) -> Cacher<T> {
        Cacher {
            calc,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calc)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}