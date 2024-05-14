/**
 * @file main.rs
 * @brief 程序入口
 * @author zys
 * @date Thu May 02 2024 03:43:18 GMT+0800 (中国标准时间)
 * @version 0.2
 */
use std::sync::Arc;
use std::{thread, time};
use alloc::{free, malloc};
use center::center_svr::center_svr;
use std::alloc::{alloc, dealloc, Layout};

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
fn main() {
    println!("====== wave begin ======");
    // pb::example();
    // udp::udp_chat::udp_chat();
    wave_svr_run();
}

fn wave_svr_run() {
    // let handle = thread::spawn(|| {
    //     let mut center_svr = center::center_svr::center_svr::new("center".to_string());
    //     center_svr::run_login(Arc::clone(&center_svr));
    //     center_svr::run_gate(Arc::clone(&center_svr));
    //     loop {
    //         center_svr.lock().unwrap().tick();
    //         let sleep_duration = time::Duration::from_secs(1);
    //         thread::sleep(sleep_duration);
    //     }
    // });
    // 跑一下机器人
    // bot::run_bot(bot::login_bot::bot_login, 1);
    // handle.join().expect("");

    let p_center = malloc(center_svr::new("center".to_string()));
    println!("&center = {:p}", p_center);
    let move_center = p_center.clone() as usize; // usize的尺寸和ptr相同, 此处不可用u8
    let handle = thread::spawn(move || {
        alloc::deref(move_center as *mut center_svr).run_center();
    });
    bot::run_bot(bot::login_bot::bot_login, 1);
    // alloc::deref(p_center).notify();
    handle.join().expect("center.join");
    free(p_center);
}