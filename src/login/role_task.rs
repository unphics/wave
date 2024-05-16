
use crate::{pb::role, proxy::proxy::proxy};
pub struct role_task {
    pub pb_bytes: Vec<u8>,
    pub proxy: *mut proxy,
    pub proto: u16,
}
impl role_task {
    pub fn new(proxy: *mut proxy, proto: u16, pb_bytes: Vec<u8>) -> Self {
        return Self {
            pb_bytes:pb_bytes,
            proxy: proxy,
            proto: proto,
        };
    }
}