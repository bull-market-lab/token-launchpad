use coin::{config::Config, msg::ConfigResponse};
use cosmwasm_std::StdResult;

pub fn query_config(config: &Config) -> StdResult<ConfigResponse> {
    Ok(ConfigResponse {
        config: config.clone(),
    })
}
