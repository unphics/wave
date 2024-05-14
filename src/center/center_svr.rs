
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
pub struct center_svr {
    name: String,
    sock: Option<UdpSocket>,
    gate_svr: *mut gate_svr,
    login_svr: *mut login_svr,
    pub stop: bool,
    mutex: Mutex<u8>,
    cond: Condvar,
}

impl center_svr {
    pub fn new(name: String) -> Self {
        return center_svr {
            name: name,
            sock: None,
            gate_svr: std::ptr::null_mut(),
            login_svr: std::ptr::null_mut(),
            stop: false,
            mutex: Mutex::new(0),
            cond: Condvar::new(),
        };
    }
    pub fn run_center(&mut self) {
        self.run_gate();
        self.run_login();
        while !self.stop {
            let mut lock = self.mutex.lock().expect("failed to lock");
            self.cond.wait_while(lock, |_| {
                return true; // 这玩意居然是true代表'不过', 真特么逆天
            }).unwrap();
            println!("center: run");
        }
    }
    pub fn shutdown(&mut self) {
        if self.stop == true {
            return;
        }
        self.stop = true;
        self.cond.notify_all();
    }
    pub fn run_gate(&mut self) {
        let p_gate = malloc(gate_svr::new("gate".to_string()));
        self.gate_svr = p_gate.clone();
        alloc::deref(p_gate).center_svr = self;
        let move_gate = p_gate.clone() as usize;
        let _ = thread::spawn(move || {
            alloc::deref(move_gate as *mut gate_svr).begin_listen();
        });
    }
    pub fn run_login(&mut self) {
        let p_login = malloc(login_svr::new("login".to_string()));
        self.login_svr = p_login.clone();
        alloc::deref(p_login).center_svr = self;
        let move_login = p_login.clone() as usize;
        let _ = thread::spawn(move || {
            alloc::deref(move_login as *mut login_svr).run_login();
        });
    }
    pub fn route_login(&self) -> *mut login_svr {
        return self.login_svr;
    }
    pub fn get_gate(&self) -> *mut gate_svr {
        return self.gate_svr;
    }
}