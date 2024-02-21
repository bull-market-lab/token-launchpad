use crate::{
    error::ContractError,
    util::nft::{
        batch_burn_nft, batch_mint_nft, calculate_nft_to_burn_for_ft_burn,
        calculate_nft_to_mint_for_ft_mint,
    },
};
use cosmwasm_std::{Addr, QuerierWrapper, Response, Storage, Uint128};
use osmosis_std::types::cosmos::bank::v1beta1::Metadata;

fn convert_to_base_denom_and_base_amount(
    denom: &str,
    amount: Uint128,
    metadata: &Metadata,
    one_denom_in_base_denom: Uint128,
) -> (String, Uint128) {
    if denom == metadata.base {
        // user is sending base denom, e.g. uatom, no action needed
        (denom.to_string(), amount)
    } else {
        // user is sending denom, e.g. atom, we need to convert it to base denom
        (metadata.base.clone(), amount * one_denom_in_base_denom)
    }
}

#[allow(clippy::too_many_arguments)]
pub fn track_before_send(
    storage: &mut dyn Storage,
    querier: QuerierWrapper,
    amount: Uint128,
    denom: &str,
    one_denom_in_base_denom: Uint128,
    metadata: &Metadata,
    from_addr: &Addr,
    to_addr: &Addr,
) -> Result<Response, ContractError> {
    // let (base_denom, base_amount) = convert_to_base_denom_and_base_amount(
    //     denom,
    //     amount,
    //     metadata,
    //     one_denom_in_base_denom,
    // );
    // let burn_nft_amount = calculate_nft_to_burn_for_ft_burn(
    //     querier,
    //     from_addr,
    //     base_denom.as_str(),
    //     base_amount,
    //     one_denom_in_base_denom,
    // )?;
    // batch_burn_nft(storage, from_addr, burn_nft_amount)?;
    // let mint_nft_amount = calculate_nft_to_mint_for_ft_mint(
    //     querier,
    //     to_addr,
    //     base_denom.as_str(),
    //     base_amount,
    //     one_denom_in_base_denom,
    // )?;
    // batch_mint_nft(storage, &metadata.uri, to_addr, mint_nft_amount)?;
    Ok(
        Response::new()
            .add_attribute("token_type", "ft")
            .add_attribute("action", "track_before_send")
            .add_attribute("from", from_addr)
            .add_attribute("to", to_addr)
            // .add_attribute("amount_in_base_denom", base_amount)
            // .add_attribute("base_denom", base_denom), 
            // .add_attribute("burn_nft_amount", burn_nft_amount)
                                                      // .add_attribute("mint_nft_amount", mint_nft_amount)
    )
}

pub fn block_before_send(
    storage: &mut dyn Storage,
    querier: QuerierWrapper,
    amount: Uint128,
    denom: &str,
    one_denom_in_base_denom: Uint128,
    metadata: &Metadata,
    from_addr: &Addr,
    to_addr: &Addr,
) -> Result<Response, ContractError> {
    let (base_denom, base_amount) = convert_to_base_denom_and_base_amount(
        denom,
        amount,
        metadata,
        one_denom_in_base_denom,
    );
    let burn_nft_amount = calculate_nft_to_burn_for_ft_burn(
        querier,
        from_addr,
        base_denom.as_str(),
        base_amount,
        one_denom_in_base_denom,
    )?;
    batch_burn_nft(storage, from_addr, burn_nft_amount)?;
    let mint_nft_amount = calculate_nft_to_mint_for_ft_mint(
        querier,
        to_addr,
        base_denom.as_str(),
        base_amount,
        one_denom_in_base_denom,
    )?;
    batch_mint_nft(storage, &metadata.uri, to_addr, mint_nft_amount)?;
    Ok(Response::new()
        .add_attribute("token_type", "ft")
        .add_attribute("action", "block_before_send")
        .add_attribute("from", from_addr)
        .add_attribute("to", to_addr)
        .add_attribute("amount_in_base_denom", base_amount)
        .add_attribute("base_denom", base_denom))
}
