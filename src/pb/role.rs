/// 选角简介, 待后续扩展
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleSelectIntro {
    #[prost(int32, tag="1")]
    pub role_id: i32,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
/// 创建角色信息, 待后续扩展
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleCreateInfo {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
