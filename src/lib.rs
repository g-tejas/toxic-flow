use barter_data::event::MarketEvent;
use barter_data::subscription::trade::PublicTrade;
use barter_integration::model::Side;
use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use serde::Serialize;
use tracing::*;
use std::collections::VecDeque;
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
#[derive(Debug, Serialize)]
pub struct MovingBucket {
	pub current_buy: f64,
	pub current_sell: f64,
	pub current_imbalance: f64,
	pub threshold: f64,
	pub dq: VecDeque<f64>,
}
impl MovingBucket {
	pub fn new(threshold: f64) -> Self {
		Self {
			current_buy: 0.,
			current_sell: 0.,
			current_imbalance: 0.,
			threshold,
			dq: VecDeque::new()
		}
	}

	#[instrument]
	pub fn process_trade(&mut self, trade: MarketEvent<PublicTrade>) {

		match trade.kind.side {
			Side::Buy => {
				self.dq.push_back(trade.kind.amount);
				self.current_buy += trade.kind.amount;
			}
			Side::Sell => {
				self.dq.push_back(-trade.kind.amount);
				self.current_sell += trade.kind.amount;
			}
		}
		loop {
			let excess = self.current_buy + self.current_sell - self.threshold;
			if excess <= 0. {
				break;
			}
			let front_most = self.dq.pop_front();
			match front_most {
				None => {break;}
				Some(curval) => {
					if curval < 0. {
						self.current_sell -= curval.abs();
					} else {
						self.current_buy -= curval.abs();
					}
					if curval.abs() < excess {
						continue;
					} 
					let to_add_back = curval.abs() - excess;
					if curval < 0. {
						self.dq.push_front(-to_add_back);
						self.current_sell += to_add_back;
					} else {
						self.dq.push_front(to_add_back);
						self.current_buy += to_add_back;
					}
				}
			}
		}

		self.current_imbalance = (self.current_buy - self.current_sell).abs();
		println!("The current imbalance is {:.6}", self.current_buy-self.current_sell);
	}
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

