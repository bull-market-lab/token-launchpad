use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct Collection {
    /// The owner of the collection
    pub owner: Addr,
    /// Whether the collection has started minting
    pub minting_started: bool,
}
