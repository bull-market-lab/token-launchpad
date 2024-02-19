use crate::{
    error::ContractError, state::RECYCLED_NFT_IDS,
    util::assert::assert_max_nft_supply_not_reached,
};
use cosmwasm_std::{
    Addr, Binary, DepsMut, Empty, Env, MessageInfo, Response, Storage, Uint128,
    Uint64,
};
use cw721_base::{
    entry::execute as cw721_execute, msg::ExecuteMsg as Cw721ExecuteMsg,
    state::TokenInfo, Cw721Contract,
};
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin as SdkCoin,
    osmosis::tokenfactory::v1beta1::{MsgBurn, MsgForceTransfer, MsgMint},
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

pub fn mint(
    deps: DepsMut,
    contract_addr_str: String,
    one_denom_in_base_denom: Uint128,
    base_denom: String,
    owner: String,
    sender_addr_ref: &Addr,
    max_denom_supply: Uint64,
) -> Result<Response, ContractError> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    let storage_mut_ref = deps.storage;

    let cw721_base_contract =
        Cw721Contract::<Empty, Empty, Empty, Empty>::default();
    let current_nft_supply =
        Uint64::from(cw721_base_contract.token_count(storage_mut_ref)?);
    assert_max_nft_supply_not_reached(
        current_nft_supply,
        max_denom_supply,
        Uint64::one(),
    )?;

    let token_id = if RECYCLED_NFT_IDS.len(storage_mut_ref)? > 0 {
        RECYCLED_NFT_IDS.pop_front(storage_mut_ref)?.unwrap()
    } else {
        // token_id starts from 1, so when current_nft_supply is 0, the next token_id is 1
        current_nft_supply + Uint64::one()
    };
    let token = TokenInfo {
        owner: owner_addr,
        approvals: vec![],
        // todo: add token_uri
        token_uri: None,
        extension: Empty {},
    };
    cw721_base_contract.tokens.update(
        storage_mut_ref,
        &token_id.to_string(),
        |old| match old {
            Some(_) => Err(ContractError::TokenIdAlreadyInUse { token_id }),
            None => Ok(token),
        },
    )?;
    cw721_base_contract.increment_tokens(storage_mut_ref)?;

    let msg = MsgMint {
        sender: contract_addr_str,
        amount: Some(SdkCoin {
            amount: one_denom_in_base_denom.to_string(),
            denom: base_denom,
        }),
        mint_to_address: owner.clone(),
    };
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "mint")
        .add_attribute("sender", sender_addr_ref)
        .add_attribute("owner", owner)
        .add_attribute("token_id", token_id))
}

pub fn batch_mint(
    deps: DepsMut,
    contract_addr_str: String,
    one_denom_in_base_denom: Uint128,
    base_denom: String,
    owner: String,
    sender_addr_ref: &Addr,
    max_denom_supply: Uint64,
    amount: Uint64,
    mint_ft: bool,
) -> Result<Response, ContractError> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    let storage_mut_ref = deps.storage;

    let cw721_base_contract =
        Cw721Contract::<Empty, Empty, Empty, Empty>::default();
    let current_nft_supply =
        Uint64::from(cw721_base_contract.token_count(storage_mut_ref)?);
    assert_max_nft_supply_not_reached(
        current_nft_supply,
        max_denom_supply,
        Uint64::one(),
    )?;

    for _ in 0..amount.into() {
        let token_id = if RECYCLED_NFT_IDS.len(storage_mut_ref)? > 0 {
            RECYCLED_NFT_IDS.pop_front(storage_mut_ref)?.unwrap()
        } else {
            // token_id starts from 1, so when current_nft_supply is 0, the next token_id is 1
            current_nft_supply + Uint64::one()
        };
        let token = TokenInfo {
            owner: owner_addr.clone(),
            approvals: vec![],
            // todo: add token_uri
            token_uri: None,
            extension: Empty {},
        };
        cw721_base_contract.tokens.update(
            storage_mut_ref,
            &token_id.to_string(),
            |old| match old {
                Some(_) => Err(ContractError::TokenIdAlreadyInUse { token_id }),
                None => Ok(token),
            },
        )?;
    }
    cw721_base_contract
        .token_count
        .save(storage_mut_ref, &((current_nft_supply + amount).u64()))?;

    let msgs = if mint_ft {
        vec![MsgMint {
            sender: contract_addr_str,
            amount: Some(SdkCoin {
                amount: (one_denom_in_base_denom * Uint128::from(amount))
                    .to_string(),
                denom: base_denom,
            }),
            mint_to_address: owner.clone(),
        }]
    } else {
        vec![]
    };
    Ok(Response::new()
        .add_messages(msgs)
        .add_attribute("action", "batch_mint")
        .add_attribute("sender", sender_addr_ref)
        .add_attribute("owner", owner)
        .add_attribute("amount", amount))
}
