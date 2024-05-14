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
pub fn bot_login() -> Option<(UdpSocket, SocketAddr)> {
    let sock = UdpSocket::bind(String::from(cfg::BOT_01_ADDR)).expect("failed to bind addr");
    let saddr = cfg::SERVER_ADDR.clone().parse::<SocketAddr>().unwrap();
    pb::send_proto(sock.try_clone().unwrap(), saddr.clone(), 10001, pb::login::CsReqLogin {account: 11111, passwword: "11111".to_string()});
    println!("bot_login send CsReqLogin");
    // 收听结果
    let mut buf: [u8; 1024] = [0u8; cfg::LISTEN_BUF_SIZE];
    let (size, addr) = sock.recv_from(&mut buf).expect("login_bot::bot_login()");
    let (proto, bytes) = pb::unpack_msg(&mut buf, size);
    println!("收到svr回包, proto = {}", proto);
    if proto != 10002 {
        println!("fatal: login recv must be 10002 !!!");
        return None;
    }
    let msg = pb::login::CsRspLogin::decode(bytes.as_slice()).expect("failed to decode msg");
    println!("result = {}, err_code = {}, {}", msg.result, msg.error_code, if msg.result  {"登录成功"} else {"登录失败"});
    return Some((sock, saddr));
}
/**
 * @brief 测试注册
 */
pub fn bot_register() -> Option<(UdpSocket, SocketAddr)>  {
    let sock = UdpSocket::bind(String::from(cfg::BOT_01_ADDR)).expect("failed to bind addr");
    let saddr = cfg::SERVER_ADDR.clone().parse::<SocketAddr>().unwrap();
    pb::send_proto(sock.try_clone().unwrap(), saddr.clone(), 10003, pb::login::CsReqRegister {account: 33333, passwword: "33333".to_string()});
    println!("bot_register: send CsReqRegister");
    // 收听结果
    let mut buf: [u8; 1024] = [0u8; cfg::LISTEN_BUF_SIZE];
    let (size, addr) = sock.recv_from(&mut buf).expect("login_bot::bot_register()");
    let (proto, bytes) = pb::unpack_msg(&mut buf, size);
    println!("收到svr回包, proto = {}", proto);
    if proto != 10004 {
        println!("fatal: register recv must be 10004 !!!");
        return None;
    }
    let msg = pb::login::CsRspRegister::decode(bytes.as_slice()).unwrap();
    println!("result = {}, err_code = {}, {}", msg.result, msg.error_code, if msg.result  {"注册成功"} else {"注册失败"});
    return Some((sock, saddr));
}
pub fn bot_select_role() -> Option<(UdpSocket, SocketAddr)>  {
    let (sock,saddr) = bot_login().unwrap();

    return None;
}