use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint64};
use osmosis_std::types::cosmos::bank::v1beta1::Metadata as DenomMetadata;

#[cw_serde]
pub struct Config {
    /// If exists, admin can mint, burn and force transfer FT
    pub admin: Option<Addr>,
    /// If exists, minter can mint FT
    pub minter: Option<Addr>,
    /// Creator of the collection
    pub creator: Addr,
    pub denom_metadata: DenomMetadata,
    pub royalty_payment_address: Option<Addr>,
    pub royalty_percentage: Option<Uint64>,
}
