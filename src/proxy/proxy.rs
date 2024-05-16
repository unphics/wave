use crate::{alloc, gate::gate_svr::gate_svr, login::login_svr::login_svr};
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
    pub fn deal_msg(&mut self, proto: u16, pb_bytes: Vec<u8>) {
        match proto {
            10100..=10199 => {
                let login = alloc::deref(self.login);
                login.send_role(self, proto, pb_bytes);
            }
            _ => {
                // 其他, 以后再说
            }
        }
    }
    pub fn check(&self, addr: &std::net::SocketAddr) -> bool {
        return self.addr == *addr;
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