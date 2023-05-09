pub use crate::msg::QueryMsg;
use cosmwasm_std::Empty;
pub use cw721_base::{
    entry::{execute as _execute, query as _query},
    ContractError, Cw721Contract, InstantiateMsg, MinterResponse, QueryMsg as Cw721QueryMsg,
};
use promise_nft::nft::Metadata;

pub mod msg;
pub mod query;
pub mod state;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw721-non-transferable";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type Extension = Option<Metadata>;

pub type Cw721SoulboundContract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty, Empty, Empty>;
pub type ExecuteMsg = cw721_base::ExecuteMsg<Extension, Empty>;

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;
    use crate::{
        msg::MigrateMsg,
        query::{query_config, query_nft_info},
        state::CONFIG,
    };
    use cosmwasm_std::{
        entry_point, from_binary, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response,
        StdError, StdResult,
    };
    use cw721::TokensResponse;
    use promise_nft::nft::Config;

    #[entry_point]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        let config = Config {
            nft_hub: deps.api.addr_validate(&msg.minter).unwrap(),
        };

        CONFIG.save(deps.storage, &config)?;

        Cw721SoulboundContract::default().instantiate(deps.branch(), env, info, msg)?;

        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        Ok(Response::default()
            .add_attribute("contract_name", CONTRACT_NAME)
            .add_attribute("contract_version", CONTRACT_VERSION))
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::Mint {
                token_id,
                owner,
                token_uri,
                extension,
            } => {
                //  only 1 nft per user
                let tokens: TokensResponse = from_binary(
                    &Cw721SoulboundContract::default()
                        .query(
                            deps.as_ref(),
                            env,
                            Cw721QueryMsg::Tokens {
                                owner: owner.clone(),
                                start_after: None,
                                limit: None,
                            },
                        )
                        .unwrap(),
                )
                .unwrap();

                if tokens.tokens.len() > 0 {
                    return Err(ContractError::Std(StdError::generic_err(
                        "Only 1 nft per user",
                    )));
                }

                Cw721SoulboundContract::default()
                    .mint(deps, info, token_id, owner, token_uri, extension)
            }
            _ => Err(ContractError::Ownership(
                cw721_base::OwnershipError::NotOwner,
            )),
        }
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::Config {} => to_binary(&query_config(deps)?),
            QueryMsg::NftInfo { token_id } => to_binary(&query_nft_info(deps, env, token_id)?),
            _ => Cw721SoulboundContract::default().query(deps, env, msg.into()),
        }
    }

    #[entry_point]
    pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
        Ok(Response::default())
    }
}
