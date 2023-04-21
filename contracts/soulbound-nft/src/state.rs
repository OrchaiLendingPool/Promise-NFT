use cw_storage_plus::Item;
use promise_nft::nft::Config;

pub const CONFIG: Item<Config> = Item::new("config");
