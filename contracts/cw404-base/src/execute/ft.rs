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
    coins, Addr, BankMsg, BankQuery, QuerierWrapper, QueryRequest, Response,
    Storage, SupplyResponse, Uint128,
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
    denom: &str,
    contract_addr: &Addr,
) -> Result<Response, ContractError> {
    let current_base_denom_supply = querier.query_supply(denom)?.amount;

    let max_nft_supply = MAX_NFT_SUPPLY.load(storage)?;
    assert_max_base_denom_supply_not_reached(
        current_base_denom_supply,
        max_nft_supply * one_denom_in_base_denom,
        amount,
    )?;

    let mint_nft_amount = calculate_nft_to_mint_for_ft_mint(
        querier,
        contract_addr,
        denom,
        amount,
        one_denom_in_base_denom,
    )?;
    batch_mint_nft(storage, contract_addr, max_nft_supply, mint_nft_amount)?;

    let mint_ft_msg = MsgMint {
        sender: contract_addr.to_string(),
        amount: Some(SdkCoin {
            amount: amount.to_string(),
            denom: denom.to_string(),
        }),
        mint_to_address: contract_addr.to_string(),
    };
    Ok(Response::new()
        .add_message(mint_ft_msg)
        .add_attribute("token_type", "ft")
        .add_attribute("action", "mint_ft")
        .add_attribute("amount", amount))
}

pub fn burn_ft(
    storage: &mut dyn Storage,
    querier: QuerierWrapper,
    amount: Uint128,
    one_denom_in_base_denom: Uint128,
    denom: &str,
    contract_addr: &Addr,
) -> Result<Response, ContractError> {
    let burn_nft_amount = calculate_nft_to_burn_for_ft_burn(
        querier,
        contract_addr,
        denom,
        amount,
        one_denom_in_base_denom,
    )?;
    batch_burn_nft(storage, contract_addr, burn_nft_amount)?;

    let msg = MsgBurn {
        sender: contract_addr.to_string(),
        amount: Some(SdkCoin {
            amount: amount.to_string(),
            denom: denom.to_string(),
        }),
        burn_from_address: contract_addr.to_string(),
    };
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("token_type", "ft")
        .add_attribute("action", "burn_ft")
        .add_attribute("amount", amount))
}

pub fn send_ft(
    amount: Uint128,
    denom: &str,
    recipient_addr: String,
) -> Result<Response, ContractError> {
    let msg = BankMsg::Send {
        to_address: recipient_addr.clone(),
        amount: coins(amount.u128(), denom),
    };
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("token_type", "ft")
        .add_attribute("action", "send_ft")
        .add_attribute("amount", amount)
        .add_attribute("recipient_addr", recipient_addr))
}

pub fn force_transfer_ft(
    amount: Uint128,
    denom: &str,
    contract_addr_str: String,
    from: String,
    to: String,
) -> Result<Response, ContractError> {
    let msg = MsgForceTransfer {
        sender: contract_addr_str,
        amount: Some(SdkCoin {
            amount: amount.to_string(),
            denom: denom.to_string(),
        }),
        transfer_from_address: from.clone(),
        transfer_to_address: to.clone(),
    };
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("token_type", "ft")
        .add_attribute("action", "force_transfer_ft")
        .add_attribute("amount", amount)
        .add_attribute("from", from)
        .add_attribute("to", to))
}
