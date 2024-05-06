use prost::Message;
use sqlite::State;

use crate::center;
use crate::center::center_svr::center_svr;
/**
 * @file gate_svr.rs
 * @brief 网关服务器
 * @author zys
 * @date Thu May 02 2024 03:43:18 GMT+0800 (中国标准时间)
 * @version 0.2
 * @descript 处理客户端连接, 分发客户端消息, 管理客户端代理
 */
use crate::cfg;
use crate::login::login_svr::login_svr;
use crate::pb::gate::CsReqLogin;
use crate::pb::unpack_udp;
use crate::proxy::proxy::proxy;
use std::collections::HashMap;
use std::net::UdpSocket;
use std::net::ToSocketAddrs;
use std::os::unix::net::SocketAddr;
use std::sync::Weak;
use std::sync::Arc;
use std::sync::Mutex;
use crate::pb;
use crate::sqlite3;


pub struct gate_svr{
    name: String,
    sock: Option<UdpSocket>,
    center_svr: Option<Weak<Mutex<center_svr>>>,
    proxys: HashMap<i32, Arc<proxy>>,
}

impl gate_svr{
    pub fn new(name: String) -> gate_svr {
        gate_svr{
            name: name,
            sock: None,
            center_svr: None,
            proxys: HashMap::new(),
        }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn begin_listen(&mut self, center_svr: Weak<Mutex<center_svr>>) {
        self.sock = Some(UdpSocket::bind(String::from(cfg::SERVER_ADDR)).expect("failed to bind addr"));
        self.center_svr = Some(Weak::clone(&center_svr));
        if let Some(sock) = &self.sock {
            loop {
                let mut buf: [u8; 1024] = [0u8; cfg::LISTEN_BUF_SIZE];
                let (size, addr) = sock.recv_from(&mut buf).expect("failed to recv");
                // 协议解包
                let (proto, pb_bytes) = pb::unpack_udp(&mut buf, size);
                self.deal_msg(addr, proto, pb_bytes);
            }
        }
    }
    /**
     * @brief 判断消息流向
     */
    fn deal_msg(&self, addr: std::net::SocketAddr, proto: u16, pb_bytes: Vec<u8>) {
        match proto {
            10000..=10999 => self.anomym_to_login(addr, proto, pb_bytes),
            _ => {
                println!("undefined proto !!!");
            }
        }
    }
    /**
     * @brief 处理匿名账户消息
     */
    fn anomym_to_login(&self, addr: std::net::SocketAddr, proto: u16, pb_bytes: Vec<u8>) {
        let center = self.center_svr.as_ref().unwrap().upgrade().unwrap();
        let login = center.lock().unwrap().route_login().unwrap().upgrade().unwrap();
        if let Some(ref_sock) = &self.sock {
            let sock = ref_sock.try_clone().unwrap();
            // login.lock().unwrap().anonym_msg(sock, addr, proto, pb_bytes);
            login_svr::anonym_msg(login, sock, addr, proto, pb_bytes)
        }
    }
    /**
     * 该账户在login_svr成功登录
     */
    pub fn on_login(&mut self, proxy: Arc<proxy>) {
        self.proxys.insert(proxy.account(), proxy);
    }

}