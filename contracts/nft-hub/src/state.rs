use cosmwasm_schema::cw_serde;
use cosmwasm_std::CanonicalAddr;
use cw_storage_plus::Item;

pub const CONFIG: Item<Config> = Item::new("config");
pub const PAUSED: Item<bool> = Item::new("paused");

#[cw_serde]
pub struct Config {
    pub owner: CanonicalAddr,
    pub pauser: CanonicalAddr,
    pub soulbound_nft: CanonicalAddr,
}
