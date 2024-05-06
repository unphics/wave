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
pub struct LoginMsg {
    #[prost(oneof="login_msg::Data", tags="10001, 10002, 10003, 10004, 10005, 10006")]
    pub data: ::core::option::Option<login_msg::Data>,
}
/// Nested message and enum types in `LoginMsg`.
pub mod login_msg {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag="10001")]
        CsReqLogin(super::CsReqLogin),
        #[prost(message, tag="10002")]
        CsRspLogin(super::CsRspLogin),
        #[prost(message, tag="10003")]
        CsReqRegister(super::CsReqRegister),
        #[prost(message, tag="10004")]
        CsRspRegister(super::CsRspRegister),
        #[prost(message, tag="10005")]
        CsReqVersionCheck(super::CsReqVersionCheck),
        #[prost(message, tag="10006")]
        CsRspVersionCheck(super::CsRspVersionCheck),
    }
}
