#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloReply {
    #[prost(string, tag="1")]
    pub message: ::prost::alloc::string::String,
}
