use coin::{
    config::Config,
    msg::{BalanceResponse, SupplyResponse},
};
use cosmwasm_std::{Addr, QuerierWrapper, StdResult};

pub fn query_balance(
    querier: QuerierWrapper,
    owner: &Addr,
    base_denom: &str,
) -> StdResult<BalanceResponse> {
    let balance_in_base_denom =
        querier.query_balance(owner, base_denom)?.amount;
    Ok(BalanceResponse {
        balance_in_base_denom,
    })
}

pub fn query_supply(
    querier: QuerierWrapper,
    config: &Config,
    base_denom: &str,
) -> StdResult<SupplyResponse> {
    let current_supply_in_base_denom = querier.query_supply(base_denom)?.amount;
    let max_supply_in_base_denom = config.max_supply_in_base_denom;
    Ok(SupplyResponse {
        current_supply_in_base_denom,
        max_supply_in_base_denom,
    })
}
