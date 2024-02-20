use cosmwasm_std::{Addr, Env, StdError, Storage, Uint128};
use osmosis_std::types::cosmos::bank::v1beta1::Metadata;

use crate::state::{ADMIN_ADDR, METADATA, SUBDENOM};

pub fn get_full_denom_from_subdenom(
    creator_addr: &Addr,
    subdenom: &str,
) -> String {
    format!("factory/{}/{}", creator_addr, subdenom)
}

pub fn get_commom_fields(
    env: Env,
    storage: &dyn Storage,
) -> Result<(Addr, Addr, Metadata, String, Uint128), StdError> {
    let contract_addr = env.contract.address;
    let admin_addr = ADMIN_ADDR.load(storage)?;
    let metadata = METADATA.load(storage)?;
    let denom =
        get_full_denom_from_subdenom(&contract_addr, &SUBDENOM.load(storage)?);
    let denom_exponent = metadata.denom_units[0].exponent;
    let one_denom_in_base_denom = Uint128::from(10u128.pow(denom_exponent));
    Ok((
        contract_addr,
        admin_addr,
        metadata,
        denom,
        one_denom_in_base_denom,
    ))
}
