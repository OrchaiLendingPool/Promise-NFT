import {Binary, Expiration, Timestamp, Uint64, Action, Metadata, Trait, Empty, Approval, Uint128} from "./types";
export interface InstantiateMsg {
  minter: string;
  name: string;
  symbol: string;
}
export type ExecuteMsg = {
  transfer_nft: {
    recipient: string;
    token_id: string;
  };
} | {
  send_nft: {
    contract: string;
    msg: Binary;
    token_id: string;
  };
} | {
  approve: {
    expires?: Expiration | null;
    spender: string;
    token_id: string;
  };
} | {
  revoke: {
    spender: string;
    token_id: string;
  };
} | {
  approve_all: {
    expires?: Expiration | null;
    operator: string;
  };
} | {
  revoke_all: {
    operator: string;
  };
} | {
  mint: {
    extension?: Metadata | null;
    owner: string;
    token_id: string;
    token_uri?: string | null;
  };
} | {
  burn: {
    token_id: string;
  };
} | {
  extension: {
    msg: Empty;
  };
} | {
  update_ownership: Action;
};
export type QueryMsg = {
  config: {};
} | {
  owner_of: {
    token_id: string;
  };
} | {
  num_tokens: {};
} | {
  contract_info: {};
} | {
  nft_info: {
    token_id: string;
  };
} | {
  all_nft_info: {
    token_id: string;
  };
} | {
  tokens: {
    limit?: number | null;
    owner: string;
    start_after?: string | null;
  };
} | {
  all_tokens: {
    limit?: number | null;
    start_after?: string | null;
  };
} | {
  minter: {};
};
export interface MigrateMsg {}
export interface AllNftInfoResponseForEmpty {
  access: OwnerOfResponse;
  info: NftInfoResponseForEmpty;
}
export interface OwnerOfResponse {
  approvals: Approval[];
  owner: string;
}
export interface NftInfoResponseForEmpty {
  extension: Empty;
  token_uri?: string | null;
}
export interface TokensResponse {
  tokens: string[];
}
export interface ConfigResponse {
  nft_hub: string;
}
export interface ContractInfoResponse {
  name: string;
  symbol: string;
}
export interface MinterResponse {
  cap?: Uint128 | null;
  minter: string;
}
export interface SoulboundNftInfoResponse {
  extension: MetadataResponse;
  token_uri?: string | null;
}
export interface MetadataResponse {
  animation_url?: string | null;
  attributes?: Trait[] | null;
  background_color?: string | null;
  description?: string | null;
  dyn_attrs?: [string, string][] | null;
  external_url?: string | null;
  image?: string | null;
  image_data?: string | null;
  name?: string | null;
  youtube_url?: string | null;
}
export interface NumTokensResponse {
  count: number;
}