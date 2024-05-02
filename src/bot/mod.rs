/**
 * @file mod.rs
 * @brief bot模块
 * @author zys
 * @date Thu May 02 2024 03:43:18 GMT+0800 (中国标准时间)
 * @version 0.1
 */
pub mod gate_bot;

use std::thread;
use std::time;

pub fn run_bot(f: fn(), delay_time: u64) {
    let thd_handle = std::thread::spawn(move || {
        let sleep_duration = time::Duration::from_secs(delay_time);
        thread::sleep(sleep_duration);
        for _ in 0 .. 100000 {
            f();
        }
    });
    // thd_handle.join();
}