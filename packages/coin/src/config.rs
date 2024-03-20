use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use osmosis_std::types::cosmos::bank::v1beta1::Metadata as DenomMetadata;

use crate::msg::SeedLiquidityConfig;

#[cw_serde]
pub struct Config {
    /// If exists, admin can mint, burn and force transfer FT
    pub admin_addr: Option<Addr>,
    /// Creator of the collection
    pub creator_addr: Addr,
    pub denom_metadata: DenomMetadata,
    /// Max supply in base denom, e.g. uatom
    pub max_supply_in_base_denom: Uint128,
    /// Seed liquidity config
    pub seed_liquidity_config: Option<SeedLiquidityConfig>,
}
