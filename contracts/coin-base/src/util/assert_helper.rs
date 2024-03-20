use cosmwasm_std::{Addr, Uint128};
use shared_pkg::error::ContractError;

pub fn assert_only_admin_can_call_this_function(
    sender: &Addr,
    admin: &Option<Addr>,
    function: &str,
) -> Result<(), ContractError> {
    match admin {
        Some(admin) => {
            if sender != admin {
                return Err(ContractError::OnlyAdminCanCallThisFunction {
                    function: function.to_string(),
                });
            }
        }
        None => {
            return Err(ContractError::OnlyAdminCanCallThisFunctionButContractHasNoAdmin {
                function: function.to_string(),
            });
        }
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
