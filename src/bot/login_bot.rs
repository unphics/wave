use prost::Message;
use crate::cfg;
use crate::pb::role;
use std::net::SocketAddr;
use std::net::UdpSocket;
use crate::pb;

pub const TEST_ACCOUNT: i32 = 11111;
pub const TEST_PASSWORD: &str = "11111";

/**
 * @brief 测试登录
 */
pub fn bot_login() -> Option<(UdpSocket, SocketAddr)> {
    let sock = UdpSocket::bind(String::from(cfg::BOT_01_ADDR)).expect("failed to bind addr");
    let saddr = cfg::SERVER_ADDR.clone().parse::<SocketAddr>().unwrap();
    pb::send_proto(sock.try_clone().unwrap(), saddr.clone(), 10001, TEST_ACCOUNT, pb::login::CsReqLogin {account: TEST_ACCOUNT, passwword: TEST_PASSWORD.to_string()});
    println!("bot_login send CsReqLogin");
    // 收听结果
    let mut buf: [u8; 1024] = [0u8; cfg::LISTEN_BUF_SIZE];
    let (size, addr) = sock.recv_from(&mut buf).expect("login_bot::bot_login()");
    let (proto, _, bytes) = pb::unpack_msg(&mut buf, size);
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
    pb::send_proto(sock.try_clone().unwrap(), saddr.clone(), 10003, TEST_ACCOUNT, pb::login::CsReqRegister {account: TEST_ACCOUNT, passwword: TEST_PASSWORD.to_string()});
    println!("bot_register: send CsReqRegister");
    // 收听结果
    let mut buf: [u8; 1024] = [0u8; cfg::LISTEN_BUF_SIZE];
    let (size, addr) = sock.recv_from(&mut buf).expect("login_bot::bot_register()");
    let (proto, _, bytes) = pb::unpack_msg(&mut buf, size);
    println!("收到svr回包, proto = {}", proto);
    if proto != 10004 {
        println!("fatal: register recv must be 10004 !!!");
        return None;
    }
    let msg = pb::login::CsRspRegister::decode(bytes.as_slice()).unwrap();
    println!("result = {}, err_code = {}, {}", msg.result, msg.error_code, if msg.result  {"注册成功"} else {"注册失败"});
    return Some((sock, saddr));
}
pub fn bot_role_list() -> Option<(UdpSocket, SocketAddr)>  {
    let (sock,saddr) = bot_login().unwrap();
    println!("bot_select_role: begin");
    // 先获取角色列表
    pb::send_proto(sock.try_clone().unwrap(), saddr, 10101, TEST_ACCOUNT, pb::login::CsReqOwnerRoleSelectIntroList {account: TEST_ACCOUNT});
    // 收听结果
    let mut buf: [u8; 1024] = [0u8; cfg::LISTEN_BUF_SIZE];
    let (size, addr) = sock.recv_from(&mut buf).expect("login_bot::bot_register()");
    let (proto, _, bytes) = pb::unpack_msg(&mut buf, size);
    if proto != 10102 {
        println!("fatal: role select intro list must be 10102 !!!");
        return None;
    }
    let msg = pb::login::CsRspOwnerRoleSelectIntroList::decode(bytes.as_slice()).unwrap();
    let role_count = msg.intro_list.len();
    println!("bot_role_list: result = {}", role_count);

    return Some((sock, saddr));
}
