
use std::{sync::{Arc, Mutex, Weak}, thread, time};
/**
 * 解决两个线程对象相互持有的方案 ::std::enable_shared_from_this
 */
struct Mgr {
    num: i32,
    entity: Option<Arc<Mutex<Entity>>>,
}
impl Mgr {
    fn new(num:i32) -> Arc<Mutex<Self>> {
        return Arc::new(Mutex::new(Mgr {
            num: num,
            entity: None,
        }));
    }
    fn create_entity(this: Arc<Mutex<Mgr>>, id: i32) {
        let entity = Arc::new(Mutex::new(Entity::new(id)));
        { this.lock().unwrap().entity = Some(Arc::clone(&entity)); }
        let mgr_weak = Arc::downgrade(&this);
        let handle = thread::spawn(move || {
            entity.lock().unwrap().entity_work(mgr_weak, 3333);
        });
        handle.join().unwrap();
    }
    fn mgr_work(&mut self, num: i32) {
        self.num = num;
        println!("mgr.work(), mgr.num = {}", self.num);
    }
}
struct Entity {
    mgr: Option<Weak<Mutex<Mgr>>>,
    id: i32,
}
impl Entity {
    pub fn new(id: i32) -> Self {
        return Entity{mgr: None, id: id};
    }
    pub fn entity_work(&mut self, mgr: Weak<Mutex<Mgr>>, id: i32) {
        self.id = id;
        self.mgr = Some(Weak::clone(&mgr));
        if let Some(weak_mgr) = &self.mgr {
            if let Some(self_mgr) = weak_mgr.upgrade() {
                self_mgr.lock().unwrap().mgr_work(555);
            }
        }
        let sleep_duration = time::Duration::from_secs(1);
        thread::sleep(sleep_duration);
        println!("entity.work() entity.id = {}", self.id);
    }
}

pub fn inner() {
    let mgr = Mgr::new(111);
    Mgr::create_entity(Arc::clone(&mgr), 111);
}