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
use std::net::UdpSocket;
use std::net::ToSocketAddrs;
use std::os::unix::net::SocketAddr;
use std::sync::Weak;
use std::sync::Arc;
use std::sync::Mutex;
use crate::pb;
use crate::sqlite3;

#[derive(Debug)]
pub struct gate_svr{
    name: String,
    sock: Option<UdpSocket>,
    center_svr: Option<Weak<Mutex<center_svr>>>,
}

impl gate_svr{
    pub fn new(name: String) -> gate_svr {
        gate_svr{
            name: name,
            sock: None,
            center_svr: None,
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
                let buf = &mut buf[.. size];
                // 协议包前usize是[内容大小段]-
                const LEN_SIZE: usize = std::mem::size_of::<usize>();
                let mut len_bytes = [0; LEN_SIZE];
                len_bytes.copy_from_slice(&buf[..LEN_SIZE]);
                let len = usize::from_be_bytes(len_bytes);
                // 然后前u16是[协议类型段]
                let proto = u16::from_be_bytes([buf[LEN_SIZE], buf[LEN_SIZE + 1]]);
                println!("recv desc: len = {}, proto = {}", len, proto);
                // 最后是[协议内容段]
                let mut pb_bytes = Vec::new();
                pb_bytes.extend_from_slice(&buf[LEN_SIZE + 2 .. size]);
                // 最后处理协议
                // self.decode_and_deal_pkg(proto, pb_bytes);
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
    pub fn on_login(&self) {
        println!("on_login");
    }

}