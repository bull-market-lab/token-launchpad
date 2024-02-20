use crate::{
    error::ContractError,
    execute::{
        config::change_admin,
        ft::{burn_ft, force_transfer_ft, mint_ft, send_ft},
        nft::{
            approve_all_nft, approve_nft, burn_nft, revoke_all_nft, revoke_nft,
            send_nft, transfer_nft,
        },
    },
    query::nft::{
        query_all_nft_infos, query_all_nfts, query_all_nfts_operators,
        query_nft_approval, query_nft_approvals, query_nft_contract_info,
        query_nft_info, query_nft_num_tokens, query_nft_operator,
        query_nft_owner, query_nfts,
    },
    state::{ADMIN_ADDR, MAX_NFT_SUPPLY, METADATA, SUBDENOM},
    util::{
        assert::assert_only_admin_can_call_this_function,
        denom::get_full_denom_from_subdenom,
        nft::parse_token_id_from_string_to_uint128,
    },
};
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, Reply, Response, StdResult, Uint128,
};
use cw2::set_contract_version;
use cw404::msg::{
    AdminResponse, BalanceResponse, DenomResponse, ExecuteMsg, InstantiateMsg,
    MigrateMsg, QueryMsg, SudoMsg, SupplyResponse,
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
            count: Uint128::from(msg.denom_metadata.denom_units.len() as u32),
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
    MAX_NFT_SUPPLY.save(deps.storage, &msg.max_nft_supply)?;
    METADATA.save(deps.storage, &msg.denom_metadata)?;

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
    nonpayable(info_ref)?;

    let sender_addr_ref = &info.clone().sender;
    let contract_addr = env.clone().contract.address;
    let contract_addr_ref = &contract_addr;
    let contract_addr_str = contract_addr.to_string();
    let denom = get_full_denom_from_subdenom(
        contract_addr_ref,
        &SUBDENOM.load(deps.storage)?,
    );
    let denom_str = denom.as_str();
    let admin_addr = ADMIN_ADDR.load(deps.storage)?;
    let admin_addr_ref = &admin_addr;
    let metadata = METADATA.load(deps.storage)?;
    let denom_exponent = metadata.denom_units[0].exponent;
    let one_denom_in_base_denom = Uint128::from(10u128.pow(denom_exponent));
    match msg {
        ExecuteMsg::ChangeAdmin { new_admin_addr } => {
            assert_only_admin_can_call_this_function(
                sender_addr_ref,
                admin_addr_ref,
                "change_admin",
            )?;
            change_admin(
                deps.storage,
                &deps.api.addr_validate(&new_admin_addr)?,
            )
        }
        // ======== FT functions ==========
        ExecuteMsg::MintTokens { amount } => {
            assert_only_admin_can_call_this_function(
                sender_addr_ref,
                admin_addr_ref,
                "mint_ft",
            )?;
            mint_ft(
                deps.storage,
                deps.querier,
                amount,
                one_denom_in_base_denom,
                denom_str,
                contract_addr_ref,
            )
        }
        ExecuteMsg::BurnTokens { amount } => {
            assert_only_admin_can_call_this_function(
                sender_addr_ref,
                admin_addr_ref,
                "burn_ft",
            )?;
            burn_ft(
                deps.storage,
                deps.querier,
                amount,
                one_denom_in_base_denom,
                denom_str,
                contract_addr_ref,
            )
        }
        ExecuteMsg::SendTokens {
            amount,
            recipient_addr,
        } => {
            assert_only_admin_can_call_this_function(
                sender_addr_ref,
                admin_addr_ref,
                "send_ft",
            )?;
            send_ft(amount, denom_str, recipient_addr)
        }
        ExecuteMsg::ForceTransfer { amount, from, to } => {
            assert_only_admin_can_call_this_function(
                sender_addr_ref,
                admin_addr_ref,
                "force_transfer_ft",
            )?;
            force_transfer_ft(amount, denom_str, contract_addr_str, from, to)
        }
        // ======== NFT cw721 functions ==========
        ExecuteMsg::Approve {
            spender,
            token_id,
            expires,
        } => approve_nft(
            deps.storage,
            &env.block,
            sender_addr_ref,
            &deps.api.addr_validate(&spender)?,
            parse_token_id_from_string_to_uint128(token_id)?,
            expires,
        ),
        ExecuteMsg::ApproveAll { operator, expires } => approve_all_nft(
            deps.storage,
            &env.block,
            sender_addr_ref,
            &deps.api.addr_validate(&operator)?,
            expires,
        ),
        ExecuteMsg::Revoke { spender, token_id } => revoke_nft(
            deps.storage,
            &env.block,
            sender_addr_ref,
            &deps.api.addr_validate(&spender)?,
            parse_token_id_from_string_to_uint128(token_id)?,
        ),
        ExecuteMsg::RevokeAll { operator } => revoke_all_nft(
            deps.storage,
            sender_addr_ref,
            &deps.api.addr_validate(&operator)?,
        ),
        ExecuteMsg::TransferNft {
            recipient,
            token_id,
        } => transfer_nft(
            deps.storage,
            &env.block,
            sender_addr_ref,
            &deps.api.addr_validate(&recipient)?,
            parse_token_id_from_string_to_uint128(token_id)?,
            one_denom_in_base_denom,
            denom_str,
            contract_addr_ref,
        ),
        ExecuteMsg::SendNft {
            contract,
            token_id,
            msg,
        } => send_nft(
            deps.storage,
            &env.block,
            sender_addr_ref,
            parse_token_id_from_string_to_uint128(token_id)?,
            one_denom_in_base_denom,
            denom_str,
            contract_addr_ref,
            &deps.api.addr_validate(&contract)?,
            msg,
        ),
        ExecuteMsg::Burn { token_id } => burn_nft(
            deps.storage,
            &env.block,
            contract_addr_ref,
            parse_token_id_from_string_to_uint128(token_id)?,
            one_denom_in_base_denom,
            denom_str,
            sender_addr_ref,
        ),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let contract_addr = env.clone().contract.address;
    let denom = get_full_denom_from_subdenom(
        &contract_addr,
        &SUBDENOM.load(deps.storage)?,
    );
    let denom_str = denom.as_str();
    let admin_addr = ADMIN_ADDR.load(deps.storage)?;
    let metadata = METADATA.load(deps.storage)?;
    let denom_exponent = metadata.denom_units[0].exponent;
    let one_denom_in_base_denom = Uint128::from(10u128.pow(denom_exponent));
    match msg {
        QueryMsg::Admin {} => to_json_binary(&AdminResponse {
            admin_addr: admin_addr.to_string(),
        }),
        // ======== FT functions ==========
        QueryMsg::Denom {} => to_json_binary(&DenomResponse {
            subdenom: SUBDENOM.load(deps.storage)?,
            full_denom: denom_str.to_string(),
            denom_metadata: metadata,
        }),
        QueryMsg::Supply {} => to_json_binary({
            let ft_supply = deps.querier.query_supply(denom)?.amount;
            &SupplyResponse {
                current_nft_supply: ft_supply / one_denom_in_base_denom,
                max_nft_supply: MAX_NFT_SUPPLY.load(deps.storage)?,
                current_ft_supply: ft_supply,
                max_ft_supply: ft_supply * one_denom_in_base_denom,
            }
        }),
        QueryMsg::Balance { owner } => {
            let ft_balance = deps.querier.query_balance(owner, denom)?.amount;
            to_json_binary(&BalanceResponse {
                nft_balance: ft_balance / one_denom_in_base_denom,
                ft_balance,
            })
        }
        // ======== NFT functions ==========
        QueryMsg::OwnerOf {
            token_id,
            include_expired,
        } => to_json_binary(&query_nft_owner(
            deps.storage,
            &env.block,
            parse_token_id_from_string_to_uint128(token_id)?,
            include_expired,
        )?),
        QueryMsg::Approval {
            token_id,
            spender,
            include_expired,
        } => to_json_binary(&query_nft_approval(
            deps.storage,
            &env.block,
            parse_token_id_from_string_to_uint128(token_id)?,
            spender,
            include_expired,
        )?),
        QueryMsg::Approvals {
            token_id,
            include_expired,
        } => to_json_binary(&query_nft_approvals(
            deps.storage,
            &env.block,
            parse_token_id_from_string_to_uint128(token_id)?,
            include_expired,
        )?),
        QueryMsg::Operator {
            owner,
            operator,
            include_expired,
        } => to_json_binary(&query_nft_operator(
            deps.storage,
            &env.block,
            &deps.api.addr_validate(&owner)?,
            &deps.api.addr_validate(&operator)?,
            include_expired,
        )?),
        QueryMsg::AllOperators {
            owner,
            include_expired,
            start_after,
            limit,
        } => to_json_binary(&query_all_nfts_operators(
            deps,
            &env.block,
            owner,
            include_expired,
            start_after,
            limit,
        )?),
        QueryMsg::NumTokens {} => to_json_binary(&query_nft_num_tokens(
            deps.querier,
            denom_str,
            one_denom_in_base_denom,
        )?),
        QueryMsg::ContractInfo {} => {
            to_json_binary(&query_nft_contract_info(metadata)?)
        }
        QueryMsg::NftInfo { token_id } => to_json_binary(&query_nft_info(
            deps,
            parse_token_id_from_string_to_uint128(token_id)?,
        )?),
        QueryMsg::AllNftInfo {
            token_id,
            include_expired,
        } => to_json_binary(&query_all_nft_infos(
            deps,
            env,
            parse_token_id_from_string_to_uint128(token_id)?,
            include_expired,
        )?),
        QueryMsg::Tokens {
            owner,
            start_after,
            limit,
        } => to_json_binary(&query_nfts(
            deps,
            &deps.api.addr_validate(&owner)?,
            start_after,
            limit,
        )?),
        QueryMsg::AllTokens { start_after, limit } => {
            to_json_binary(&query_all_nfts(deps.storage, start_after, limit)?)
        }
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
