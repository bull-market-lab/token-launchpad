use crate::{
    error::ContractError,
    util::nft::{
        batch_burn_nft, batch_mint_nft, calculate_nft_to_burn_for_ft_burn,
        calculate_nft_to_mint_for_ft_mint,
    },
};
use cosmwasm_std::{Addr, QuerierWrapper, Response, Storage, Uint128};

#[allow(clippy::too_many_arguments)]
pub fn track_before_send(
    storage: &mut dyn Storage,
    querier: QuerierWrapper,
    amount: Uint128,
    denom: &str,
    one_denom_in_base_denom: Uint128,
    base_uri: &str,
    from_addr: &Addr,
    to_addr: &Addr,
) -> Result<Response, ContractError> {
    let burn_nft_amount = calculate_nft_to_burn_for_ft_burn(
        querier,
        from_addr,
        denom,
        amount,
        one_denom_in_base_denom,
    )?;
    batch_burn_nft(storage, from_addr, burn_nft_amount)?;
    let mint_nft_amount = calculate_nft_to_mint_for_ft_mint(
        querier,
        from_addr,
        denom,
        amount,
        one_denom_in_base_denom,
    )?;
    batch_mint_nft(
        storage,
        querier,
        denom,
        base_uri,
        one_denom_in_base_denom,
        to_addr,
        mint_nft_amount,
    )?;
    Ok(Response::new()
        .add_attribute("token_type", "ft")
        .add_attribute("action", "track_before_send")
        .add_attribute("from", from_addr)
        .add_attribute("to", to_addr)
        .add_attribute("amount", amount)
        .add_attribute("denom", denom)
        .add_attribute("burn_nft_amount", burn_nft_amount)
        .add_attribute("mint_nft_amount", mint_nft_amount))
}
