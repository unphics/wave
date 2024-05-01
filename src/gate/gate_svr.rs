/**
 * @file gate_svr.rs
 * @brief 网关服务器
 * @author zys
 * @date Thu May 02 2024 03:43:18 GMT+0800 (中国标准时间)
 * @version 0.1
 * @descript 
 */
use crate::cfg;
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
    pub fn begin_listen(&mut self) {
        self.sock = Some(UdpSocket::bind(String::from(cfg::SERVER_ADDR)).expect("failed to bind addr"));
        if let Some(sock) = &mut self.sock {
            loop {
                let mut buf = [0u8; cfg::LISTEN_BUF_SIZE];
                let (size, addr) = sock.recv_from(&mut buf).expect("failed to recv");
                let buf = &mut buf[.. size];
                
                const LEN_SIZE: usize = std::mem::size_of::<usize>();
                let mut len_bytes = [0; LEN_SIZE];
                len_bytes.copy_from_slice(&buf[..LEN_SIZE]);
                let len = usize::from_be_bytes(len_bytes);
                
                let proto = u16::from_be_bytes([buf[LEN_SIZE], buf[LEN_SIZE + 1]]);

                let mut msg = String::new();
                for v in buf[LEN_SIZE  + 2 ..].iter() {
                    if '\n'== (*v as char) || '\r' == (*v as char) {
                        continue;
                    }
                    msg.push(*v as char);
                }
                println!("recv: len = {}, proto = {}, msg = {}", len, proto, msg);
            }
        }
    }
}