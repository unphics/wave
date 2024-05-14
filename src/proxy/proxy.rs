use crate::{gate::gate_svr::gate_svr, login::login_svr::login_svr};
/**
 * @file proxy
 * @brief 客户端代理
 * @author zys
 * @date
 * @version 0.1
 */
pub struct proxy {
    addr: std::net::SocketAddr,
    account: i32,
    login: *mut login_svr,
    gate: *mut gate_svr,
}

impl proxy {
    pub fn new(addr: std::net::SocketAddr, account: i32) -> proxy {
        return proxy {
            addr: addr,
            account: account,
            login: std::ptr::null_mut(),
            gate: std::ptr::null_mut(),
        };
    }
    pub fn check(&self, addr: std::net::SocketAddr, account: i32) -> bool {
        return self.addr == addr && self.account == account;
    }
    pub fn account(&self) -> i32 {
        return self.account;
    }
    pub fn set_login(&mut self, login: *mut login_svr) {
        self.login = login;
    }
    pub fn set_gate(&mut self, gate: *mut gate_svr) {
        self.gate = gate;
    }
}