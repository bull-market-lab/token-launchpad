use crate::state::CW404_COLLECTIONS;
use cosmwasm_std::{DepsMut, Reply, Response};
use launchpad_pkg::token::TokenContract;
use shared_pkg::error::ContractError;

pub fn reply_instantiate_cw404_contract(
    deps: DepsMut,
    msg: Reply,
) -> Result<Response, ContractError> {
    let reply = msg.result.unwrap();
    let event = reply
                .events
                .iter()
                .find(|event| {
                    event
                        .attributes
                        .iter()
                        .any(|attr| attr.key == "action" && attr.value == "instantiate")
                })
                .ok_or({
                    ContractError::ErrorGettingEventFromInstantiateReplyOfCw404Contract {}
                })?;
    let contract_addr = deps.api.addr_validate(
                &event
                    .attributes
                    .iter()
                    .find(|attr| attr.key == "contract_addr")
                    .ok_or(ContractError::ErrorGettingContractAddrFromInstantiateReplyOfCw404Contract{})?
                    .value,
            )?;
    let creator_addr = deps.api.addr_validate(
                &event
                    .attributes
                    .iter()
                    .find(|attr| attr.key == "creator_addr")
                    .ok_or(ContractError::ErrorGettingCreatorAddrFromInstantiateReplyOfCw404Contract{})?
                    .value,
            )?;
    CW404_COLLECTIONS().update(
        deps.storage,
        contract_addr.clone(),
        |existing| match existing {
            None => Ok(TokenContract {
                creator_addr,
                contract_addr: contract_addr.clone(),
            }),
            Some(_) => Err(ContractError::CollectionAlreadyExists {
                collection_addr: contract_addr.to_string(),
            }),
        },
    )?;
    Ok(Response::new()
        .add_attribute("action", "reply_instantiate_cw404_contract"))
}
