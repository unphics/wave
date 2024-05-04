use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Weak;
use std::sync::Mutex;
use crate::center::center_svr::center_svr;
use crate::pb;
use crate::sqlite3;
use std::net::UdpSocket;
use prost::Message;
use crate::proxy;
/**
 * @file login_svr.rs
 * @brief 登录服务器
 * @author zys
 * @date Thu May 02 2024 22:17:10 GMT+0800 (中国标准时间)
 * @version 0.1
 * @description 处理注册-登录-角色选择等事务, 登录成功后创建proxy转发给gate和role, 同时进入选角色流程
 */

pub struct login_svr {
    name: String,
    center_svr: Option<Weak<Mutex<center_svr>>>,
    proxys: HashMap<i32, Arc<proxy::proxy::proxy>>
}

impl login_svr {
    pub fn new(name: String) -> login_svr {
        return login_svr {
            name: name,
            center_svr: None,
            proxys: HashMap::new(),
        };
    }
    /**
     * @brief init
     */
    pub fn begin_login(&mut self, center_svr: Weak<Mutex<center_svr>>) {
        self.center_svr = Some(Weak::clone(&center_svr));
    }
    /**
     * @brief 匿名消息
     * @description 没有登录成功的客户端发送的都算匿名消息, 成功登录后才会走proxy
     * @todo 匿名消息的处理先放到这里, 后面出一个struct或者trait
     */
    pub fn anonym_msg(&mut self, sock: UdpSocket, addr: std::net::SocketAddr, proto: u16, pb_bytes: Vec<u8>) {
        match proto {
            10001 => {
                let msg = pb::gate::CsReqLogin::decode(pb_bytes.as_slice()).expect("failed to decodelogin proto");
                println!("client request login: {:?}", msg);

                if sqlite3::data::exit_row("users", msg.account as i64) {
                    println!("账号存在, 登录成功");
                    // todo 忘记验证密码了
                    pb::send_proto(sock, addr.clone(), proto, pb::gate::CsRspLogin{result: true,error_code: 10001});
                    // 登录成功后续流程
                    self.create_proxy(addr, msg.account);
                } else {
                    println!("账号不存在, 需要注册");
                    pb::send_proto(sock, addr, proto, pb::gate::CsRspLogin{result: false,error_code: 10002});
                }
            }
            10003 => {
                let msg = pb::gate::CsReqRegister::decode(pb_bytes.as_slice()).expect("failed to decodelogin proto");
                println!("client request register: {:?}", msg);
                if sqlite3::data::exit_row("users", msg.account as i64) {
                    println!("账号已存在, 不需要注册");
                    pb::send_proto(sock, addr, proto, pb::gate::CsRspLogin{result: false,error_code: 10103});
                } else {
                    println!("账号不存在, 可以注册");
                    if sqlite3::data::insert_row("users", "account, password", "?, ?", |statement: &mut sqlite::Statement| {
                        statement.bind((1, msg.account as i64)).expect("state.bind");
                        statement.bind((2, msg.passwword.as_str())).expect("state.bind");
                    }) {
                        println!("注册成功");
                        pb::send_proto(sock, addr, proto, pb::gate::CsRspLogin{result: true,error_code: 10101});
                    } else {
                        println!("注册失败");
                        pb::send_proto(sock, addr, proto, pb::gate::CsRspLogin{result: false,error_code: 10102});
                    }
                }
            }
            _ => println!("anonym_msg: undefined proto !!!")
        }
    }
    fn create_proxy(&mut self, addr: std::net::SocketAddr, account: i32) {
        // let gate = self.center_svr.as_ref().unwrap().upgrade().unwrap().lock().unwrap().get_gate();
        // gate.unwrap().upgrade().unwrap().lock().unwrap().on_login();
        let proxy = Arc::new(proxy::proxy::proxy::new(addr, account));
        // let arc = Arc::new(Mutex::new(&*self));
        // let weak: Weak<Mutex<&login_svr>> = Arc::downgrade(&arc);
        // // proxy.set_login(weak);
        self.proxys.insert(proxy.account(), Arc::clone(&proxy));
    }
}