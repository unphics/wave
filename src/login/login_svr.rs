use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Weak;
use std::sync::Mutex;
use std::collections::VecDeque;
use crate::center::center_svr::center_svr;
use crate::pb;
use crate::sqlite3;
use std::net::UdpSocket;
use prost::Message;
use crate::proxy;
use crate::login::anonym_task::anonym_task;
/**
 * @file login_svr.rs
 * @brief 登录服务器
 * @author zys
 * @date Thu May 02 2024 22:17:10 GMT+0800 (中国标准时间)
 * @version 0.2
 * @description 处理注册-登录-角色选择等事务, 登录成功后创建proxy转发给gate和role, 同时进入选角色流程
 */

pub struct login_svr {
    name: String,
    pub center_svr: *mut center_svr,
    proxys: HashMap<i32, Arc<proxy::proxy::proxy>>,
    mutex: Mutex<u8>,
    cond: Condvar,
    stop: bool,
    queue: Mutex<VecDeque<anonym_task>>,
}

impl login_svr {
    pub fn new(name: String) -> login_svr {
        return login_svr {
            name: name,
            center_svr: std::ptr::null_mut(),
            proxys: HashMap::new(),
            mutex: Mutex::new(1),
            cond: Condvar::new(),
            stop: false,
            queue: Mutex::new(VecDeque::new()),
        };
    }
    pub fn run_login(&mut self) {
        while !self.stop {
            let lock = self.mutex.lock().unwrap();
            self.cond.wait_while(lock, |pending| {
                if self.queue.lock().unwrap().len() > 0 {
                    return false;
                }
                return true;
            }).unwrap();
            println!("login: run");
            self.work();
        }
    }
    pub fn work(&mut self) {
        self.deal_with_anonym();
    }
    // 匿名消息, 没有登录成功的客户端发送的都算匿名消息, 成功登录后才会走proxy, 匿名消息的处理先放到这里, 后面出一个struct或者trait
    pub fn send_anonym(&mut self, sock: UdpSocket, addr: std::net::SocketAddr, proto: u16, pb_bytes: Vec<u8>) {
        let task = anonym_task::new(sock, addr, proto, pb_bytes);
        self.queue.lock().unwrap().push_back(task);
        self.cond.notify_one();
    }
    fn create_proxy(this: Arc<Mutex<login_svr>>, addr: std::net::SocketAddr, account: i32) {
        let mut proxy = Arc::new(proxy::proxy::proxy::new(addr, account));
        let weak = Arc::downgrade(&this);
        let arc = weak.upgrade().unwrap();
        // proxy.set_login(weak.clone()); // todo last
        this.lock().unwrap().proxys.insert(proxy.account(), Arc::clone(&proxy));
        println!("client has been login succeed, and the proxy has been insert !");
        
        // let gate = arc.lock().unwrap().center_svr.as_ref().unwrap().upgrade().unwrap().lock().unwrap().get_gate();
        // gate.unwrap().upgrade().unwrap().lock().unwrap().on_login(proxy);
    }
    pub fn deal_with_anonym(&mut self) {
        if !(self.queue.lock().unwrap().len() > 0) {
            return;
        }
        let task = self.queue.lock().unwrap().pop_front().unwrap();
        match task.proto {
            10001 => {
                let msg = pb::login::CsReqLogin::decode(task.pb_bytes.as_slice()).expect("failed to decodelogin proto");
                println!("client request login: {:?}", msg);

                if sqlite3::data::exit_row("users", msg.account as i64) {
                    println!("账号存在, 登录成功");
                    // todo 忘记验证密码了, 而且也没验证已登录
                    pb::send_proto(task.sock, task.addr.clone(), 10002, pb::login::CsRspLogin{result: true,error_code: 10001});
                    // 登录成功后续流程 todo
                    // login_svr::create_proxy(this, task.addr, msg.account);
                } else {
                    println!("账号不存在, 需要注册");
                    pb::send_proto(task.sock, task.addr, 10002, pb::login::CsRspLogin{result: false,error_code: 10002});
                }
            }
            10003 => {
                let msg = pb::login::CsReqRegister::decode(task.pb_bytes.as_slice()).expect("failed to decodelogin proto");
                println!("client request register: {:?}", msg);
                if sqlite3::data::exit_row("users", msg.account as i64) {
                    println!("账号已存在, 不需要注册");
                    pb::send_proto(task.sock, task.addr, task.proto, pb::login::CsRspLogin{result: false,error_code: 10103});
                } else {
                    println!("账号不存在, 可以注册");
                    if sqlite3::data::insert_row("users", "account, password", "?, ?", |statement: &mut sqlite::Statement| {
                        statement.bind((1, msg.account as i64)).expect("state.bind");
                        statement.bind((2, msg.passwword.as_str())).expect("state.bind");
                    }) {
                        println!("注册成功");
                        pb::send_proto(task.sock, task.addr, 10004, pb::login::CsRspLogin{result: true,error_code: 10101});
                    } else {
                        println!("注册失败");
                        pb::send_proto(task.sock, task.addr, 10004, pb::login::CsRspLogin{result: false,error_code: 10102});
                    }
                }
            }
            _ => println!("anonym_msg: undefined proto !!!")
        }
    }
}