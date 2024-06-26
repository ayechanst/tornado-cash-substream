// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deposits {
    #[prost(message, repeated, tag="1")]
    pub deposits: ::prost::alloc::vec::Vec<Deposit>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deposit {
    #[prost(string, tag="1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub tx_value: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
