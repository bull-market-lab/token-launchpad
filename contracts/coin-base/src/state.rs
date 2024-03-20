use coin::config::Config;
use cw_storage_plus::Item;

pub const DENOM_EXPONENT: u32 = 6;

pub const CONFIG: Item<Config> = Item::new("CONFIG");
