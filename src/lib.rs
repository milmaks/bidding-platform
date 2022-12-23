#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, StdResult, Response, Deps, Binary, to_binary};
use error::ContractError;
use msg::InstantiateMsg;


mod contract;
pub mod msg;
mod state;
pub mod error;
#[cfg(any(test, feature="tests"))]
pub mod multitest;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    use msg::QueryMsg::*;

    match msg {
        Value {} => to_binary(&contract::query::value(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: msg::ExecMsg,
) -> Result<Response, ContractError> {
    use msg::ExecMsg::*;

    match msg {
        Bid {} => contract::exec::bid(deps, info),
        Close {} => contract::exec::close(deps, info),
        Retract { receiver } => contract::exec::retract(deps, info, receiver),
    }
}
