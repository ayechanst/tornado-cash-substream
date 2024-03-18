#[path = "./abi/TornadoRouter.rs"]
mod erc721;
mod helpers;
mod pb;

// use std::vec;

use helpers::*;
use pb::schema::{Deposit, Deposits};
// use substreams_ethereum::block_view::LogView;
// use substreams_ethereum::block_view::ReceiptView;
// use substreams_ethereum::pb::eth;
use substreams_ethereum::pb::eth::v2::Block;
// store related things
use substreams::{
    log,
    scalar::BigInt,
    store::{StoreAdd, StoreAddBigInt, StoreGet, StoreGetBigInt, StoreNew},
};

// database related things
use substreams_database_change::pb::database::DatabaseChanges;
// use substreams_database_change::tables::Tables;

pub const ADDRESS: &str = "0xd90e2f925DA726b50C4Ed8D0Fb90Ad053324F31b";
pub const TC01: &str = "0x12D66f87A04A9E220743712cE6d9bB1B5616B8Fc";
pub const TC1: &str = "0x47CE0C6eD5B0Ce3d3A51fdb1C52DC66a7c3c2936";
pub const TC10: &str = "0x910Cbd523D972eb0a6f4cAe4618aD62622b39DbF";
pub const TC100: &str = "0xA160cdAB225685dA1d56aa342Ad8841c3b53f291";

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
                let value = &log.receipt.transaction.value;
                if let Some(value) = value {
                    Some(Deposit {
                        from: format_hex(&log.receipt.transaction.from),
                        to: format_hex(&log.receipt.transaction.to),
                        address: format_hex(&log.log.address),
                        tx_hash: format_hex(&log.receipt.transaction.hash),
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
fn store_deposits(deposits: Deposits, store: StoreAddBigInt) {
    for deposit in deposits.deposits {
        let value_as_string = deposit.tx_value;
        let value_as_bigint: BigInt = value_as_string.parse().unwrap();
        store.add(0, "total", value_as_bigint);
    }
}

#[substreams::handlers::map]
pub fn db_out(
    deposits: Deposits,
    all_deposits: StoreGetBigInt,
) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut tables = substreams_database_change::tables::Tables::new();

    for deposit in deposits.deposits {
        let key = format!("{}:{}", deposit.from, deposit.tx_hash);
        let deposit_as_eth = wei_to_eth(deposit.tx_value.parse().unwrap());
        tables
            .create_row("deposits", key)
            .set("from_address", deposit.from)
            .set("to_address", deposit.to)
            .set("tx_hash", deposit.tx_hash)
            .set("tx_value", deposit_as_eth.to_string());
    }

    if let Some(value) = all_deposits.get_at(0, "total") {
        tables
            .create_row("total_deposits", ADDRESS)
            .set("total_value", value.to_string());
    }

    // transfers_to_database_changes(&mut tables, &transfers);

    Ok(tables.to_database_changes())
}
