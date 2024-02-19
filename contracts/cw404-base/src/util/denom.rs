use cosmwasm_std::Addr;

pub fn get_full_denom_from_subdenom(
    creator_addr: &Addr,
    subdenom: &str,
) -> String {
    format!("factory/{}/{}", creator_addr, subdenom)
}
