use std::thread;

mod udp;
mod pb;
mod gate;
mod cfg;
mod bot;
fn main() {
    println!("====== wave begin ======");
    // pb::example();
    // udp::udp_chat::udp_chat();

    let handle = thread::spawn(|| {
        let mut gate_svr = gate::gate_svr::gate_svr::new("gate".to_string());
        gate_svr.begin_listen();
    });

    // 跑机器人
    bot::run_bot(bot::bot::bot_01, 1);

    handle.join();
}