use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct Config {
    /// Launchpad admin
    pub admin: Addr,
    /// Mint fee
    pub mint_fee: Uint128,
}
