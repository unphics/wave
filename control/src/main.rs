use std::net::UdpSocket;
fn main() {
    let sock = UdpSocket::bind(String::from("127.0.0.1:9553")).unwrap();
    let mut bytes:Vec<u8> = Vec::new();
    let proto: u16 = 1001;
    println!("send cmd: {}", proto);
    bytes.extend_from_slice(&proto.to_be_bytes());
    sock.send_to(&bytes, String::from("127.0.0.1:9666")).unwrap();
}
