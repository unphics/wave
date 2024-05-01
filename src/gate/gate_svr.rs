
/**
 * @brief 网关服务器
 */

use std::net::UdpSocket;
use std::net::ToSocketAddrs;

pub struct gate_svr {
    name: String,
    sock: Option<UdpSocket>,
}

impl gate_svr {
    pub fn new(name: String) -> gate_svr {
        gate_svr{
            name: name,
            sock: None,
        }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    fn svr_run(&mut self) {
        self.init()
    }
    fn init(&mut self) {
        let addr = String::from("127.0.0.1:9527");
        self.sock = Some(UdpSocket::bind(addr).expect("failed to bind addr"));
    }
}