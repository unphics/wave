use std::net::TcpStream;
// use std::net::TcpListener;
// use std::net::SocketAddr;

use std::io::Write;
fn main() {
    println!("client begin");
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    stream.write_all(b"hello").unwrap();
    println!("client end");
}
