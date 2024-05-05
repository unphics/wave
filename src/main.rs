/**
 * @file main.rs
 * @brief 程序入口
 * @author zys
 * @date Thu May 02 2024 03:43:18 GMT+0800 (中国标准时间)
 * @version 0.2
 */
use std::sync::Arc;
use std::{thread, time};
use center::center_svr::center_svr;

mod center;
mod udp;
mod pb;
mod gate;
mod cfg;
mod bot;
mod sqlite3;
mod login;
mod proxy;
fn main() {
    println!("====== wave begin ======");
    // pb::example();
    // udp::udp_chat::udp_chat();
    wave_svr_run();
}

fn wave_svr_run() {
    let handle = thread::spawn(|| {
        let mut center_svr = center::center_svr::center_svr::new("center".to_string());
        center_svr::run_login(Arc::clone(&center_svr));
        center_svr::run_gate(Arc::clone(&center_svr));
        loop {
            center_svr.lock().unwrap().tick();
            let sleep_duration = time::Duration::from_secs(1);
            thread::sleep(sleep_duration);
        }
    });

    // 跑机器人
    // bot::run_bot(bot::gate_bot::bot_01, 1);
    bot::run_bot(bot::login_bot::bot_login, 1);
    // bot::run_bot(bot::login_bot::bot_register, 1);
    
    handle.join().expect("");
}