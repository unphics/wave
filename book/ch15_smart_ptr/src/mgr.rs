use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Weak;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time;
/**
 * 管理器的生命周期跟随main, 实体是由管理器创建并关联, 实体在另一个线程运行
 * 管理器创建实体时调用实体的初始化方法, 实体运行的时候有需求要调用管理器的方法
 */
struct Mgr {
    entity: Option<Arc<Mutex<Entity>>>,
    num: i32,
}
impl Mgr {
    pub fn new() -> Mgr {
        return Mgr{entity: None, num: 3};
    }
    pub fn create_entity(& mut self) {
        let entity = Arc::new(Mutex::new(Entity::new()));
        self.entity = Some(Arc::clone(&entity));
        let mgr_weak = Arc::downgrade(&Arc::new(Mutex::new(self)));
        let handle = thread::spawn(move || {
            let mut entity_lock = entity.lock().unwrap();
            // let mut mgr_lock = shard.lock().unwrap();
            // let weak = Arc::downgrade(&shard);
            // entity_lock.entity_work(mgr_weak , 111);
        });
        handle.join().unwrap();
    }
    pub fn mgr_work(& mut self, num: i32) {
        self.num = num;
        println!("mgr.work(), mgr.num = {}", self.num);
    }
}
struct Entity {
    mgr: Option<Arc<Mutex<Mgr>>>,
    id: i32,
}
impl Entity {
    pub fn new() -> Entity {
        return Entity{mgr: None, id: 0};
    }
    pub fn entity_work(& mut self, mgr: Weak<Mutex<&Mgr>>, id: i32) {
        self.id = id;
        // self.mgr = Some(Weak::clone(&mgr));
        // if let Some(self_mgr) = &self.mgr {
        //     if let Some(mgr_weak) = self_mgr.upgrade() {
        //         let mut mgr_lock = mgr_weak.lock().unwrap();
        //         mgr_lock.mgr_work(11111);
        //     }
        // }
        let sleep_duration = time::Duration::from_secs(1);
        thread::sleep(sleep_duration);
        println!("entity.work()");
    }
    pub fn fn1(&mut self) {
        self.id = 3;
    }
}
pub fn mgr() {
    let mut mgr = Mgr::new();
    mgr.num = 55;
    mgr.create_entity();
}

/*
use std::borrow::BorrowMut;
use std::env::args;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Weak;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time;
/**
 * 管理器的生命周期跟随main, 实体是由管理器创建并关联, 实体在另一个线程运行
 * 管理器创建实体时调用实体的初始化方法, 实体运行的时候有需求要调用管理器的方法
 */
struct Mgr {
    entity: Option<Arc<Mutex<Entity>>>,
    num: i32,
}
impl Mgr {
    pub fn new() -> Mgr {
        return Mgr{entity: None, num: 3};
    }
    pub fn create_entity(& mut self) {
        let entity = Arc::new(Mutex::new(Entity::new()));
        self.entity = Some(Arc::clone(&entity));
        let mgr_weak = Arc::downgrade(&Arc::new(Mutex::new(&*self)));
        let handle = thread::spawn(move || {
            let mut entity_lock = entity.lock().unwrap();
            entity_lock.entity_work(mgr_weak , 111);
        });
        handle.join().unwrap();
    }
    pub fn mgr_work(& mut self, num: i32) {
        self.num = num;
        println!("mgr.work(), mgr.num = {}", self.num);
    }
}
struct Entity {
    mgr: Option<Arc<Mutex<Mgr>>>,
    id: i32,
}
impl Entity {
    pub fn new() -> Entity {
        return Entity{mgr: None, id: 0};
    }
    pub fn entity_work(& mut self, mgr: Weak<Mutex<&Mgr>>, id: i32) {
        self.id = id;
        // self.mgr = Some(Weak::clone(&mgr));
        // if let Some(self_mgr) = & self.mgr {
        //     if let Some(mgr_weak) = self_mgr.upgrade() {
        //         let mut mgr_lock = mgr_weak.lock().unwrap();
        //         mgr_lock.mgr_work(11111);
        //     }
        // }
        let sleep_duration = time::Duration::from_secs(1);
        thread::sleep(sleep_duration);
        println!("entity.work()");
    }
    pub fn fn1(&mut self) {
        self.id = 3;
    }
}
pub fn mgr() {
    let mut mgr = Mgr::new();
    mgr.num = 55;
    mgr.create_entity();
}
*/