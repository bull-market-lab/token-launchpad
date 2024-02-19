use crate::error::ContractError;
use cosmwasm_std::{Addr, BlockInfo, Empty, Storage, Uint128, Uint64};
use cw721_base::Cw721Contract;

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
    cw721_base_contract: &Cw721Contract<'_, Empty, Empty, Empty, Empty>,
    token_id: &String,
) -> Result<(), ContractError> {
    let token = cw721_base_contract.tokens.load(storage, &token_id)?;

    // owner can send
    if token.owner == sender_addr_ref {
        return Ok(());
    }

    // any non-expired token approval can send
    if token
        .approvals
        .iter()
        .any(|apr| apr.spender == sender_addr_ref && !apr.is_expired(block))
    {
        return Ok(());
    }

    // operator can send
    let op = cw721_base_contract
        .operators
        .may_load(storage, (&token.owner, sender_addr_ref))?;
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
