use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct TokenContract {
    /// The creator of the cw404 collection or coin
    pub creator_addr: Addr,
    /// The address of the token contract that manages the coin or cw404 collection
    pub contract_addr: Addr,
}
