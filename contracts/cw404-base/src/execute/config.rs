use cosmwasm_std::{Addr, Response, Storage};

use crate::{error::ContractError, state::ADMIN_ADDR};

pub fn change_admin(
    storage: &mut dyn Storage,
    new_admin_addr: &Addr,
) -> Result<Response, ContractError> {
    ADMIN_ADDR.save(storage, &new_admin_addr)?;
    Ok(Response::new()
        .add_attribute("token_type", "ft")
        .add_attribute("action", "change_admin")
        .add_attribute("new_admin_addr", new_admin_addr))
}
