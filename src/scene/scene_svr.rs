use std::collections::HashMap;
use std::sync::Mutex;
use crate::alloc;
use crate::proxy::proxy::proxy;
use crate::center::center_svr::center_svr;
use crate::svr::base;
use std::thread;
use std::time;
use crate::recast;
pub struct scene_svr {
    name: String,
    pub center_svr: *mut center_svr,
    proxys: Mutex<HashMap<i32, *mut proxy>>,
    stop: bool,
}
impl base for scene_svr {
    fn new(name: String) -> Self {
        return scene_svr {
            name: name,
            center_svr: std::ptr::null_mut(),
            proxys: Mutex::new(HashMap::new()),
            stop: false,
        }
    }
    fn begin(&mut self) {
        let result = unsafe{recast::ffi::recast_init()};
        if !result {
            println!("failed to int recast");
        }
    }
    fn run(&mut self) {
        let sleep_duration = time::Duration::from_millis(30);
        self.begin();
        while !self.stop {
            self.tick();
            thread::sleep(sleep_duration);
        }
    }
    fn end(&mut self) {
        unsafe{recast::ffi::recast_fini()};
    }
    fn shutdown(&mut self) {
        if self.stop == true {
            return;
        }
        self.stop = true;
    }
    fn name(&self) -> String {
        return self.name.clone();
    }
}
impl scene_svr {
    pub fn tick(&mut self) {
        // println!("tick");
    }
    pub fn send_new_proxy(&mut self, p_proxy: *mut proxy) {
        let proxy = alloc::deref(p_proxy);
        self.proxys.lock().unwrap().insert(proxy.account(), p_proxy);
        proxy.set_scene(self);
        // todo last 该写新角色进入场景的逻辑了
    }
}