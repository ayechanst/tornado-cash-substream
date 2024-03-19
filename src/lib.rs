#[path = "./abi/TornadoRouter.rs"]
mod erc721;
mod helpers;
mod pb;

use helpers::*;
use pb::schema::{Deposit, Deposits};
use substreams_ethereum::pb::eth::v2::Block;

// store related things
use substreams::{
    log,
    scalar::BigInt,
    store::{StoreAdd, StoreAddBigInt, StoreGet, StoreGetBigInt, StoreNew},
};

// database related things

pub const ADDRESS: &str = "0xd90e2f925DA726b50C4Ed8D0Fb90Ad053324F31b";
pub const TC01: &str = "0x12D66f87A04A9E220743712cE6d9bB1B5616B8Fc";
pub const TC1: &str = "0x47CE0C6eD5B0Ce3d3A51fdb1C52DC66a7c3c2936";
pub const TC10: &str = "0x910Cbd523D972eb0a6f4cAe4618aD62622b39DbF";
pub const TC100: &str = "0xA160cdAB225685dA1d56aa342Ad8841c3b53f291";
pub const DEPOSIT_TOPIC_0: &str =
    "0xa945e51eec50ab98c161376f0db4cf2aeba3ec92755fe2fcd388bdbbb80ff196";

#[substreams::handlers::map]
fn map_deposits(blk: Block) -> Result<Deposits, substreams::errors::Error> {
    let deposits = blk
        .logs()
        .filter_map(|log| {
            if format_hex(&log.log.address) == TC01.to_lowercase()
                || format_hex(&log.log.address) == TC1.to_lowercase()
                || format_hex(&log.log.address) == TC10.to_lowercase()
                || format_hex(&log.log.address) == TC100.to_lowercase()
            {
                if format_hex(&log.log.topics[0]) == DEPOSIT_TOPIC_0.to_lowercase() {
                    let address = format_hex(&log.log.address);
                    let value = &log.receipt.transaction.value;
                    if let Some(value) = value {
                        Some(Deposit {
                            from: format_hex(&log.receipt.transaction.from),
                            to: format_hex(&log.receipt.transaction.to),
                            address: get_pool_name(&address),
                            tx_hash: format_hex(&log.receipt.transaction.hash),
                            tx_value: BigInt::from_unsigned_bytes_be(&value.bytes).to_string(),
                        })
                    } else {
                        None
                    }
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
fn store_deposits(deposits: Deposits, store: StoreAddBigInt) {
    for deposit in deposits.deposits {
        let value_as_string = deposit.tx_value;
        substreams::log::info!("value as string {}", value_as_string);
        let value_as_bigint: BigInt = value_as_string.parse().unwrap();
        substreams::log::info!("value as bigint {}", value_as_bigint);
        store.add(0, "total", value_as_bigint);
    }
}

// #[substreams::handlers::map]
// fn graph_out(deposits: Deposits) -> Result<EntityChanges, substreams::errors::Error> {}
