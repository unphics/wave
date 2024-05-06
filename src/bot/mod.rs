/**
 * @file mod.rs
 * @brief bot模块
 * @author zys
 * @date Thu May 02 2024 03:43:18 GMT+0800 (中国标准时间)
 * @version 0.1
 */
pub mod gate_bot;
pub mod login_bot;

use std::net::SocketAddr;
use std::net::UdpSocket;
use std::thread;
use std::time;

pub fn run_bot(f: fn() -> Option<(UdpSocket, SocketAddr)>, delay_time: u64) {
    let _ = std::thread::spawn(move || {
        let sleep_duration = time::Duration::from_secs(delay_time);
        thread::sleep(sleep_duration);
        f().unwrap();
    });
}