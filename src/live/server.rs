use barter_data::{
	exchange::binance::futures::BinanceFuturesUsd,
	streams::Streams,
	subscription::trade::PublicTrades,
};
use barter_integration::model::InstrumentKind;
use futures::StreamExt;
use tracing::*;

use crate::MovingBucket;

use super::Options;

#[instrument]
#[tokio::main]
pub async fn run(opts: &Options) -> Result<(), Box<dyn std::error::Error>> {
	let mut vpin = MovingBucket::new(10.);

	let streams = Streams::<PublicTrades>::builder()
		.subscribe([(BinanceFuturesUsd::default(), "btc", "usdt", InstrumentKind::FuturePerpetual, PublicTrades)])
		.init()
		.await
		.unwrap();

	let mut joined_stream = streams.join_map().await;
	while let Some((_exchange, trade)) = joined_stream.next().await {
		vpin.process_trade(trade);
	}

	Ok(())
}
