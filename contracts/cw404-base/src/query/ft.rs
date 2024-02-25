use crate::state::{CURRENT_NFT_SUPPLY, MAX_NFT_SUPPLY};
use cosmwasm_std::{Addr, QuerierWrapper, StdResult, Storage, Uint128};
use cw404::msg::{BalanceResponse, SupplyResponse};

pub fn query_supply(
    querier: QuerierWrapper,
    storage: &dyn Storage,
    base_denom: &str,
    one_denom_in_base_denom: Uint128,
) -> StdResult<SupplyResponse> {
    let ft_supply = querier.query_supply(base_denom)?.amount;
    let current_nft_supply = CURRENT_NFT_SUPPLY.load(storage)?;
    let max_nft_supply = MAX_NFT_SUPPLY.load(storage)?;
    Ok(SupplyResponse {
        current_ft_supply: ft_supply,
        max_ft_supply: max_nft_supply * one_denom_in_base_denom,
        current_nft_supply,
        max_nft_supply,
    })
}

pub fn query_balance(
    querier: QuerierWrapper,
    owner: &Addr,
    base_denom: &str,
    one_denom_in_base_denom: Uint128,
) -> StdResult<BalanceResponse> {
    let ft_balance = querier.query_balance(owner, base_denom)?.amount;
    Ok(BalanceResponse {
        nft_balance: ft_balance / one_denom_in_base_denom,
        ft_balance,
    })
}
