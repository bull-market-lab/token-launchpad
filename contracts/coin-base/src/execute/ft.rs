use crate::util::assert_helper::assert_max_base_denom_supply_not_reached;
use coin::config::Config;
use cosmwasm_std::{coins, Addr, BankMsg, QuerierWrapper, Response, Uint128};
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin as SdkCoin,
    osmosis::tokenfactory::v1beta1::{MsgBurn, MsgForceTransfer, MsgMint},
};
use shared_pkg::error::ContractError;

pub fn mint(
    querier: QuerierWrapper,
    config: &Config,
    mint_amount: Uint128,
    base_denom: &str,
    contract_addr: &Addr,
    recipient_addr: &Addr,
) -> Result<Response, ContractError> {
    assert_max_base_denom_supply_not_reached(
        querier.query_supply(base_denom)?.amount,
        config.max_supply_in_base_denom,
        mint_amount,
    )?;
    let mint_ft_msg = MsgMint {
        sender: contract_addr.to_string(),
        amount: Some(SdkCoin {
            amount: mint_amount.to_string(),
            denom: base_denom.to_string(),
        }),
        // TODO: test if we can mint straight to the recipient
        mint_to_address: contract_addr.to_string(),
    };
    let bank_msg = BankMsg::Send {
        to_address: recipient_addr.to_string(),
        amount: coins(mint_amount.u128(), base_denom),
    };
    Ok(Response::new()
        .add_message(mint_ft_msg)
        .add_message(bank_msg)
        .add_attribute("action", "mint")
        .add_attribute("amount", mint_amount)
        .add_attribute("recipient", recipient_addr))
}

pub fn burn(
    amount: Uint128,
    base_denom: &str,
    contract_addr: &Addr,
) -> Result<Response, ContractError> {
    let msg = MsgBurn {
        sender: contract_addr.to_string(),
        amount: Some(SdkCoin {
            amount: amount.to_string(),
            denom: base_denom.to_string(),
        }),
        // TODO: test if we can burn straight from the a designated address
        burn_from_address: contract_addr.to_string(),
    };
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "burn")
        .add_attribute("amount", amount))
}

pub fn force_transfer(
    amount: Uint128,
    base_denom: &str,
    contract_addr: &Addr,
    from_addr: &Addr,
    to_addr: &Addr,
) -> Result<Response, ContractError> {
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
        .add_attribute("action", "force_transfer")
        .add_attribute("amount", amount)
        .add_attribute("from", from_addr)
        .add_attribute("to", to_addr))
}
