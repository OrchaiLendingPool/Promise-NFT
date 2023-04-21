use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

use crate::nft::Metadata;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub pauser: Addr,
    pub nft_admin: Addr,
    pub soulbound_nft_code_id: u64,
    pub nft_name: String,
    pub nft_symbol: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Mint {
        token_uri: Option<String>,
        extension: Metadata,
    },
    UpdateConfig {
        owner: Option<Addr>,
        pauser: Option<Addr>,
    },
}

#[cw_serde]
pub enum Cw20HookMsg {}

#[cw_serde]
pub enum QueryMsg {
    DynamicAttributesNftInfo { token_id: String },
}

#[cw_serde]
pub struct MigrateMsg {}
