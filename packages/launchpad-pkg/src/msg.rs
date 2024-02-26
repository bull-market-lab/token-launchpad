use crate::config::Config;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128, Uint64};
use cw404::mint_group::MintGroup;

// ========== instantiate ==========

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,
    pub fee_collector: String,
    pub cw404_code_id: Uint64,
    pub create_collection_fee: Uint128,
    pub mint_fee: Uint128,
}

// ========== execute ==========

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        new_admin_addr: Option<String>,
        new_fee_collector_addr: Option<String>,
        new_cw404_code_id: Option<Uint64>,
        new_create_collection_fee: Option<Uint128>,
        new_mint_fee: Option<Uint128>,
    },
    CreateCollection {
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
    MintFt {
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
    // Call 404's mint_nft (to be added) function and get FT implicitly
    // MintNft {
    //     collection_addr: String,
    //     recipient_addr: String,
    // },
}

// ========== query ==========

#[cw_serde]
pub struct ConfigResponse {
    pub config: Config,
}

#[cw_serde]
pub struct CreatorResponse {
    pub creator_addr: Addr,
}

#[cw_serde]
pub struct CollectionsResponse {
    pub collection_addrs: Vec<Addr>,
}

#[derive(QueryResponses)]
#[cw_serde]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(CreatorResponse)]
    CollectionCreator { collection_addr: String },
    #[returns(CollectionsResponse)]
    CreatorCollections {
        creator_addr: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

// ========== migrate ==========

#[cw_serde]
pub enum MigrateMsg {}
