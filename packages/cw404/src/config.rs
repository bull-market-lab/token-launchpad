use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint64};
use osmosis_std::types::cosmos::bank::v1beta1::Metadata as DenomMetadata;

#[cw_serde]
pub struct Config {
    /// If exists, admin can mint, burn and force transfer FT
    pub admin_addr: Option<Addr>,
    /// If exists, minter can mint FT
    pub minter_addr: Addr,
    /// Creator of the collection
    pub creator_addr: Addr,
    pub denom_metadata: DenomMetadata,
    pub royalty_payment_addr: Addr,
    pub royalty_percentage: Uint64,
}
