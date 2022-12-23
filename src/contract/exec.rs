use cosmwasm_std::{coins, ensure, BankMsg, DepsMut, MessageInfo, Response, Uint128};

use crate::{
    error::ContractError,
    state::{BIDS, HIGHEST_BID, OWNER, STATE},
};

pub fn bid(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    ensure!(state.open, ContractError::BiddingClosed);

    let mut bid: Uint128 = Uint128::zero();
    let mut comission: Uint128 = Uint128::zero();

    for coin in info.funds.iter() {
        if coin.denom == state.token {
            bid = coin.amount;
            comission = bid * state.part;
            bid = bid - comission;
        }
    }

    let highest = HIGHEST_BID.load(deps.storage)?.1;
    let mut sender_total = BIDS
        .may_load(deps.storage, &info.sender)?
        .unwrap_or_default();
    ensure!(
        sender_total + bid > highest,
        ContractError::BidLow {
            highest,
            sender_total
        }
    );

    sender_total += bid;
    BIDS.save(deps.storage, &info.sender.clone(), &sender_total)?;
    HIGHEST_BID.save(deps.storage, &(info.sender.clone(), sender_total))?;

    let commision_message = BankMsg::Send {
        to_address: OWNER.load(deps.storage)?.into(),
        amount: coins(comission.u128(), state.token),
    };

    let resp = Response::new()
        .add_attribute("action", "bid")
        .add_attribute("sender", info.sender.as_str())
        .add_attribute("sender_total", sender_total)
        .add_message(commision_message);

    Ok(resp)
}

pub fn close(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    ensure!(state.open, ContractError::BiddingAlreadyClosed);

    let owner = OWNER.load(deps.storage)?;
    ensure!(
        info.sender == owner,
        ContractError::Unauthorized {
            owner: owner.into()
        }
    );

    state.open = false;
    STATE.save(deps.storage, &state)?;

    let highest_bid = HIGHEST_BID.load(deps.storage)?;

    if !Uint128::is_zero(&highest_bid.1) {
        let paying_message = BankMsg::Send {
            to_address: OWNER.load(deps.storage)?.into(),
            amount: coins(highest_bid.1.u128(), state.token),
        };

        BIDS.remove(deps.storage, &highest_bid.0);

        let resp = Response::new()
            .add_attribute("action", "close")
            .add_attribute("sender", info.sender.as_str())
            .add_message(paying_message);

        Ok(resp)
    } else {
        let resp = Response::new()
            .add_attribute("action", "close")
            .add_attribute("sender", info.sender.as_str());

        Ok(resp)
    }
}

pub fn retract(
    deps: DepsMut,
    info: MessageInfo,
    receiver: Option<String>,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    ensure!(!state.open, ContractError::EarlyRetractErr);

    ensure!(
        BIDS.has(deps.storage, &info.sender),
        ContractError::NoBidsRetractErr
    );

    let total = BIDS.load(deps.storage, &info.sender).unwrap();
    BIDS.remove(deps.storage, &info.sender);

    if let Some(receiver) = receiver {
        let transfer_message = BankMsg::Send {
            to_address: receiver.clone(),
            amount: coins(total.u128(), state.token),
        };

        let resp = Response::new()
            .add_attribute("action", "close")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("receiver", receiver.as_str())
            .add_message(transfer_message);

        Ok(resp)
    } else {
        let transfer_message = BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: coins(total.u128(), state.token),
        };

        let resp = Response::new()
            .add_attribute("action", "close")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("receiver", info.sender.as_str())
            .add_message(transfer_message);

        Ok(resp)
    }
}
