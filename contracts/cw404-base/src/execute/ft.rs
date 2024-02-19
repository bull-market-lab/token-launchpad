use crate::{
    error::ContractError,
    util::{
        assert::assert_max_base_denom_supply_not_reached, nft::batch_mint_nft,
    },
};
use cosmwasm_std::{
    coins, Addr, BankMsg, BankQuery, DepsMut, QueryRequest, Response,
    SupplyResponse, Uint128, Uint64,
};
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin as SdkCoin,
    osmosis::tokenfactory::v1beta1::{MsgBurn, MsgForceTransfer, MsgMint},
};

pub fn mint_tokens(
    deps: DepsMut,
    amount: Uint128,
    max_denom_supply: Uint64,
    base_denom: String,
    one_denom_in_base_denom: Uint128,
    denom: String,
    contract_addr_str: String,
    sender_addr_ref: &Addr,
) -> Result<Response, ContractError> {
    let current_base_denom_supply: SupplyResponse =
        deps.querier.query(&QueryRequest::Bank(BankQuery::Supply {
            denom: base_denom.clone(),
        }))?;
    assert_max_base_denom_supply_not_reached(
        current_base_denom_supply.amount.amount,
        Uint128::from(max_denom_supply) * one_denom_in_base_denom,
        amount,
    )?;

    batch_mint_nft(
        deps,
        contract_addr_str.clone(),
        one_denom_in_base_denom,
        base_denom,
        contract_addr_str.clone(),
        sender_addr_ref,
        max_denom_supply,
        Uint64::from((amount / one_denom_in_base_denom).u128() as u64),
    )?;

    let mint_ft_msg = MsgMint {
        sender: contract_addr_str.clone(),
        amount: Some(SdkCoin {
            amount: amount.to_string(),
            denom: denom.clone(),
        }),
        mint_to_address: contract_addr_str,
    };
    Ok(Response::new()
        .add_message(mint_ft_msg)
        .add_attribute("token_type", "ft")
        .add_attribute("action", "mint_tokens")
        .add_attribute("amount", amount))
}

pub fn burn_tokens(
    amount: Uint128,
    denom: String,
    contract_addr_str: String,
) -> Result<Response, ContractError> {
    let msg = MsgBurn {
        sender: contract_addr_str.clone(),
        amount: Some(SdkCoin {
            amount: amount.to_string(),
            denom: denom.clone(),
        }),
        burn_from_address: contract_addr_str,
    };
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("token_type", "ft")
        .add_attribute("action", "burn_tokens")
        .add_attribute("amount", amount))
}

pub fn send_tokens(
    amount: Uint128,
    denom: String,
    recipient_addr: String,
) -> Result<Response, ContractError> {
    let msg = BankMsg::Send {
        to_address: recipient_addr.clone(),
        amount: coins(amount.u128(), denom),
    };
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("token_type", "ft")
        .add_attribute("action", "send_tokens")
        .add_attribute("amount", amount)
        .add_attribute("recipient_addr", recipient_addr))
}

pub fn force_transfer(
    amount: Uint128,
    denom: String,
    contract_addr_str: String,
    from: String,
    to: String,
) -> Result<Response, ContractError> {
    let msg = MsgForceTransfer {
        sender: contract_addr_str,
        amount: Some(SdkCoin {
            amount: amount.to_string(),
            denom: denom.clone(),
        }),
        transfer_from_address: from.clone(),
        transfer_to_address: to.clone(),
    };
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("token_type", "ft")
        .add_attribute("action", "force_transfer")
        .add_attribute("amount", amount)
        .add_attribute("from", from)
        .add_attribute("to", to))
}
