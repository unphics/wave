/**
 * @file main.rs
 * @brief 程序入口
 * @author zys
 * @date Thu May 02 2024 03:43:18 GMT+0800 (中国标准时间)
 * @version 0.2
 */
use std::thread;

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
        center_svr.run_center();
    });

    // 跑机器人
    // bot::run_bot(bot::gate_bot::bot_01, 1);
    bot::run_bot(bot::gate_bot::bot_login, 1);
    // bot::run_bot(bot::gate_bot::bot_register, 1);
    
    handle.join().expect("");
}