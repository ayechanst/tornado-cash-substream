#[path = "./abi/TornadoRouter.rs"]
mod erc721;
mod helpers;
mod pb  } else {
//                     None
//                 }
//             } else {
//                 None
//             }
//         })
//         .map(|(transfer, hash)| Transfer {
//             from: format_hex(&transfer.from),
//             to: format_hex(&transfer.to),
//             token_id: transfer.token_id.to_string(),
//             tx_hash: hash,
//         })
//         .collect::<Vec<Transfer>>();

//     Ok(Transfers { transfers })
// }

#[substreams::handlers::map]
fn map_deposits(block: eth::v2::Block) -> Result<Deposits, substreams::errors::Error> {
    let deposits = block
        .calls()
        .filter_map(|callview| {
            if format_hex(&callview.call.address) == ADDRESS.to_lowercase() {
                if let Some(value) = callview.call.value {
                    println!("value: {}", value);
                    Some(Deposit {
                        from: format_hex(&callview.transaction.from),
                        to: format_hex(&callview.transaction.to),
                        tx_hash: format_hex(&callview.transaction.hash),
                        tx_value: "big int".to_string(),
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
