#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Binary, CanonicalAddr, Decimal256, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128,
};
use cw20_base::msg::QueryMsg;
use staking::hub::{ExecuteMsg, InstantiateMsg, MigrateMsg};

use crate::{
    error::ContractError,
    state::{Config, DistributionSchedule, State, CONFIG, DISTRIBUTION_SCHEDULE, PAUSED, STATE},
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    CONFIG.save(
        deps.storage,
        &Config {
            owner: deps.api.addr_canonicalize(msg.owner.as_str())?,
            pauser: deps.api.addr_canonicalize(msg.pauser.as_str())?,
            staking_token_addr: deps
                .api
                .addr_canonicalize(msg.staking_token_addr.as_str())?,
            soulbound_nft_addr: CanonicalAddr::from(vec![]),
        },
    )?;

    PAUSED.save(deps.storage, &false)?;

    STATE.save(
        deps.storage,
        &State {
            global_index: Decimal256::zero(),
            last_distributed: env.block.time.seconds(),
            total_staked: Uint128::zero(),
        },
    )?;

    DISTRIBUTION_SCHEDULE.save(
        deps.storage,
        0,
        &DistributionSchedule {
            id: 0,
            start: env.block.time.seconds(),
            end: env.block.time.seconds(),
            emission_per_second: Uint128::zero(),
        },
    )?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Receive(msg) => Ok(Response::default()),
        ExecuteMsg::AddNewDistribution {
            start,
            end,
            emission_per_second,
        } => Ok(Response::default()),
        ExecuteMsg::Unbond { amount } => Ok(Response::default()),
        ExecuteMsg::UpdateConfig { owner, pauser } => Ok(Response::default()),
        ExecuteMsg::UpdateCurrentDistribution { start, end } => Ok(Response::default()),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    Ok(Binary::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
