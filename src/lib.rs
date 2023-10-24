#[path = "./abi/TornadoRouter.rs"]
mod erc721;
mod helpers;
mod pb;

use substreams::scalar::BigInt;
use pb::schema::{Deposit, Deposits};
// use pb::schema::{Deposit, Deposits, Withdraw, Withdraws};
// use substreams::{pb::substreams::Clock, scalar::BigInt};
// use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};
use substreams_ethereum::pb::eth;

use helpers::*;

// use substreams::scalar::{BigDecimal, BigInt};
// use erc721::events::Transfer as TransferEvent;

pub const ADDRESS: &str = "0xd90e2f925DA726b50C4Ed8D0Fb90Ad053324F31b";
// const START_BLOCK: u64 = 12287507;
//
// let tc01 = "0x12D66f87A04A9E220743712cE6d9bB1B5616B8Fc";
// let tc1 = "0x47CE0C6eD5B0Ce3d3A51fdb1C52DC66a7c3c2936";
// let tc10 = "0x910Cbd523D972eb0a6f4cAe4618aD62622b39DbF";
// let tc100 = "0xA160cdAB225685dA1d56aa342Ad8841c3b53f291";

#[substreams::handlers::map]
fn map_deposits(block: eth::v2::Block) -> Result<Deposits, substreams::errors::Error> {
    let deposits = block
        .calls()
        .filter_map(|callview| {
            if format_hex(&callview.call.address) == ADDRESS.to_lowercase() {
                if let Some(value) = &callview.call.value {
                    Some(Deposit {
                        from: format_hex(&callview.transaction.from),
                        to: format_hex(&callview.transaction.to),
                        tx_hash: format_hex(&callview.transaction.hash),
                        tx_value: BigInt::from_unsigned_bytes_be(&value.bytes).to_string(),
                    })
                } else {
                    None
                }
            } else {
                None
            }
        })
            .collect::<Vec<Deposit>>();
        Ok(Deposits { deposits })
}

#[substreams::handlers::store]
fn store_deposits()

// #[substreams::handlers::map]
// fn map_withdraws(block: eth::v2::Block) -> Result<Withdraws, substreams::errors::Error> {
//     let withdraws = block
//         .calls()
//         .filter_map(|callview| {
//             if format_hex(&callview.call.address) == ADDRESS.to_lowercase() {
//                 if let Some(value) = &callview.call.value {
//                     Some(Withdraw {
//                         from: format_hex(&callview.transaction.from),
//                         to: format_hex(&callview.transaction.to),
//                         tx_hash: format_hex(&callview.transaction.hash),
//                         tx_value:
//                     })
//                 }
//             }
//         })
//         .collect::<Vec<Withdraw>>();
//         Ok(Withdraws { withdraws })
// }



// #[substreams::handlers::map]
// pub fn graph_out(
//     clock: Clock,
//     // transfers: Transfers,
// ) -> Result<EntityChanges, substreams::errors::Error> {
//     let mut tables = Tables::new();

//     if clock.number == START_BLOCK {
//         // Create the collection, we only need to do this once
//         tables.create_row("Collection", ADDRESS.to_string());
//     }

//     transfers_to_table_changes(&mut tables, &transfers);

//     Ok(tables.to_entity_changes())
// }
