
pub fn senior_type() {
    // 1. 使用newtype模式实现类型安全和抽象
    // newtype模式可以: 用来静态的保证各种值之间不会混淆并表明值的单位, 为类型的某些细节提供抽象能力
    // 使用类型别名创建类型同义词
    let x: i32 = 5;
    let y: kilo_meters = 5;
    println!("x + y = {}", x + y);
    let f: Thunk = Box::new(|| println!("hi"));
}
/// 1.
type kilo_meters = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;
fn takes_long_type(f: Thunk) {}
fn returns_long_type() -> Thunk {
    Box::new( || println!("hi"))
}