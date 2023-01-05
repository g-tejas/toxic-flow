use std::mem;

use serde::{Deserialize, Serialize};

use chrono::{Duration, NaiveDate};

#[derive(Debug, Deserialize)]
pub struct InputRow {
    pub id: String,
    pub price: f64,
    pub qty: f64,
    pub quote_qty: f64,
    pub time: i64,
    pub is_buyer_maker: bool,
}

// start_time needs to be signed because of algo
#[derive(Debug, Serialize)]
pub struct OutputRow {
    pub bucket_no: i64,
    pub agg_buy: f64,
    pub agg_sell: f64,
    pub start_time: i64,
    pub end_time: i64,
    pub order_imbalance: f64,
}

pub struct DateRange(pub NaiveDate, pub NaiveDate);

impl Iterator for DateRange {
    type Item = NaiveDate;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 {
            let next = self.0 + Duration::days(1);
            Some(mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
}
