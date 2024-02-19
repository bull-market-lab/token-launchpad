use cosmwasm_std::{Addr, Uint64};
use cw_storage_plus::{Deque, Item};
use osmosis_std::types::cosmos::bank::v1beta1::Metadata;

pub const SUBDENOM: Item<String> = Item::new("SUBDENOM");
pub const ADMIN_ADDR: Item<Addr> = Item::new("ADMIN_ADDR");
// 1 denom = 1 NFT = 1 * 10 ** exponent base denom
pub const MAX_DENOM_SUPPLY: Item<Uint64> = Item::new("MAX_DENOM_SUPPLY");
pub const METADATA: Item<Metadata> = Item::new("METADATA");
pub const RECYCLED_NFT_IDS: Deque<Uint64> = Deque::new("RECYCLED_NFT_IDS");
