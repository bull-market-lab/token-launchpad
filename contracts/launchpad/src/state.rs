use cosmwasm_std::Addr;
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex};
use launchpad_pkg::collection::Collection;
use launchpad_pkg::config::Config;

pub const DEFAULT_LIMIT: u32 = 10;
pub const MAX_LIMIT: u32 = 100;

pub const CONFIG: Item<Config> = Item::new("CONFIG");
pub const FEE_DENOM: &str = "untrn";

pub struct CollectionIndexes<'a> {
    pub owner: MultiIndex<'a, Addr, Collection, Addr>,
}
impl<'a> IndexList<Collection> for CollectionIndexes<'a> {
    fn get_indexes(
        &'_ self,
    ) -> Box<dyn Iterator<Item = &'_ dyn Index<Collection>> + '_> {
        let v: Vec<&dyn Index<Collection>> = vec![&self.owner];
        Box::new(v.into_iter())
    }
}
// key is collection address, value is Collection, indexed by creator
#[allow(non_snake_case)]
pub fn COLLECTIONS<'a>(
) -> IndexedMap<'a, Addr, Collection, CollectionIndexes<'a>> {
    let indexes = CollectionIndexes {
        owner: MultiIndex::new(
            |_token_id, collection| collection.creator_addr.clone(),
            "COLLECTIONS",
            "COLLECTIONS_CREATOR",
        ),
    };
    IndexedMap::new("COLLECTIONS", indexes)
}
