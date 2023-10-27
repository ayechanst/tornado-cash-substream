#[path = "./abi/TornadoRouter.rs"]
mod erc721;
mod helpers;
mod pb;

// use substreams::key;

// use pb::schema::{Deposit, Deposits, Withdraw, Withdraws};

use pb::schema::{Deposit, Deposits};
use substreams::pb::substreams::Clock;
use substreams::{scalar::BigInt, store::{StoreAdd, StoreAddInt64}};
use substreams_ethereum::pb::eth;
use substreams::store::{StoreNew, StoreGetInt64};
use helpers::*;

// database related things
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables;


pub const ADDRESS: &str = "0xd90e2f925DA726b50C4Ed8D0Fb90Ad053324F31b";
const START_BLOCK: u64 = 18410656;
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
                        tx_value: BigInt::from_unsigned_bytes_be(&value.bytes).to_u64(),
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
fn store_deposits(deposits: Deposits, store: StoreAddInt64) {
    for deposit in deposits.deposits {
        store.add(0, "total", deposit.tx_value as i64);
    }
}

#[substreams::handlers::map]
pub fn db_out(
    clock: Clock,
    deposits: Deposits,
    all_deposits: StoreGetInt64
) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut tables = Tables::new();
    if clock.number == START_BLOCK {
        for deposit in deposits.deposits {
            tables.create_row("Collection", ADDRESS)
                .set("from", deposit.from)
                .set("to", deposit.to)
                .set("tx_hash", deposit.tx_hash)
                .set("tx_value", deposit.tx_value);
        }
        // Create the collection, we only need to do this once
    }

    for all_deposits in all_deposits.all_deposits {
        tables.create_row("All Deposists", ADDRESS)
            .set("all_deposits", all_deposits);
    }
    // transfers_to_database_changes(&mut tables, &transfers);

    Ok(tables.to_database_changes())
}
