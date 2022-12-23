use cosmwasm_std::{coins, Addr, Decimal, Empty, Uint128};
use cw_multi_test::{App, Contract, ContractWrapper};

use crate::{execute, instantiate, msg::Bid, multitest::BiddingPlatform, query, error::ContractError};

fn bidding_platform() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}

const ATOM: &str = "atom";

#[test]
fn query_value() {
    let mut app = App::default();
    let sender = Addr::unchecked("sender");

    let contract_id = app.store_code(bidding_platform());

    let contract = BiddingPlatform::instantiate(
        &mut app,
        contract_id,
        &sender,
        "Bidding contract",
        None,
        Decimal::percent(10),
        ATOM.to_string(),
    )
    .unwrap();

    let resp = contract.query_value(&app).unwrap();

    assert_eq!(resp.open, true);
    assert_eq!(resp.token, "atom");
    assert_eq!(resp.owner, sender);
    assert_eq!(resp.part, Decimal::percent(10));
    assert_eq!(resp.bids, vec![]);
    assert_eq!(
        resp.highest_bid,
        Bid {
            addr: sender,
            amount: Uint128::zero()
        }
    );
}

#[test]
fn create_contract_with_owner() {
    let mut app = App::default();
    let sender = Addr::unchecked("sender");
    let owner = Addr::unchecked("owner");

    let contract_id = app.store_code(bidding_platform());

    let contract = BiddingPlatform::instantiate(
        &mut app,
        contract_id,
        &sender,
        "Bidding contract",
        Some(&owner),
        Decimal::percent(10),
        ATOM.to_string(),
    )
    .unwrap();

    let resp = contract.query_value(&app).unwrap();

    assert_eq!(resp.open, true);
    assert_eq!(resp.token, "atom");
    assert_eq!(resp.owner, owner);
    assert_eq!(resp.part, Decimal::percent(10));
    assert_eq!(resp.bids, vec![]);
    assert_eq!(
        resp.highest_bid,
        Bid {
            addr: sender,
            amount: Uint128::zero()
        }
    );
}

#[test]
pub fn bid_close_retract() {
    let sender1 = Addr::unchecked("sender1");
    let sender2 = Addr::unchecked("sender2");
    let sender3 = Addr::unchecked("sender3");
    let owner = Addr::unchecked("owner");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender1, coins(20, ATOM))
            .unwrap();

        router
            .bank
            .init_balance(storage, &sender2, coins(10, ATOM))
            .unwrap();

        router
            .bank
            .init_balance(storage, &sender3, coins(5, ATOM))
            .unwrap();
    });

    let contract_id = app.store_code(bidding_platform());

    let contract = BiddingPlatform::instantiate(
        &mut app,
        contract_id,
        &sender1,
        "Bidding contract",
        Some(&owner),
        Decimal::percent(10),
        ATOM.to_string(),
    )
    .unwrap();

    contract.bid(&mut app, &sender2, &coins(10, ATOM)).unwrap();

    let resp = contract.query_value(&app).unwrap();

    assert_eq!(resp.open, true);
    assert_eq!(resp.token, "atom");
    assert_eq!(resp.owner, owner);
    assert_eq!(resp.part, Decimal::percent(10));
    assert_eq!(
        resp.bids,
        vec![Bid {
            addr: sender2.clone(),
            amount: Uint128::new(9)
        }]
    );
    assert_eq!(
        resp.highest_bid,
        Bid {
            addr: sender2.clone(),
            amount: Uint128::new(9)
        }
    );
    assert_eq!(
        app.wrap().query_all_balances(&owner).unwrap(),
        coins(1, ATOM)
    );
    assert_eq!(
        app.wrap().query_all_balances(&sender1).unwrap(),
        coins(20, ATOM)
    );
    assert_eq!(
        app.wrap().query_all_balances(&sender2).unwrap(),
        vec![]
    );
    assert_eq!(
        app.wrap().query_all_balances(contract.addr()).unwrap(),
        coins(9, ATOM)
    );

    let err = contract.bid(&mut app, &sender3, &coins(5, ATOM)).unwrap_err();

    assert_eq!(
        err,
        ContractError::BidLow { highest: resp.highest_bid.amount, sender_total: Uint128::zero() }
    );

    contract.bid(&mut app, &sender1, &coins(20, ATOM)).unwrap();

    let mut resp = contract.query_value(&app).unwrap();

    assert_eq!(resp.open, true);
    assert_eq!(resp.token, "atom");
    assert_eq!(resp.owner, owner);
    assert_eq!(resp.part, Decimal::percent(10));
    assert_eq!(
        resp.bids,
        vec![Bid {
            addr: sender1.clone(),
            amount: Uint128::new(18)
        },
        Bid {
            addr: sender2.clone(),
            amount: Uint128::new(9)
        }]
    );
    assert_eq!(
        resp.highest_bid,
        Bid {
            addr: sender1.clone(),
            amount: Uint128::new(18)
        }
    );
    assert_eq!(
        app.wrap().query_all_balances(&owner).unwrap(),
        coins(3, ATOM)
    );
    assert_eq!(
        app.wrap().query_all_balances(&sender1).unwrap(),
        vec![]
    );
    assert_eq!(
        app.wrap().query_all_balances(&sender2).unwrap(),
        vec![]
    );
    assert_eq!(
        app.wrap().query_all_balances(contract.addr()).unwrap(),
        coins(27, ATOM)
    );

    contract.close(&mut app, &owner).unwrap();
    contract.retract(&mut app, &sender2, Some(sender1.to_string())).unwrap();

    resp = contract.query_value(&app).unwrap();
    assert_eq!(resp.open, false);
    assert_eq!(resp.bids, vec![]);
    assert_eq!(
        app.wrap().query_all_balances(&sender1).unwrap(),
        coins(9, ATOM)
    );
    assert_eq!(
        app.wrap().query_all_balances(&sender2).unwrap(),
        vec![]
    );
    assert_eq!(
        app.wrap().query_all_balances(contract.addr()).unwrap(),
        vec![]
    );
}

