use crate::state::CONFIG;
use cosmwasm_std::{Api, Response, Storage};
use shared_pkg::error::ContractError;

pub fn update_config(
    api: &dyn Api,
    storage: &mut dyn Storage,
    new_admin_addr: Option<String>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(storage)?;
    config.admin_addr = match new_admin_addr {
        Some(admin) => Some(api.addr_validate(&admin)?),
        None => config.admin_addr,
    };
    CONFIG.save(storage, &config)?;
    Ok(Response::new().add_attribute("action", "update_config"))
}
