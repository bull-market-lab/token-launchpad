use crate::error::ContractError;
use cosmwasm_std::Addr;

pub fn convert_subdenom_to_denom(
    subdenom: &str,
    contract_addr: &Addr,
) -> String {
    // e.g. subdenom = atom
    // e.g. denom = factory/contract_addr/atom
    format!("factory/{}/{}", contract_addr, subdenom)
}

pub fn convert_subdenom_to_base_subdenom(subdenom: &str) -> String {
    // e.g. base_subdenom = uatom
    format!("u{}", subdenom)
}

pub fn convert_subdenom_to_base_denom(
    subdenom: &str,
    contract_addr: &Addr,
) -> String {
    // e.g. base_denom = factory/contract_addr/uatom
    format!(
        "factory/{}/{}",
        contract_addr,
        convert_subdenom_to_base_subdenom(subdenom)
    )
}

pub fn convert_base_denom_to_base_subdenom(
    base_denom: &str,
) -> Result<String, ContractError> {
    // e.g. base_denom = factory/contract_addr/uatom
    // e.g. base_subdenom = uatom
    match base_denom.split('/').collect::<Vec<&str>>().last() {
        Some(base_subdenom) => Ok(base_subdenom.to_string()),
        None => Err(ContractError::CannotConvertBaseDenomToBaseSubdenom {
            base_denom: base_denom.to_string(),
        }),
    }
}

pub fn convert_base_subdenom_to_subdenom(base_subdenom: &str) -> String {
    // e.g. base_subdenom = uatom
    // e.g. subdenom = atom
    base_subdenom.split_at(1).1.to_string()
}
