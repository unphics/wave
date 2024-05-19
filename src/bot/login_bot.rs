use prost::Message;
use crate::cfg;
use crate::error::ResultExt;
use crate::pb::role;
use crate::pb::role::RoleCreateInfo;
use crate::pb::role::RoleSelectIntro;
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
    let (size, _) = sock.recv_from(&mut buf).expect("login_bot::bot_login()");
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
    let (size, _) = sock.recv_from(&mut buf).expect("login_bot::bot_register()");
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
/**
 * @brief 测试角色选择整个流程
 */
pub fn bot_role_select() -> Option<(UdpSocket, SocketAddr)>  {
    let (sock,saddr) = bot_login().unwrap();
    println!("bot_select_role: begin");
    // 先获取角色列表
    pb::send_proto(sock.try_clone().unwrap(), saddr, 10101, TEST_ACCOUNT, pb::login::CsReqOwnerRoleSelectIntroList {account: TEST_ACCOUNT});
    // 收听角色列表
    let mut buf: [u8; 1024] = [0u8; cfg::LISTEN_BUF_SIZE];
    let (size, _) = sock.recv_from(&mut buf).expect("login_bot::bot_register()");
    let (proto, _, bytes) = pb::unpack_msg(&mut buf, size);
    if proto != 10102 {
        println!("fatal: role select intro list must be 10102 !!!");
        return None;
    }
    let msg = pb::login::CsRspOwnerRoleSelectIntroList::decode(bytes.as_slice()).unwrap();
    let role_count = msg.intro_list.len();
    for intro in &msg.intro_list {
        println!("bot_role_select: role: {}, name: {}", intro.role_id, intro.name);
    }
    println!("bot_role_select: result = {}", role_count);
    let mut role_id = 0;
    if !(role_count > 0) {
        // 如果没有, 则创建一个新角色
        pb::send_proto(sock.try_clone().handle(), saddr, 10103, TEST_ACCOUNT,
            pb::login::CsReqCreateRole{info: Some(RoleCreateInfo{name: "ikun".to_string()})});
        // 收听创建结果
        let mut buf: [u8; 1024] = [0u8; cfg::LISTEN_BUF_SIZE];
        let (size, _) = sock.recv_from(&mut buf).unwrap();
        let (proto, _, bytes) = pb::unpack_msg(&mut buf, size);
        let msg = pb::login::CsRspCreateRole::decode(bytes.as_slice()).unwrap();
        if proto != 10104 {
            println!("fatal: role create recv must be 10104 !!!");
            return None;
        }
        println!("role create result: {}, role_id: {}", msg.error_code, msg.role_id);
        role_id = msg.role_id;
    } else {
        role_id = msg.intro_list[0].role_id;
    }
    // 选择角色
    pb::send_proto(sock.try_clone().handle(), saddr, 10105, TEST_ACCOUNT,
        pb::login::CsReqSelectRole{role_id: role_id});
    // 监听选择结果
    let mut buf: [u8; 1024] = [0u8; cfg::LISTEN_BUF_SIZE];
    let (size, _) = sock.recv_from(&mut buf).unwrap();
    let (proto, _, bytes) = pb::unpack_msg(&mut buf, size);
    if proto != 10106 {
        println!("fatal: role select recv must be 10106 !!!");
        return None;
    }
    let msg = pb::login::CsRspSelectRole::decode(bytes.as_slice()).unwrap();
    println!("role select result: err_code = {}", msg.error_code);
    return Some((sock, saddr));
}
