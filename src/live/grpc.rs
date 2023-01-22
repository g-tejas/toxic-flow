use std::pin::Pin;

use futures::Stream;
use tonic::{Request, Response, Status};
use tracing::*;

pub mod proto {
	tonic::include_proto!("toxicflow");
}

#[derive(Debug, Default)]
pub struct ToxicFlowService;

#[tonic::async_trait]
impl proto::toxic_flow_server::ToxicFlow for ToxicFlowService {
	type OrderFlowImbalanceStream =
	Pin<Box<dyn Stream<Item = Result<proto::BucketResponse, Status>> + Send + 'static>>;

	async fn order_flow_imbalance(
		&self,
		request: Request<proto::Empty>,
	) -> Result<Response<Self::OrderFlowImbalanceStream>, Status> {
		info!("Received a request: {:?}", request);
		let _req = request.into_inner();

		let output = async_stream::try_stream! {
			let test = "string".to_string();
			yield proto::BucketResponse{test};
		};
		Ok(Response::new(Box::pin(output) as Self::OrderFlowImbalanceStream))
	}
}
