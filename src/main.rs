mod udp;
mod pb;
fn main() {
    println!("====== wave begin ======");
    // udp::udp_chat::udp_chat();

    let req = pb::hello::HelloRequest{
        name: "hello".to_string(),
    };
    println!("request: {:?}", req);
}