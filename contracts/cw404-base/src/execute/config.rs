use crate::{error::ContractError, state::CONFIG};
use cosmwasm_std::{Api, Response, Storage, Uint64};

pub fn update_config(
    api: &dyn Api,
    storage: &mut dyn Storage,
    new_admin: Option<String>,
    new_minter: Option<String>,
    new_royalty_payment_address: Option<String>,
    new_royalty_percentage: Option<Uint64>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(storage)?;
    config.admin = match new_admin {
        Some(admin) => Some(api.addr_validate(&admin)?),
        None => config.admin,
    };
    config.minter = match new_minter {
        Some(minter) => Some(api.addr_validate(&minter)?),
        None => config.minter,
    };
    config.royalty_payment_address = match new_royalty_payment_address {
        Some(royalty_payment_address) => {
            Some(api.addr_validate(&royalty_payment_address)?)
        }
        None => config.royalty_payment_address,
    };
    config.royalty_percentage = match new_royalty_percentage {
        Some(royalty_percentage) => Some(royalty_percentage),
        None => config.royalty_percentage,
    };
    CONFIG.save(storage, &config)?;
    Ok(Response::new().add_attribute("action", "update_config"))
}
