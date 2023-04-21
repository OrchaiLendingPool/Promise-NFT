#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, CanonicalAddr, CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo,
    Reply, Response, StdError, StdResult, SubMsg,
};
use protobuf::Message;

use crate::{
    error::ContractError,
    response::MsgInstantiateContractResponse,
    state::{Config, CONFIG},
};
use cw721_base::InstantiateMsg as Cw721InstantiateMsg;
use promise_nft::{
    hub::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
    nft::Metadata,
};

pub type Extension = Option<Metadata>;
pub type Cw721ExecuteMsg = cw721_base::ExecuteMsg<Extension, Empty>;

const INIT_SOULBOUND_NFT: u64 = 1;

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
            soulbound_nft: CanonicalAddr::from(vec![]),
        },
    )?;

    let messages: SubMsg = SubMsg::reply_on_success(
        CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Instantiate {
            admin: msg.nft_admin.to_string().into(),
            code_id: msg.soulbound_nft_code_id,
            msg: to_binary(&Cw721InstantiateMsg {
                name: msg.nft_name,
                symbol: msg.nft_symbol,
                minter: env.contract.address.to_string(),
            })?,
            funds: vec![],
            label: "orchai-promise-nft".to_string(),
        }),
        INIT_SOULBOUND_NFT,
    );
    Ok(Response::new().add_submessage(messages))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig { owner, pauser } => {
            execute_update_config(deps, info, owner, pauser)
        }
        ExecuteMsg::Mint {
            token_uri,
            extension,
        } => execute_mint(deps, env, info, token_uri, extension),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        INIT_SOULBOUND_NFT => {
            // get new token's contract address
            let res: MsgInstantiateContractResponse = Message::parse_from_bytes(
                msg.result.unwrap().data.unwrap().as_slice(),
            )
            .map_err(|_| {
                ContractError::Std(StdError::parse_err(
                    "MsgInstantiateContractResponse",
                    "failed to parse data",
                ))
            })?;

            let soulbound_nft_addr = Addr::unchecked(res.get_contract_address());

            let mut config = CONFIG.load(deps.storage)?;
            if config.soulbound_nft != CanonicalAddr::from(vec![]) {
                return Err(ContractError::Unauthorized {});
            }

            config.soulbound_nft = deps.api.addr_canonicalize(soulbound_nft_addr.as_str())?;
            CONFIG.save(deps.storage, &config)?;
            Ok(Response::default())
        }
        _ => Err(ContractError::InvalidReplyId {}),
    }
}

fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    owner: Option<Addr>,
    pauser: Option<Addr>,
) -> Result<Response, ContractError> {
    let sender_raw = deps.api.addr_canonicalize(info.sender.as_str())?;
    let mut config = CONFIG.load(deps.storage)?;

    if sender_raw != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(owner) = owner {
        config.owner = deps.api.addr_canonicalize(owner.as_str())?;
    }
    if let Some(pauser) = pauser {
        config.pauser = deps.api.addr_canonicalize(pauser.as_str())?;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}
fn execute_mint(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    token_uri: Option<String>,
    extension: Metadata,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let sender = info.sender.to_string();
    let soulbound_nft_contract = deps.api.addr_humanize(&config.soulbound_nft)?;

    let messages = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: soulbound_nft_contract.to_string(),
        msg: to_binary(&Cw721ExecuteMsg::Mint {
            token_id: sender.clone(),
            owner: sender.clone(),
            token_uri,
            extension: extension.into(),
        })?,
        funds: vec![],
    });

    Ok(Response::default()
        .add_message(messages)
        .add_attributes(vec![
            ("action", "mintSoulboundNft"),
            ("sender", sender.as_str()),
        ]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::DynamicAttributesNftInfo { token_id } => {
            to_binary(&query_dynamic_attributes_nft_info(deps, env, token_id)?)
        }
    }
}

fn query_dynamic_attributes_nft_info(
    _deps: Deps,
    _env: Env,
    _owner: String,
) -> StdResult<Vec<(String, String)>> {
    Ok(vec![("lmao".to_string(), "lmao".to_string())])
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
