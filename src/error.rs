use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized - only {owner} can call it")]
    Unauthorized { owner: String },

    #[error("Bidding is closed")]
    BiddingClosed,

    #[error("Bid low. Highest bid: {highest}, sender total: {sender_total}")]
    BidLow {highest: Uint128, sender_total: Uint128},

    #[error("Bidding is already closed")]
    BiddingAlreadyClosed,

    #[error("Cant retract until bidding is closed")]
    EarlyRetractErr,

    #[error("Dont have any bids")]
    NoBidsRetractErr,
}