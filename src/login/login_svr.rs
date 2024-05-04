use std::sync::Weak;
use std::sync::Mutex;
use crate::center::center_svr::center_svr;


/**
 * @file login_svr.rs
 * @brief 登录服务器
 * @author zys
 * @date Thu May 02 2024 22:17:10 GMT+0800 (中国标准时间)
 * @version 0.1
 */

pub struct login_svr {
    name: String,
    center_svr: Option<Weak<Mutex<center_svr>>>,
}

impl login_svr {
    pub fn new(name: String) -> login_svr {
        return login_svr {
            name: name,
            center_svr: None,
        };
    }
    /**
     * @brief init
     */
    pub fn begin_login(&mut self, center_svr: Weak<Mutex<center_svr>>) {
        self.center_svr = Some(Weak::clone(&center_svr));
    }
    pub fn push_new_proxy(&self) {
        
    }
}