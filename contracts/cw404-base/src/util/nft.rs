use crate::{
    error::ContractError,
    state::{CURRENT_NFT_SUPPLY, NFTS, RECYCLED_NFT_IDS},
    util::assert::assert_max_nft_supply_not_reached,
};
use cosmwasm_std::{
    Addr, BlockInfo, DepsMut, Order, StdError, StdResult, Storage, Uint128,
    Uint64,
};
use cw404::nft::Nft;
use cw721::Approval;
use cw_utils::Expiration;

use super::assert::{assert_can_send, assert_can_update_approvals};

fn humanize_approval(approval: &Approval) -> Approval {
    Approval {
        spender: approval.clone().spender,
        expires: approval.expires,
    }
}
pub fn humanize_approvals(
    block: &BlockInfo,
    nft: &Nft,
    include_expired: bool,
) -> Vec<Approval> {
    nft.approvals
        .iter()
        .filter(|apr| include_expired || !apr.expires.is_expired(block))
        .map(humanize_approval)
        .collect()
}

pub fn parse_token_id_from_string_to_uint64(
    token_id: String,
) -> StdResult<Uint64> {
    let token_id_in_u64 = token_id
        .parse::<u64>()
        .map_err(|_| StdError::generic_err("token_id is not a valid u64"))?;
    Ok(Uint64::from(token_id_in_u64))
}

pub fn calculate_nft_to_mint_for_ft_mint(
    owner_addr: String,
    ft_mint_amount: Uint128,
    one_denom_in_base_denom: Uint128,
) -> Uint64 {
    Uint64::zero()
    // get before nft balance
    // get before ft balance
    // calculate after ft balance
    // calculate after nft balance
    // mint amount = after nft balance - before nft balance
}

pub fn calculate_nft_to_burn_for_ft_burn() -> Uint64 {
    Uint64::zero()
}

pub fn batch_mint_nft(
    deps: DepsMut,
    owner: String,
    max_denom_supply: Uint64,
    amount: Uint64,
) -> Result<(), ContractError> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    let storage_mut_ref = deps.storage;

    let current_nft_supply = CURRENT_NFT_SUPPLY.load(storage_mut_ref)?;
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
        NFTS().update(storage_mut_ref, token_id.u64(), |old| match old {
            Some(_) => Err(ContractError::TokenIdAlreadyInUse { token_id }),
            None => Ok(Nft {
                owner: owner_addr.clone(),
                approvals: vec![],
                token_uri: None,
            }),
        })?;
    }

    let updated_nft_supply = current_nft_supply + amount;
    CURRENT_NFT_SUPPLY.save(storage_mut_ref, &updated_nft_supply)?;

    Ok(())
}

pub fn batch_burn_nft(
    deps: DepsMut,
    owner: String,
    amount: Uint64,
) -> Result<(), ContractError> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    let storage_mut_ref = deps.storage;

    let current_nft_supply = CURRENT_NFT_SUPPLY.load(storage_mut_ref)?;

    let token_ids: Vec<u64> = NFTS()
        .idx
        .owner
        .prefix(owner_addr)
        .keys(storage_mut_ref, None, None, Order::Ascending)
        .take(amount.u64() as usize)
        .collect::<StdResult<Vec<_>>>()?;
    if token_ids.len() != amount.u64() as usize {
        return Err(ContractError::CannotBurnMoreNftThanOwned {
            available: Uint64::from(token_ids.len() as u64),
            try_to_burn: amount,
        });
    }

    for token_id in token_ids {
        NFTS().remove(storage_mut_ref, token_id)?;
    }
    let updated_nft_supply = current_nft_supply - amount;
    CURRENT_NFT_SUPPLY.save(storage_mut_ref, &updated_nft_supply)?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn update_approvals(
    storage: &mut dyn Storage,
    block: &BlockInfo,
    sender_addr: &Addr,
    spender_addr: &Addr,
    token_id: Uint64,
    // if add == false, remove. if add == true, remove then set with this expiration
    add: bool,
    expires: Option<Expiration>,
) -> Result<Nft, ContractError> {
    let mut nft = NFTS().load(storage, token_id.u64())?;
    // ensure we have permissions
    assert_can_update_approvals(storage, block, &nft.owner, sender_addr)?;

    // update the approval list (remove any for the same spender before adding)
    nft.approvals
        .retain(|apr| apr.spender != spender_addr.to_string());

    // only difference between approve and revoke
    if add {
        // reject expired data as invalid
        let expires = expires.unwrap_or_default();
        if expires.is_expired(block) {
            return Err(ContractError::Expired {});
        }
        let approval = Approval {
            spender: spender_addr.to_string(),
            expires,
        };
        nft.approvals.push(approval);
    }

    NFTS().save(storage, token_id.u64(), &nft)?;

    Ok(nft)
}

pub fn transfer_nft_helper(
    storage: &mut dyn Storage,
    block: &BlockInfo,
    sender_addr: &Addr,
    recipient_addr: &Addr,
    token_id: Uint64,
) -> Result<Nft, ContractError> {
    let mut nft = NFTS().load(storage, token_id.u64())?;
    // ensure we have permissions
    assert_can_send(storage, block, sender_addr, token_id)?;
    // set owner and remove existing approvals
    nft.owner = recipient_addr.clone();
    nft.approvals = vec![];
    NFTS().save(storage, token_id.u64(), &nft)?;
    Ok(nft)
}
