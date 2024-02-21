use crate::state::{ADMIN_ADDR, DENOM_EXPONENT, DENOM_METADATA};
use cosmwasm_std::{Addr, Env, StdError, Storage, Uint128};
use osmosis_std::types::cosmos::bank::v1beta1::Metadata;

pub fn get_commom_fields(
    storage: &dyn Storage,
    env: Env,
) -> Result<(Addr, Addr, Uint128, Metadata), StdError> {
    let contract_addr = env.contract.address;
    let admin_addr = ADMIN_ADDR.load(storage)?;
    let metadata = DENOM_METADATA.load(storage)?;
    let one_denom_in_base_denom = Uint128::from(10u128.pow(DENOM_EXPONENT));
    Ok((contract_addr, admin_addr, one_denom_in_base_denom, metadata))
}
