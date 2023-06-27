export type Binary = string;
export type Expiration = {
  at_height: number;
} | {
  at_time: Timestamp;
} | {
  never: {};
};
export type Timestamp = Uint64;
export type Uint64 = string;
export type Action = {
  transfer_ownership: {
    expiry?: Expiration | null;
    new_owner: string;
  };
} | "accept_ownership" | "renounce_ownership";
export interface Metadata {
  animation_url?: string | null;
  attributes?: Trait[] | null;
  background_color?: string | null;
  description?: string | null;
  external_url?: string | null;
  image?: string | null;
  image_data?: string | null;
  name?: string | null;
  youtube_url?: string | null;
}
export interface Trait {
  display_type?: string | null;
  trait_type: string;
  value: string;
}
export interface Empty {}
export interface Approval {
  expires: Expiration;
  spender: string;
}
export type Uint128 = string;
export { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";