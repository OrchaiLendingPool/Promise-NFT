#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, CanonicalAddr, CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo,
    Reply, Response, StdError, StdResult, SubMsg, Uint128,
};
use cw721::OwnerOfResponse;
use protobuf::Message;

use crate::{
    error::ContractError,
    response::MsgInstantiateContractResponse,
    state::{Config, NftInfo, CONFIG, EXTERNAL_CONTRACT, NFT_INFO, TOKEN_ID},
};
use cw721_base::InstantiateMsg as Cw721InstantiateMsg;

use promise_nft::{
    external::{
        ScAtomPromiseStakingVaultsQueryMsg, ScAtomPromiseStakingVaultsStakerResponse,
        SoulboundNftQueryMsg,
    },
    hub::{
        ConfigResponse, ExecuteMsg, ExternalContractResponse, InstantiateMsg, MigrateMsg, QueryMsg,
    },
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

    TOKEN_ID.save(deps.storage, &Uint128::zero())?;

    NFT_INFO.save(
        deps.storage,
        &NftInfo {
            token_uri: msg.token_uri,
            extension: msg.extension,
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
        ExecuteMsg::Mint {} => execute_mint(deps, env, info),
        ExecuteMsg::RegisterExternalContract {
            sc_atom_promise_staking,
        } => execute_register_external_contract(deps, info, sc_atom_promise_staking),
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
fn execute_mint(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let nft_info = NFT_INFO.load(deps.storage)?;

    let sender = info.sender.to_string();
    let soulbound_nft_contract = deps.api.addr_humanize(&config.soulbound_nft)?;

    let token_id = TOKEN_ID.load(deps.storage)? + Uint128::one();

    let messages = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: soulbound_nft_contract.to_string(),
        msg: to_binary(&Cw721ExecuteMsg::Mint {
            token_id: token_id.to_string(),
            owner: sender.clone(),
            token_uri: nft_info.token_uri,
            extension: nft_info.extension.into(),
        })?,
        funds: vec![],
    });

    TOKEN_ID.save(deps.storage, &token_id)?;

    Ok(Response::default()
        .add_message(messages)
        .add_attributes(vec![
            ("action", "mint_soulbound_nft"),
            ("sender", sender.as_str()),
        ]))
}

fn execute_register_external_contract(
    deps: DepsMut,
    info: MessageInfo,
    sc_atom_promise_staking: Option<Addr>,
) -> Result<Response, ContractError> {
    let sender_raw = deps.api.addr_canonicalize(info.sender.as_str())?;
    let config = CONFIG.load(deps.storage)?;
    let mut external_contract = EXTERNAL_CONTRACT.load(deps.storage)?;

    if sender_raw != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(sc_atom_promise_staking) = sc_atom_promise_staking {
        external_contract.sc_atom_promise_staking = Some(
            deps.api
                .addr_canonicalize(sc_atom_promise_staking.as_str())?,
        );
    }

    Ok(Response::new().add_attribute("action", "register_external_contract"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::DynamicAttributesNftInfo { token_id } => {
            to_binary(&query_dynamic_attributes_nft_info(deps, env, token_id)?)
        }
        QueryMsg::NftInfo {} => to_binary(&query_nft_info(deps)?),
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::ExternalContract {} => to_binary(&query_external_contract(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;

    Ok(ConfigResponse {
        owner: deps.api.addr_humanize(&config.owner)?,
        pauser: deps.api.addr_humanize(&config.pauser)?,
        soulbound_nft: deps.api.addr_humanize(&config.soulbound_nft)?,
    })
}

fn query_external_contract(deps: Deps) -> StdResult<ExternalContractResponse> {
    let external_contract = EXTERNAL_CONTRACT.load(deps.storage)?;

    let mut res = ExternalContractResponse {
        sc_atom_promise_staking: None,
    };
    if external_contract.sc_atom_promise_staking.is_some() {
        res.sc_atom_promise_staking = Some(
            deps.api
                .addr_humanize(&external_contract.sc_atom_promise_staking.unwrap())?,
        )
    }

    Ok(res)
}

fn query_dynamic_attributes_nft_info(
    deps: Deps,
    _env: Env,
    token_id: String,
) -> StdResult<Vec<(String, String)>> {
    let config = CONFIG.load(deps.storage)?;
    let external_contract = EXTERNAL_CONTRACT.load(deps.storage)?;
    let soulbound_nft_addr = deps.api.addr_humanize(&config.soulbound_nft)?;

    let mut res: Vec<(String, String)> = vec![];
    // query scAtom staking vault rewards
    if external_contract.sc_atom_promise_staking.is_some() {
        // query token_id owner
        let owner_of: OwnerOfResponse = deps.querier.query_wasm_smart(
            soulbound_nft_addr.to_string(),
            &SoulboundNftQueryMsg::OwnerOf {
                token_id: token_id.clone(),
            },
        )?;

        let sc_atom_promise_staking_contract = deps
            .api
            .addr_humanize(&external_contract.sc_atom_promise_staking.unwrap())
            .unwrap()
            .to_string();
        let staker_info: ScAtomPromiseStakingVaultsStakerResponse = deps.querier.query_wasm_smart(
            sc_atom_promise_staking_contract,
            &ScAtomPromiseStakingVaultsQueryMsg::Staker {
                staker: deps.api.addr_validate(&owner_of.owner).unwrap(),
            },
        )?;

        res.push((
            "scAtom_pending_reward".to_string(),
            staker_info.pending_amount.to_string(),
        ))
    }

    Ok(res)
}

fn query_nft_info(deps: Deps) -> StdResult<NftInfo> {
    let nft_info = NFT_INFO.load(deps.storage)?;
    Ok(nft_info)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
