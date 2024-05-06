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
    let saddr = cfg::SERVER_ADDR.clone().parse::<SocketAddr>().unwrap();
    pb::send_proto(sock.try_clone().unwrap(), saddr.clone(), 10001, pb::gate::CsReqLogin {account: 11111, passwword: "11111".to_string()});
    println!("bot_login send CsReqLogin");
    // 收听结果
    let mut buf: [u8; 1024] = [0u8; cfg::LISTEN_BUF_SIZE];
    let (size, addr) = sock.recv_from(&mut buf).expect("login_bot::bot_login()");
    let (proto, bytes) = pb::unpack_msg(&mut buf, size);
    println!("收到svr回包, proto = {}", proto);
    if proto != 10002 {
        println!("fatal: login recv must be 10002 !!!");
        return;
    }
    let msg = pb::gate::CsRspLogin::decode(bytes.as_slice()).expect("failed to decode msg");
    println!("result = {}, err_code = {}, {}", msg.result, msg.error_code, if msg.result  {"登录成功"} else {"登录失败"});
    return;
}
/**
 * @brief 测试注册
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