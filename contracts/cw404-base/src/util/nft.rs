use crate::{
    error::ContractError, state::RECYCLED_NFT_IDS,
    util::assert::assert_max_nft_supply_not_reached,
};
use cosmwasm_std::{Addr, DepsMut, Empty, Uint128, Uint64};
use cw721_base::{state::TokenInfo, Cw721Contract};

pub fn mint_nft(
    deps: DepsMut,
    contract_addr_str: String,
    one_denom_in_base_denom: Uint128,
    base_denom: String,
    owner: String,
    sender_addr_ref: &Addr,
    max_denom_supply: Uint64,
) -> Result<(), ContractError> {
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
    Ok(())
}

pub fn batch_mint_nft(
    deps: DepsMut,
    contract_addr_str: String,
    one_denom_in_base_denom: Uint128,
    base_denom: String,
    owner: String,
    sender_addr_ref: &Addr,
    max_denom_supply: Uint64,
    amount: Uint64,
) -> Result<(), ContractError> {
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

    Ok(())
}
