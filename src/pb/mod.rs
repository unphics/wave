use std::net::UdpSocket;

use prost::Message;
use crate::cfg;
pub mod hello;
pub mod role;
pub mod login;

/**
 * pb 示例
 */
pub fn example() {
        // const LEN_USIZE: usize = std::mem::size_of::<usize>();
        // const LEN_U16: usize = std::mem::size_of::<u16>();
        // const LEN_I32: usize = std::mem::size_of::<i32>();
        // println!("len_usize = {}, len_u16 = {}, len_i32 = {}, another = {}", LEN_USIZE, LEN_U16, LEN_I32, std::mem::size_of::<bool>());
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
/**
 * @brief 发送消息
 */
pub fn send_proto<T>(sock: UdpSocket, addr: std::net::SocketAddr, proto: u16, account: i32, obj_pb: T) where T: Message {
    sock.send_to(&serialize(proto, account, obj_pb), addr).unwrap();
}
/**
 * @brief 序列化包
 */
pub fn serialize<T>(proto: u16, account: i32, obj_pb: T) -> Vec<u8> where T: Message {
    let mut pb_bytes = Vec::new();
    obj_pb.encode(&mut pb_bytes).expect("failed to encode");
    let len = pb_bytes.len();

    let mut send_bytes = Vec::new();
    send_bytes.extend_from_slice(&len.to_be_bytes()); // 包头第一段 uize : 内容段的长度
    send_bytes.extend_from_slice(&proto.to_be_bytes()); // 包头第二段 u16 : 协议类型
    send_bytes.extend_from_slice(&account.to_be_bytes()); // 包头第三段 i32 : 账号(或也可用于其他用途)
    send_bytes.extend_from_slice(&pb_bytes); // 协议内容
    return send_bytes;
}
/**
 * @brief 解开网络包得到proto段和内容段
 */
const LEN_USIZE: usize = std::mem::size_of::<usize>();
const LEN_U16: usize = std::mem::size_of::<u16>();
const LEN_I32: usize = std::mem::size_of::<i32>();
pub fn unpack_msg(buf: &mut [u8; 1024], size: usize) ->(u16, i32, Vec<u8>) {
    let buf = &mut buf[.. size];
    // 协议包前usize是[内容大小段]
    let len = usize::from_be_bytes(buf[0 .. LEN_USIZE].try_into().unwrap());
    // 然后前u16是[协议类型段]
    let proto = u16::from_be_bytes(buf[LEN_USIZE .. LEN_USIZE + LEN_U16].try_into().unwrap());
    // 然后前i32是[账号段]
    let account: i32 = i32::from_be_bytes(buf[LEN_USIZE + LEN_U16 .. LEN_USIZE + LEN_U16 + LEN_I32].try_into().unwrap());
    println!("recv desc: len = {}, proto = {}, account = {}", len, proto, account);
    // 最后是[协议内容段]
    let mut pb_bytes = Vec::new();
    pb_bytes.extend_from_slice(&buf[LEN_USIZE + LEN_U16 + LEN_I32 .. size]);
    return (proto, account, pb_bytes);
}

