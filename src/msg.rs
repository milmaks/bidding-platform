use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Addr, Uint128};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ValueResponse)]
    Value {},
}

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Option<String>,
    pub part: Decimal,
    pub token: String
}

#[cw_serde]
pub enum ExecMsg {
    Bid {},
    Close {},
    Retract {
        receiver: Option<String>
    }
}

#[cw_serde]
pub struct ValueResponse {
    pub open: bool,
    pub token: String,
    pub owner: Addr,
    pub part: Decimal,
    pub bids: Vec<Bid>,
    pub highest_bid: Bid
}

#[cw_serde]
pub struct Bid {
    pub addr: Addr,
    pub amount: Uint128,
}