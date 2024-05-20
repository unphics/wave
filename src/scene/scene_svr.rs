use std::collections::HashMap;
use crate::proxy::proxy::proxy;
use crate::center::center_svr::center_svr;
use std::thread;
use std::time;

pub struct scene_svr {
    name: String,
    pub center_svr: *mut center_svr,
    proxys: HashMap<i32, *mut proxy>,
    stop: bool,
}

impl scene_svr {
    pub fn new(name: String) -> scene_svr {
        return scene_svr {
            name: name,
            center_svr: std::ptr::null_mut(),
            proxys: HashMap::new(),
            stop: false,
        }
    }
    pub fn run_scene(&mut self) {
        let sleep_duration = time::Duration::from_millis(30);
        while !self.stop {
            self.tick();
            thread::sleep(sleep_duration);
        }
    }
    pub fn tick(&mut self) {
        // println!("tick");
    }
    pub fn send_new_proxy(&mut self, proxy: *mut proxy) {
        
    }
}