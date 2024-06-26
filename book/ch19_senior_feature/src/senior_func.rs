
pub fn senior_func() {
    // 1.函数指针
    let answer = do_twice(add_one, 5);
    println!("the answer is {}", answer);
}
fn add_one(x: i32) -> i32 {
    x + 1
}
fn do_twice(f: fn(i32)->i32, arg:i32) -> i32 {
    f(arg) + f(arg)
}