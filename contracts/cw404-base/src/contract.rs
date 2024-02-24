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
    query::{
        config::query_admin,
        ft::{query_balance, query_denom_metadata, query_supply},
        nft::{
            query_all_nft_infos, query_all_nfts, query_all_nfts_operators,
            query_nft_approval, query_nft_approvals, query_nft_contract_info,
            query_nft_info, query_nft_num_tokens, query_nft_operator,
            query_nft_owner, query_nfts, query_recycled_nft_token_ids,
        },
    },
    state::{
        ADMIN_ADDR, CURRENT_NFT_SUPPLY, DENOM_EXPONENT, DENOM_METADATA,
        MAX_NFT_SUPPLY, RECYCLED_NFT_IDS,
    },
    sudo::ft::{block_before_send, track_before_send},
    util::{
        assert::assert_only_admin_can_call_this_function,
        commom::get_commom_fields, nft::parse_token_id_from_string_to_uint128,
    },
};
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, Reply, Response, StdResult, Uint128,
};
use cw2::set_contract_version;
use cw404::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, SudoMsg};
use cw_utils::nonpayable;
use osmosis_std::types::{
    cosmos::bank::v1beta1::{DenomUnit, Metadata},
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
    set_contract_version(
        deps.storage,
        format!("crates.io:{CONTRACT_NAME}"),
        CONTRACT_VERSION,
    )?;
    let contract_info = env.contract.clone();
    let contract_addr = contract_info.address;
    let admin_addr_ref = &deps.api.addr_validate(&msg.admin_addr)?;

    // e.g. atom
    let subdenom = msg.subdenom;
    // e.g. factory/contract_addr/atom
    let denom = format!("factory/{}/{}", contract_addr, subdenom);
    // e.g. uatom
    let base_subdenom = format!("u{}", subdenom);
    // e.g. factory/contract_addr/uatom
    let base_denom = format!("factory/{}/{}", contract_addr, base_subdenom);

    ADMIN_ADDR.save(deps.storage, admin_addr_ref)?;
    MAX_NFT_SUPPLY.save(deps.storage, &msg.max_nft_supply)?;
    CURRENT_NFT_SUPPLY.save(deps.storage, &Uint128::zero())?;

    let metadata = Metadata {
        description: msg.denom_description,
        denom_units: vec![
            DenomUnit {
                // e.g. factory/contract_addr/uatom
                denom: base_denom.clone(),
                exponent: 0,
                // e.g. uatom
                aliases: vec![base_subdenom.clone()],
            },
            DenomUnit {
                // e.g. factory/contract_addr/atom
                denom: denom.clone(),
                exponent: DENOM_EXPONENT,
                // e.g. atom
                aliases: vec![subdenom],
            },
        ],
        // e.g. factory/contract_addr/uatom
        base: base_denom.clone(),
        // e.g. factory/contract_addr/atom
        display: denom.clone(),
        // e.g. Cosmos Hub
        name: msg.denom_name,
        // e.g. ATOM
        symbol: msg.denom_symbol,
        uri: msg.denom_uri,
        uri_hash: msg.denom_uri_hash,
    };
    DENOM_METADATA.save(deps.storage, &metadata)?;

    let msgs: Vec<CosmosMsg> = vec![
        MsgCreateDenom {
            sender: contract_addr.to_string(),
            // e.g. uatom
            subdenom: base_subdenom,
        }
        .into(),
        MsgSetDenomMetadata {
            sender: contract_addr.to_string(),
            metadata: Some(metadata),
        }
        .into(),
        MsgSetBeforeSendHook {
            sender: contract_addr.to_string(),
            // e.g. factory/contract_addr/uatom
            denom: base_denom.clone(),
            cosmwasm_address: contract_addr.to_string(),
        }
        .into(),
    ];
    Ok(Response::new()
        .add_messages(msgs)
        .add_attribute("action", "instantiate")
        .add_attribute("admin_addr", msg.admin_addr)
        .add_attribute("denom", denom)
        .add_attribute("base_denom", base_denom)
        .add_attribute("max_nft_supply", msg.max_nft_supply))
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
    let (contract_addr, admin_addr, one_denom_in_base_denom, metadata) =
        get_commom_fields(deps.storage, env.clone())?;
    let base_denom = metadata.base.as_str();
    match msg {
        ExecuteMsg::ChangeAdmin { new_admin_addr } => {
            assert_only_admin_can_call_this_function(
                sender_addr_ref,
                &admin_addr,
                "change_admin",
            )?;
            change_admin(
                deps.storage,
                &deps.api.addr_validate(&new_admin_addr)?,
            )
        }
        // ======== FT (cosmos sdk native coin) functions ==========
        ExecuteMsg::MintFt { amount } => {
            assert_only_admin_can_call_this_function(
                sender_addr_ref,
                &admin_addr,
                "mint_ft",
            )?;
            mint_ft(
                deps.storage,
                deps.querier,
                amount,
                one_denom_in_base_denom,
                base_denom,
                metadata.uri.as_str(),
                &contract_addr,
            )
        }
        ExecuteMsg::BurnFt { amount } => {
            assert_only_admin_can_call_this_function(
                sender_addr_ref,
                &admin_addr,
                "burn_ft",
            )?;
            burn_ft(
                deps.storage,
                deps.querier,
                amount,
                one_denom_in_base_denom,
                base_denom,
                &contract_addr,
            )
        }
        ExecuteMsg::SendFt {
            amount,
            recipient_addr,
        } => {
            assert_only_admin_can_call_this_function(
                sender_addr_ref,
                &admin_addr,
                "send_ft",
            )?;
            send_ft(
                amount,
                base_denom,
                &deps.api.addr_validate(&recipient_addr)?,
            )
        }
        ExecuteMsg::ForceTransferFt { amount, from, to } => {
            assert_only_admin_can_call_this_function(
                sender_addr_ref,
                &admin_addr,
                "force_transfer_ft",
            )?;
            force_transfer_ft(
                deps.storage,
                deps.querier,
                amount,
                base_denom,
                one_denom_in_base_denom,
                metadata.uri.as_str(),
                &contract_addr,
                &deps.api.addr_validate(&from)?,
                &deps.api.addr_validate(&to)?,
            )
        }
        // ======== NFT (cw721) functions ==========
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
            base_denom,
            &contract_addr,
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
            base_denom,
            &contract_addr,
            &deps.api.addr_validate(&contract)?,
            msg,
        ),
        ExecuteMsg::Burn { token_id } => burn_nft(
            deps.storage,
            &env.block,
            &contract_addr,
            parse_token_id_from_string_to_uint128(token_id)?,
            one_denom_in_base_denom,
            base_denom,
            sender_addr_ref,
        ),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let (_contract_addr, admin_addr, one_denom_in_base_denom, metadata) =
        get_commom_fields(deps.storage, env.clone())?;
    let base_denom = metadata.base.as_str();
    match msg {
        // ======== custom functions ==========
        QueryMsg::Admin {} => to_json_binary(&query_admin(&admin_addr)?),
        QueryMsg::RecycledNftTokenIds {} => {
            to_json_binary(&query_recycled_nft_token_ids(deps.storage)?)
        }
        // ======== FT functions ==========
        QueryMsg::DenomMetadata {} => {
            to_json_binary(&query_denom_metadata(deps.storage)?)
        }
        QueryMsg::Supply {} => to_json_binary({
            &query_supply(
                deps.querier,
                deps.storage,
                base_denom,
                one_denom_in_base_denom,
            )?
        }),
        QueryMsg::Balance { owner } => to_json_binary(&query_balance(
            deps.querier,
            &deps.api.addr_validate(&owner)?,
            base_denom,
            one_denom_in_base_denom,
        )?),
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
        QueryMsg::NumTokens {} => {
            to_json_binary(&query_nft_num_tokens(deps.storage)?)
        }
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
pub fn sudo(
    deps: DepsMut,
    env: Env,
    msg: SudoMsg,
) -> Result<Response, ContractError> {
    let (_contract_addr, _admin_addr, one_denom_in_base_denom, metadata) =
        get_commom_fields(deps.storage, env.clone())?;
    match msg {
        SudoMsg::TrackBeforeSend { from, to, amount } => track_before_send(
            deps.storage,
            deps.querier,
            amount.amount,
            amount.denom.as_str(),
            one_denom_in_base_denom,
            &metadata,
            &deps.api.addr_validate(&from)?,
            &deps.api.addr_validate(&to)?,
        ),
        SudoMsg::BlockBeforeSend { from, to, amount } => block_before_send(
            deps.storage,
            deps.querier,
            amount.amount,
            amount.denom.as_str(),
            one_denom_in_base_denom,
            &metadata,
            &deps.api.addr_validate(&from)?,
            &deps.api.addr_validate(&to)?,
        ),
    }
}
