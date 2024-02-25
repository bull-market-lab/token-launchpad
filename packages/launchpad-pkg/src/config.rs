use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128, Uint64};

#[cw_serde]
pub struct Config {
    /// Launchpad admin
    pub admin: Addr,
    /// Launchpad fee collector
    pub fee_collector: Addr,
    /// CW404 code ID
    pub cw404_code_id: Uint64,
    /// Create collection fee
    pub create_collection_fee: Uint128,
    /// Mint fee
    pub mint_fee: Uint128,
}
