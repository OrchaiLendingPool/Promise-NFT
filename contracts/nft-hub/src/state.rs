use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CanonicalAddr, Uint128};
use cw_storage_plus::Item;
use promise_nft::nft::Metadata;

pub const CONFIG: Item<Config> = Item::new("config");
pub const PAUSED: Item<bool> = Item::new("paused");
pub const TOKEN_ID: Item<Uint128> = Item::new("token_id");
pub const NFT_INFO: Item<NftInfo> = Item::new("nft_info");
pub const EXTERNAL_CONTRACT: Item<ExternalContract> = Item::new("external_contract");

#[cw_serde]
pub struct Config {
    pub owner: CanonicalAddr,
    pub pauser: CanonicalAddr,
    pub soulbound_nft: CanonicalAddr,
    pub campaign_end: u64,
}

#[cw_serde]
pub struct NftInfo {
    pub token_uri: Option<String>,
    pub extension: Option<Metadata>,
}

#[cw_serde]
pub struct ExternalContract {
    pub sc_atom_promise_staking: Option<CanonicalAddr>,
}
