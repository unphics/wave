/**
 * @file center_svr.rs
 * @brief 中心服务器
 * @author zys
 * @date Thu May 02 2024 03:56:57 GMT+0800 (中国标准时间)
 * @version 0.1
 */
use std::net::UdpSocket;
use std::option;
use std::rc::Rc;
use std::sync::Weak;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::thread;
use tokio::runtime::Handle;
use std::time;

use crate::gate;
use crate::login;
pub struct center_svr {
    name: String,
    sock: Option<UdpSocket>,
    gate_svr: Option<Arc<Mutex<gate::gate_svr::gate_svr>>>,
    login_svr: Option<Arc<Mutex<login::login_svr::login_svr>>>,
    stop: bool,
    mutex: Mutex<()>,
    cond: Condvar,
}

impl center_svr {
    pub fn new(name: String) -> Arc<Mutex<Self>> {
        return Arc::new(Mutex::new(center_svr {
            name: name,
            sock: None,
            gate_svr: None,
            login_svr: None,
            stop: false,
            mutex: Mutex::new(()),
            cond: Condvar::new(),
        }));
    }
    pub fn run_center(&mut self) {
        while self.stop != true {
            let mut lock = self.mutex.lock().expect("failed to lock");
            self.cond.wait(lock);
        }
    }
    pub fn tick(&self) {
        println!("tick");
    }
    pub fn shutdonw(&mut self) {
        if self.stop == true {
            return;
        }
        self.stop = true;
        self.cond.notify_all();
    }
    /**
     * 倒数第二步, 启动网关服务器
     */
    pub fn run_gate(this: Arc<Mutex<center_svr>>) {
        let gate = Arc::new(Mutex::new(gate::gate_svr::gate_svr::new("gate".to_string())));
        {
            this.lock().expect("111").gate_svr = Some(Arc::clone(&gate));
        }
        {
            this.lock().unwrap();
        }
        let weak_center = Arc::downgrade(&this);
        let handle = thread::spawn(move || {
            gate.lock().expect("222").begin_listen(weak_center);
        });
    }
    /**
     * @brief 启动登录服务器
     */
    pub fn run_login(this: Arc<Mutex<center_svr>>) {
        let login = Arc::new(Mutex::new(login::login_svr::login_svr::new("login".to_string())));
        {
            this.lock().expect("set center.login").login_svr = Some(Arc::clone(&login));
        }
        let weak_center = Arc::downgrade(&this);
        let handle = thread::spawn(move || {
            login.lock().expect("login begin").begin_login(weak_center);
        });
    }
    pub fn route_login(&self) -> Option<Weak<Mutex<login::login_svr::login_svr>>> {
        if let Some(login) = &self.login_svr {
            let weak = Arc::downgrade(&login);
            return Some(weak);
        }
        return None;
    }
}
/*
gate_svr和login_svr还有proxy怎么设计呢？
比如说gate收到一个新客户端的登录申请，那么
gate创建一个shared_proxy索引住并发给login
然后proxy里的current_svr=login,然后登录
申请的协议内容发给proxy，然后proxy找到自
己的current_svr也就是login处理登录事务

比如说有gate_svr,login_svr,scene_svr,logic_svr等等
question:
    proxy同时只存在于一个svr吗(登录后网关服务器,场景服务器,和场景无关的纯逻辑服务器等可以同时处理一个角色)
        答:存在多个svr, svr的weak保存在proxy中
    gate收到的消息发给proxy, proxy转发给具体svr, 还是gate收到消息带着account_id发给具体svr
        答:发给proxy, proxy转发给具体svr; gate中保存所有proxy
    心跳网上都是做在gate里, 但是感觉做在proxy里也可以吧
        答:做在proxy里, 由有tick能力的svr执行
*/