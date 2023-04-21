use crate::{
    msg::{ConfigResponse, MetadataResponse, SoulboundNftInfoResponse},
    state::CONFIG,
    Cw721SoulboundContract,
};
use cosmwasm_std::{from_binary, Deps, Env, StdResult};
use cw721::NftInfoResponse;
use cw721_base::msg::QueryMsg as Cw721QueryMsg;
use promise_nft::nft::{ExternalQueryMsg, Metadata};

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        nft_hub: config.nft_hub.to_string(),
    })
}

pub fn query_nft_info(
    deps: Deps,
    env: Env,
    token_id: String,
) -> StdResult<SoulboundNftInfoResponse> {
    let config = CONFIG.load(deps.storage)?;

    let nft_info: NftInfoResponse<Option<Metadata>> = from_binary(
        &Cw721SoulboundContract::default()
            .query(
                deps,
                env,
                Cw721QueryMsg::NftInfo {
                    token_id: token_id.clone(),
                },
            )
            .unwrap(),
    )
    .unwrap();

    let metadata: Metadata = nft_info.extension.unwrap_or(Metadata::default());

    let dyn_attrs: Vec<(String, String)> = deps
        .querier
        .query_wasm_smart(
            config.nft_hub,
            &ExternalQueryMsg::DynamicAttributesNftInfo { token_id },
        )
        .unwrap_or(vec![]);

    Ok(SoulboundNftInfoResponse {
        token_uri: nft_info.token_uri,
        extension: MetadataResponse {
            image: metadata.image,
            image_data: metadata.image_data,
            external_url: metadata.external_url,
            description: metadata.description,
            name: metadata.name,
            animation_url: metadata.animation_url,
            attributes: metadata.attributes,
            background_color: metadata.background_color,
            youtube_url: metadata.youtube_url,
            dyn_attrs: Some(dyn_attrs),
        },
    })
}
