use crate::{
    state::{
        CURRENT_NFT_SUPPLY, NFTS, NFT_OPERATORS, RECYCLED_NFTS,
        RECYCLED_NFT_IDS,
    },
    util::{
        assert_helper::assert_can_send,
        nft::{transfer_nft_helper, update_approvals},
    },
};
use cosmwasm_std::{Addr, Binary, BlockInfo, Response, Storage, Uint128};
use cw721::Cw721ReceiveMsg;
use cw_utils::Expiration;
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin as SdkCoin,
    osmosis::tokenfactory::v1beta1::{MsgBurn, MsgForceTransfer},
};
use shared_pkg::error::ContractError;

pub fn approve_nft(
    storage: &mut dyn Storage,
    block: &BlockInfo,
    sender_addr: &Addr,
    spender_addr: &Addr,
    token_id: u128,
    expires: Option<Expiration>,
) -> Result<Response, ContractError> {
    update_approvals(
        storage,
        block,
        sender_addr,
        spender_addr,
        token_id,
        true,
        expires,
    )?;
    Ok(Response::new()
        .add_attribute("action", "approve")
        .add_attribute("sender", sender_addr)
        .add_attribute("spender", spender_addr)
        .add_attribute("token_id", token_id.to_string()))
}

pub fn approve_all_nft(
    storage: &mut dyn Storage,
    block: &BlockInfo,
    sender_addr: &Addr,
    operator_addr: &Addr,
    expires: Option<Expiration>,
) -> Result<Response, ContractError> {
    // reject expired data as invalid
    let expires = expires.unwrap_or_default();
    if expires.is_expired(block) {
        return Err(ContractError::Expired {});
    }
    NFT_OPERATORS.save(storage, (sender_addr, operator_addr), &expires)?;
    Ok(Response::new()
        .add_attribute("action", "approve_all")
        .add_attribute("sender", sender_addr)
        .add_attribute("operator", operator_addr))
}

pub fn revoke_nft(
    storage: &mut dyn Storage,
    block: &BlockInfo,
    sender_addr: &Addr,
    spender_addr: &Addr,
    token_id: u128,
) -> Result<Response, ContractError> {
    update_approvals(
        storage,
        block,
        sender_addr,
        spender_addr,
        token_id,
        false,
        None,
    )?;
    Ok(Response::new()
        .add_attribute("action", "revoke")
        .add_attribute("sender", sender_addr)
        .add_attribute("spender", spender_addr)
        .add_attribute("token_id", token_id.to_string()))
}

pub fn revoke_all_nft(
    storage: &mut dyn Storage,
    sender_addr: &Addr,
    operator_addr: &Addr,
) -> Result<Response, ContractError> {
    NFT_OPERATORS.remove(storage, (sender_addr, operator_addr));
    Ok(Response::new()
        .add_attribute("action", "revoke_all")
        .add_attribute("sender", sender_addr)
        .add_attribute("operator", operator_addr))
}

pub fn transfer_nft(
    storage: &mut dyn Storage,
    block: &BlockInfo,
    sender_addr: &Addr,
    recipient_addr: &Addr,
    token_id: u128,
    one_denom_in_base_denom: Uint128,
    base_denom: &str,
    contract_addr: &Addr,
) -> Result<Response, ContractError> {
    transfer_nft_helper(storage, block, sender_addr, recipient_addr, token_id)?;
    let msg = MsgForceTransfer {
        sender: contract_addr.to_string(),
        amount: Some(SdkCoin {
            amount: one_denom_in_base_denom.to_string(),
            denom: base_denom.to_string(),
        }),
        transfer_from_address: sender_addr.to_string(),
        transfer_to_address: recipient_addr.to_string(),
    };
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "transfer_nft")
        .add_attribute("sender", sender_addr)
        .add_attribute("recipient", recipient_addr)
        .add_attribute("token_id", token_id.to_string()))
}

pub fn send_nft(
    storage: &mut dyn Storage,
    block: &BlockInfo,
    sender_addr: &Addr,
    token_id: u128,
    one_denom_in_base_denom: Uint128,
    base_denom: &str,
    contract_addr: &Addr,
    recipient_contract_addr: &Addr,
    msg: Binary,
) -> Result<Response, ContractError> {
    transfer_nft_helper(
        storage,
        block,
        sender_addr,
        recipient_contract_addr,
        token_id,
    )?;
    let send = Cw721ReceiveMsg {
        sender: sender_addr.to_string(),
        token_id: token_id.to_string(),
        msg,
    };
    let msg = MsgForceTransfer {
        sender: contract_addr.to_string(),
        amount: Some(SdkCoin {
            amount: one_denom_in_base_denom.to_string(),
            denom: base_denom.to_string(),
        }),
        transfer_from_address: sender_addr.to_string(),
        transfer_to_address: contract_addr.to_string(),
    };
    Ok(Response::new()
        .add_message(msg)
        .add_message(send.into_cosmos_msg(recipient_contract_addr.clone())?)
        .add_attribute("action", "send_nft")
        .add_attribute("sender", sender_addr)
        .add_attribute("recipient", recipient_contract_addr)
        .add_attribute("token_id", token_id.to_string()))
}

pub fn burn_nft(
    storage: &mut dyn Storage,
    block: &BlockInfo,
    contract_addr: &Addr,
    token_id: u128,
    one_denom_in_base_denom: Uint128,
    base_denom: &str,
    sender_addr: &Addr,
) -> Result<Response, ContractError> {
    let current_nft_supply = CURRENT_NFT_SUPPLY.load(storage)?;
    assert_can_send(storage, block, sender_addr, token_id)?;
    let burned_nft = NFTS().load(storage, token_id)?;
    RECYCLED_NFT_IDS.push_back(storage, &token_id)?;
    RECYCLED_NFTS.save(storage, token_id, &burned_nft)?;
    NFTS().remove(storage, token_id)?;
    let msg = MsgBurn {
        sender: contract_addr.to_string(),
        amount: Some(SdkCoin {
            amount: one_denom_in_base_denom.to_string(),
            denom: base_denom.to_string(),
        }),
        burn_from_address: sender_addr.to_string(),
    };
    let updated_nft_supply = current_nft_supply - Uint128::new(1);
    CURRENT_NFT_SUPPLY.save(storage, &updated_nft_supply)?;
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "burn")
        .add_attribute("sender", sender_addr)
        .add_attribute("token_id", token_id.to_string()))
}
