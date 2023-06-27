use cosmwasm_schema::write_api;
use cosmwasm_std::Empty;

use cw721_base::{ExecuteMsg, InstantiateMsg};
use promise_nft::nft::Metadata;
use promise_nft::soulbound_nft::{QueryMsg, MigrateMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg<Option<Metadata>, Empty>,
        query: QueryMsg,
        migrate: MigrateMsg,
    }
}