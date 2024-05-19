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
/// 客户端请求拥有角色选角简介列表
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsReqOwnerRoleSelectIntroList {
    #[prost(int32, tag="1")]
    pub account: i32,
}
/// 服务端回复拥有角色选角简介列表
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsRspOwnerRoleSelectIntroList {
    #[prost(int32, tag="1")]
    pub error_code: i32,
    #[prost(message, repeated, tag="2")]
    pub intro_list: ::prost::alloc::vec::Vec<super::role::RoleSelectIntro>,
}
/// 客户端请求创建角色
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsReqCreateRole {
    #[prost(message, optional, tag="1")]
    pub info: ::core::option::Option<super::role::RoleCreateInfo>,
}
/// 服务端回复创建角色
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsRspCreateRole {
    #[prost(int32, tag="1")]
    pub error_code: i32,
    #[prost(int32, tag="2")]
    pub role_id: i32,
}
/// 客户端请求选择角色
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsReqSelectRole {
    #[prost(int32, tag="1")]
    pub role_id: i32,
}
/// 服务端回复选择角色
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsRspSelectRole {
    #[prost(int32, tag="1")]
    pub error_code: i32,
}
/// 网关服务器协议
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginMsg {
    #[prost(oneof="login_msg::Data", tags="10001, 10002, 10003, 10004, 10005, 10006, 10101, 10102, 10103, 10104, 10105, 10106")]
    pub data: ::core::option::Option<login_msg::Data>,
}
/// Nested message and enum types in `LoginMsg`.
pub mod login_msg {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        /// Anonym
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
        /// SelectRole
        #[prost(message, tag="10101")]
        CsReqOwnerRoleSelectIntroList(super::CsReqOwnerRoleSelectIntroList),
        #[prost(message, tag="10102")]
        CsRspOwnerRoleSelectIntroList(super::CsRspOwnerRoleSelectIntroList),
        #[prost(message, tag="10103")]
        CsReqCreateRole(super::CsReqCreateRole),
        #[prost(message, tag="10104")]
        CsRspCreateRole(super::CsRspCreateRole),
        #[prost(message, tag="10105")]
        CsReqSelectRole(super::CsReqSelectRole),
        #[prost(message, tag="10106")]
        CsRspSelectRole(super::CsRspSelectRole),
    }
}
