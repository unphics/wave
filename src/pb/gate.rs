/// 客户端请求登录
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsReqLogin {
    /// 账号
    #[prost(int32, tag="1")]
    pub account: i32,
    /// 密码
    #[prost(string, tag="2")]
    pub passwword: ::prost::alloc::string::String,
}
/// 服务端回复登录
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsRspLogin {
    /// 登录结果
    #[prost(bool, tag="1")]
    pub result: bool,
    /// 错误码(如有)
    #[prost(int32, tag="2")]
    pub error_code: i32,
}
/// 客户端请求注册
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsReqRegister {
    /// 账号
    #[prost(int32, tag="1")]
    pub account: i32,
    /// 密码
    #[prost(string, tag="2")]
    pub passwword: ::prost::alloc::string::String,
}
/// 服务端回复注册
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsRspRegister {
    /// 注册结果
    #[prost(bool, tag="1")]
    pub result: bool,
    /// 错误码(如有)
    #[prost(int32, tag="2")]
    pub error_code: i32,
}
/// 客户端请求版本一致性检查
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsReqVersionCheck {
    #[prost(string, tag="1")]
    pub code: ::prost::alloc::string::String,
}
/// 服务端回复版本一致性检查
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsRspVersionCheck {
    #[prost(int32, tag="1")]
    pub error_code: i32,
}
/// 网关服务器协议
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GateMsgGate {
    #[prost(oneof="gate_msg_gate::Data", tags="1, 2, 3, 4, 5, 6")]
    pub data: ::core::option::Option<gate_msg_gate::Data>,
}
/// Nested message and enum types in `GateMsgGate`.
pub mod gate_msg_gate {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag="1")]
        CsReqLogin(super::CsReqLogin),
        #[prost(message, tag="2")]
        CsRspLogin(super::CsRspLogin),
        #[prost(message, tag="3")]
        CsReqRegister(super::CsReqRegister),
        #[prost(message, tag="4")]
        CsRspRegister(super::CsRspRegister),
        #[prost(message, tag="5")]
        CsReqVersionCheck(super::CsReqVersionCheck),
        #[prost(message, tag="6")]
        CsRspVersionCheck(super::CsRspVersionCheck),
    }
}
