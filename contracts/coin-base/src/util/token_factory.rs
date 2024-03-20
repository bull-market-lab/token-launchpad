use crate::state::{CONFIG, DENOM_EXPONENT};
use coin::{config::Config, msg::SeedLiquidityConfig};
use cosmwasm_std::{Addr, Api, CosmosMsg, Storage, Uint128};
use osmosis_std::types::{
    cosmos::bank::v1beta1::{DenomUnit, Metadata},
    cosmos::base::v1beta1::Coin as SdkCoin,
    osmosis::tokenfactory::v1beta1::{
        MsgCreateDenom, MsgMint, MsgSetDenomMetadata,
    },
};
use shared_pkg::error::ContractError;

pub fn create_and_mint_token(
    api: &dyn Api,
    storage: &mut dyn Storage,
    contract_addr: &Addr,
    admin: Option<String>,
    creator_addr: &Addr,
    initial_supply_in_base_denom: Uint128,
    max_supply_in_base_denom: Uint128,
    seed_liquidity_config: Option<SeedLiquidityConfig>,
    subdenom: &str,
    denom_description: &str,
    denom_name: &str,
    denom_symbol: &str,
    denom_uri: &str,
    denom_uri_hash: &str,
) -> Result<(String, Vec<CosmosMsg>), ContractError> {
    // e.g. factory/contract_addr/atom
    let denom = format!("factory/{}/{}", contract_addr, subdenom);
    // e.g. uatom
    let base_subdenom = format!("u{}", subdenom);
    // e.g. factory/contract_addr/uatom
    let base_denom = format!("factory/{}/{}", contract_addr, base_subdenom);

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
            admin_addr: admin
                .clone()
                .map(|addr| api.addr_validate(&addr).unwrap()),
            creator_addr: creator_addr.clone(),
            denom_metadata: metadata.clone(),
            max_supply_in_base_denom,
            seed_liquidity_config,
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
        MsgMint {
            sender: contract_addr.to_string(),
            amount: Some(SdkCoin {
                amount: initial_supply_in_base_denom.to_string(),
                denom: base_denom.to_string(),
            }),
            mint_to_address: contract_addr.to_string(),
        }
        .into(),
    ];

    Ok((base_denom, msgs))
}
