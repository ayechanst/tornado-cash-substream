use std::collections::HashMap;

// use crate::{
//     pb::schema::{Approvals, Transfers},
//     pb::schema::Transfers,
//     ADDRESS,
// };
//
use substreams::{scalar::BigInt, Hex};
// use substreams_entity_change::tables::Tables;

// pub fn transfers_to_table_changes(tables: &mut Tables, transfers: &Transfers) {
//     for transfer in transfers.transfers.iter() {
//         // handle the transfer
//         let key = format!("{}-{}", transfer.tx_hash, transfer.token_id);
//         let row = tables.update_row("Transfer", key);
//         row.set("from", &transfer.from);
//         row.set("to", &transfer.to);
//         row.set("tokenId", &transfer.token_id);

//         // handle the accounts
//         tables.create_row("Account", &transfer.from);
//         tables.create_row("Account", &transfer.to);

//         // handle updating the token owner
//         tables
//             .update_row("Token", format!("{}", &transfer.token_id))
//             .set("collection", ADDRESS.to_string())
//             .set("owner", &transfer.to);
//     }
// }
pub const TC01: &str = "0x12D66f87A04A9E220743712cE6d9bB1B5616B8Fc";
pub const TC1: &str = "0x47CE0C6eD5B0Ce3d3A51fdb1C52DC66a7c3c2936";
pub const TC10: &str = "0x910Cbd523D972eb0a6f4cAe4618aD62622b39DbF";
pub const TC100: &str = "0xA160cdAB225685dA1d56aa342Ad8841c3b53f291";

pub fn wei_to_eth(wei: BigInt) -> f32 {
    ((wei.to_u64()) / 1000000000000000000) as f32
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
