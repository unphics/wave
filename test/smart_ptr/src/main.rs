use std::rc::Rc;



fn main() {
    row_ptr();
    rc_ptr();
}
// 裸指针
fn row_ptr() {
    // 不可变
    let x: usize = 1;
    let raw_ptr_1: *const usize = &x as *const usize;
    let raw_ptr_2: *const usize = &x;
    // 可变
    let mut y: usize = 2;
    let raw_mut_ptr_1: *mut usize = &mut y as *mut usize;
    let raw_mut_ptr_2: *mut usize = &mut y;
    // 解引用
    let some_usize_1: usize = unsafe {*raw_ptr_1};
    println!("some_usize_1 = {some_usize_1}");
    let some_usize_2: usize = unsafe {*raw_ptr_2};
    println!("some_usize_2 = {some_usize_2}");
    let mut some_mut_usize_1 = unsafe {*raw_mut_ptr_1};
    println!("some_mut_usize_1 = {some_mut_usize_1}");
    let mut some_mut_usize_2 = unsafe {*raw_mut_ptr_2};
    println!("some_mut_usize_2 = {some_mut_usize_2}");
}
// Rc指针
#[derive(Debug)]
struct Cat {}
fn rc_ptr() {
    // let cat1: Cat = Cat{};
    // let cat2: Cat = Cat{};
    // let cat3: Cat = Cat{};
    // let cats1: Vec<Cat> = vec![cat1, cat2];
    // let cats2: Vec<Cat> = vec![cat2, cat3];
    // println!("{:?}", cats2);

    // 原始类型
    let a = 1;
    let b = 2;
    let c = 3;
    let vec_num_1 = vec![a, b];
    let vec_num_2 = vec![b, c];
    println!("vec_num_1: {:?}, vec_num_2: {:?}", vec_num_1, vec_num_2);

    let rc_cat_1 = Rc::new(Cat{});
    let rc_cat_2 = Rc::new(Cat{});
    let rc_cat_3 = Rc::new(Cat{});
    let vec_cat_1 = vec![rc_cat_1, rc_cat_2];
    // let vec_cat_2 = vec![rc_cat_2, rc_cat_3];
    // println!("vec_cat_1: {:?}, vec_cat_2: {:?}", vec_cat_1, vec_cat_2);
}