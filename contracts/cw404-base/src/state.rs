use cosmwasm_std::{Addr, Uint128};
use cw2981_royalties::Extension as NftExtension;
use cw404::config::Config;
use cw721_base::state::TokenInfo as NftTokenInfo;
use cw_storage_plus::{
    Deque, Index, IndexList, IndexedMap, Item, Map, MultiIndex,
};
use cw_utils::Expiration;

pub const DEFAULT_LIMIT: u32 = 10;
pub const MAX_LIMIT: u32 = 100;
pub const DENOM_EXPONENT: u32 = 6;

pub const CONFIG: Item<Config> = Item::new("CONFIG");

/// 1 NFT = 1 denom (e.g. ATOM) = 1 * 10 ** exponent base denom (uatom)
/// e.g. 1 ATOM = 1_000_000 uatom when exponent = 6, ATOM is both denom (FT) and NFT
pub const MAX_NFT_SUPPLY: Item<Uint128> = Item::new("MAX_NFT_SUPPLY");
/// Current NFT supply
pub const CURRENT_NFT_SUPPLY: Item<Uint128> = Item::new("CURRENT_NFT_SUPPLY");

/// Recycled NFT IDs, avaliable for minting
/// When burned, the NFT ID is recycled and added to end of the queue
/// When minted, the NFT ID is removed from the front of the queue or created if empty
pub const RECYCLED_NFT_IDS: Deque<Uint128> = Deque::new("RECYCLED_NFT_IDS");

/// Stored as (granter, operator) giving operator full control over granter's account
pub const NFT_OPERATORS: Map<(&Addr, &Addr), Expiration> =
    Map::new("NFT_OPERATORS");

pub struct NftIndexes<'a> {
    pub owner: MultiIndex<'a, Addr, NftTokenInfo<NftExtension>, u128>,
}
impl<'a> IndexList<NftTokenInfo<NftExtension>> for NftIndexes<'a> {
    fn get_indexes(
        &'_ self,
    ) -> Box<dyn Iterator<Item = &'_ dyn Index<NftTokenInfo<NftExtension>>> + '_>
    {
        let v: Vec<&dyn Index<NftTokenInfo<NftExtension>>> = vec![&self.owner];
        Box::new(v.into_iter())
    }
}
#[allow(non_snake_case)]
pub fn NFTS<'a>(
) -> IndexedMap<'a, u128, NftTokenInfo<NftExtension>, NftIndexes<'a>> {
    let indexes = NftIndexes {
        owner: MultiIndex::new(
            |_token_id, nft| nft.owner.clone(),
            "NFTS",
            "NFTS_NFT_OWNER",
        ),
    };
    IndexedMap::new("NFTS", indexes)
}
