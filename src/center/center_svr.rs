
/**
 * @file center_svr.rs
 * @brief 中心服务器
 * @author zys
 * @date Thu May 02 2024 03:56:57 GMT+0800 (中国标准时间)
 * @version 0.2
 */
use std::net::UdpSocket;
use std::sync::Condvar;
use std::sync::Mutex;
use std::thread;

use crate::alloc;
use crate::alloc::malloc;
use crate::gate::gate_svr::gate_svr;
use crate::login::login_svr::login_svr;
use crate::scene::scene_svr::scene_svr;
use crate::svr;
use crate::svr::base;
pub struct center_svr {
    name: String,
    sock: Option<UdpSocket>,
    gate_svr: *mut gate_svr,
    login_svr: *mut login_svr,
    scene_svr: *mut scene_svr,
    pub stop: bool,
    mutex: Mutex<u8>,
    cond: Condvar,
}
impl base for center_svr {
    fn new(name: String) -> Self {
        return center_svr {
            name: name,
            sock: None,
            gate_svr: std::ptr::null_mut(),
            login_svr: std::ptr::null_mut(),
            scene_svr: std::ptr::null_mut(),
            stop: false,
            mutex: Mutex::new(0),
            cond: Condvar::new(),
        };
    }
    fn begin(&mut self) {
        self.create_gate();
        self.create_login();
        self.create_scene();
    }
    fn run(&mut self) {
        while !self.stop {
            let mut lock = self.mutex.lock().expect("failed to lock");
            self.cond.wait_while(lock, |_| {
                return true; // 这玩意居然是true代表'不过', 真特么逆天
            }).unwrap();
            println!("center: run");
        }
    }
    fn end(&mut self) {}
    fn shutdown(&mut self) {
        if self.stop == true {
            return;
        }
        self.stop = true;
        self.cond.notify_all();
    }
    fn name(&self) -> String {
        return self.name.clone();
    }
}
impl center_svr {
    pub fn create_gate(&mut self) {
        self.gate_svr = svr::create("gate".to_string());
        alloc::deref(self.gate_svr).center_svr = self;
        let move_gate = self.gate_svr as usize;
        let _ = thread::spawn(move || {
            alloc::deref(move_gate as *mut gate_svr).begin();
        });
    }
    pub fn create_login(&mut self) {
        let p_login = malloc(login_svr::new("login".to_string()));
        self.login_svr = p_login.clone();
        alloc::deref(p_login).center_svr = self;
        let move_login = p_login as usize;
        let _ = thread::spawn(move || {
            alloc::deref(move_login as *mut login_svr).run();
        });
    }
    pub fn create_scene(&mut self) {
        let p_scene = malloc(scene_svr::new("scene".to_string()));
        self.scene_svr = p_scene.clone();
        alloc::deref(p_scene).center_svr = self;
        let move_scene = p_scene as usize;
        let _ = thread::spawn(move || {
            alloc::deref(move_scene as *mut scene_svr).run();
        });
    }
    pub fn route_scene(&self) -> *mut scene_svr {
        return self.scene_svr;
    }
    pub fn route_login(&self) -> *mut login_svr {
        return self.login_svr;
    }
    pub fn get_gate(&self) -> *mut gate_svr {
        return self.gate_svr;
    }
}