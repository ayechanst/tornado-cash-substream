#[path = "./abi/TornadoRouter.rs"]
mod erc721;
mod helpers;
mod pb;

use pb::schema::{Transfer, Transfers, Deposit, Deposits};
// use substreams::{pb::substreams::Clock, scalar::BigInt};
// use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};
use substreams_ethereum::pb::eth;

use helpers::*;

// use substreams::scalar::{BigDecimal, BigInt};
use erc721::events::Transfer as TransferEvent;

pub const ADDRESS: &str = "0xd90e2f925DA726b50C4Ed8D0Fb90Ad053324F31b";
const START_BLOCK: u64 = 12287507;

#[substreams::handlers::map]
fn map_transfers(block: eth::v2::Block) -> Result<Transfers, substreams::errors::Error> {
    let transfers = block
        .logs()
        .filter_map(|log| {
            if format_hex(log.address()) == ADDRESS.to_lowercase() {
                if let Some(transfer) = TransferEvent::match_and_decode(log) {
                    Some((transfer, format_hex(&log.receipt.transaction.hash)))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .map(|(transfer, hash)| Transfer {
            from: format_hex(&transfer.from),
            to: format_hex(&transfer.to),
            token_id: transfer.token_id.to_string(),
            tx_hash: hash,
        })
        .collect::<Vec<Transfer>>();

    Ok(Transfers { transfers })
}

#[substreams::handlers::map]
fn map_deposits(block: eth::v2::Block) -> Result<Deposits, substreams::errors::Error> {
    let deposits = block
        .calls()
        .filter_map(|callview| {
            if format_hex(&callview.call.address) == ADDRESS.to_lowercase() {
                if let Some(value) = callview.call.value {
                    Some(Deposit {
                        // use callview feilds
                        from: format_hex(&value.from),
                        to: format_hex(&transfer.to),
                        tx_hash: hash,
                        tx_value: deposit,
                    })
                    .collect::<Vec<Deposits>>();
                Ok(Deposits { deposits })
                } else {
                    None
                }
            } else {
                None
            }
        });
        // .map(|(deposit, hash)| Deposits {
        //     from: format_hex(&transfer.from),
        //     to: format_hex(&transfer.to),
        //     tx_hash: hash,
        //     tx_value: deposit,
        // })
        // .collect::<Vec<Deposits>>();

    // Ok(Deposits { deposits })
}



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
