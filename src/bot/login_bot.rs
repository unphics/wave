use prost::Message;
use crate::cfg;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::net::ToSocketAddrs;
use crate::pb;
use std::net::{IpAddr, Ipv4Addr};
/**
 * @brief 测试登录
 */
pub fn bot_login() {
    let sock = UdpSocket::bind(String::from(cfg::BOT_01_ADDR)).expect("failed to bind addr");
    let addr = cfg::SERVER_ADDR.clone().parse::<SocketAddr>().unwrap();
    pb::send_proto(sock.try_clone().unwrap(), addr.clone(), 10001, pb::gate::CsReqLogin {account: 11111, passwword: "11111".to_string()});
    println!("bot_login send CsReqLogin");
    // todo last 收听结果
}

/**
 * 测试注册
 */
pub fn bot_register() {
    let sock = UdpSocket::bind(String::from(cfg::BOT_01_ADDR)).expect("failed to bind addr");
    let addr = cfg::SERVER_ADDR.clone().parse::<SocketAddr>().unwrap();
    pb::send_proto(sock.try_clone().unwrap(), addr.clone(), 10003, pb::gate::CsReqRegister {account: 22222, passwword: "22222".to_string()});
    println!("bot_register: send CsReqRegister");
    // todo last 收听结果
}
pub fn bot_select_role() {

}