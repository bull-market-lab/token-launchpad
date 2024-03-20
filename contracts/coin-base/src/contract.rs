use crate::{
    execute::{
        config::update_config,
        ft::{burn, force_transfer, mint},
    },
    query::{
        config::query_config,
        ft::{query_balance, query_supply},
    },
    reply::create_pair_reply,
    state::{CONFIG, DENOM_EXPONENT},
    util::{
        assert_helper::assert_only_admin_can_call_this_function,
        astroport::create_pair, token_factory::create_and_mint_token,
    },
};
use coin::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo,
    Reply, Response, StdResult, Uint128,
};
use cw2::set_contract_version;
use cw_utils::{must_pay, nonpayable};
use shared_pkg::error::ContractError;

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const REPLY_ID_CREATE_PAIR: u64 = 0;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let _seed_liquidity = match msg.clone().seed_liquidity_config {
        Some(cfg) => must_pay(&info, cfg.paired_base_denom.as_str())?,
        None => {
            nonpayable(&info)?;
            Uint128::zero()
        }
    };

    set_contract_version(
        deps.storage,
        format!("crates.io:{CONTRACT_NAME}"),
        CONTRACT_VERSION,
    )?;
    let contract_info = env.contract.clone();
    let contract_addr = contract_info.address;

    let one_denom_in_base_denom = Uint128::from(10u128.pow(DENOM_EXPONENT));

    let (base_denom, create_and_mint_token_msgs) = create_and_mint_token(
        deps.api,
        deps.storage,
        &contract_addr,
        msg.admin_addr.clone(),
        &deps.api.addr_validate(&msg.creator_addr)?,
        msg.initial_supply_in_denom * one_denom_in_base_denom,
        msg.max_supply_in_denom * one_denom_in_base_denom,
        msg.clone().seed_liquidity_config,
        msg.subdenom.as_str(),
        msg.denom_description.as_str(),
        msg.denom_name.as_str(),
        msg.denom_symbol.as_str(),
        msg.denom_uri.as_str(),
        msg.denom_uri_hash.as_str(),
    )?;

    let (create_pair_submsg, create_pair_attributes) =
        match msg.seed_liquidity_config {
            Some(cfg) => {
                let (submsg, attributes) = create_pair(
                    &deps.api.addr_validate(&cfg.astroport_factory_addr)?,
                    &cfg.paired_base_denom,
                    base_denom.as_str(),
                )?;
                (vec![submsg], attributes)
            }
            None => (vec![], vec![]),
        };

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract_addr", env.contract.address)
        .add_attribute(
            "admin_addr",
            match msg.admin_addr {
                Some(addr) => addr,
                None => "None".to_string(),
            },
        )
        .add_attribute("creator_addr", msg.creator_addr)
        .add_attribute("subdenom", msg.subdenom)
        .add_attribute("max_supply_in_denom", msg.max_supply_in_denom)
        .add_attribute("initial_supply_in_denom", msg.initial_supply_in_denom)
        .add_messages(create_and_mint_token_msgs)
        .add_submessages(create_pair_submsg)
        .add_attributes(create_pair_attributes))
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

    let contract_addr_ref = &env.contract.address;
    let sender_addr_ref = &info.clone().sender;
    let config_ref = &CONFIG.load(deps.storage)?;
    let base_denom = config_ref.denom_metadata.base.as_str();
    match msg {
        ExecuteMsg::UpdateConfig { new_admin_addr } => {
            assert_only_admin_can_call_this_function(
                sender_addr_ref,
                &config_ref.admin_addr,
                "update_admin",
            )?;
            update_config(deps.api, deps.storage, new_admin_addr)
        }
        ExecuteMsg::Mint { amount, recipient } => {
            assert_only_admin_can_call_this_function(
                sender_addr_ref,
                &config_ref.admin_addr,
                "mint",
            )?;
            mint(
                deps.querier,
                config_ref,
                amount,
                base_denom,
                contract_addr_ref,
                &deps.api.addr_validate(&recipient)?,
            )
        }
        ExecuteMsg::Burn { amount } => {
            assert_only_admin_can_call_this_function(
                sender_addr_ref,
                &config_ref.admin_addr,
                "burn",
            )?;
            burn(amount, base_denom, contract_addr_ref)
        }
        ExecuteMsg::ForceTransfer { amount, from, to } => {
            assert_only_admin_can_call_this_function(
                sender_addr_ref,
                &config_ref.admin_addr,
                "force_transfer",
            )?;
            force_transfer(
                amount,
                base_denom,
                contract_addr_ref,
                &deps.api.addr_validate(&from)?,
                &deps.api.addr_validate(&to)?,
            )
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let config_ref = &CONFIG.load(deps.storage)?;
    let base_denom = config_ref.denom_metadata.base.as_str();
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(config_ref)?),
        QueryMsg::Supply {} => to_json_binary({
            &query_supply(deps.querier, config_ref, base_denom)?
        }),
        QueryMsg::Balance { owner } => to_json_binary(&query_balance(
            deps.querier,
            &deps.api.addr_validate(&owner)?,
            base_denom,
        )?),
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(
    deps: DepsMut,
    env: Env,
    msg: Reply,
) -> Result<Response, ContractError> {
    match msg.id {
        REPLY_ID_CREATE_PAIR => create_pair_reply(
            deps.querier,
            deps.as_ref().storage,
            msg,
            &env.contract.address,
        ),
        _ => Err(ContractError::UnknownReplyId { reply_id: msg.id }),
    }
}
