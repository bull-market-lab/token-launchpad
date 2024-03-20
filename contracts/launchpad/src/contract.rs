use crate::{
    execute::{
        coin::create_coin,
        config::{
            update_coin_config, update_cw404_config, update_shared_config,
        },
        cw404::{create_cw404_collection, mint_ft_of_cw404},
    },
    query::{
        coin::{
            query_coin_by_contract_addr, query_coins,
            query_coins_by_creator_addr,
        },
        config::{query_config, query_stats},
        cw404::{
            query_cw404_collection_by_contract_addr, query_cw404_collections,
            query_cw404_collections_by_creator_addr,
        },
    },
    reply::{
        coin::reply_instantiate_coin_contract,
        cw404::reply_instantiate_cw404_contract,
    },
    state::{CONFIG, FEE_DENOM},
};
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo,
    Reply, Response, StdResult, Uint128,
};
use cw2::set_contract_version;
use cw_utils::{may_pay, must_pay, nonpayable};
use launchpad_pkg::{
    config::{CoinConfig, Config, Cw404Config},
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
};
use shared_pkg::error::ContractError;

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const REPLY_ID_INSTANTIATE_CW404_CONTRACT: u64 = 0;
pub const REPLY_ID_INSTANTIATE_COIN_CONTRACT: u64 = 1;

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
            admin_addr: deps.api.addr_validate(&msg.admin_addr)?,
            astroport_factory_addr: deps
                .api
                .addr_validate(&msg.astroport_factory_addr)?,
            cw404_config: Cw404Config {
                fee_collector: deps
                    .api
                    .addr_validate(&msg.cw404_fee_collector)?,
                cw404_code_id: msg.cw404_code_id,
                collection_creation_fee: msg.cw404_collection_creation_fee,
                mint_fee: msg.cw404_mint_fee,
            },
            coin_config: CoinConfig {
                fee_collector: deps
                    .api
                    .addr_validate(&msg.coin_fee_collector)?,
                coin_code_id: msg.coin_code_id,
                coin_creation_fee: msg.coin_creation_fee,
            },
        },
    )?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract_addr", env.contract.address)
        .add_attribute("admin_addr", msg.admin_addr)
        .add_attribute("astroport_factory_addr", msg.astroport_factory_addr)
        .add_attribute("cw404_fee_collector_addr", msg.cw404_fee_collector)
        .add_attribute("cw404_code_id", msg.cw404_code_id.to_string())
        .add_attribute(
            "cw404_collection_creation_fee",
            msg.cw404_collection_creation_fee.to_string(),
        )
        .add_attribute("cw404_mint_fee", msg.cw404_mint_fee.to_string())
        .add_attribute("coin_fee_collector_addr", msg.coin_fee_collector)
        .add_attribute("coin_code_id", msg.coin_code_id.to_string())
        .add_attribute("coin_creation_fee", msg.coin_creation_fee.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let info_ref = &info;
    let sender_addr_ref = &info.clone().sender;
    let config_ref = &CONFIG.load(deps.storage)?;
    match msg {
        ExecuteMsg::UpdateSharedConfig {
            new_admin_addr,
            new_astroport_factory_addr,
        } => {
            nonpayable(info_ref)?;
            if sender_addr_ref != config_ref.admin_addr {
                return Err(ContractError::OnlyAdminCanCallThisFunction {
                    function: "update_shared_config".to_string(),
                });
            }
            update_shared_config(
                deps.api,
                deps.storage,
                new_admin_addr,
                new_astroport_factory_addr,
            )
        }
        ExecuteMsg::UpdateCw404Config {
            new_fee_collector_addr,
            new_cw404_code_id,
            new_collection_creation_fee,
            new_mint_fee,
        } => {
            nonpayable(info_ref)?;
            if sender_addr_ref != config_ref.admin_addr {
                return Err(ContractError::OnlyAdminCanCallThisFunction {
                    function: "update_cw404_config".to_string(),
                });
            }
            update_cw404_config(
                deps.api,
                deps.storage,
                new_fee_collector_addr,
                new_cw404_code_id,
                new_collection_creation_fee,
                new_mint_fee,
            )
        }
        ExecuteMsg::UpdateCoinConfig {
            new_fee_collector_addr,
            new_coin_code_id,
            new_coin_creation_fee,
        } => {
            nonpayable(info_ref)?;
            if sender_addr_ref != config_ref.admin_addr {
                return Err(ContractError::OnlyAdminCanCallThisFunction {
                    function: "update_coin_config".to_string(),
                });
            }
            update_coin_config(
                deps.api,
                deps.storage,
                new_fee_collector_addr,
                new_coin_code_id,
                new_coin_creation_fee,
            )
        }
        ExecuteMsg::CreateCw404Collection {
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
            let creator_paid_amount =
                if config_ref.cw404_config.collection_creation_fee.is_zero() {
                    nonpayable(info_ref)?;
                    Uint128::zero()
                } else {
                    must_pay(info_ref, FEE_DENOM)?
                };
            create_cw404_collection(
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
        ExecuteMsg::MintFtOfCw404 {
            collection_addr,
            amount,
            recipient,
            mint_group_name,
            merkle_proof,
        } => {
            let user_paid_amount = if config_ref.cw404_config.mint_fee.is_zero()
            {
                // may still pay to creator
                may_pay(info_ref, FEE_DENOM)?
            } else {
                must_pay(info_ref, FEE_DENOM)?
            };
            mint_ft_of_cw404(
                config_ref,
                deps.api.addr_validate(&collection_addr)?,
                deps.api.addr_validate(&recipient)?,
                user_paid_amount,
                amount,
                mint_group_name,
                merkle_proof,
            )
        }
        ExecuteMsg::CreateCoin {
            subdenom,
            denom_description,
            denom_name,
            denom_symbol,
            denom_uri,
            denom_uri_hash,
            initial_supply_in_denom,
            max_supply_in_denom,
            immutable,
        } => {
            let creator_paid_amount =
                if config_ref.coin_config.coin_creation_fee.is_zero() {
                    // may still pay for seed liquidity
                    may_pay(info_ref, FEE_DENOM)?
                } else {
                    must_pay(info_ref, FEE_DENOM)?
                };
            create_coin(
                config_ref,
                env.contract.address,
                sender_addr_ref.clone(),
                creator_paid_amount,
                immutable,
                initial_supply_in_denom,
                max_supply_in_denom,
                subdenom,
                denom_description,
                denom_name,
                denom_symbol,
                denom_uri,
                denom_uri_hash,
            )
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps.storage)?),
        QueryMsg::Stats {} => to_json_binary(&query_stats(deps.storage)?),
        QueryMsg::Cw404CollectionByContract { contract_addr } => {
            to_json_binary(&query_cw404_collection_by_contract_addr(
                deps.storage,
                deps.api.addr_validate(&contract_addr)?,
            )?)
        }
        QueryMsg::Cw404CollectionsByCreator {
            creator_addr,
            start_after,
            limit,
        } => to_json_binary(&query_cw404_collections_by_creator_addr(
            deps.storage,
            deps.api.addr_validate(&creator_addr)?,
            start_after,
            limit,
        )?),
        QueryMsg::Cw404Collections { start_after, limit } => to_json_binary(
            &query_cw404_collections(deps.storage, start_after, limit)?,
        ),
        QueryMsg::CoinByContract { contract_addr } => {
            to_json_binary(&query_coin_by_contract_addr(
                deps.storage,
                deps.api.addr_validate(&contract_addr)?,
            )?)
        }
        QueryMsg::CoinsByCreator {
            creator_addr,
            start_after,
            limit,
        } => to_json_binary(&query_coins_by_creator_addr(
            deps.storage,
            deps.api.addr_validate(&creator_addr)?,
            start_after,
            limit,
        )?),
        QueryMsg::Coins { start_after, limit } => {
            to_json_binary(&query_coins(deps.storage, start_after, limit)?)
        }
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
            reply_instantiate_cw404_contract(deps, msg)
        }
        REPLY_ID_INSTANTIATE_COIN_CONTRACT => {
            reply_instantiate_coin_contract(deps, msg)
        }
        _ => Err(ContractError::UnknownReplyId { reply_id: msg.id }),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    _deps: DepsMut,
    _env: Env,
    msg: MigrateMsg,
) -> Result<Response, ContractError> {
    match msg {
        MigrateMsg::FromCompatible {} => Ok(Response::default()),
    }
}
