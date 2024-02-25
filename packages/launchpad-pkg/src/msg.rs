use crate::{collection::Collection, config::Config};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;
use cw404::msg::InstantiateMsg as Cw404InstantiateMsg;

// ========== instantiate ==========

#[cw_serde]
pub struct InstantiateMsg {
    pub admin_addr: String,
    pub mint_fee: Uint128,
    pub cw404_code_id: Uint128,
}

// ========== execute ==========

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        new_admin_addr: Option<String>,
        new_mint_fee: Option<Uint128>,
    },
    UpdateCollectionAdmin {
        collection_addr: String,
        new_admin_addr: String,
    },
    CreateCollection {
        instantiate_msg: Cw404InstantiateMsg,
    },
    StartCollectionMinting {
        collection_addr: String,
    },
    Mint {
        collection_addr: String,
        recipient_addr: String,
    },
}

// ========== query ==========

#[cw_serde]
pub struct ConfigResponse {
    pub config: Config,
}

#[cw_serde]
pub struct CollectionResponse {
    pub collection: Collection,
}

#[derive(QueryResponses)]
#[cw_serde]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(CollectionResponse)]
    Collection { collection_addr: String },
}

// ========== migrate ==========

#[cw_serde]
pub enum MigrateMsg {}
