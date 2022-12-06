use chrono::{Date, DateTime, Duration, Utc};

pub trait StartToEnd {
    fn get_start_time(&self) -> i64;
    fn get_end_time(&self) -> i64;
    fn get_duration(&self) -> i64 {
        self.get_end_time() - self.get_start_time()
    }

    fn set_end_time(&mut self, new_time: i64);
}

pub trait PriceOperation {
    fn set_base_price(&mut self, price: f64);
    fn set_current_price(&mut self, price: f64);
    fn set_transaction_price(&mut self, price: f64);

    fn get_base_price(&self) -> f64;
    fn get_current_price(&self) -> f64;
    fn get_transaction_price(&self) -> f64;
}

pub trait PersonOperation {
    fn get_creator(&self) -> String;
    fn get_temp_owner(&self) -> String;

    fn set_temp_owner(&mut self, account: String);
}

pub trait Bidding {
    fn bidding(&mut self, account_id: String, price: f64, time_stamp: i64);
}

pub trait FinshBidding {
    fn finish_bidding(&self, timestamp: i64);
}
