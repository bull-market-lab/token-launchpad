use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint64};
use osmosis_std::types::cosmos::bank::v1beta1::Metadata as DenomMetadata;

#[cw_serde]
pub struct Config {
    /// Admin can mint, burn and force transfer FT
    pub admin: Option<Addr>,
    /// Minter can mint FT
    pub minter: Option<Addr>,
    pub denom_metadata: DenomMetadata,
    pub royalty_payment_address: Option<Addr>,
    pub royalty_percentage: Option<Uint64>,
}
