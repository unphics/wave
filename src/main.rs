/**
 * @file main.rs
 * @brief 程序入口
 * @author zys
 * @date Thu May 02 2024 03:43:18 GMT+0800 (中国标准时间)
 * @version 0.2
 */
use std::{net::UdpSocket, thread};
use center::center_svr::center_svr;

use crate::svr::base;

mod error;
mod center;
mod udp;
mod pb;
mod gate;
mod cfg;
mod bot;
mod sqlite3;
mod login;
mod proxy;
mod alloc;
mod role;
mod scene;
mod recast;
mod svr;
mod math;

use my_macro;
fn main() {
    println!("====== wave begin ======");
    // pb::example();
    // udp::udp_chat::udp_chat();
    // wave_run();
}

fn wave_run() {
    let p_center = svr::create::<center_svr>("center".to_string());
    println!("&center = {:p}", p_center);
    let move_center = p_center.clone() as usize; // usize的尺寸和ptr相同, 此处不可用u8
    let handle = thread::spawn(move || {
        alloc::deref(move_center as *mut center_svr).run();
    });
    bot::run_bot(bot::login_bot::bot_role_select, 1);
    // handle.join().expect("center.join");
    wait_cmd();
    alloc::deref(move_center as *mut center_svr).shutdown();
    svr::destroy(p_center);
}
fn wait_cmd() {
    println!("wait cmd ...");
    loop {
        let sock = UdpSocket::bind(String::from("127.0.0.1:9666"));
        let mut buf: [u8; 1024] = [0u8; 1024];
        let (size, caddr) = sock.as_ref().unwrap().recv_from(&mut buf).unwrap();
        let proto = u16::from_be_bytes(buf[0 .. std::mem::size_of::<u16>()].try_into().unwrap());
        println!("recv cmd: {}", proto);
        match proto {
            1001 => break,
            _ => println!("undefined cmd: {}", proto)
        }
    }
}