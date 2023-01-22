use clap::Args;

pub mod server;
pub mod grpc;

#[derive(Args, Debug)]
pub struct Options {
    text: String
}
