use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Uint128, Uint64};

#[cw_serde]
pub struct MintGroup {
    /// name of the mint group, e.g. allowlist, public
    pub name: String,
    /// hex-encoded merkle root
    pub merkle_root: Option<Vec<u8>>,
    /// URI to the merkle tree
    pub merkle_tree_uri: Option<String>,
    pub max_base_denom_amount_per_mint: Uint128,
    pub price_per_base_denom: Uint128,
    pub start_time: Uint64,
    pub end_time: Uint64,
}
