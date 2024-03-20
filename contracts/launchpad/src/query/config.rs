use crate::state::{CONFIG, STATS};
use cosmwasm_std::{StdResult, Storage};
use launchpad_pkg::msg::{ConfigResponse, StatsResponse};

pub fn query_config(storage: &dyn Storage) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(storage)?;
    Ok(ConfigResponse { config })
}

pub fn query_stats(storage: &dyn Storage) -> StdResult<StatsResponse> {
    let stats = STATS.load(storage)?;
    Ok(StatsResponse { stats })
}
