use cosmwasm_std::StdResult;
use cw404::{config::Config, msg::ConfigResponse};

pub fn query_config(config: &Config) -> StdResult<ConfigResponse> {
    Ok(ConfigResponse {
        config: config.clone(),
    })
}
