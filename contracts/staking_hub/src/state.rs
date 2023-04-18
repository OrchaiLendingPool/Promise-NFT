use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, CanonicalAddr, Decimal256, Uint128};
use cw_storage_plus::{Item, Map};

pub const CONFIG: Item<Config> = Item::new("config");
pub const PAUSED: Item<bool> = Item::new("paused");
pub const DISTRIBUTION_SCHEDULE: Map<u64, DistributionSchedule> = Map::new("distribution_schedule");
pub const STATE: Item<State> = Item::new("state");
pub const STAKER_INFO: Map<&Addr, StakerInfo> = Map::new("staker_info");

#[cw_serde]
pub struct Config {
    pub owner: CanonicalAddr,
    pub pauser: CanonicalAddr,
    pub staking_token_addr: CanonicalAddr,
    pub soulbound_nft_addr: CanonicalAddr,
}

#[cw_serde]
pub struct DistributionSchedule {
    pub id: u64,
    pub start: u64,
    pub end: u64,
    pub emission_per_second: Uint128,
}

#[cw_serde]
pub struct State {
    pub global_index: Decimal256,
    pub last_distributed: u64,
    pub total_staked: Uint128,
}

#[cw_serde]
pub struct StakerInfo {
    pub user_index: Decimal256,
    pub pending_amount: Decimal256,
    pub nft_id: u64,
}
