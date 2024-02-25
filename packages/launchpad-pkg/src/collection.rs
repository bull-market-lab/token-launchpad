use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct Collection {
    /// The creator of the collection
    pub creator_addr: Addr,
    /// The address of the collection
    pub collection_addr: Addr,
}
