use crate::state::CONFIG;
use cosmwasm_std::{Api, Response, Storage, Uint128, Uint64};
use shared_pkg::error::ContractError;

pub fn update_shared_config(
    api: &dyn Api,
    storage: &mut dyn Storage,
    new_admin: Option<String>,
    new_astroport_factory_addr: Option<String>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(storage)?;
    config.admin_addr = match new_admin {
        Some(admin) => api.addr_validate(&admin)?,
        None => config.admin_addr,
    };
    config.astroport_factory_addr = match new_astroport_factory_addr {
        Some(astroport_factory_addr) => {
            api.addr_validate(&astroport_factory_addr)?
        }
        None => config.astroport_factory_addr,
    };
    CONFIG.save(storage, &config)?;
    Ok(Response::new().add_attribute("action", "update_shared_config"))
}

pub fn update_cw404_config(
    api: &dyn Api,
    storage: &mut dyn Storage,
    new_fee_collector: Option<String>,
    new_cw404_code_id: Option<Uint64>,
    new_collection_creation_fee: Option<Uint128>,
    new_mint_fee: Option<Uint128>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(storage)?;
    config.cw404_config.fee_collector = match new_fee_collector {
        Some(fee_collector) => api.addr_validate(&fee_collector)?,
        None => config.cw404_config.fee_collector,
    };
    config.cw404_config.cw404_code_id = match new_cw404_code_id {
        Some(cw404_code_id) => cw404_code_id,
        None => config.cw404_config.cw404_code_id,
    };
    config.cw404_config.collection_creation_fee =
        match new_collection_creation_fee {
            Some(create_collection_fee) => create_collection_fee,
            None => config.cw404_config.collection_creation_fee,
        };
    config.cw404_config.mint_fee = match new_mint_fee {
        Some(mint_fee) => mint_fee,
        None => config.cw404_config.mint_fee,
    };
    CONFIG.save(storage, &config)?;
    Ok(Response::new().add_attribute("action", "update_cw404_config"))
}

pub fn update_coin_config(
    api: &dyn Api,
    storage: &mut dyn Storage,
    new_fee_collector: Option<String>,
    new_coin_code_id: Option<Uint64>,
    new_coin_creation_fee: Option<Uint128>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(storage)?;
    config.coin_config.fee_collector = match new_fee_collector {
        Some(fee_collector) => api.addr_validate(&fee_collector)?,
        None => config.coin_config.fee_collector,
    };
    config.coin_config.coin_code_id = match new_coin_code_id {
        Some(coin_code_id) => coin_code_id,
        None => config.coin_config.coin_code_id,
    };
    config.coin_config.coin_creation_fee = match new_coin_creation_fee {
        Some(coin_creation_fee) => coin_creation_fee,
        None => config.coin_config.coin_creation_fee,
    };
    CONFIG.save(storage, &config)?;
    Ok(Response::new().add_attribute("action", "update_coin_config"))
}
