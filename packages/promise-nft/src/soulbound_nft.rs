use cosmwasm_schema::cw_serde;
use cosmwasm_std::Empty;
use cw721_base::msg::QueryMsg as Cw721QueryMsg;
use crate::nft::Trait;

#[cw_serde]
pub enum QueryMsg {
    Config {},
    OwnerOf {
        token_id: String,
    },
    NumTokens {},
    ContractInfo {},
    NftInfo {
        token_id: String,
    },
    AllNftInfo {
        token_id: String,
    },
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    Minter {},
}

impl From<QueryMsg> for Cw721QueryMsg<Empty> {
    fn from(msg: QueryMsg) -> Cw721QueryMsg<Empty> {
        match msg {
            QueryMsg::OwnerOf { token_id } => Cw721QueryMsg::OwnerOf {
                token_id,
                include_expired: None,
            },
            QueryMsg::NumTokens {} => Cw721QueryMsg::NumTokens {},
            QueryMsg::ContractInfo {} => Cw721QueryMsg::ContractInfo {},
            QueryMsg::NftInfo { token_id } => Cw721QueryMsg::NftInfo { token_id },
            QueryMsg::AllNftInfo { token_id } => Cw721QueryMsg::AllNftInfo {
                token_id,
                include_expired: None,
            },
            QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            } => Cw721QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            },
            QueryMsg::AllTokens { start_after, limit } => {
                Cw721QueryMsg::AllTokens { start_after, limit }
            }
            QueryMsg::Minter {} => Cw721QueryMsg::Minter {},
            _ => unreachable!("cannot convert {:?} to Cw721QueryMsg", msg),
        }
    }
}

#[cw_serde]
pub struct ConfigResponse {
    pub nft_hub: String,
}

#[cw_serde]
pub struct SoulboundNftInfoResponse {
    /// Universal resource identifier for this NFT
    /// Should point to a JSON file that conforms to the ERC721
    /// Metadata JSON Schema
    pub token_uri: Option<String>,
    /// You can add any custom metadata here when you extend cw721-base
    pub extension: MetadataResponse,
}

#[cw_serde]
#[derive(Default)]
pub struct MetadataResponse {
    pub image: Option<String>,
    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub attributes: Option<Vec<Trait>>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
    pub dyn_attrs: Option<Vec<(String, String)>>,
}

#[cw_serde]
pub struct MigrateMsg {}
