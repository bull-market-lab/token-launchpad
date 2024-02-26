use crate::{
    error::ContractError,
    state::FEE_DENOM,
    util::nft::{
        assert_can_mint, batch_burn_nft, batch_mint_nft,
        calculate_nft_to_burn_for_ft_burn, calculate_nft_to_mint_for_ft_mint,
    },
};
use cosmwasm_std::{
    coins, Addr, BankMsg, BlockInfo, QuerierWrapper, Response, Storage, Uint128,
};
use cw404::config::Config;
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin as SdkCoin,
    osmosis::tokenfactory::v1beta1::{MsgBurn, MsgForceTransfer, MsgMint},
};

pub fn mint_ft(
    storage: &mut dyn Storage,
    querier: QuerierWrapper,
    block: &BlockInfo,
    config: &Config,
    mint_amount: Uint128,
    one_denom_in_base_denom: Uint128,
    base_denom: &str,
    base_uri: &str,
    contract_addr: &Addr,
    recipient_addr: &Addr,
    user_paid_amount: Uint128,
    mint_group_name: &str,
    merkle_proof: Option<Vec<Vec<u8>>>,
) -> Result<Response, ContractError> {
    assert_can_mint(
        storage,
        querier,
        block,
        mint_amount,
        one_denom_in_base_denom,
        base_denom,
        recipient_addr,
        user_paid_amount,
        mint_group_name,
        merkle_proof,
    )?;
    let mint_nft_amount = calculate_nft_to_mint_for_ft_mint(
        querier,
        contract_addr,
        base_denom,
        mint_amount,
        one_denom_in_base_denom,
    )?;
    batch_mint_nft(storage, base_uri, contract_addr, mint_nft_amount)?;
    let mint_ft_msg = MsgMint {
        sender: contract_addr.to_string(),
        amount: Some(SdkCoin {
            amount: mint_amount.to_string(),
            denom: base_denom.to_string(),
        }),
        // TODO: test if we can mint straight to the recipient
        mint_to_address: contract_addr.to_string(),
    };
    let mut bank_msgs = vec![BankMsg::Send {
        to_address: recipient_addr.to_string(),
        amount: coins(mint_amount.u128(), base_denom),
    }];
    if !user_paid_amount.is_zero() {
        bank_msgs.push(BankMsg::Send {
            to_address: config.royalty_payment_address.to_string(),
            amount: coins(user_paid_amount.u128(), FEE_DENOM),
        });
    }
    Ok(Response::new()
        .add_message(mint_ft_msg)
        .add_messages(bank_msgs)
        .add_attribute("token_type", "ft")
        .add_attribute("action", "mint_ft")
        .add_attribute("amount", mint_amount)
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
        // TODO: test if we can burn straight from the a designated address
        burn_from_address: contract_addr.to_string(),
    };
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("token_type", "ft")
        .add_attribute("action", "burn_ft")
        .add_attribute("amount", amount)
        .add_attribute("burn_nft_amount", burn_nft_amount))
}

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
        to_addr,
        base_denom,
        amount,
        one_denom_in_base_denom,
    )?;
    batch_mint_nft(storage, base_uri, to_addr, mint_nft_amount)?;
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
