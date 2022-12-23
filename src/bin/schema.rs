use bidding_platform::msg::{InstantiateMsg, ExecMsg, QueryMsg};
use cosmwasm_schema::write_api;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecMsg,
        query: QueryMsg
    }
}