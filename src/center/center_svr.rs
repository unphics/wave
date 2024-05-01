/**
 * @file center_svr.rs
 * @brief 中心服务器
 * @author zys
 * @date Thu May 02 2024 03:56:57 GMT+0800 (中国标准时间)
 * @version 0.1
 */
use std::{net::UdpSocket, rc::Rc, sync::{Arc, Mutex, Condvar}};
use crate::gate::gate_svr::gate_svr;
pub struct center_svr {
    name: String,
    sock: Option<UdpSocket>,
    gate_svr: Option<Rc<gate_svr>>,
    stop: bool,
    mutex: Mutex<()>,
    cond: Condvar,
}

impl center_svr {
    pub fn new(name: String) -> center_svr {
        return  center_svr {
            name: name,
            sock: None,
            gate_svr: None,
            stop: false,
            mutex: Mutex::new(()),
            cond: Condvar::new(),
        };
    }
    pub fn run_center(&self) {
        self.run_gate();

        // TODO LAST
    }
    fn run_gate(&self) {

    }
}