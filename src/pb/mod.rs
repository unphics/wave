use std::net::UdpSocket;

use prost::Message;

pub mod hello;
pub mod gate;

/**
 * pb 示例
 */
pub fn example() {
    let req = hello::HelloRequest {
        name: "hello world".to_string(),
    };
    println!("req: {:?}", req);

    let mut bytes = Vec::new();
    req.encode(&mut bytes).unwrap(); // 序列化
    let msg_decoded = hello::HelloRequest::decode(bytes.as_slice()).unwrap();
    println!("msg decoded: {:?}", msg_decoded);

    // 序列化反序列化
    let num: u32 = 1001;
    let str = "hello world";
    let mut byte_data: Vec<u8> = Vec::new();
    byte_data.extend_from_slice(&num.to_be_bytes());
    byte_data.extend_from_slice(str.as_bytes());
    println!("byte_data: {:?}", byte_data);
    let num_bytes:[u8; 4] = [byte_data[0], byte_data[1], byte_data[2], byte_data[3]];
    let de_num = u32::from_be_bytes(num_bytes);
    let de_str = std::str::from_utf8(&byte_data[4..]).unwrap();
    println!("de: num: {}, str: {}", de_num, de_str);
}

pub fn send_proto<T>(sock: UdpSocket, addr: std::net::SocketAddr, proto: u16, obj_pb: T) where T: Message {
    sock.send_to(&serialize(proto, obj_pb), addr).unwrap();
}

pub fn serialize<T>(proto: u16, obj_pb: T) -> Vec<u8> where T: Message {
    let mut pb_bytes = Vec::new();
    obj_pb.encode(&mut pb_bytes).expect("failed to encode");
    let len = pb_bytes.len();

    let mut send_bytes = Vec::new();
    send_bytes.extend_from_slice(&len.to_be_bytes());
    send_bytes.extend_from_slice(&proto.to_be_bytes());
    send_bytes.extend_from_slice(&pb_bytes);
    return send_bytes;
}