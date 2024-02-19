use crate::{error::ContractError, state::RECYCLED_NFT_IDS};
use cosmwasm_std::{
    Addr, Binary, DepsMut, Empty, Env, MessageInfo, Response, Storage, Uint128,
    Uint64,
};
use cw721_base::{
    entry::execute as cw721_execute, msg::ExecuteMsg as Cw721ExecuteMsg,
    Cw721Contract,
};
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin as SdkCoin,
    osmosis::tokenfactory::v1beta1::{MsgBurn, MsgForceTransfer},
};

pub fn transfer_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    contract_addr_str: String,
    recipient: String,
    token_id: String,
    one_denom_in_base_denom: Uint128,
    base_denom: String,
    sender_addr_ref: &Addr,
) -> Result<Response, ContractError> {
    let resp = cw721_execute(
        deps,
        env,
        info,
        Cw721ExecuteMsg::TransferNft {
            recipient: recipient.clone(),
            token_id,
        },
    )?;
    let msg = MsgForceTransfer {
        sender: contract_addr_str,
        amount: Some(SdkCoin {
            amount: one_denom_in_base_denom.to_string(),
            denom: base_denom,
        }),
        transfer_from_address: sender_addr_ref.to_string(),
        transfer_to_address: recipient,
    };
    Ok(resp.add_message(msg))
}

pub fn send_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    contract_addr_str: String,
    contract: String,
    token_id: String,
    msg: Binary,
    one_denom_in_base_denom: Uint128,
    base_denom: String,
    sender_addr_ref: &Addr,
) -> Result<Response, ContractError> {
    let resp = cw721_execute(
        deps,
        env,
        info,
        Cw721ExecuteMsg::SendNft {
            contract: contract.clone(),
            token_id,
            msg,
        },
    )?;
    let msg = MsgForceTransfer {
        sender: contract_addr_str,
        amount: Some(SdkCoin {
            amount: one_denom_in_base_denom.to_string(),
            denom: base_denom,
        }),
        transfer_from_address: sender_addr_ref.to_string(),
        transfer_to_address: contract,
    };
    Ok(resp.add_message(msg))
}

pub fn burn(
    storage_mut_ref: &mut dyn Storage,
    cw721_base_contract: &Cw721Contract<Empty, Empty, Empty, Empty>,
    contract_addr_str: String,
    token_id: String,
    one_denom_in_base_denom: Uint128,
    base_denom: String,
    sender_addr_ref: &Addr,
) -> Result<Response, ContractError> {
    let token_id_in_u64: u64 = token_id.parse().unwrap();
    RECYCLED_NFT_IDS
        .push_back(storage_mut_ref, &Uint64::from(token_id_in_u64))?;

    cw721_base_contract
        .tokens
        .remove(storage_mut_ref, &token_id)?;
    cw721_base_contract.decrement_tokens(storage_mut_ref)?;

    let msg = MsgBurn {
        sender: contract_addr_str,
        amount: Some(SdkCoin {
            amount: one_denom_in_base_denom.to_string(),
            denom: base_denom,
        }),
        burn_from_address: sender_addr_ref.to_string(),
    };
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "burn")
        .add_attribute("sender", sender_addr_ref)
        .add_attribute("token_id", token_id))
}
