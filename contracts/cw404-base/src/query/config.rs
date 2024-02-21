use cosmwasm_std::{Addr, StdResult};
use cw404::msg::AdminResponse;

pub fn query_admin(admin_addr: &Addr) -> StdResult<AdminResponse> {
    Ok(AdminResponse {
        admin_addr: admin_addr.to_string(),
    })
}
