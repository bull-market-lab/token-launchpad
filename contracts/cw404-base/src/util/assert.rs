use crate::{
    error::ContractError,
    state::{NFTS, NFT_OPERATORS},
};
use cosmwasm_std::{Addr, BlockInfo, Storage, Uint128, Uint64};

pub fn assert_only_admin_can_call_this_function(
    sender: &Addr,
    admin: &Addr,
    function: &str,
) -> Result<(), ContractError> {
    if sender != admin {
        return Err(ContractError::OnlyAdminCanCallThisFunction {
            function: function.to_string(),
        });
    }
    Ok(())
}

pub fn assert_max_base_denom_supply_not_reached(
    current_base_denom_supply: Uint128,
    max_base_denom_supply: Uint128,
    mint_amount: Uint128,
) -> Result<(), ContractError> {
    if current_base_denom_supply + mint_amount > max_base_denom_supply {
        return Err(ContractError::MaxBaseDenomSupplyReached {
            current_base_denom_supply,
            max_base_denom_supply,
            mint_amount,
        });
    }
    Ok(())
}

pub fn assert_max_nft_supply_not_reached(
    current_nft_supply: Uint64,
    max_nft_supply: Uint64,
    mint_amount: Uint64,
) -> Result<(), ContractError> {
    if current_nft_supply + mint_amount > max_nft_supply {
        return Err(ContractError::MaxNftSupplyReached {
            current_nft_supply,
            max_nft_supply,
            mint_amount,
        });
    }
    Ok(())
}

/// returns true if the sender can transfer ownership of the token
pub fn assert_can_send(
    storage: &dyn Storage,
    block: &BlockInfo,
    sender_addr_ref: &Addr,
    token_id: Uint64,
) -> Result<(), ContractError> {
    let nft = NFTS().load(storage, token_id.u64())?;

    // owner can send
    if nft.owner == sender_addr_ref {
        return Ok(());
    }

    // any non-expired token approval can send
    if nft.approvals.iter().any(|apr| {
        apr.spender == sender_addr_ref.to_string()
            && !apr.expires.is_expired(block)
    }) {
        return Ok(());
    }

    // operator can send
    let op = NFT_OPERATORS.may_load(storage, (&nft.owner, sender_addr_ref))?;
    match op {
        Some(ex) => {
            if ex.is_expired(block) {
                Err(ContractError::NoAccessToSend {})
            } else {
                Ok(())
            }
        }
        None => Err(ContractError::NoAccessToSend {}),
    }
}

/// returns true if the sender can execute approve or reject on the contract
pub fn assert_can_update_approvals(
    storage: &dyn Storage,
    block: &BlockInfo,
    owner_addr_ref: &Addr,
    sender_addr_ref: &Addr,
) -> Result<(), ContractError> {
    // owner can approve
    if owner_addr_ref == sender_addr_ref {
        return Ok(());
    }
    // operator can approve
    let op =
        NFT_OPERATORS.may_load(storage, (owner_addr_ref, sender_addr_ref))?;
    match op {
        Some(ex) => {
            if ex.is_expired(block) {
                Err(ContractError::NoAccessToApproval {})
            } else {
                Ok(())
            }
        }
        None => Err(ContractError::NoAccessToApproval {}),
    }
}
