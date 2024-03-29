use crate::{
    execute::{
        config::update_config,
        ft::{burn_ft, force_transfer_ft, mint_ft},
        nft::{
            approve_all_nft, approve_nft, burn_nft, revoke_all_nft, revoke_nft,
            send_nft, transfer_nft,
        },
    },
    query::{
        config::query_config,
        ft::{query_balance, query_supply},
        nft::{
            query_all_nft_infos, query_all_nfts, query_all_nfts_operators,
            query_nft_approval, query_nft_approvals, query_nft_contract_info,
            query_nft_info, query_nft_num_tokens, query_nft_operator,
            query_nft_owner, query_nfts, query_recycled_nft,
            query_recycled_nfts,
        },
    },
    state::{
        CONFIG, CURRENT_NFT_SUPPLY, DENOM_EXPONENT, FEE_DENOM, MAX_NFT_SUPPLY,
        MINT_GROUPS,
    },
    sudo::ft::block_before_send,
    util::{
        assert_helper::{
            assert_only_admin_can_call_this_function,
            assert_only_admin_or_minter_can_mint,
        },
        nft::parse_token_id_from_string_to_uint128,
        token_factory::create_and_mint_token,
    },
};
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo,
    Reply, Response, StdResult, Uint128,
};
use cw2::set_contract_version;
use cw404::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, SudoMsg};
use cw_utils::{may_pay, nonpayable};
use shared_pkg::error::ContractError;

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

    MAX_NFT_SUPPLY.save(deps.storage, &msg.max_nft_supply)?;
    CURRENT_NFT_SUPPLY.save(deps.storage, &Uint128::zero())?;

    for mint_group in msg.mint_groups {
        MINT_GROUPS.update(
            deps.storage,
            mint_group.clone().name.as_str(),
            |exist| match exist {
                Some(_) => Err(ContractError::DuplicateMintGroup {
                    name: mint_group.name,
                }),
                None => Ok(mint_group),
            },
        )?;
    }

    let create_and_mint_token_msgs = create_and_mint_token(
        deps.api,
        deps.storage,
        &contract_addr,
        msg.admin_addr.clone(),
        &deps.api.addr_validate(&msg.minter_addr)?,
        &deps.api.addr_validate(&msg.creator_addr)?,
        &deps.api.addr_validate(&msg.royalty_payment_addr)?,
        msg.royalty_percentage,
        msg.subdenom.as_str(),
        msg.denom_description.as_str(),
        msg.denom_name.as_str(),
        msg.denom_symbol.as_str(),
        msg.denom_uri.as_str(),
        msg.denom_uri_hash.as_str(),
    )?;

    Ok(Response::new()
        .add_messages(create_and_mint_token_msgs)
        .add_attribute("action", "instantiate")
        .add_attribute("contract_addr", env.contract.address)
        .add_attribute("subdenom", msg.subdenom)
        .add_attribute(
            "admin_addr",
            match msg.admin_addr {
                Some(addr) => addr,
                None => "None".to_string(),
            },
        )
        .add_attribute("minter_addr", msg.minter_addr)
        .add_attribute("creator_addr", msg.creator_addr)
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
    let contract_addr_ref = &env.contract.address;
    let sender_addr_ref = &info.clone().sender;
    let config_ref = &CONFIG.load(deps.storage)?;
    let one_denom_in_base_denom = Uint128::from(10u128.pow(DENOM_EXPONENT));
    let base_denom = config_ref.denom_metadata.base.as_str();
    match msg {
        ExecuteMsg::UpdateConfig {
            new_admin_addr,
            new_minter_addr,
            new_royalty_payment_addr,
            new_royalty_percentage,
        } => {
            nonpayable(info_ref)?;
            assert_only_admin_can_call_this_function(
                sender_addr_ref,
                &config_ref.admin_addr,
                "update_admin",
            )?;
            update_config(
                deps.api,
                deps.storage,
                new_admin_addr,
                new_minter_addr,
                new_royalty_payment_addr,
                new_royalty_percentage,
            )
        }
        // ======== FT (cosmos sdk native coin) functions ==========
        ExecuteMsg::MintFt {
            amount,
            recipient,
            mint_group_name,
            merkle_proof,
        } => {
            let user_paid_amount = may_pay(info_ref, FEE_DENOM)?;
            assert_only_admin_or_minter_can_mint(
                sender_addr_ref,
                &config_ref.admin_addr,
                &config_ref.minter_addr,
            )?;
            mint_ft(
                deps.storage,
                deps.querier,
                &env.block,
                config_ref,
                amount,
                one_denom_in_base_denom,
                base_denom,
                config_ref.denom_metadata.uri.as_str(),
                contract_addr_ref,
                &deps.api.addr_validate(&recipient)?,
                user_paid_amount,
                mint_group_name.as_str(),
                merkle_proof,
            )
        }
        ExecuteMsg::BurnFt { amount } => {
            nonpayable(info_ref)?;
            assert_only_admin_can_call_this_function(
                sender_addr_ref,
                &config_ref.admin_addr,
                "burn_ft",
            )?;
            burn_ft(
                deps.storage,
                deps.querier,
                amount,
                one_denom_in_base_denom,
                base_denom,
                contract_addr_ref,
            )
        }
        ExecuteMsg::ForceTransferFt { amount, from, to } => {
            nonpayable(info_ref)?;
            assert_only_admin_can_call_this_function(
                sender_addr_ref,
                &config_ref.admin_addr,
                "force_transfer_ft",
            )?;
            force_transfer_ft(
                deps.storage,
                deps.querier,
                amount,
                base_denom,
                one_denom_in_base_denom,
                config_ref.denom_metadata.uri.as_str(),
                contract_addr_ref,
                &deps.api.addr_validate(&from)?,
                &deps.api.addr_validate(&to)?,
            )
        }
        // ======== NFT (cw721) functions ==========
        ExecuteMsg::Approve {
            spender,
            token_id,
            expires,
        } => {
            nonpayable(info_ref)?;
            approve_nft(
                deps.storage,
                &env.block,
                sender_addr_ref,
                &deps.api.addr_validate(&spender)?,
                parse_token_id_from_string_to_uint128(token_id)?,
                expires,
            )
        }
        ExecuteMsg::ApproveAll { operator, expires } => {
            nonpayable(info_ref)?;
            approve_all_nft(
                deps.storage,
                &env.block,
                sender_addr_ref,
                &deps.api.addr_validate(&operator)?,
                expires,
            )
        }
        ExecuteMsg::Revoke { spender, token_id } => {
            nonpayable(info_ref)?;
            revoke_nft(
                deps.storage,
                &env.block,
                sender_addr_ref,
                &deps.api.addr_validate(&spender)?,
                parse_token_id_from_string_to_uint128(token_id)?,
            )
        }
        ExecuteMsg::RevokeAll { operator } => {
            nonpayable(info_ref)?;
            revoke_all_nft(
                deps.storage,
                sender_addr_ref,
                &deps.api.addr_validate(&operator)?,
            )
        }
        ExecuteMsg::TransferNft {
            recipient,
            token_id,
        } => {
            nonpayable(info_ref)?;
            transfer_nft(
                deps.storage,
                &env.block,
                sender_addr_ref,
                &deps.api.addr_validate(&recipient)?,
                parse_token_id_from_string_to_uint128(token_id)?,
                one_denom_in_base_denom,
                base_denom,
                contract_addr_ref,
            )
        }
        ExecuteMsg::SendNft {
            contract,
            token_id,
            msg,
        } => {
            nonpayable(info_ref)?;
            send_nft(
                deps.storage,
                &env.block,
                sender_addr_ref,
                parse_token_id_from_string_to_uint128(token_id)?,
                one_denom_in_base_denom,
                base_denom,
                contract_addr_ref,
                &deps.api.addr_validate(&contract)?,
                msg,
            )
        }
        ExecuteMsg::Burn { token_id } => {
            nonpayable(info_ref)?;
            burn_nft(
                deps.storage,
                &env.block,
                contract_addr_ref,
                parse_token_id_from_string_to_uint128(token_id)?,
                one_denom_in_base_denom,
                base_denom,
                sender_addr_ref,
            )
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let config_ref = &CONFIG.load(deps.storage)?;
    let one_denom_in_base_denom = Uint128::from(10u128.pow(DENOM_EXPONENT));
    let base_denom = config_ref.denom_metadata.base.as_str();
    match msg {
        // ======== general functions ==========
        QueryMsg::Config {} => to_json_binary(&query_config(config_ref)?),
        QueryMsg::RecycledNftTokenIds {
            start_after_idx,
            limit,
        } => to_json_binary(&query_recycled_nfts(
            deps.storage,
            start_after_idx,
            limit,
        )?),
        QueryMsg::RecycledNftInfo { token_id } => {
            to_json_binary(&query_recycled_nft(deps.storage, token_id)?)
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
            deps.api,
            deps.storage,
            &env.block,
            owner,
            include_expired,
            start_after,
            limit,
        )?),
        QueryMsg::NumTokens {} => {
            to_json_binary(&query_nft_num_tokens(deps.storage)?)
        }
        QueryMsg::ContractInfo {} => to_json_binary(&query_nft_contract_info(
            &config_ref.denom_metadata,
        )?),
        QueryMsg::NftInfo { token_id } => to_json_binary(&query_nft_info(
            deps.storage,
            parse_token_id_from_string_to_uint128(token_id)?,
        )?),
        QueryMsg::AllNftInfo {
            token_id,
            include_expired,
        } => to_json_binary(&query_all_nft_infos(
            deps.storage,
            env,
            parse_token_id_from_string_to_uint128(token_id)?,
            include_expired,
        )?),
        QueryMsg::Tokens {
            owner,
            start_after,
            limit,
        } => to_json_binary(&query_nfts(
            deps.storage,
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
    _env: Env,
    msg: SudoMsg,
) -> Result<Response, ContractError> {
    let config_ref = &CONFIG.load(deps.storage)?;
    let one_denom_in_base_denom = Uint128::from(10u128.pow(DENOM_EXPONENT));
    match msg {
        SudoMsg::BlockBeforeSend { from, to, amount } => block_before_send(
            deps.storage,
            deps.querier,
            amount.amount,
            amount.denom.as_str(),
            one_denom_in_base_denom,
            &config_ref.denom_metadata,
            &deps.api.addr_validate(&from)?,
            &deps.api.addr_validate(&to)?,
        ),
    }
}
