/**
 * @file center_svr.rs
 * @brief 中心服务器
 * @author zys
 * @date Thu May 02 2024 03:56:57 GMT+0800 (中国标准时间)
 * @version 0.1
 */
use std::{net::UdpSocket, rc::Rc, sync::{Arc, Condvar, Mutex}, thread};
use crate::gate;
pub struct center_svr {
    name: String,
    sock: Option<UdpSocket>,
    gate_svr: Option<Arc<Mutex<gate::gate_svr::gate_svr>>>,
    stop: bool,
    mutex: Mutex<()>,
    cond: Condvar,
}

impl center_svr {
    pub fn new(name: String) -> Arc<Mutex<Self>> {
        return Arc::new(Mutex::new(center_svr {
            name: name,
            sock: None,
            gate_svr: None,
            stop: false,
            mutex: Mutex::new(()),
            cond: Condvar::new(),
        }));
    }
    pub fn run_center(&mut self) {
        while self.stop != true {
            let mut lock = self.mutex.lock().expect("failed to lock");
            self.cond.wait(lock);
        }
    }
    pub fn shutdonw(&mut self) {
        if self.stop == true {
            return;
        }
        self.stop = true;
        self.cond.notify_all();
    }
    pub fn run_gate(this: Arc<Mutex<center_svr>>) {
        let gate = Arc::new(Mutex::new(gate::gate_svr::gate_svr::new("gate".to_string())));
        {
            this.lock().expect("111").gate_svr = Some(Arc::clone(&gate));
        }
        let center_weak = Arc::downgrade(&this);
        let handle = thread::spawn(move || {
            gate.lock().expect("222").begin_listen(center_weak);
        });
    }
}