use crate::config::Config;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

// ========== instantiate ==========

#[cw_serde]
pub struct SeedLiquidityConfig {
    // astroport factory address
    pub astroport_factory_addr: String,
    // paired base denom, e.g. uatom
    pub paired_base_denom: String,
    // paired base denom amount, e.g. 1_000_000
    pub paired_base_denom_amount: Uint128,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub admin_addr: Option<String>,
    pub creator_addr: String,
    // initial_supply is in denom, e.g. atom, not base denom
    pub initial_supply_in_denom: Uint128,
    // max_supply is in denom, e.g. atom, not base denom
    pub max_supply_in_denom: Uint128,
    pub seed_liquidity_config: Option<SeedLiquidityConfig>,
    // e.g. subdenom = atom, then base subdenom is uatom,
    // denom is factory/contract_addr/atom, base denom is factory/contract_addr/uatom
    // 1 atom = 1_000_000 uatom
    pub subdenom: String,
    pub denom_description: String,
    pub denom_name: String,
    pub denom_symbol: String,
    pub denom_uri: String,
    pub denom_uri_hash: String,
}

// ========== execute ==========

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        new_admin_addr: Option<String>,
    },
    /// Only admin can execute this
    Mint {
        /// amount is in base denom, e.g. uatom
        amount: Uint128,
        /// recipient address
        recipient: String,
    },
    /// Only admin can execute this
    Burn {
        /// amount is in base denom
        amount: Uint128,
    },
    /// Force transfer from one account to another
    /// Only admin can execute this
    ForceTransfer {
        /// amount is in base denom
        amount: Uint128,
        from: String,
        to: String,
    },
}

// ========== query ==========

#[cw_serde]
pub struct ConfigResponse {
    pub config: Config,
}

#[cw_serde]
pub struct SupplyResponse {
    pub current_supply_in_base_denom: Uint128,
    pub max_supply_in_base_denom: Uint128,
}

#[cw_serde]
pub struct BalanceResponse {
    /// balance in base denom
    pub balance_in_base_denom: Uint128,
}

#[derive(QueryResponses)]
#[cw_serde]
pub enum QueryMsg {
    // ========== general functions ==========
    #[returns(ConfigResponse)]
    Config {},
    #[returns(SupplyResponse)]
    Supply {},
    #[returns(BalanceResponse)]
    Balance { owner: String },
}

// ========== migrate ==========
#[cw_serde]
pub enum MigrateMsg {}
