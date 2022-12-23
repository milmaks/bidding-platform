use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult, Uint128};
use cw2::set_contract_version;

use crate::{
    msg::InstantiateMsg,
    state::{State, HIGHEST_BID, OWNER, STATE},
};

pub fn instantiate(deps: DepsMut, info: MessageInfo, msg: InstantiateMsg) -> StdResult<Response> {
    set_contract_version(
        deps.storage,
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    )?;
    
    STATE.save(
        deps.storage,
        &State {
            open: true,
            part: msg.part,
            token: msg.token,
        },
    )?;

    if let Some(addr) = msg.owner {
        OWNER.save(deps.storage, &deps.api.addr_validate(&addr)?)?
    } else {
        OWNER.save(deps.storage, &info.sender)?;
    };

    HIGHEST_BID.save(deps.storage, &(info.sender.clone(), Uint128::zero()))?;
    
    Ok(Response::new())
}

pub mod query {
    use cosmwasm_std::{Deps, Order, StdResult};

    use crate::{
        msg::{Bid, ValueResponse},
        state::{BIDS, HIGHEST_BID, OWNER, STATE},
    };

    pub fn value(deps: Deps) -> StdResult<ValueResponse> {
        let state = STATE.load(deps.storage)?;
        let owner = OWNER.load(deps.storage)?;
        let highest_bid = HIGHEST_BID.load(deps.storage)?;

        let bids = BIDS
            .range(deps.storage, None, None, Order::Ascending)
            .map(|item| {
                let (addr, amount) = item?;
                Ok(Bid { addr, amount })
            })
            .collect::<StdResult<_>>()?;

        Ok(ValueResponse {
            bids: bids,
            highest_bid: Bid {
                addr: highest_bid.0,
                amount: highest_bid.1,
            },
            owner: owner,
            open: state.open,
            part: state.part,
            token: state.token,
        })
    }
}

pub mod exec;