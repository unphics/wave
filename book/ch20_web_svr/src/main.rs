use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    println!("Hello, world!");

    // bind返回Result<T, E>, 绑定是会失败的, 比如90端口需要管理员权限(非管理员必须高于1023)
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // incoming返回TcpListener的一个迭代器, 提供一系列流, 流表示客户端和服务端之间的开发链接
    // 如果有任何错误, unwrap将终止程序
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_conn(stream);
        println!("connect established"); // established已建立的
    }
}

fn handle_conn(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_req: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("req: {:#?}", http_req);
}