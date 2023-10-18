use crate::{
    // pb::schema::{Approvals, Transfers},
    ADDRESS,
};
use substreams::Hex;
use substreams_entity_change::tables::Tables;

pub fn transfers_to_table_changes(tables: &mut Tables, transfers: &Transfers) {
    for transfer in transfers.transfers.iter() {
        // handle the transfer
        let key = format!("{}-{}", transfer.tx_hash, transfer.token_id);
        let row = tables.update_row("Transfer", key);
        row.set("from", &transfer.from);
        row.set("to", &transfer.to);
        row.set("tokenId", &transfer.token_id);

        // handle the accounts
        tables.create_row("Account", &transfer.from);
        tables.create_row("Account", &transfer.to);

        // handle updating the token owner
        tables
            .update_row("Token", format!("{}", &transfer.token_id))
            .set("collection", ADDRESS.to_string())
            .set("owner", &transfer.to);
    }
}


pub fn format_hex(address: &[u8]) -> String {
    format!("0x{}", Hex(address).to_string())
}