#[test]
fn close() {
    let mut app = App::default();
    let sender = Addr::unchecked("sender");
    let owner = Addr::unchecked("owner");

    let contract_id = app.store_code(bidding_platform());

    let contract = BiddingPlatform::instantiate(
        &mut app,
        contract_id,
        &sender,
        "Bidding contract",
        Some(&owner),
        Decimal::percent(10),
        ATOM.to_string(),
    )
    .unwrap();

    let mut resp = contract.query_value(&app).unwrap();

    assert_eq!(resp.open, true);
    assert_eq!(resp.token, "atom");
    assert_eq!(resp.owner, owner.clone());
    assert_eq!(resp.part, Decimal::percent(10));
    assert_eq!(resp.bids, vec![]);
    assert_eq!(
        resp.highest_bid,
        Bid {
            addr: sender.clone(),
            amount: Uint128::zero()
        }
    );

    let mut err = contract.close(&mut app, &sender).unwrap_err();
    assert_eq!(
        err,
        ContractError::Unauthorized {
            owner: owner.to_string()
        }
    );

    contract.close(&mut app, &owner).unwrap();
    resp = contract.query_value(&app).unwrap();
    assert_eq!(resp.open, false);

    err = contract.close(&mut app, &owner).unwrap_err();
    assert_eq!(
        err,
        ContractError::BiddingAlreadyClosed {}
    );

}

#[test]
fn close_with_bids() {
    let sender = Addr::unchecked("sender");
    let owner = Addr::unchecked("owner");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(10, ATOM))
            .unwrap();
    });

    let contract_id = app.store_code(bidding_platform());

    let contract = BiddingPlatform::instantiate(
        &mut app,
        contract_id,
        &sender,
        "Bidding contract",
        Some(&owner),
        Decimal::percent(10),
        ATOM.to_string(),
    )
    .unwrap();

    let mut resp = contract.query_value(&app).unwrap();

    assert_eq!(resp.open, true);
    assert_eq!(resp.token, "atom");
    assert_eq!(resp.owner, owner.clone());
    assert_eq!(resp.part, Decimal::percent(10));
    assert_eq!(resp.bids, vec![]);
    assert_eq!(
        resp.highest_bid,
        Bid {
            addr: sender.clone(),
            amount: Uint128::zero()
        }
    );

    let mut err = contract.close(&mut app, &sender).unwrap_err();
    assert_eq!(
        err,
        ContractError::Unauthorized {
            owner: owner.to_string()
        }
    );

    contract.bid(&mut app, &sender, &coins(10, ATOM)).unwrap();

    resp = contract.query_value(&app).unwrap();

    assert_eq!(resp.open, true);
    assert_eq!(resp.token, "atom");
    assert_eq!(resp.owner, owner);
    assert_eq!(resp.part, Decimal::percent(10));
    assert_eq!(
        resp.bids,
        vec![Bid {
            addr: sender.clone(),
            amount: Uint128::new(9)
        }]
    );
    assert_eq!(
        resp.highest_bid,
        Bid {
            addr: sender.clone(),
            amount: Uint128::new(9)
        }
    );
    assert_eq!(
        app.wrap().query_all_balances(&owner).unwrap(),
        coins(1, ATOM)
    );
    assert_eq!(
        app.wrap().query_all_balances(&sender).unwrap(),
        vec![]
    );
    assert_eq!(
        app.wrap().query_all_balances(contract.addr()).unwrap(),
        coins(9, ATOM)
    );

    contract.close(&mut app, &owner).unwrap();

    resp = contract.query_value(&app).unwrap();

    assert_eq!(resp.open, false);
    assert_eq!(resp.bids, vec![]);
    assert_eq!(
        app.wrap().query_all_balances(&owner).unwrap(),
        coins(10, ATOM)
    );
    assert_eq!(
        app.wrap().query_all_balances(contract.addr()).unwrap(),
        vec![]
    );

    err = contract.close(&mut app, &owner).unwrap_err();
    assert_eq!(
        err,
        ContractError::BiddingAlreadyClosed {}
    );

}