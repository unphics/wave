/**
 * @file proxy
 * @brief 客户端代理
 * @author zys
 * @date
 * @version 0.1
 */
pub struct proxy {
    addr: String,
    account: u32,
}

impl proxy {
    pub fn new(addr: String) -> proxy {
        return proxy {
            addr: addr,
            account: 0,
        };
    }
}