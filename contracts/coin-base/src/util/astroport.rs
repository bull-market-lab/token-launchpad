use crate::contract::REPLY_ID_CREATE_PAIR;
use astroport::{
    asset::{Asset, AssetInfo},
    factory::{ExecuteMsg::CreatePair, PairType},
    pair::ExecuteMsg::ProvideLiquidity,
};
use cosmwasm_std::{
    coin, to_json_binary, Addr, Attribute, CosmosMsg, ReplyOn, SubMsg, Uint128,
    WasmMsg,
};
use shared_pkg::error::ContractError;

pub fn create_pair(
    astroport_factory_addr: &Addr,
    chain_base_denom: &str,
    base_denom: &str,
) -> Result<(SubMsg, Vec<Attribute>), ContractError> {
    let msg: SubMsg = SubMsg {
        id: REPLY_ID_CREATE_PAIR,
        msg: CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: astroport_factory_addr.to_string(),
            msg: to_json_binary(&CreatePair {
                pair_type: PairType::Xyk {},
                asset_infos: vec![
                    AssetInfo::NativeToken {
                        denom: chain_base_denom.to_string(),
                    },
                    AssetInfo::NativeToken {
                        denom: base_denom.to_string(),
                    },
                ],
                init_params: None,
            })?,
            funds: vec![],
        }),
        gas_limit: None,
        reply_on: ReplyOn::Always,
    };

    let attrs = vec![
        Attribute::new("action", "create_pair"),
        Attribute::new("token_1", chain_base_denom.to_string()),
        Attribute::new("token_2", base_denom),
    ];
    Ok((msg, attrs))
}

pub fn provide_seed_liquidity(
    paired_base_denom: &str,
    pair_addr: &Addr,
    base_denom: &str,
    creator_addr: &Addr,
    paired_base_denom_amount: Uint128,
    total_supply: Uint128,
) -> Result<(CosmosMsg, Vec<Attribute>), ContractError> {
    let msg: CosmosMsg = WasmMsg::Execute {
        contract_addr: pair_addr.to_string(),
        msg: to_json_binary(&ProvideLiquidity {
            assets: vec![
                Asset {
                    info: AssetInfo::NativeToken {
                        denom: paired_base_denom.to_string(),
                    },
                    amount: paired_base_denom_amount,
                },
                Asset {
                    info: AssetInfo::NativeToken {
                        denom: base_denom.to_string(),
                    },
                    amount: total_supply,
                },
            ],
            slippage_tolerance: None,
            auto_stake: Some(false),
            receiver: Some(creator_addr.to_string()),
        })?,
        funds: vec![
            coin(
                paired_base_denom_amount.u128(),
                paired_base_denom.to_string(),
            ),
            coin(total_supply.u128(), base_denom),
        ],
    }
    .into();

    let attrs = vec![
        Attribute::new("action", "provide_seed_liquidity"),
        Attribute::new("creator", creator_addr.to_string()),
        Attribute::new("token_1", paired_base_denom.to_string()),
        Attribute::new("token_2", base_denom),
        Attribute::new("token_1_liquidity", paired_base_denom_amount),
        Attribute::new("token_2_liquidity", total_supply),
    ];
    Ok((msg, attrs))
}
