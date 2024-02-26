use crate::{
    contract::REPLY_ID_INSTANTIATE_CW404_CONTRACT,
    error::ContractError,
    state::{CONFIG, FEE_DENOM},
};
use cosmwasm_std::{
    coins, to_json_binary, Addr, Api, BankMsg, CosmosMsg, QuerierWrapper,
    ReplyOn, Response, Storage, SubMsg, Uint128, Uint64, WasmMsg,
};
use cw404::{mint_group::MintGroup, msg::InstantiateMsg as Cw404InstantiateMsg};
use launchpad_pkg::config::Config;

pub fn update_config(
    api: &dyn Api,
    storage: &mut dyn Storage,
    new_admin: Option<String>,
    new_fee_collector: Option<String>,
    new_cw404_code_id: Option<Uint64>,
    new_create_collection_fee: Option<Uint128>,
    new_mint_fee: Option<Uint128>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(storage)?;
    config.admin = match new_admin {
        Some(admin) => api.addr_validate(&admin)?,
        None => config.admin,
    };
    config.fee_collector = match new_fee_collector {
        Some(fee_collector) => api.addr_validate(&fee_collector)?,
        None => config.fee_collector,
    };
    config.cw404_code_id = match new_cw404_code_id {
        Some(cw404_code_id) => cw404_code_id,
        None => config.cw404_code_id,
    };
    config.create_collection_fee = match new_create_collection_fee {
        Some(create_collection_fee) => create_collection_fee,
        None => config.create_collection_fee,
    };
    config.mint_fee = match new_mint_fee {
        Some(mint_fee) => mint_fee,
        None => config.mint_fee,
    };
    CONFIG.save(storage, &config)?;
    Ok(Response::new().add_attribute("action", "update_config"))
}

pub fn create_collecion(
    config: &Config,
    launchpad_contract_addr: Addr,
    creator_addr: Addr,
    creator_paid_amount: Uint128,
    royalty_payment_address: Option<String>,
    royalty_percentage: Option<Uint64>,
    max_nft_supply: Uint128,
    subdenom: String,
    denom_description: String,
    denom_name: String,
    denom_symbol: String,
    denom_uri: String,
    denom_uri_hash: String,
    mint_groups: Vec<MintGroup>,
) -> Result<Response, ContractError> {
    if creator_paid_amount < config.create_collection_fee {
        return Err(ContractError::InsufficientFundsToCreateCollection {
            paid: creator_paid_amount,
            required: config.create_collection_fee,
        });
    }
    let send_creation_fee_to_fee_collector_msg =
        CosmosMsg::Bank(BankMsg::Send {
            to_address: config.fee_collector.to_string(),
            amount: coins(creator_paid_amount.u128(), FEE_DENOM),
        });
    let instantiate_cw404_collection_submsg = SubMsg {
        id: REPLY_ID_INSTANTIATE_CW404_CONTRACT,
        msg: CosmosMsg::Wasm(WasmMsg::Instantiate {
            admin: None,
            code_id: config.cw404_code_id.u64(),
            msg: to_json_binary(&Cw404InstantiateMsg {
                // set to None so no one can burn or force transfer FT of the new CW404 collection
                admin: None,
                // set minter to launchpad contract address so only launchpad contract can mint NFTs
                // so all users mint through launchpad contract
                minter: Some(launchpad_contract_addr.to_string()),
                creator: creator_addr.to_string(),
                max_nft_supply,
                subdenom,
                denom_description,
                denom_name,
                denom_symbol,
                denom_uri,
                denom_uri_hash,
                royalty_payment_address,
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
        .add_attribute("action", "create_collection")
        .add_attribute("amount", creator_paid_amount))
}

pub fn mint_ft(
    querier: &QuerierWrapper,
    config: &Config,
    collection_addr: Addr,
    recipient_addr: Addr,
    user_paid_amount: Uint128,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "mint_ft"))
}
