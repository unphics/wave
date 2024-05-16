
use crate::alloc;
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
use crate::proxy::proxy::proxy;
use std::collections::HashMap;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use crate::pb;


pub struct gate_svr{
    name: String,
    sock: Option<UdpSocket>,
    pub center_svr: *mut center_svr,
    proxys: Mutex<HashMap<i32, *mut proxy>>,
}

impl gate_svr{
    pub fn new(name: String) -> gate_svr {
        gate_svr {
            name: name,
            sock: None,
            center_svr: std::ptr::null_mut(),
            proxys: Mutex::new(HashMap::new()),
        }
    }
    pub fn begin_listen(&mut self) {
        self.sock = Some(UdpSocket::bind(String::from(cfg::SERVER_ADDR)).expect("failed to bind addr"));
        loop {
            let mut buf: [u8; 1024] = [0u8; cfg::LISTEN_BUF_SIZE];
            println!("gate: recv-ing...");
            let (size, caddr) = self.sock.as_ref().unwrap().recv_from(&mut buf).expect("failed to recv");
            // 协议解包
            let (proto, account, pb_bytes) = pb::unpack_msg(&mut buf, size);
            // todo 临时测试 去掉后续流程
            self.deal_msg(caddr, proto, account, pb_bytes);
        }
    }
    // 判断消息流向
    fn deal_msg(&mut self, caddr: std::net::SocketAddr, proto: u16, account: i32, pb_bytes: Vec<u8>) {
        match proto {
            10000..=10099 => self.anomym_to_login(caddr, proto, pb_bytes),
            _ => {
                self.forward_proxy(caddr, proto, account, pb_bytes);
                println!("undefined proto !!!");
            }
        }
    }
    fn forward_proxy(&mut self, caddr: std::net::SocketAddr, proto: u16, account: i32, pb_bytes: Vec<u8>) {
        let guard = self.proxys.lock().unwrap(); // 将锁定的互斥锁绑定到一个变量
        // 如果不这样的话: self.proxys.lock().unwrap()创建了一个临时值, 这个临时值在match语句结束时就会被释放从而导致在match语句中引用的值无效
        // 为了避免这个问题, 需要将锁的结果绑定到一个变量上使其在整个match期间都是有效的

        let option = guard.get(&account);
        match option {
            Some(p) => {
                let proxy = alloc::deref(*p);
                if proxy.check(&caddr) {
                    proxy.deal_msg(proto, pb_bytes);
                } else {
                    // 谁特么冒名顶替别人发消息?
                }
            }
            None => {
                // 该账户未登陆
            }
        }
    }
    /**
     * @brief 处理匿名账户消息
     * @description 匿名消息, 没有登录成功的客户端发送的都算匿名消息, 成功登录后才会走proxy
     */
    fn anomym_to_login(&self, caddr: std::net::SocketAddr, proto: u16, pb_bytes: Vec<u8>) {
        let center = alloc::deref(self.center_svr);
        let login = alloc::deref(center.route_login());
        login.send_anonym(self.sock.as_ref().unwrap().try_clone().unwrap(), caddr, proto, pb_bytes);
    }
    pub fn on_login(&mut self, p_proxy: *mut proxy) {
        let proxy = alloc::deref(p_proxy);
        proxy.set_gate(self);
        self.proxys.lock().unwrap().insert(proxy.account(), p_proxy);
    }
    
}