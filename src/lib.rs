use clap::{Parser, Subcommand};

pub mod historical;
pub mod montecarlo;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    Historical(historical::Options),
    MonteCarlo(montecarlo::Options),
}

