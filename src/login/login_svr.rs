
use std::collections::HashMap;
use std::sync::Condvar;
use std::sync::Mutex;
use std::collections::VecDeque;
use crate::alloc;
use crate::center::center_svr::center_svr;
use crate::pb;
use crate::proxy::proxy::proxy;
use crate::sqlite3;
use std::net::UdpSocket;
use prost::Message;
use sqlite::State;
use crate::login::anonym_task::anonym_task;
use crate::error::ResultExt;

use super::role_task::role_task;
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
    proxys: HashMap<i32, *mut proxy>,
    mutex: Mutex<u8>,
    cond: Condvar,
    stop: bool,
    anonym_queue: Mutex<VecDeque<anonym_task>>,
    role_queue: Mutex<VecDeque<role_task>>
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
            anonym_queue: Mutex::new(VecDeque::new()),
            role_queue: Mutex::new(VecDeque::new()),
        };
    }
    pub fn run_login(&mut self) {
        while !self.stop {
            let lock = self.mutex.lock().unwrap();
            self.cond.wait_while(lock, |_| {
                if self.anonym_queue.lock().unwrap().len() > 0 {
                    return false;
                } else if self.role_queue.lock().unwrap().len() > 0 {
                    return false;
                }
                return true;
            }).unwrap();
            println!("login: run");
            self.work();
        }
    }
    pub fn work(&mut self) {
        self.deal_with_role();
        self.deal_with_anonym();
    }
    pub fn send_anonym(&mut self, sock: UdpSocket, caddr: std::net::SocketAddr, proto: u16, pb_bytes: Vec<u8>) {
        let task = anonym_task::new(sock, caddr, proto, pb_bytes);
        self.anonym_queue.lock().unwrap().push_back(task);
        self.cond.notify_one();
    }
    pub fn send_role(&mut self, proxy: *mut proxy, proto: u16, pb_bytes: Vec<u8>) {
        let task = role_task::new(proxy, proto, pb_bytes);
        self.role_queue.lock().unwrap().push_back(task);
        self.cond.notify_one();
    }
    /**
     * @brief 创建客户端代理(当登录成功后), 保持此代理, 并将代理同步注册到网关服务器
     */
    fn create_proxy(&mut self, sock: UdpSocket, addr: std::net::SocketAddr, account: i32) {
        let p_proxy = alloc::malloc(proxy::new(addr, account, sock));
        self.proxys.insert(account, p_proxy);
        let proxy = alloc::deref(p_proxy);
        proxy.set_login(self);

        let center = alloc::deref(self.center_svr);
        alloc::deref(center.get_gate()).on_login(p_proxy);
    }
    pub fn deal_with_role(&mut self) {
        if !(self.role_queue.lock().unwrap().len() > 0) {
            return;
        }
        let role_task = self.role_queue.lock().unwrap().pop_front().unwrap();
        println!("role_task.proto {}", role_task.proto);
        // todo: 角色相关的还有好多好多情况没处理, 等以后有心情再写, 先把主要流程写了
        match role_task.proto {
            10101 => {
                let conn = sqlite::open("sqlite/wave_data.db").expect("sqlite::open");
                let query = format!("select role_0, role_1, role_2 from users where account = ?", );
                let mut statement = conn.prepare(query).expect("conn.prepare");
                let proxy = alloc::deref(role_task.proxy);
                statement.bind((1, proxy.account() as i64)).map_err(|e| e.to_string()).expect("");
                match statement.next() {
                    Ok(State::Row) => {
                        let mut row_data = Vec::new();
                        row_data.push(statement.read::<i64, _>(0).handle());
                        row_data.push(statement.read::<i64, _>(1).handle());
                        row_data.push(statement.read::<i64, _>(2).map_err(|e| e.to_string()).unwrap());
                        println!("读表结果; role1 = {}, role2 = {}, role3 = {}", row_data[0], row_data[1], row_data[2]);
                        let mut intro_list = Vec::new();
                        for role_id in row_data {
                            if role_id > 0 {
                                intro_list.push(pb::role::RoleSelectIntro{role_id: role_id as i32, name: "ww".to_string()});
                            }
                        }
                        let obj_pb = pb::login::CsRspOwnerRoleSelectIntroList{error_code: 10101, intro_list: intro_list};
                        pb::send_proto(proxy.sock() ,proxy.addr(), 10102, proxy.account(), obj_pb);
                    }
                    _ => println!("no role list"),
                }
            }
            10103 => {
                let msg = pb::login::CsReqCreateRole::decode(role_task.pb_bytes.as_slice()).handle();
                let proxy = alloc::deref(role_task.proxy);
                // todo: role_id的值得算出来
                let role_id = sqlite3::data::get_row_count("role") + 1;
                let info = msg.info.unwrap();
                if sqlite3::data::insert_row("role", "role_id, name", "?, ?", |statement| {
                    statement.bind((1, role_id as i64)).handle();
                    statement.bind((2, info.name.as_str())).handle();
                }) {
                    sqlite3::data::modify_field_val("users", "account", proxy.account() as u32, "role_0", role_id.to_string().as_str());
                    println!("login: 创建成功");
                    // todo: 现在强制把创建的角色放到该账户的第一个位置, 后面根据role_list的加载策略更正
                    let obj_pb = pb::login::CsRspCreateRole{error_code: 1, role_id: role_id as i32};
                    pb::send_proto(proxy.sock() ,proxy.addr(), 10104, proxy.account(), obj_pb);
                } else {
                    println!("login: 创建失败");
                }
            }
            10105 => {
                let proxy = alloc::deref(role_task.proxy);
                let msg = pb::login::CsReqSelectRole::decode(role_task.pb_bytes.as_slice()).handle();
                let role_id = msg.role_id;
                // todo 加上验证等等
                // todo last: 选择角色在proxy中的创建role的逻辑
                let obj_pb = pb::login::CsRspSelectRole{error_code: 1};
                pb::send_proto(proxy.sock() ,proxy.addr(), 10106, proxy.account(), obj_pb);
            }
            _ => println!("undefined proto: {} !!!", role_task.proto)
        }
    }
    pub fn deal_with_anonym(&mut self) {
        if !(self.anonym_queue.lock().unwrap().len() > 0) {
            return;
        }
        let anonym = self.anonym_queue.lock().unwrap().pop_front().unwrap();
        match anonym.proto {
            10001 => {
                let msg = pb::login::CsReqLogin::decode(anonym.pb_bytes.as_slice()).expect("failed to decodelogin proto");
                println!("client request login: {:?}", msg);

                if sqlite3::data::exit_row("users", msg.account as i64) {
                    println!("账号存在, 登录成功");
                    // todo 忘记验证密码了, 而且也没验证已登录
                    pb::send_proto(anonym.sock.try_clone().handle(), anonym.caddr, 10002, msg.account, pb::login::CsRspLogin{result: true,error_code: 10001});
                    self.create_proxy(anonym.sock, anonym.caddr, msg.account);
                } else {
                    println!("账号不存在, 需要注册");
                    pb::send_proto(anonym.sock, anonym.caddr, 10002, msg.account, pb::login::CsRspLogin{result: false,error_code: 10002});
                }
            }
            10003 => {
                let msg = pb::login::CsReqRegister::decode(anonym.pb_bytes.as_slice()).expect("failed to decodelogin proto");
                println!("client request register: {:?}", msg);
                if sqlite3::data::exit_row("users", msg.account as i64) {
                    println!("账号已存在, 不需要注册");
                    pb::send_proto(anonym.sock, anonym.caddr, 10004, msg.account, pb::login::CsRspLogin{result: false,error_code: 10103});
                } else {
                    println!("账号不存在, 可以注册");
                    if sqlite3::data::insert_row("users", "account, password", "?, ?", |statement: &mut sqlite::Statement| {
                        statement.bind((1, msg.account as i64)).expect("state.bind");
                        statement.bind((2, msg.passwword.as_str())).expect("state.bind");
                    }) {
                        println!("注册成功");
                        pb::send_proto(anonym.sock, anonym.caddr, 10004, msg.account, pb::login::CsRspLogin{result: true,error_code: 10101});
                    } else {
                        println!("注册失败");
                        pb::send_proto(anonym.sock, anonym.caddr, 10004, msg.account, pb::login::CsRspLogin{result: false,error_code: 10102});
                    }
                }
            }
            _ => println!("anonym_msg: undefined proto !!!")
        }
    }
}