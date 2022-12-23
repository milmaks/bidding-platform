use cosmwasm_std::{Addr, Coin, Decimal, StdResult};
use cw_multi_test::{App, Executor};

use crate::{
    error::ContractError,
    msg::{ExecMsg, InstantiateMsg, QueryMsg, ValueResponse},
};

#[cfg(test)]
mod tests;

pub struct BiddingPlatform(Addr);

impl BiddingPlatform {
    pub fn addr(&self) -> &Addr {
        &self.0
    }

    #[track_caller]
    pub fn instantiate(
        app: &mut App,
        code_id: u64,
        sender: &Addr,
        label: &str,
        owner: Option<&Addr>,
        part: Decimal,
        token: String,
    ) -> StdResult<BiddingPlatform> {
        app.instantiate_contract(
            code_id,
            sender.clone(),
            &InstantiateMsg {
                owner: owner.map(Addr::to_string),
                part: part,
                token: token,
            },
            &[],
            label,
            None,
        )
        .map_err(|err| err.downcast().unwrap())
        .map(BiddingPlatform)
    }

    #[track_caller]
    pub fn bid(&self, app: &mut App, sender: &Addr, funds: &[Coin]) -> Result<(), ContractError> {
        app.execute_contract(sender.clone(), self.0.clone(), &ExecMsg::Bid {}, funds)
            .map_err(|err| err.downcast::<ContractError>().unwrap())?;

        Ok(())
    }

    #[track_caller]
    pub fn close(&self, app: &mut App, sender: &Addr) -> Result<(), ContractError> {
        app.execute_contract(sender.clone(), self.0.clone(), &ExecMsg::Close {}, &[])
            .map_err(|err| err.downcast::<ContractError>().unwrap())?;

        Ok(())
    }

    #[track_caller]
    pub fn retract(
        &self,
        app: &mut App,
        sender: &Addr,
        receiver: Option<String>,
    ) -> Result<(), ContractError> {
        app.execute_contract(
            sender.clone(),
            self.0.clone(),
            &ExecMsg::Retract { receiver: receiver },
            &[],
        )
        .map_err(|err| err.downcast::<ContractError>().unwrap())?;

        Ok(())
    }

    pub fn query_value(&self, app: &App) -> StdResult<ValueResponse> {
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::Value {})
    }
}
