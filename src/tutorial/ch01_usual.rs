pub fn ch01_usual_do() {
    println!("===== ch01_usual =====");
    let a = 1;
    let mut b = 2;
    println!("a = {}, b = {}", a, b);
    b = 3;
    const WORD_COUNT:u32 = 3; // 常量需要使用大写命名
    let a = 3; // 覆盖第一个a
    println!("a = {}, b = {}, count = {}", a, b, WORD_COUNT);
    let a:f32 = 1.414;
    println!("a = {}", a);
    let tp = (500, 3, 3.14);
    println!("tp.1 = {}, tp.2 = {}, tp.3 = {}", tp.0, tp.1, tp.2);
    let arr = [1, 3, 5, 7];
    println!("arr[1] = {}", arr[1]);
    let arr = [5;3];
    println!("arr[0] = {}, arr[1] = {}, arr[2] = {}",arr[0], arr[1], arr[2]);
    
}