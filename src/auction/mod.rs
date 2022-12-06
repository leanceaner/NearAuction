pub mod DutchAuction;
pub mod EnglishAuction;
pub mod traits;
use chrono::{DateTime, Duration, TimeZone, Utc};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use crate::auction::traits::*;

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct Auction {
    name: String,
    // account_id -> string
    creator: String,
    temp_owner: String,

    start_time: i64,
    end_time: i64,
    duration: i64,
    good: String,
    // later change
    clearing_form: String,
    base_price: f64,
    current_price: f64,
    transaction_price: f64,
}

impl Auction {
    pub fn new(
        name: String,
        creator: String,
        temp_owner: String,

        start_time: i64,
        end_time: i64,
        duration: i64,
        good: String,

        clearing_form: String,
        base_price: f64,
        current_price: f64,
        transaction_price: f64,
    ) -> Self {
        Auction {
            name,
            creator,
            temp_owner,
            start_time,
            end_time,
            duration,
            good,
            clearing_form,
            base_price,
            current_price,
            transaction_price,
        }
    }
}

impl StartToEnd for Auction {
    fn get_start_time(&self) -> i64 {
        self.start_time.clone()
    }

    fn get_end_time(&self) -> i64 {
        self.end_time.clone()
    }

    fn set_end_time(&mut self, new_time: i64) {
        self.end_time = new_time
    }
}
impl PriceOperation for Auction {
    fn get_base_price(&self) -> f64 {
        self.base_price.clone()
    }

    fn get_current_price(&self) -> f64 {
        self.current_price.clone()
    }

    fn get_transaction_price(&self) -> f64 {
        self.transaction_price.clone()
    }

    fn set_base_price(&mut self, price: f64) {
        self.base_price = price
    }

    fn set_current_price(&mut self, price: f64) {
        self.current_price = price
    }

    fn set_transaction_price(&mut self, price: f64) {
        self.transaction_price = price
    }
}

impl PersonOperation for Auction {
    fn get_creator(&self) -> String {
        self.creator.clone()
    }
    fn get_temp_owner(&self) -> String {
        self.temp_owner.clone()
    }
    fn set_temp_owner(&mut self, account: String) {
        self.temp_owner = account;
    }
}

impl FinshBidding for Auction {
    fn finish_bidding(&self, timestamp: i64) {
        //assert!(timestamp > self.get_end_time(),"time not after auction end");
        assert!(
            self.get_temp_owner() != self.get_creator(),
            "not need change owner"
        );
        //change_owner()
        //change_goods_status()
    }
}
