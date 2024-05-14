/// 选角简介
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleSelectIntro {
    #[prost(int32, tag="1")]
    pub role_id: i32,
}
/// 创建角色信息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleCreateInfo {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
