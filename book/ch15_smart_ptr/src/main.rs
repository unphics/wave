mod mgr;
mod inner;
use std::rc::{self, Rc};
struct Cat {
    id: i32,
}
fn main() {
    // mgr::mgr();
    inner::inner();
    // let b = Box::new(5);
    // println!("b = {}", b);

    let mut cat = Rc::new(Cat{id:1});

    if let Some(mut_cat) = Rc::get_mut(&mut cat) {
        mut_cat.id = 2;
    }
}
/*
use std::rc::Rc;
use std::cell::RefCell;

// 定义 Mgr 结构体
struct Mgr<'a> {
    // Mgr 持有 Actor 的可变引用
    actor: Option<Rc<RefCell<Actor<'a>>>>,
}

// 定义 Actor 结构体
struct Actor<'a> {
    // Actor 持有 Mgr 的引用
    mgr: &'a Mgr<'a>,
}

impl<'a> Mgr<'a> {
    // 创建 Mgr
    fn new() -> Mgr<'a> {
        Mgr { actor: None }
    }

    // 将 Actor 添加到 Mgr
    fn add_actor(&mut self, actor: Rc<RefCell<Actor<'a>>>) {
        self.actor = Some(actor);
    }

    // Mgr 的方法，供 Actor 调用
    fn mgr_method(&self) {
        println!("Mgr method called");
    }
}

impl<'a> Actor<'a> {
    // 创建 Actor
    fn new(mgr: &'a Mgr<'a>) -> Actor<'a> {
        Actor { mgr }
    }

    // Actor 的方法，可以调用 Mgr 的方法
    fn actor_method(&self) {
        println!("Actor method called");
        self.mgr.mgr_method();
    }
}

fn main() {
    // 创建 Mgr
    let mut mgr = Mgr::new();

    // 创建 Actor，并传入 Mgr 的引用
    let actor = Rc::new(RefCell::new(Actor::new(&mgr)));

    // 将 Actor 添加到 Mgr 中
    mgr.add_actor(Rc::clone(&actor));

    // 调用 Actor 的方法，Actor 可以调用 Mgr 的方法
    actor.borrow().actor_method();
}
*/