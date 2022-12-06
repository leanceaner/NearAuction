use std::sync::WaitTimeoutResult;

use chrono::DateTime;
use chrono::Duration;
use chrono::TimeZone;
use chrono::Utc;

use super::traits::Bidding;
use super::traits::PriceOperation;
use crate::auction::traits::*;
use crate::auction::Auction;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

/*
The most seen type of auction, it’s also known as Open ascending auction.
The buyers will start bidding with a low price, then the bid prices go up
until there’s no more price bidding a higher price. The last person to bid,
 which is also the person who bids the highest price, will get the item.
 If the seller has a predetermined reserved price, we need to make sure the
 highest bid excesses the reserved price, or the item will not be sold to anyone
  (the seller keeps it since he/she values the item more than any bidder).
  The dominant strategy of bidder in an English auction is to bid a price
  less than their value for the item. However, in the real-world scenario,
   there exists a term called the “Winner’s Curse”, which refers to the circumstance
   when a bidder behaves irrationally and bid a price that’s higher than his/her value of the item.
 */
#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct EnglishAuction(Auction, i64);

impl EnglishAuction {
    pub fn new(auction: Auction, delay: i64) -> Self {
        EnglishAuction(auction, delay)
    }

    pub fn as_auction(&self) -> &Auction {
        &self.0
    }

    fn as_mut_auction(&mut self) -> &mut Auction {
        &mut self.0
    }

    fn get_delay(&self) -> i64 {
        self.1.clone()
    }

    fn set_delay(&mut self, new_delay: i64) {
        self.1 = new_delay
    }
}

impl Bidding for EnglishAuction {
    fn bidding(&mut self, account_id: String, price: f64, time_stamp: i64) {
        assert!(
            time_stamp > self.as_auction().get_start_time()
                && time_stamp < self.as_auction().get_end_time(),
            "time is not in auction"
        );
        let end_time = self.as_auction().get_end_time();
        let delay = self.get_delay();
        if end_time - time_stamp < delay {
            self.as_mut_auction().set_end_time(time_stamp + delay);
        }
        assert!(
            price > self.as_auction().get_current_price(),
            "you price is too low"
        );
        self.as_mut_auction().set_current_price(price);
        self.as_mut_auction().set_temp_owner(account_id);
        // transaction lastprice -> temp_owner nowprice -> auction
    }
}

