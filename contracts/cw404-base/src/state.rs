use cosmwasm_std::{Addr, Uint64};
use cw404::nft::Nft;
use cw_storage_plus::{
    Deque, Index, IndexList, IndexedMap, Item, Map, MultiIndex,
};
use cw_utils::Expiration;
use osmosis_std::types::cosmos::bank::v1beta1::Metadata;

pub const SUBDENOM: Item<String> = Item::new("SUBDENOM");
pub const ADMIN_ADDR: Item<Addr> = Item::new("ADMIN_ADDR");
pub const METADATA: Item<Metadata> = Item::new("METADATA");

/// 1 NFT = 1 denom (e.g. BAD) = 1 * 10 ** exponent base denom (ubad)
pub const MAX_NFT_SUPPLY: Item<Uint64> = Item::new("MAX_NFT_SUPPLY");

/// Current NFT supply
pub const CURRENT_NFT_SUPPLY: Item<Uint64> = Item::new("CURRENT_NFT_SUPPLY");

/// Recycled NFT IDs, avaliable for minting
pub const RECYCLED_NFT_IDS: Deque<Uint64> = Deque::new("RECYCLED_NFT_IDS");

/// Balances for NFT, since 1 NFT = 1 denom, so also balance for denom
/// Key is addr, value is balance
pub const NFT_BALANCES: Map<&Addr, Uint64> = Map::new("NFT_BALANCES");

/// Stored as (granter, operator) giving operator full control over granter's account
pub const NFT_OPERATORS: Map<(&Addr, &Addr), Expiration> =
    Map::new("NFT_OPERATORS");

pub struct NftIndexes<'a> {
    pub owner: MultiIndex<'a, Addr, Nft, u64>,
}
impl<'a> IndexList<Nft> for NftIndexes<'a> {
    fn get_indexes(
        &'_ self,
    ) -> Box<dyn Iterator<Item = &'_ dyn Index<Nft>> + '_> {
        let v: Vec<&dyn Index<Nft>> = vec![&self.owner];
        Box::new(v.into_iter())
    }
}
#[allow(non_snake_case)]
pub fn NFTS<'a>() -> IndexedMap<'a, u64, Nft, NftIndexes<'a>> {
    let indexes = NftIndexes {
        owner: MultiIndex::new(
            |_token_id, nft| nft.owner.clone(),
            "NFTS",
            "NFTS_NFT_OWNER",
        ),
    };
    IndexedMap::new("NFTS", indexes)
}
