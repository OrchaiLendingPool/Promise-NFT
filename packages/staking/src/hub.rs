use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw20::Cw20ReceiveMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub pauser: Addr,
    pub staking_token_addr: Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    Receive(Cw20ReceiveMsg),
    Unbond {
        amount: Uint128,
    },
    UpdateConfig {
        owner: Option<Addr>,
        pauser: Option<Addr>,
    },
    UpdateCurrentDistribution {
        start: Option<u64>,
        end: Option<u64>,
    },
    AddNewDistribution {
        start: u64,
        end: u64,
        emission_per_second: Uint128,
    },
}

#[cw_serde]
pub enum Cw20HookMsg {
    Bond { chain_id: String },
}

#[cw_serde]
pub enum QueryMsg {
    Config {},
    State {},
    CurrentDistribution {},
    AllDistributions {
        start_from: Option<u64>,
        limit: Option<u32>,
    },
    Staker {
        staker: Addr,
    },
    AllStakers {
        start_from: Option<Addr>,
        limit: Option<u32>,
    },
}

#[cw_serde]
pub struct MigrateMsg {}
