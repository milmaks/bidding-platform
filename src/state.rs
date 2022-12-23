use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_storage_plus::{Item, Map};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct State {
    pub open: bool,
    pub token: String,
    pub part: Decimal
}

pub const STATE: Item<State> = Item::new("state");
pub const OWNER: Item<Addr> = Item::new("owner");
pub const BIDS: Map<&Addr, Uint128> = Map::new("bids");
pub const HIGHEST_BID: Item<(Addr, Uint128)> = Item::new("highest_bid");