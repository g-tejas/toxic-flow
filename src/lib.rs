use barter_data::event::MarketEvent;
use barter_data::subscription::trade::PublicTrade;
use barter_integration::model::Side;
use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use serde::Serialize;
use tracing::*;

pub mod historical;
pub mod montecarlo;
pub mod live;

#[derive(Parser)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
	Historical(historical::Options),
	MonteCarlo(montecarlo::Options),
	Live(live::Options),
}

/// start_time needs to be signed because of how the algo works
#[derive(Debug, Serialize, Clone)]
pub struct Bucket {
	pub bucket_no: i64,
	pub start_time: i64,
	pub end_time: i64,
	pub agg_buy: f64,
	pub agg_sell: f64,
	pub order_imbalance: f64,
}

/// Stores vec of Bucket
#[derive(Debug, Serialize)]
pub struct Buckets {
	pub buckets: Vec<Bucket>,
	pub previous_time: DateTime<Utc>,
	pub init: bool,
	pub current_buy: f64,
	pub current_sell: f64,
	pub threshold: f64,
	pub current_id: i64,
}

impl Buckets {
	pub fn new(threshold: f64) -> Self {
		Self {
			buckets: Vec::new(),
			previous_time: Utc::now(),
			init: false,
			current_buy: 0.,
			current_sell: 0.,
			threshold,
			current_id: 1,
		}
	}

	#[instrument]
	pub fn process_trade(&mut self, trade: MarketEvent<PublicTrade>) {
		if !self.init {
			self.init = true;
			self.previous_time = trade.exchange_time;
		}

		match trade.kind.side {
			Side::Buy => self.current_buy += trade.kind.amount,
			Side::Sell => self.current_sell += trade.kind.amount,
		}

		while self.current_buy + self.current_sell > self.threshold {
			let agg_sell;
			let agg_buy;
			match trade.kind.side {
				Side::Buy => {
					agg_sell = self.current_sell;
					agg_buy = self.threshold - agg_sell;
				}
				Side::Sell => {
					agg_buy = self.current_buy;
					agg_sell = self.threshold - agg_buy;
				}
			}
			let new_bucket = Bucket {
				bucket_no: self.current_id,
				start_time: self.previous_time.timestamp_millis(),
				end_time: trade.exchange_time.timestamp_millis(),
				agg_buy,
				agg_sell,
				order_imbalance: (agg_buy - agg_sell).abs(),
			};
			self.previous_time = trade.exchange_time;
			self.current_buy -= agg_buy;
			self.current_sell -= agg_sell;
			self.current_id += 1;
			self.buckets.push(new_bucket.clone());
			println!("{new_bucket:?}");
			//info!("{new_bucket:?}");
		}
	}
}
