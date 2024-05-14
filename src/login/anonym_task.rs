use std::net::SocketAddr;
use std::net::UdpSocket;
pub struct anonym_task {
    pub sock: UdpSocket,
    pub addr: SocketAddr,
    pub proto: u16,
    pub pb_bytes: Vec<u8>,
}
impl anonym_task {
    pub fn new(sock: UdpSocket, addr: SocketAddr, proto: u16, pb_bytes: Vec<u8>) -> Self {
        return anonym_task {
            sock: sock,
            addr: addr,
            proto: proto,
            pb_bytes: pb_bytes,
        };
    }
}