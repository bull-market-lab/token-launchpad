use crate::{
    error::ContractError,
    util::{
        assert::assert_max_base_denom_supply_not_reached,
        nft::{batch_burn_nft, batch_mint_nft},
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
    max_denom_supply: Uint128,
    base_denom: String,
    one_denom_in_base_denom: Uint128,
    denom: String,
    contract_addr: &Addr,
) -> Result<Response, ContractError> {
    let current_base_denom_supply: SupplyResponse =
        querier.query(&QueryRequest::Bank(BankQuery::Supply {
            denom: base_denom.clone(),
        }))?;
    assert_max_base_denom_supply_not_reached(
        current_base_denom_supply.amount.amount,
        Uint128::from(max_denom_supply) * one_denom_in_base_denom,
        amount,
    )?;

    batch_mint_nft(
        storage,
        contract_addr,
        max_denom_supply,
        amount / one_denom_in_base_denom,
    )?;

    let mint_ft_msg = MsgMint {
        sender: contract_addr.to_string(),
        amount: Some(SdkCoin {
            amount: amount.to_string(),
            denom: denom.clone(),
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
    amount: Uint128,
    one_denom_in_base_denom: Uint128,
    denom: String,
    contract_addr: &Addr,
) -> Result<Response, ContractError> {
    batch_burn_nft(storage, contract_addr, amount / one_denom_in_base_denom)?;

    let msg = MsgBurn {
        sender: contract_addr.to_string(),
        amount: Some(SdkCoin {
            amount: amount.to_string(),
            denom: denom.clone(),
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
        .add_attribute("action", "send_ft")
        .add_attribute("amount", amount)
        .add_attribute("recipient_addr", recipient_addr))
}

pub fn force_transfer_ft(
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
        .add_attribute("action", "force_transfer_ft")
        .add_attribute("amount", amount)
        .add_attribute("from", from)
        .add_attribute("to", to))
}
