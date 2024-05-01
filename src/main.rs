mod udp;
mod pb;
mod gate;
fn main() {
    println!("====== wave begin ======");
    // pb::example();
    udp::udp_chat::udp_chat();
}