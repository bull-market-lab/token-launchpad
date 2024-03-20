use crate::{
    config::{Config, Stats},
    token::TokenContract,
};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Uint128, Uint64};
use cw404::mint_group::MintGroup;

// ========== instantiate ==========

#[cw_serde]
pub struct InstantiateMsg {
    pub admin_addr: String,
    pub astroport_factory_addr: String,
    pub cw404_fee_collector: String,
    pub cw404_code_id: Uint64,
    pub cw404_collection_creation_fee: Uint128,
    pub cw404_mint_fee: Uint128,
    pub coin_fee_collector: String,
    pub coin_code_id: Uint64,
    pub coin_creation_fee: Uint128,
}

// ========== execute ==========

#[cw_serde]
pub enum ExecuteMsg {
    /// Update shared config
    UpdateSharedConfig {
        new_admin_addr: Option<String>,
        new_astroport_factory_addr: Option<String>,
    },
    /// Update CW404 related config
    UpdateCw404Config {
        new_fee_collector_addr: Option<String>,
        new_cw404_code_id: Option<Uint64>,
        new_collection_creation_fee: Option<Uint128>,
        new_mint_fee: Option<Uint128>,
    },
    /// Update coin related config
    UpdateCoinConfig {
        new_fee_collector_addr: Option<String>,
        new_coin_code_id: Option<Uint64>,
        new_coin_creation_fee: Option<Uint128>,
    },
    /// Create a new CW404 collection
    CreateCw404Collection {
        royalty_payment_address: String,
        royalty_percentage: Uint64,
        max_nft_supply: Uint128,
        // e.g. subdenom = atom, then base subdenom is uatom,
        // denom is factory/contract_addr/atom, base denom is factory/contract_addr/uatom
        // 1 atom = 1_000_000 uatom, 1 atom = 1 atom NFT,
        subdenom: String,
        denom_description: String,
        denom_name: String,
        denom_symbol: String,
        denom_uri: String,
        denom_uri_hash: String,
        mint_groups: Vec<MintGroup>,
    },
    /// Call 404's mint_ft function and get NFT implicitly
    MintFtOfCw404 {
        /// collection address
        collection_addr: String,
        /// amount is in base denom, e.g. uatom
        amount: Uint128,
        /// recipient address
        recipient: String,
        /// mint group name
        mint_group_name: String,
        /// merkle proof for recipient address
        merkle_proof: Option<Vec<Vec<u8>>>,
    },
    // TODO: implement this
    /// Call 404's mint_nft (to be added) function and get FT implicitly
    // MintNftOfCw404 {
    //     collection_addr: String,
    //     recipient_addr: String,
    // },
    /// Create a new Cosmos SDK native coin managed by token factory module
    CreateCoin {
        // initial_supply is in denom, e.g. atom, not base denom
        initial_supply_in_denom: Uint128,
        // max_supply is in denom, e.g. atom, not base denom
        max_supply_in_denom: Uint128,
        // immutable means no one can mint or burn or force transfer after creation
        immutable: bool,
        // same as subdenom in CreateCw404Collection
        subdenom: String,
        denom_description: String,
        denom_name: String,
        denom_symbol: String,
        denom_uri: String,
        denom_uri_hash: String,
    },
}

// ========== query ==========

#[cw_serde]
pub struct ConfigResponse {
    pub config: Config,
}

#[cw_serde]
pub struct StatsResponse {
    pub stats: Stats,
}

#[cw_serde]
pub struct TokenContractResponse {
    pub token_contract: TokenContract,
}

#[cw_serde]
pub struct TokenContractsResponse {
    pub token_contracts: Vec<TokenContract>,
}

#[derive(QueryResponses)]
#[cw_serde]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(StatsResponse)]
    Stats {},
    #[returns(TokenContractResponse)]
    Cw404CollectionByContract { contract_addr: String },
    #[returns(TokenContractsResponse)]
    Cw404CollectionsByCreator {
        creator_addr: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(TokenContractsResponse)]
    Cw404Collections {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(TokenContractResponse)]
    CoinByContract { contract_addr: String },
    #[returns(TokenContractsResponse)]
    CoinsByCreator {
        creator_addr: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(TokenContractsResponse)]
    Coins {
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

// ========== migrate ==========

#[cw_serde]
pub enum MigrateMsg {
    FromCompatible {},
}
