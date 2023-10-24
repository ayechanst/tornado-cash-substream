// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contracts {
    #[prost(message, repeated, tag="1")]
    pub contracts: ::prost::alloc::vec::Vec<Contract>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contract {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub timestamp: ::prost::alloc::string::String,
}
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
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub tx_value: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Withdraws {
    #[prost(message, repeated, tag="1")]
    pub withdraws: ::prost::alloc::vec::Vec<Withdraw>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Withdraw {
    #[prost(string, tag="1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub tx_value: u64,
}
// @@protoc_insertion_point(module)
