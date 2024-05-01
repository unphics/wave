/**
 * @file udp_chat.rs
 * @brief udp 示例
 * @author zys
 * @date Thu May 02 2024 03:43:18 GMT+0800 (中国标准时间)
 * @version 0.1
 */
use std::net::UdpSocket;
use std::net::ToSocketAddrs;
use std::thread;
use std::io;

pub fn udp_chat() {
    println!("udp chat app");

    let mut cnt = 0;
    let (mut src, mut tar) = (None, None);
    let mut name = String::from("user_default");
    for arg in std::env::args() {
        if 1 == cnt {
            src = Some(arg);
        } else if 2 == cnt {
            tar = Some(arg);
        } else if 3 == cnt {
            name = String::from(arg);
        }
        cnt += 1;
    }
    if let (Some(v1), Some(v2)) = (src, tar) {
        create(v1, v2, name);
    }
}

fn create<A: ToSocketAddrs>(src: A, tar: String, name: String) {
    println!("create a chat app");
    let sock = UdpSocket::bind(src);
    if sock.is_err() {
        return;
    }
    let sock = sock.unwrap();
    // if sock.connect(tar).is_err() {
    //     return;
    // }
    let lis_tk = sock.try_clone().unwrap();
    let rep_tk = sock.try_clone().unwrap();
    let n1 = name.clone();
    let n2 = name.clone();
    let handle_lis = thread::spawn(move || {
        listen(lis_tk, n1)
    });
    let handle_rep = thread::spawn(move || {
        replys(rep_tk, n2, tar)
    });
    handle_lis.join();
    handle_rep.join();
}

fn listen(sock: UdpSocket, name: String) {
    loop {
        let mut buf = [0u8; 256];
        let (amt, _) = sock.recv_from(&mut buf).unwrap();
        let buf = &mut buf[..amt];
        let mut info = String::new();
        for v in buf {
            if '\n'== (*v as char) || '\r' == (*v as char) {
                continue;
            }
            info.push(*v as char);
        }
        println!("[{}] recv info : {}", name, info);
    }
}

fn replys(sock: UdpSocket, name: String, tar: String) {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let sinfo = format!("[{}] : {}", name, input);
        let tar_addr = tar.clone();
        if sock.send_to(sinfo.as_bytes(), tar_addr).is_err() {
            continue;
        }
        println!("[{}] send info : {}", name, input);
    }
}