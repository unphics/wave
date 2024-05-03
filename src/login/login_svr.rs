
/**
 * @file login_svr.rs
 * @brief 登录服务器
 * @author zys
 * @date Thu May 02 2024 22:17:10 GMT+0800 (中国标准时间)
 * @version 0.1
 */

struct login_svr {
    name: String,
}

impl login_svr {
    pub fn new(name: String) -> login_svr {
        return login_svr {
            name: name,
        };
    }
    pub fn push_new_proxy(&self) {
        
    }
}