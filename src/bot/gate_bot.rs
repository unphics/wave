
use crate::cfg;
use std::net::UdpSocket;

/**
 * @file gate_bot.rs
 * @breif 网关服务器测试机器人
 * @author zys
 * @date Thu May 02 2024 03:43:18 GMT+0800 (中国标准时间)
 * @version 0.2
 */

/**
 * @brief 测试100000个udp包丢包率
 * @record 100000个udp包, 去掉那行print丢包率40%, 加上那行print丢包率0.2%; 1000个包, 结果同上, 故不宜发包过快
 */
pub fn bot_01() {
    for _ in 0 .. 1000 {
        let sock = UdpSocket::bind(String::from(cfg::BOT_01_ADDR)).expect("failed to bind addr");
        let msg = String::from("bot_01 : qqqqqqqqqq");
        let len = msg.len();
        let mut pb_bytes = Vec::new();
        let proto: u16 = 1001;
        pb_bytes.extend_from_slice(&len.to_be_bytes());
        pb_bytes.extend_from_slice(&proto.to_be_bytes());
        pb_bytes.extend_from_slice(msg.as_bytes());
        sock.send_to(&pb_bytes, cfg::SERVER_ADDR).expect("failed to send msg");
        println!("bot_01 send msg: {}", msg);
    }
}