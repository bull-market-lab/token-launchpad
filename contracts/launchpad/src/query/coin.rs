use crate::state::{COINS, DEFAULT_LIMIT, MAX_LIMIT};
use cosmwasm_std::{Addr, Order, StdResult, Storage};
use cw_storage_plus::Bound;
use launchpad_pkg::msg::{TokenContractResponse, TokenContractsResponse};

pub fn query_coin_by_contract_addr(
    storage: &dyn Storage,
    contract_addr: Addr,
) -> StdResult<TokenContractResponse> {
    let token_contract = COINS().load(storage, contract_addr)?;
    Ok(TokenContractResponse { token_contract })
}

pub fn query_coins_by_creator_addr(
    storage: &dyn Storage,
    creator_addr: Addr,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<TokenContractsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));
    let token_contracts = COINS()
        .idx
        .owner
        .prefix(creator_addr)
        .range(storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(_, token_contract)| token_contract))
        .collect::<StdResult<Vec<_>>>()?;
    Ok(TokenContractsResponse { token_contracts })
}

pub fn query_coins(
    storage: &dyn Storage,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<TokenContractsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));
    let token_contracts = COINS()
        .idx
        .owner
        .range(storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(_, token_contract)| token_contract))
        .collect::<StdResult<Vec<_>>>()?;
    Ok(TokenContractsResponse { token_contracts })
}
