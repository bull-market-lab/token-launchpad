use crate::state::{COLLECTIONS, CONFIG, DEFAULT_LIMIT, MAX_LIMIT};
use cosmwasm_std::{Addr, Order, StdResult, Storage};
use cw_storage_plus::Bound;
use launchpad_pkg::msg::{
    CollectionsResponse, ConfigResponse, CreatorResponse,
};

pub fn query_config(storage: &dyn Storage) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(storage)?;
    Ok(ConfigResponse { config })
}

pub fn query_collection_creator(
    storage: &dyn Storage,
    collection_addr: Addr,
) -> StdResult<CreatorResponse> {
    let collection = COLLECTIONS().load(storage, collection_addr)?;
    Ok(CreatorResponse {
        creator_addr: collection.creator_addr,
    })
}

pub fn query_creator_collections(
    storage: &dyn Storage,
    creator_addr: Addr,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<CollectionsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));
    let collection_addrs: Vec<Addr> = COLLECTIONS()
        .idx
        .owner
        .prefix(creator_addr)
        .keys(storage, start, None, Order::Ascending)
        .take(limit)
        .collect::<StdResult<Vec<_>>>()?;
    Ok(CollectionsResponse { collection_addrs })
}
