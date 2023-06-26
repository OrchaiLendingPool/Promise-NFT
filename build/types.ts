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
export interface Empty {}
export interface Approval {
  expires: Expiration;
  spender: string;
}
export type Null = null;
export interface OwnershipForString {
  owner?: string | null;
  pending_expiry?: Expiration | null;
  pending_owner?: string | null;
}
export { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";