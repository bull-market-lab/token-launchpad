use crate::{contract::REPLY_ID_INSTANTIATE_CW404_CONTRACT, state::FEE_DENOM};
use cosmwasm_std::{
    coins, to_json_binary, Addr, BankMsg, CosmosMsg, ReplyOn, Response, SubMsg,
    Uint128, Uint64, WasmMsg,
};
use cw404::{
    mint_group::MintGroup,
    msg::{
        ExecuteMsg as Cw404ExecuteMsg, InstantiateMsg as Cw404InstantiateMsg,
    },
};
use launchpad_pkg::config::Config;
use shared_pkg::error::ContractError;

pub fn create_cw404_collection(
    config: &Config,
    launchpad_addr: Addr,
    creator_addr: Addr,
    creator_paid_amount: Uint128,
    royalty_payment_addr: String,
    royalty_percentage: Uint64,
    max_nft_supply: Uint128,
    subdenom: String,
    denom_description: String,
    denom_name: String,
    denom_symbol: String,
    denom_uri: String,
    denom_uri_hash: String,
    mint_groups: Vec<MintGroup>,
) -> Result<Response, ContractError> {
    if creator_paid_amount != config.cw404_config.collection_creation_fee {
        return Err(ContractError::FundsMisMatchToCreateCw404Collection {
            paid: creator_paid_amount,
            required: config.cw404_config.collection_creation_fee,
        });
    }
    let send_creation_fee_to_fee_collector_msg = BankMsg::Send {
        to_address: config.cw404_config.fee_collector.to_string(),
        amount: coins(
            config.cw404_config.collection_creation_fee.u128(),
            FEE_DENOM,
        ),
    };
    let instantiate_cw404_collection_submsg = SubMsg {
        id: REPLY_ID_INSTANTIATE_CW404_CONTRACT,
        msg: CosmosMsg::Wasm(WasmMsg::Instantiate {
            admin: None,
            code_id: config.cw404_config.cw404_code_id.u64(),
            msg: to_json_binary(&Cw404InstantiateMsg {
                // set to None so no one can burn or force transfer FT of the new CW404 collection
                // TODO: support letting creator to mint / burn / force transfer FT
                admin_addr: None,
                // set minter to launchpad contract address so only launchpad contract can mint NFTs
                // so all users mint through launchpad contract
                minter_addr: launchpad_addr.to_string(),
                creator_addr: creator_addr.to_string(),
                max_nft_supply,
                subdenom,
                denom_description,
                denom_name,
                denom_symbol,
                denom_uri,
                denom_uri_hash,
                royalty_payment_addr,
                royalty_percentage,
                mint_groups,
            })
            .unwrap(),
            funds: vec![],
            label: "CW404".to_string(),
        }),
        gas_limit: None,
        reply_on: ReplyOn::Always,
    };
    Ok(Response::new()
        .add_submessage(instantiate_cw404_collection_submsg)
        .add_message(send_creation_fee_to_fee_collector_msg)
        .add_attribute("action", "create_cw404_collection")
        .add_attribute("amount", creator_paid_amount))
}

pub fn mint_ft_of_cw404(
    config: &Config,
    collection_addr: Addr,
    recipient_addr: Addr,
    user_paid_amount: Uint128,
    mint_amount: Uint128,
    mint_group_name: String,
    merkle_proof: Option<Vec<Vec<u8>>>,
) -> Result<Response, ContractError> {
    let mint_fee = config.cw404_config.mint_fee;
    if user_paid_amount < mint_fee {
        return Err(ContractError::InsufficientFundsToMintNft {
            paid: user_paid_amount,
            required: mint_fee,
        });
    }
    let send_mint_fee_to_fee_collector_msg = BankMsg::Send {
        to_address: config.cw404_config.fee_collector.to_string(),
        amount: coins(mint_fee.u128(), FEE_DENOM),
    };
    let mint_msg = WasmMsg::Execute {
        contract_addr: collection_addr.to_string(),
        msg: to_json_binary(&Cw404ExecuteMsg::MintFt {
            amount: mint_amount,
            recipient: recipient_addr.to_string(),
            mint_group_name,
            merkle_proof,
        })
        .unwrap(),
        funds: coins((user_paid_amount - mint_fee).u128(), FEE_DENOM),
    };
    Ok(Response::new()
        .add_message(send_mint_fee_to_fee_collector_msg)
        .add_message(mint_msg)
        .add_attribute("action", "mint_ft_of_cw_404"))
}
