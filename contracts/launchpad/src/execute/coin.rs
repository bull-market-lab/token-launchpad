use crate::{contract::REPLY_ID_INSTANTIATE_CW404_CONTRACT, state::FEE_DENOM};
use coin::msg::{InstantiateMsg as CoinInstantiateMsg, SeedLiquidityConfig};
use cosmwasm_std::{
    coins, to_json_binary, Addr, BankMsg, CosmosMsg, ReplyOn, Response, SubMsg,
    Uint128, WasmMsg,
};
use launchpad_pkg::config::Config;
use shared_pkg::error::ContractError;

pub fn create_coin(
    config: &Config,
    launchpad_addr: Addr,
    creator_addr: Addr,
    creator_paid_amount: Uint128,
    immutable: bool,
    initial_supply_in_denom: Uint128,
    max_supply_in_denom: Uint128,
    subdenom: String,
    denom_description: String,
    denom_name: String,
    denom_symbol: String,
    denom_uri: String,
    denom_uri_hash: String,
) -> Result<Response, ContractError> {
    if creator_paid_amount < config.coin_config.coin_creation_fee {
        return Err(ContractError::InsufficientFundsToCreateCoin {
            paid: creator_paid_amount,
            required: config.coin_config.coin_creation_fee,
        });
    }
    let seed_liquidity =
        creator_paid_amount - config.coin_config.coin_creation_fee;
    let send_creation_fee_to_fee_collector_msg: BankMsg = BankMsg::Send {
        to_address: config.coin_config.fee_collector.to_string(),
        amount: coins(config.coin_config.coin_creation_fee.u128(), FEE_DENOM),
    };
    let instantiate_coin_submsg = SubMsg {
        id: REPLY_ID_INSTANTIATE_CW404_CONTRACT,
        msg: CosmosMsg::Wasm(WasmMsg::Instantiate {
            admin: if immutable {
                None
            } else {
                Some(launchpad_addr.to_string())
            },
            code_id: config.coin_config.coin_code_id.u64(),
            msg: to_json_binary(&CoinInstantiateMsg {
                admin_addr: if immutable {
                    None
                } else {
                    Some(creator_addr.to_string())
                },
                creator_addr: creator_addr.to_string(),
                max_supply_in_denom,
                initial_supply_in_denom,
                seed_liquidity_config: if Uint128::is_zero(&seed_liquidity) {
                    None
                } else {
                    Some(SeedLiquidityConfig {
                        astroport_factory_addr: config
                            .astroport_factory_addr
                            .to_string(),
                        paired_base_denom: FEE_DENOM.to_string(),
                        paired_base_denom_amount: seed_liquidity,
                    })
                },
                subdenom,
                denom_description,
                denom_name,
                denom_symbol,
                denom_uri,
                denom_uri_hash,
            })
            .unwrap(),
            funds: if Uint128::is_zero(&seed_liquidity) {
                vec![]
            } else {
                coins(seed_liquidity.u128(), FEE_DENOM)
            },
            label: "Cosmos SDK native coin".to_string(),
        }),
        gas_limit: None,
        reply_on: ReplyOn::Always,
    };
    Ok(Response::new()
        .add_submessage(instantiate_coin_submsg)
        .add_message(send_creation_fee_to_fee_collector_msg)
        .add_attribute("action", "create_coin")
        .add_attribute("amount", creator_paid_amount))
}
