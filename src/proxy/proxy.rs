use std::sync::Weak;
use std::sync::Mutex;
use crate::login::login_svr::login_svr;
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
    login:Option<Weak<Mutex<login_svr>>>,
}

impl proxy {
    pub fn new(addr: std::net::SocketAddr, account: i32) -> proxy {
        return proxy {
            addr: addr,
            account: account,
            login: None,
        };
    }
    pub fn account(&self) -> i32 {
        return self.account;
    }
    pub fn set_login(&self, login: Weak<Mutex<login_svr>>) {

    }
}