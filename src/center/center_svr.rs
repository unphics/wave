/**
 * @file center_svr.rs
 * @brief 中心服务器
 * @author zys
 * @date Thu May 02 2024 03:56:57 GMT+0800 (中国标准时间)
 * @version 0.1
 */
use std::net::UdpSocket;
use std::option;
use std::rc::Rc;
use std::sync::Weak;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::thread;
use std::thread::sleep;
use std::time;

use crate::gate;
use crate::login;
pub struct center_svr {
    name: String,
    sock: Option<UdpSocket>,
    gate_svr: Option<Arc<Mutex<gate::gate_svr::gate_svr>>>,
    login_svr: Option<Arc<Mutex<login::login_svr::login_svr>>>,
    pub stop: bool,
    mutex: Mutex<(bool)>,
    cond: Condvar,
}

impl center_svr {
    pub fn new(name: String) -> Self {
        return center_svr {
            name: name,
            sock: None,
            gate_svr: None,
            login_svr: None,
            stop: false,
            mutex: Mutex::new(false),
            cond: Condvar::new(),
        };
    }
    pub fn run_center(&self) {
        let mut lock = self.mutex.lock().expect("failed to lock");
        while !*lock {
            lock = self.cond.wait(lock).unwrap();
        }
        // let sleep_duration = time::Duration::from_secs(5);
        // thread::sleep(sleep_duration);
    }
    pub fn tick(&self) {
        println!("tick");
    }
    pub fn shutdonw(&mut self) {
        if self.stop == true {
            return;
        }
        self.stop = true;
        self.cond.notify_all();
    }
    /**
     * 倒数第二步, 启动网关服务器
     */
    pub fn run_gate(this: Arc<Mutex<center_svr>>) {
        let gate = Arc::new(Mutex::new(gate::gate_svr::gate_svr::new("gate".to_string())));
        {
            this.lock().expect("111").gate_svr = Some(Arc::clone(&gate));
        }
        {
            this.lock().unwrap();
        }
        let weak_center = Arc::downgrade(&this);
        let handle = thread::spawn(move || {
            gate.lock().expect("222").begin_listen(weak_center);
        });
    }
    /**
     * @brief 启动登录服务器
     */
    pub fn run_login(this: Arc<Mutex<center_svr>>) {
        let login = Arc::new(Mutex::new(login::login_svr::login_svr::new("login".to_string())));
        {
            this.lock().expect("set center.login").login_svr = Some(Arc::clone(&login));
        }
        let weak_center = Arc::downgrade(&this);
        let handle = thread::spawn(move || {
            login.lock().expect("login begin").begin_login(weak_center);
        });
    }
    pub fn route_login(&self) -> Option<Weak<Mutex<login::login_svr::login_svr>>> {
        if let Some(login) = &self.login_svr {
            let weak = Arc::downgrade(&login);
            return Some(weak);
        }
        return None;
    }
    pub fn get_gate(&self) -> Option<Weak<Mutex<gate::gate_svr::gate_svr>>> {
        if let Some(gate) = &self.gate_svr {
            if let weak = Arc::downgrade(&gate) {
                return Some(weak);
            }
        }
        return None;
    }
}