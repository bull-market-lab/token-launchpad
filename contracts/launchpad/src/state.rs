use cosmwasm_std::Addr;
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex};
use launchpad_pkg::config::{Config, Stats};
use launchpad_pkg::token::TokenContract;

pub const DEFAULT_LIMIT: u32 = 10;
pub const MAX_LIMIT: u32 = 100;

pub const CONFIG: Item<Config> = Item::new("CONFIG");
pub const STATS: Item<Stats> = Item::new("STATS");

pub const FEE_DENOM: &str = "untrn";

pub struct TokenContractIndexes<'a> {
    pub owner: MultiIndex<'a, Addr, TokenContract, Addr>,
}
impl<'a> IndexList<TokenContract> for TokenContractIndexes<'a> {
    fn get_indexes(
        &'_ self,
    ) -> Box<dyn Iterator<Item = &'_ dyn Index<TokenContract>> + '_> {
        let v: Vec<&dyn Index<TokenContract>> = vec![&self.owner];
        Box::new(v.into_iter())
    }
}
// key is collection address, value is Collection, indexed by creator
// this map serves as a registry for all launchpad created CW404 collections
#[allow(non_snake_case)]
pub fn CW404_COLLECTIONS<'a>(
) -> IndexedMap<'a, Addr, TokenContract, TokenContractIndexes<'a>> {
    let indexes = TokenContractIndexes {
        owner: MultiIndex::new(
            |_token_id, collection| collection.creator_addr.clone(),
            "CW404_COLLECTIONS",
            "CW404_COLLECTIONS_CREATOR",
        ),
    };
    IndexedMap::new("CW404_COLLECTIONS", indexes)
}
// key is collection address, value is Collection, indexed by creator
// this map serves as a registry for all launchpad created Cosmos SDK native coins managed by token factory module
#[allow(non_snake_case)]
pub fn COINS<'a>(
) -> IndexedMap<'a, Addr, TokenContract, TokenContractIndexes<'a>> {
    let indexes = TokenContractIndexes {
        owner: MultiIndex::new(
            |_token_id, collection| collection.creator_addr.clone(),
            "COINS",
            "COINS_CREATOR",
        ),
    };
    IndexedMap::new("COINS", indexes)
}
