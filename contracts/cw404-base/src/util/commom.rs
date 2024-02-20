use crate::state::{ADMIN_ADDR, DENOM_EXPONENT, SUBDENOM};
use cosmwasm_std::{
    Addr, BankQuery, DenomMetadata, DenomMetadataResponse, Env, QuerierWrapper,
    QueryRequest, StdError, Storage, Uint128,
};

pub fn get_denom_from_subdenom(
    creator_addr: &Addr,
    subdenom: &str,
    is_base: bool,
) -> String {
    if is_base {
        format!("factory/{}/u{}", creator_addr, subdenom)
    } else {
        format!("factory/{}/{}", creator_addr, subdenom)
    }
}

pub fn get_commom_fields(
    querier: QuerierWrapper,
    storage: &dyn Storage,
    env: Env,
) -> Result<(Addr, Addr, String, Uint128, DenomMetadata), StdError> {
    let contract_addr = env.contract.address;
    let admin_addr = ADMIN_ADDR.load(storage)?;
    let base_denom =
        get_denom_from_subdenom(&contract_addr, &SUBDENOM.load(storage)?, true);
    let one_denom_in_base_denom = Uint128::from(10u128.pow(DENOM_EXPONENT));
    let metadata_resp: DenomMetadataResponse =
        querier.query(&QueryRequest::Bank(BankQuery::DenomMetadata {
            denom: base_denom.clone(),
        }))?;
    Ok((
        contract_addr,
        admin_addr,
        base_denom,
        one_denom_in_base_denom,
        metadata_resp.metadata,
    ))
}
