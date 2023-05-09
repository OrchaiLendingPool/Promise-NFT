use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};

#[cw_serde]
pub enum ScAtomPromiseStakingVaultsQueryMsg {
    Staker { staker: Addr },
}

#[cw_serde]
pub struct ScAtomPromiseStakingVaultsStakerResponse {
    pub staker: Addr,
    pub user_index: Decimal,
    pub pending_amount: Decimal,
    pub balance: Uint128,
}

#[cw_serde]
pub enum SoulboundNftQueryMsg {
    OwnerOf { token_id: String },
}
