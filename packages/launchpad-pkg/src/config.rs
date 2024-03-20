use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128, Uint64};

#[cw_serde]
pub struct Cw404Config {
    /// Launchpad fee collector
    pub fee_collector: Addr,
    /// CW404 contract code ID
    pub cw404_code_id: Uint64,
    /// Create collection fee
    pub collection_creation_fee: Uint128,
    /// Mint fee
    pub mint_fee: Uint128,
}

#[cw_serde]
pub struct CoinConfig {
    /// Launchpad fee collector
    pub fee_collector: Addr,
    /// Coin contract code ID
    pub coin_code_id: Uint64,
    /// Create coin fee
    pub coin_creation_fee: Uint128,
}

#[cw_serde]
pub struct Config {
    /// Launchpad admin
    pub admin_addr: Addr,
    /// Astroport pool factory contract address
    pub astroport_factory_addr: Addr,
    /// CW404 related config
    pub cw404_config: Cw404Config,
    /// Coin related config
    pub coin_config: CoinConfig,
}

#[cw_serde]
pub struct Stats {
    /// Total number of CW404 collections created
    pub cw404_collection_created: Uint64,
    /// Total number of Cosmos SDK native coins created
    pub coin_created: Uint64,
}
