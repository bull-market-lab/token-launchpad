use crate::{
    error::ContractError,
    state::MAX_NFT_SUPPLY,
    util::{
        assert::assert_max_base_denom_supply_not_reached,
        nft::{
            batch_burn_nft, batch_mint_nft, calculate_nft_to_burn_for_ft_burn,
            calculate_nft_to_mint_for_ft_mint,
        },
    },
};
use cosmwasm_std::{
    coins, Addr, BankMsg, QuerierWrapper, Response, Storage, Uint128,
};
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin as SdkCoin,
    osmosis::tokenfactory::v1beta1::{MsgBurn, MsgForceTransfer, MsgMint},
};

pub fn mint_ft(
    storage: &mut dyn Storage,
    querier: QuerierWrapper,
    amount: Uint128,
    one_denom_in_base_denom: Uint128,
    base_denom: &str,
    base_uri: &str,
    contract_addr: &Addr,
) -> Result<Response, ContractError> {
    let current_base_denom_supply = querier.query_supply(base_denom)?.amount;
    let max_nft_supply = MAX_NFT_SUPPLY.load(storage)?;
    assert_max_base_denom_supply_not_reached(
        current_base_denom_supply,
        max_nft_supply * one_denom_in_base_denom,
        amount,
    )?;
    let mint_nft_amount = calculate_nft_to_mint_for_ft_mint(
        querier,
        contract_addr,
        base_denom,
        amount,
        one_denom_in_base_denom,
    )?;
    batch_mint_nft(
        storage,
        querier,
        base_denom,
        base_uri,
        one_denom_in_base_denom,
        contract_addr,
        mint_nft_amount,
    )?;
    let mint_ft_msg = MsgMint {
        sender: contract_addr.to_string(),
        amount: Some(SdkCoin {
            amount: amount.to_string(),
            denom: base_denom.to_string(),
        }),
        mint_to_address: contract_addr.to_string(),
    };
    Ok(Response::new()
        .add_message(mint_ft_msg)
        .add_attribute("token_type", "ft")
        .add_attribute("action", "mint_ft")
        .add_attribute("amount", amount)
        .add_attribute("mint_nft_amount", mint_nft_amount))
}

pub fn burn_ft(
    storage: &mut dyn Storage,
    querier: QuerierWrapper,
    amount: Uint128,
    one_denom_in_base_denom: Uint128,
    base_denom: &str,
    contract_addr: &Addr,
) -> Result<Response, ContractError> {
    let burn_nft_amount = calculate_nft_to_burn_for_ft_burn(
        querier,
        contract_addr,
        base_denom,
        amount,
        one_denom_in_base_denom,
    )?;
    batch_burn_nft(storage, contract_addr, burn_nft_amount)?;
    let msg = MsgBurn {
        sender: contract_addr.to_string(),
        amount: Some(SdkCoin {
            amount: amount.to_string(),
            denom: base_denom.to_string(),
        }),
        burn_from_address: contract_addr.to_string(),
    };
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("token_type", "ft")
        .add_attribute("action", "burn_ft")
        .add_attribute("amount", amount)
        .add_attribute("burn_nft_amount", burn_nft_amount))
}

#[allow(clippy::too_many_arguments)]
pub fn send_ft(
    storage: &mut dyn Storage,
    querier: QuerierWrapper,
    amount: Uint128,
    base_denom: &str,
    one_denom_in_base_denom: Uint128,
    recipient_addr: &Addr,
    base_uri: &str,
    contract_addr: &Addr,
) -> Result<Response, ContractError> {
    let burn_nft_amount = calculate_nft_to_burn_for_ft_burn(
        querier,
        contract_addr,
        base_denom,
        amount,
        one_denom_in_base_denom,
    )?;
    batch_burn_nft(storage, contract_addr, burn_nft_amount)?;
    let mint_nft_amount = calculate_nft_to_mint_for_ft_mint(
        querier,
        contract_addr,
        base_denom,
        amount,
        one_denom_in_base_denom,
    )?;
    batch_mint_nft(
        storage,
        querier,
        base_denom,
        base_uri,
        one_denom_in_base_denom,
        recipient_addr,
        mint_nft_amount,
    )?;
    let msg = BankMsg::Send {
        to_address: recipient_addr.to_string(),
        amount: coins(amount.u128(), base_denom),
    };
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("token_type", "ft")
        .add_attribute("action", "send_ft")
        .add_attribute("amount", amount)
        .add_attribute("recipient_addr", recipient_addr)
        .add_attribute("burn_nft_amount", burn_nft_amount)
        .add_attribute("mint_nft_amount", mint_nft_amount))
}

#[allow(clippy::too_many_arguments)]
pub fn force_transfer_ft(
    storage: &mut dyn Storage,
    querier: QuerierWrapper,
    amount: Uint128,
    base_denom: &str,
    one_denom_in_base_denom: Uint128,
    base_uri: &str,
    contract_addr: &Addr,
    from_addr: &Addr,
    to_addr: &Addr,
) -> Result<Response, ContractError> {
    let burn_nft_amount = calculate_nft_to_burn_for_ft_burn(
        querier,
        from_addr,
        base_denom,
        amount,
        one_denom_in_base_denom,
    )?;
    batch_burn_nft(storage, from_addr, burn_nft_amount)?;
    let mint_nft_amount = calculate_nft_to_mint_for_ft_mint(
        querier,
        from_addr,
        base_denom,
        amount,
        one_denom_in_base_denom,
    )?;
    batch_mint_nft(
        storage,
        querier,
        base_denom,
        base_uri,
        one_denom_in_base_denom,
        to_addr,
        mint_nft_amount,
    )?;
    let msg = MsgForceTransfer {
        sender: contract_addr.to_string(),
        amount: Some(SdkCoin {
            amount: amount.to_string(),
            denom: base_denom.to_string(),
        }),
        transfer_from_address: from_addr.to_string(),
        transfer_to_address: to_addr.to_string(),
    };
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("token_type", "ft")
        .add_attribute("action", "force_transfer_ft")
        .add_attribute("amount", amount)
        .add_attribute("from", from_addr)
        .add_attribute("to", to_addr)
        .add_attribute("burn_nft_amount", burn_nft_amount)
        .add_attribute("mint_nft_amount", mint_nft_amount))
}
