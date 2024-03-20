use std::collections::HashMap;
use substreams::scalar::BigDecimal;
use substreams::{scalar::BigInt, Hex};

pub const TC01: &str = "0x12D66f87A04A9E220743712cE6d9bB1B5616B8Fc";
pub const TC1: &str = "0x47CE0C6eD5B0Ce3d3A51fdb1C52DC66a7c3c2936";
pub const TC10: &str = "0x910Cbd523D972eb0a6f4cAe4618aD62622b39DbF";
pub const TC100: &str = "0xA160cdAB225685dA1d56aa342Ad8841c3b53f291";

// pub fn wei_to_eth(wei: BigInt) -> f64 {
//     ((wei.to_u64()) / 1000000000000000000) as f64
// }

pub fn wei_to_eth(wei: BigInt) -> BigDecimal {
    // Convert wei to BigDecimal before division
    let wei_big_decimal = BigDecimal::from(wei);
    let eth_big_decimal = wei_big_decimal / (1000000000000000000 as u64);
    eth_big_decimal
}

pub fn format_hex(address: &[u8]) -> String {
    format!("0x{}", Hex(address).to_string())
}

pub fn get_pool_name(address: &str) -> String {
    let mut address_to_name = HashMap::new();
    address_to_name.insert(TC01.to_lowercase(), "0.1 Eth Pool");
    address_to_name.insert(TC1.to_lowercase(), "1 Eth Pool");
    address_to_name.insert(TC10.to_lowercase(), "10 Eth Pool");
    address_to_name.insert(TC100.to_lowercase(), "100 Eth Pool");
    match address_to_name.get(address) {
        Some(pool_name) => pool_name.to_string(),
        None => "Unknown".to_string(),
    }
}
