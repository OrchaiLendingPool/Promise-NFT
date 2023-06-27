use cosmwasm_schema::{cw_serde, QueryResponses};
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
    pub token_uri: Option<String>,
    pub extension: Option<Metadata>,
    pub campaign_end: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    Mint {},
    UpdateConfig {
        owner: Option<Addr>,
        pauser: Option<Addr>,
        campaign_end: Option<u64>,
    },
    RegisterExternalContract {
        sc_atom_promise_staking: Option<Addr>,
    },
}

#[cw_serde]
pub enum Cw20HookMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Vec<(String, String)>)]
    DynamicAttributesNftInfo { token_id: String },
    #[returns(NftInfoResponse)]
    NftInfo {},
    #[returns(ConfigResponse)]
    Config {},
    #[returns(ExternalContractResponse)]
    ExternalContract {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
    pub pauser: Addr,
    pub soulbound_nft: Addr,
    pub campaign_end: u64,
}

#[cw_serde]
pub struct ExternalContractResponse {
    pub sc_atom_promise_staking: Option<Addr>,
}

#[cw_serde]
pub struct NftInfoResponse {
    pub token_uri: Option<String>,
    pub extension: Option<Metadata>,
}
#[cw_serde]
pub struct MigrateMsg {}
