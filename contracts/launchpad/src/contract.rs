use crate::{
    error::ContractError,
    execute::{create_collecion, mint_ft, update_config},
    query::{
        query_collection_creator, query_config, query_creator_collections,
    },
    state::{COLLECTIONS, CONFIG, FEE_DENOM},
};
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo,
    Reply, Response, StdResult,
};
use cw2::set_contract_version;
use cw_utils::{may_pay, nonpayable};
use launchpad_pkg::{
    collection::Collection,
    config::Config,
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
};

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const REPLY_ID_INSTANTIATE_CW404_CONTRACT: u64 = 0;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;
    set_contract_version(
        deps.storage,
        format!("crates.io:{CONTRACT_NAME}"),
        CONTRACT_VERSION,
    )?;
    CONFIG.save(
        deps.storage,
        &Config {
            admin: deps.api.addr_validate(&msg.admin)?,
            fee_collector: deps.api.addr_validate(&msg.fee_collector)?,
            cw404_code_id: msg.cw404_code_id,
            create_collection_fee: msg.create_collection_fee,
            mint_fee: msg.mint_fee,
        },
    )?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract_addr", env.contract.address)
        .add_attribute("admin_addr", msg.admin)
        .add_attribute("fee_collector_addr", msg.fee_collector)
        .add_attribute("cw404_code_id", msg.cw404_code_id)
        .add_attribute("create_collection_fee", msg.create_collection_fee)
        .add_attribute("mint_fee", msg.mint_fee))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let info_ref = &info;
    nonpayable(info_ref)?;
    let sender_addr_ref = &info.clone().sender;
    let config_ref = &CONFIG.load(deps.storage)?;
    match msg {
        ExecuteMsg::UpdateConfig {
            new_admin_addr,
            new_fee_collector_addr,
            new_cw404_code_id,
            new_create_collection_fee,
            new_mint_fee,
        } => {
            nonpayable(info_ref)?;
            if sender_addr_ref != config_ref.admin {
                return Err(ContractError::OnlyAdminCanCallThisFunction {
                    function: "update_config".to_string(),
                });
            }
            update_config(
                deps.api,
                deps.storage,
                new_admin_addr,
                new_fee_collector_addr,
                new_cw404_code_id,
                new_create_collection_fee,
                new_mint_fee,
            )
        }
        ExecuteMsg::CreateCollection {
            royalty_payment_address,
            royalty_percentage,
            max_nft_supply,
            subdenom,
            denom_description,
            denom_name,
            denom_symbol,
            denom_uri,
            denom_uri_hash,
            mint_groups,
        } => {
            let creator_paid_amount = may_pay(info_ref, FEE_DENOM)?;
            create_collecion(
                config_ref,
                env.contract.address,
                sender_addr_ref.clone(),
                creator_paid_amount,
                royalty_payment_address,
                royalty_percentage,
                max_nft_supply,
                subdenom,
                denom_description,
                denom_name,
                denom_symbol,
                denom_uri,
                denom_uri_hash,
                mint_groups,
            )
        }
        ExecuteMsg::MintFt {
            collection_addr,
            recipient_addr,
        } => {
            let creator_paid_amount = may_pay(info_ref, FEE_DENOM)?;
            mint_ft(
                &deps.querier,
                config_ref,
                deps.api.addr_validate(&collection_addr)?,
                deps.api.addr_validate(&recipient_addr)?,
                creator_paid_amount,
            )
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps.storage)?),
        QueryMsg::CollectionCreator { collection_addr } => {
            to_json_binary(&query_collection_creator(
                deps.storage,
                deps.api.addr_validate(&collection_addr)?,
            )?)
        }
        QueryMsg::CreatorCollections {
            creator_addr,
            start_after,
            limit,
        } => to_json_binary(&query_creator_collections(
            deps.storage,
            deps.api.addr_validate(&creator_addr)?,
            start_after,
            limit,
        )?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(
    deps: DepsMut,
    _env: Env,
    msg: Reply,
) -> Result<Response, ContractError> {
    match msg.id {
        REPLY_ID_INSTANTIATE_CW404_CONTRACT => {
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
            let collection_addr = deps.api.addr_validate(
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
            COLLECTIONS().update(
                deps.storage,
                collection_addr.clone(),
                |existing| match existing {
                    None => Ok(Collection {
                        creator_addr,
                        collection_addr: collection_addr.clone(),
                    }),
                    Some(_) => Err(ContractError::CollectionAlreadyExists {
                        collection_addr: collection_addr.to_string(),
                    }),
                },
            )?;
            Ok(Response::new()
                .add_attribute("action", "reply_instantiate_cw404_contract"))
        }
        _ => Err(ContractError::UnknownReplyId { reply_id: msg.id }),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    _deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}
