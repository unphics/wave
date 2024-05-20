/**
 * @file main.rs
 * @brief 程序入口
 * @author zys
 * @date Thu May 02 2024 03:43:18 GMT+0800 (中国标准时间)
 * @version 0.2
 */
use std::thread;
use alloc::{free, malloc};
use center::center_svr::center_svr;

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
fn main() {
    println!("====== wave begin ======");
    // pb::example();
    // udp::udp_chat::udp_chat();
    wave_svr_run();

    // const LEN_USIZE: usize = std::mem::size_of::<usize>();
    // const LEN_U16: usize = std::mem::size_of::<u16>();
    // const LEN_I32: usize = std::mem::size_of::<i32>();
    // println!("len_usize = {}, len_u16 = {}, len_i32 = {}, another = {}", LEN_USIZE, LEN_U16, LEN_I32, std::mem::size_of::<bool>());
}

fn wave_svr_run() {
    let p_center = malloc(center_svr::new("center".to_string()));
    println!("&center = {:p}", p_center);
    let move_center = p_center.clone() as usize; // usize的尺寸和ptr相同, 此处不可用u8
    let handle = thread::spawn(move || {
        alloc::deref(move_center as *mut center_svr).run_center();
    });
    bot::run_bot(bot::login_bot::bot_role_select, 1);
    handle.join().expect("center.join");
    free(p_center);
}