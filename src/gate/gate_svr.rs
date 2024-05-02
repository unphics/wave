use prost::Message;

/**
 * @file gate_svr.rs
 * @brief 网关服务器
 * @author zys
 * @date Thu May 02 2024 03:43:18 GMT+0800 (中国标准时间)
 * @version 0.1
 * @descript 处理客户端连接, 处理客户端登入, 分发客户端消息, 创建客户端代理, 分发客户端代理, 管理客户端代理
 */
use crate::cfg;
use crate::pb::gate::CsReqLogin;
use std::net::UdpSocket;
use std::net::ToSocketAddrs;
use crate::pb;

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
    pub fn begin_listen(&mut self) {
        self.sock = Some(UdpSocket::bind(String::from(cfg::SERVER_ADDR)).expect("failed to bind addr"));
        if let Some(sock) = &self.sock {
            loop {
                let mut buf = [0u8; cfg::LISTEN_BUF_SIZE];
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
                self.decode_and_deal_pkg(proto, pb_bytes);
            }
        }
    }
    fn decode_and_deal_pkg(&self, proto: u16, pb_bytes: Vec<u8>) {
        match proto {
            10001 => {
                let msg = pb::gate::CsReqLogin::decode(pb_bytes.as_slice()).expect("failed to decodelogin proto");
                println!("client request login: {:?}", msg);
            }
            10003 => {
                let msg = pb::gate::CsReqRegister::decode(pb_bytes.as_slice()).expect("failed to decodelogin proto");
                println!("client request register: {:?}", msg);
            }
            _ => {
                println!("undefined proto !!!");
            }
        }
    }
}