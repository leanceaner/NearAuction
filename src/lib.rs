use std::collections::{hash_map, HashMap};
use std::path::PrefixComponent;
use std::time::SystemTime;

use crate::auction::traits::{PriceOperation, StartToEnd};
use crate::auction::Auction;
use auction::traits::Bidding;
use auction::EnglishAuction::EnglishAuction;
use chrono::{Duration, TimeZone, Utc};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::{account_balance, block_timestamp};
use near_sdk::test_utils::VMContextBuilder;
use near_sdk::{near_bindgen, AccountId, PanicOnDefault};

pub mod auction;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    auction: EnglishAuction,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        let stime = block_timestamp() as i64;
        let etime = stime + 24 * 3600 * 1_000_000_000;
        let name = "near_nft".to_string();
        let creator = "zhangwei".to_string();
        let temp_creator = "nn_auction".to_string();
        let good = "a nft".to_string();
        let clearing_form = "near".to_string();
        let base_price = 2.5;
        let current_price = base_price.clone();
        let transaction_price = base_price.clone();
        let a = Auction::new(
            name,
            creator,
            temp_creator,
            stime,
            etime,
            etime - stime,
            good,
            clearing_form,
            base_price,
            current_price,
            transaction_price,
        );
        Self {
            auction: EnglishAuction::new(a, 10 * 60 * 1_000_000_000),
        }
    }

    pub fn bidding(&mut self, account_id: AccountId, price: f64) {
        self.auction
            .bidding(account_id.to_string(), price, block_timestamp() as i64)
    }

    pub fn get_price(&self) -> f64 {
        self.auction.as_auction().get_current_price()
    }
}

#[test]
    fn test_cliff_claim() {
        let mut context = VMContextBuilder::new();
        let contract = Contract::new();
        
    }