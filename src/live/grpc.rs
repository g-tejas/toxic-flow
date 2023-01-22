use tonic::{transport::Server, Request, Response, Status};
use toxicflow::{ToxicFlow, BucketResponse};
use toxicflow::toxicflow_server::{ToxicFlow, ToxicFlowServer};

pub mod toxicflow {
    tonic::include_proto!("ToxicFlow");
}

#[derive(Debug, Default)]
pub struct ToxicFlowService;

#[tonic::async_trait]
impl ToxicFlow for ToxicFlowService {
    async fn OrderflowImbalance(&self, request: Request<proto::Empty>) -> Result<Response<Self::BucketResponse>, Status> {
        info!("Got a request: {:?}", request);

        let req = request.into_inner();

        let reply = BucketResponse {
            test: "Helo".into()
        };

        Ok(Response::new(reply))
    }
}
