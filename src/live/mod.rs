use clap::Args;

pub mod server;

#[derive(Args, Debug)]
pub struct Options {
    text: String
}
