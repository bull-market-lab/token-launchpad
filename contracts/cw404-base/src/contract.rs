use crate::{
    error::ContractError,
    execute::{
        ft::{burn_tokens, force_transfer, mint_tokens, send_tokens},
        nft::{burn, send_nft, transfer_nft},
    },
    state::{ADMIN_ADDR, MAX_DENOM_SUPPLY, METADATA, SUBDENOM},
    util::{
        assert::{assert_can_send, assert_only_admin_can_call_this_function},
        denom::get_full_denom_from_subdenom,
    },
};
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, CosmosMsg, Deps, DepsMut, Empty, Env,
    MessageInfo, Reply, Response, StdResult, Uint128, Uint64,
};
use cw2::set_contract_version;
use cw404::msg::{
    AdminResponse, ExecuteMsg, FullDenomResponse, InstantiateMsg, MigrateMsg,
    QueryMsg, SudoMsg,
};
use cw721_base::{
    entry::{
        execute as cw721_execute, instantiate as cw721_instantiate,
        query as cw721_query,
    },
    msg::{
        ExecuteMsg as Cw721ExecuteMsg, InstantiateMsg as Cw721InstantiateMsg,
        QueryMsg as Cw721QueryMsg,
    },
    Cw721Contract,
};
use cw_utils::nonpayable;
use osmosis_std::types::{
    cosmos::bank::v1beta1::Metadata,
    osmosis::tokenfactory::v1beta1::{
        MsgCreateDenom, MsgSetBeforeSendHook, MsgSetDenomMetadata,
    },
};

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;
    if msg.denom_metadata.denom_units.len() != 1 {
        return Err(ContractError::ExpectExactlyOneDenomUnit {
            count: Uint64::from(msg.denom_metadata.denom_units.len() as u32),
        });
    }

    set_contract_version(
        deps.storage,
        format!("crates.io:{CONTRACT_NAME}"),
        CONTRACT_VERSION,
    )?;

    let contract_info = env.contract.clone();
    let contract_addr = contract_info.address;
    let contract_addr_str = contract_addr.to_string();
    let admin_addr_ref = &deps.api.addr_validate(&msg.admin_addr)?;

    ADMIN_ADDR.save(deps.storage, admin_addr_ref)?;
    SUBDENOM.save(deps.storage, &msg.subdenom)?;
    MAX_DENOM_SUPPLY.save(deps.storage, &msg.max_denom_supply)?;
    METADATA.save(deps.storage, &msg.denom_metadata)?;

    cw721_instantiate(
        deps,
        env,
        info,
        Cw721InstantiateMsg {
            name: msg.denom_metadata.name.clone(),
            symbol: msg.denom_metadata.symbol.clone(),
            minter: admin_addr_ref.to_string(),
        },
    )?;

    let full_denom =
        get_full_denom_from_subdenom(&contract_addr, &msg.subdenom);
    let msgs: Vec<CosmosMsg> = vec![
        MsgCreateDenom {
            sender: contract_addr_str.clone(),
            subdenom: msg.subdenom.clone(),
        }
        .into(),
        MsgSetBeforeSendHook {
            sender: contract_addr_str.clone(),
            denom: full_denom.clone(),
            cosmwasm_address: contract_addr_str.clone(),
        }
        .into(),
        MsgSetDenomMetadata {
            sender: contract_addr_str,
            metadata: Some(Metadata {
                description: msg.denom_metadata.description,
                denom_units: msg.denom_metadata.denom_units,
                base: msg.denom_metadata.base,
                display: msg.denom_metadata.display,
                name: msg.denom_metadata.name,
                symbol: msg.denom_metadata.symbol,
                uri: msg.denom_metadata.uri,
                uri_hash: msg.denom_metadata.uri_hash,
            }),
        }
        .into(),
    ];
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("subdenom", msg.subdenom)
        .add_attribute("admin_addr", msg.admin_addr)
        .add_attribute("full_denom", full_denom)
        .add_messages(msgs))
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
    let contract_addr = env.clone().contract.address;
    let contract_addr_ref = &contract_addr;
    let contract_addr_str = contract_addr.to_string();
    let denom = get_full_denom_from_subdenom(
        contract_addr_ref,
        &SUBDENOM.load(deps.storage)?,
    );
    let admin_addr = ADMIN_ADDR.load(deps.storage)?;
    let max_denom_supply = MAX_DENOM_SUPPLY.load(deps.storage)?;
    let admin_addr_ref = &admin_addr;
    let metadata = METADATA.load(deps.storage)?;
    let base_denom = metadata.base;
    let denom_exponent = metadata.denom_units[0].exponent;
    let one_denom_in_base_denom = Uint128::from(10u128.pow(denom_exponent));
    match msg {
        ExecuteMsg::ChangeAdmin { new_admin_addr } => {
            nonpayable(info_ref)?;
            assert_only_admin_can_call_this_function(
                sender_addr_ref,
                &admin_addr_ref,
                "change_admin",
            )?;
            let new_admin_addr = deps.api.addr_validate(&new_admin_addr)?;
            ADMIN_ADDR.save(deps.storage, &new_admin_addr)?;
            Ok(Response::new()
                .add_attribute("token_type", "ft")
                .add_attribute("action", "change_admin")
                .add_attribute("new_admin_addr", new_admin_addr))
        }
        // ======== FT functions ==========
        ExecuteMsg::MintTokens { amount } => {
            nonpayable(info_ref)?;
            assert_only_admin_can_call_this_function(
                sender_addr_ref,
                &admin_addr_ref,
                "mint_tokens",
            )?;
            mint_tokens(
                deps,
                amount,
                max_denom_supply,
                base_denom.clone(),
                one_denom_in_base_denom,
                denom,
                contract_addr_str,
                sender_addr_ref,
            )
        }
        ExecuteMsg::BurnTokens { amount } => {
            nonpayable(info_ref)?;
            assert_only_admin_can_call_this_function(
                sender_addr_ref,
                &admin_addr_ref,
                "burn_tokens",
            )?;
            burn_tokens(amount, denom, contract_addr_str)
        }
        ExecuteMsg::SendTokens {
            amount,
            recipient_addr,
        } => {
            nonpayable(info_ref)?;
            assert_only_admin_can_call_this_function(
                sender_addr_ref,
                &admin_addr_ref,
                "send_tokens",
            )?;
            send_tokens(amount, denom, recipient_addr)
        }
        ExecuteMsg::ForceTransfer { amount, from, to } => {
            nonpayable(info_ref)?;
            assert_only_admin_can_call_this_function(
                sender_addr_ref,
                &admin_addr_ref,
                "force_transfer",
            )?;
            force_transfer(amount, denom, contract_addr_str, from, to)
        }
        // ======== NFT cw721 functions ==========
        ExecuteMsg::Approve {
            spender,
            token_id,
            expires,
        } => Ok(cw721_execute(
            deps,
            env,
            info,
            Cw721ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            },
        )?),
        ExecuteMsg::ApproveAll { operator, expires } => Ok(cw721_execute(
            deps,
            env,
            info,
            Cw721ExecuteMsg::ApproveAll { operator, expires },
        )?),
        ExecuteMsg::Revoke { spender, token_id } => Ok(cw721_execute(
            deps,
            env,
            info,
            Cw721ExecuteMsg::Revoke { spender, token_id },
        )?),
        ExecuteMsg::RevokeAll { operator } => Ok(cw721_execute(
            deps,
            env,
            info,
            Cw721ExecuteMsg::RevokeAll { operator },
        )?),
        ExecuteMsg::TransferNft {
            recipient,
            token_id,
        } => transfer_nft(
            deps,
            env,
            info,
            contract_addr_str,
            recipient,
            token_id,
            one_denom_in_base_denom,
            base_denom,
            sender_addr_ref,
        ),
        ExecuteMsg::SendNft {
            contract,
            token_id,
            msg,
        } => send_nft(
            deps,
            env,
            info,
            contract_addr_str,
            contract,
            token_id,
            msg,
            one_denom_in_base_denom,
            base_denom,
            sender_addr_ref,
        ),
        ExecuteMsg::Burn { token_id } => {
            let storage_mut_ref = deps.storage;
            let cw721_base_contract =
                &Cw721Contract::<Empty, Empty, Empty, Empty>::default();
            assert_can_send(
                storage_mut_ref,
                &env.block,
                sender_addr_ref,
                &cw721_base_contract,
                &token_id,
            )?;
            burn(
                storage_mut_ref,
                cw721_base_contract,
                contract_addr_str,
                token_id,
                one_denom_in_base_denom,
                base_denom,
                sender_addr_ref,
            )
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let contract_addr = env.clone().contract.address;
    let denom = &SUBDENOM.load(deps.storage)?;
    let admin_addr = ADMIN_ADDR.load(deps.storage)?;
    match msg {
        // ======== FT functions ==========
        QueryMsg::FullDenom {} => {
            let full_denom =
                get_full_denom_from_subdenom(&contract_addr, &denom);
            to_json_binary(&FullDenomResponse { full_denom })
        }
        QueryMsg::Admin {} => to_json_binary(&AdminResponse {
            admin_addr: admin_addr.to_string(),
        }),
        // ======== NFT functions ==========
        QueryMsg::OwnerOf {
            token_id,
            include_expired,
        } => cw721_query(
            deps,
            env,
            Cw721QueryMsg::OwnerOf {
                token_id,
                include_expired,
            },
        ),
        QueryMsg::Approval {
            token_id,
            spender,
            include_expired,
        } => cw721_query(
            deps,
            env,
            Cw721QueryMsg::Approval {
                token_id,
                spender,
                include_expired,
            },
        ),
        QueryMsg::Approvals {
            token_id,
            include_expired,
        } => cw721_query(
            deps,
            env,
            Cw721QueryMsg::Approvals {
                token_id,
                include_expired,
            },
        ),
        QueryMsg::Operator {
            owner,
            operator,
            include_expired,
        } => cw721_query(
            deps,
            env,
            Cw721QueryMsg::Operator {
                owner,
                operator,
                include_expired,
            },
        ),
        QueryMsg::AllOperators {
            owner,
            include_expired,
            start_after,
            limit,
        } => cw721_query(
            deps,
            env,
            Cw721QueryMsg::AllOperators {
                owner,
                include_expired,
                start_after,
                limit,
            },
        ),
        QueryMsg::NumTokens {} => {
            cw721_query(deps, env, Cw721QueryMsg::NumTokens {})
        }
        QueryMsg::ContractInfo {} => {
            cw721_query(deps, env, Cw721QueryMsg::ContractInfo {})
        }
        QueryMsg::NftInfo { token_id } => {
            cw721_query(deps, env, Cw721QueryMsg::NftInfo { token_id })
        }
        QueryMsg::AllNftInfo {
            token_id,
            include_expired,
        } => cw721_query(
            deps,
            env,
            Cw721QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            },
        ),
        QueryMsg::Tokens {
            owner,
            start_after,
            limit,
        } => cw721_query(
            deps,
            env,
            Cw721QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            },
        ),
        QueryMsg::AllTokens { start_after, limit } => cw721_query(
            deps,
            env,
            Cw721QueryMsg::AllTokens { start_after, limit },
        ),
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
    _deps: DepsMut,
    _env: Env,
    _msg: Reply,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(_deps: DepsMut, _env: Env, _msg: SudoMsg) -> StdResult<Response> {
    Ok(Response::new())
}
