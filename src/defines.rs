use serde::{Deserialize, Serialize};

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
