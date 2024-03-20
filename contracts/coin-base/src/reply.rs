use crate::{state::CONFIG, util::astroport::provide_seed_liquidity};
use astroport::{
    asset::{AssetInfo, PairInfo},
    factory::QueryMsg::Pair,
};
use cosmwasm_std::{
    Addr, QuerierWrapper, Reply, Response, Storage, SubMsgResult,
};
use shared_pkg::error::ContractError;

pub fn create_pair_reply(
    querier: QuerierWrapper,
    storage: &dyn Storage,
    msg: Reply,
    contract_addr: &Addr,
) -> Result<Response, ContractError> {
    let resp = match msg.result {
        SubMsgResult::Ok(resp) => resp,
        SubMsgResult::Err(err) => {
            return Err(ContractError::ErrorCreatePairInAstroport { err })
        }
    };

    let event =
        resp.events
            .iter()
            .find(|event| {
                event.attributes.iter().any(|attr| {
                    attr.key == "action" && attr.value == "create_pair"
                })
            })
            .ok_or({
                ContractError::CannotFindCreatePairEventFromAstroportReply {}
            })?;
    let pair = &event
        .attributes
        .iter()
        .find(|attr| attr.key == "pair")
        .ok_or(ContractError::CannotFindPairFromAstroportReply {})?
        .value;
    // e.g. pair = untrn-base_denom, base_denom is in the format of factory/contract_addr/base_subdenom
    let tokens = pair.split('-').collect::<Vec<&str>>();
    let mut maybe_base_denom = None;
    for token in tokens {
        if token.contains("factory") {
            maybe_base_denom = Some(token);
        }
    }

    let base_denom = match maybe_base_denom {
        Some(base_denom) => base_denom,
        None => {
            return Err(
                ContractError::CannotFindCreatedTokenFromAstroportReply {},
            )
        }
    };

    let config = CONFIG.load(storage)?;

    let seed_liquidity_config = match config.seed_liquidity_config {
        Some(cfg) => cfg,
        None => return Err(ContractError::CannotFindSeedLiquidityConfig {}),
    };

    let pair: PairInfo = querier.query_wasm_smart(
        seed_liquidity_config.clone().astroport_factory_addr,
        &Pair {
            asset_infos: vec![
                AssetInfo::NativeToken {
                    denom: seed_liquidity_config.clone().paired_base_denom,
                },
                AssetInfo::NativeToken {
                    denom: base_denom.to_string(),
                },
            ],
        },
    )?;
    let pair_addr = pair.contract_addr;

    let total_supply = querier.query_balance(contract_addr, base_denom)?.amount;
    let (provide_seed_liquidity_msg, provide_seed_liquidity_attributes) =
        provide_seed_liquidity(
            seed_liquidity_config.paired_base_denom.as_str(),
            &pair_addr,
            base_denom,
            &config.creator_addr,
            seed_liquidity_config.paired_base_denom_amount,
            total_supply,
        )?;

    Ok(Response::new()
        .add_message(provide_seed_liquidity_msg)
        .add_attributes(provide_seed_liquidity_attributes))
}
