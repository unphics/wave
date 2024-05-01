
/**
 * @brief 网关服务器
 */

use std::net::UdpSocket;
use std::net::ToSocketAddrs;

pub struct gate_svr {
    name: String,
}

impl gate_svr {
    pub fn new(name: String) -> gate_svr {
        gate_svr{
            name: name
        }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    fn svr_run(&self) {
        self.init()
    }
    fn init(&self) {
        // let addr: ToSocketAddrs = "127.0.0.1:9527".to_string();
        // let sock = UdpSocket::bind(addr).exception("");
        // if sock.is_err() {
        //     panic
        // }
    }
}