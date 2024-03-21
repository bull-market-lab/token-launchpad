use crate::state::{CONFIG, DENOM_EXPONENT};
use cosmwasm_std::{Addr, Api, CosmosMsg, Storage, Uint64};
use cw404::config::Config;
use osmosis_std::types::{
    cosmos::bank::v1beta1::{DenomUnit, Metadata},
    osmosis::tokenfactory::v1beta1::{
        MsgCreateDenom, MsgSetBeforeSendHook, MsgSetDenomMetadata,
    },
};
use shared_pkg::{
    denom_helpers::{
        convert_subdenom_to_base_denom, convert_subdenom_to_base_subdenom,
        convert_subdenom_to_denom,
    },
    error::ContractError,
};

pub fn create_and_mint_token(
    api: &dyn Api,
    storage: &mut dyn Storage,
    contract_addr: &Addr,
    admin_addr: Option<String>,
    minter_addr: &Addr,
    creator_addr: &Addr,
    royalty_payment_addr: &Addr,
    royalty_percentage: Uint64,
    subdenom: &str,
    denom_description: &str,
    denom_name: &str,
    denom_symbol: &str,
    denom_uri: &str,
    denom_uri_hash: &str,
) -> Result<Vec<CosmosMsg>, ContractError> {
    let (denom, base_subdenom, base_denom) = (
        convert_subdenom_to_denom(subdenom, contract_addr),
        convert_subdenom_to_base_subdenom(subdenom),
        convert_subdenom_to_base_denom(subdenom, contract_addr),
    );

    let metadata = Metadata {
        description: denom_description.to_string(),
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
                aliases: vec![subdenom.to_string()],
            },
        ],
        // e.g. factory/contract_addr/uatom
        base: base_denom.clone(),
        // e.g. factory/contract_addr/atom
        display: denom.clone(),
        // e.g. Cosmos Hub
        name: denom_name.to_string(),
        // e.g. ATOM
        symbol: denom_symbol.to_string(),
        uri: denom_uri.to_string(),
        uri_hash: denom_uri_hash.to_string(),
    };

    CONFIG.save(
        storage,
        &Config {
            admin_addr: admin_addr
                .map(|addr| api.addr_validate(&addr).unwrap()),
            minter_addr: minter_addr.clone(),
            creator_addr: creator_addr.clone(),
            denom_metadata: metadata.clone(),
            royalty_payment_addr: royalty_payment_addr.clone(),
            royalty_percentage,
        },
    )?;
    let msgs: Vec<CosmosMsg> = vec![
        MsgCreateDenom {
            sender: contract_addr.to_string(),
            // e.g. uatom
            subdenom: base_subdenom.clone(),
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

    Ok(msgs)
}
