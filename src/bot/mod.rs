pub mod bot;

use std::thread;
use std::time;

pub fn run_bot(f: fn(), delay_time: u64) {
    let thd_handle = std::thread::spawn(move || {
        let sleep_duration = time::Duration::from_secs(delay_time);
        thread::sleep(sleep_duration);
        f();
        f();
        f();
    });
    // thd_handle.join();
}