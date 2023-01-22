/// https://docs.rs/csv/latest/csv/tutorial/
/// For optimization, refer to documentation above, but for now it runs pretty fast.
use clap::Parser;
use tflow::*;

fn main() {
    let cli = Cli::parse();

     match &cli.command {
        Commands::Historical(opts) => historical::parser::run(opts).expect(" ❌ Failed to process csv."),
        Commands::MonteCarlo(opts) => montecarlo::simulation::run(opts).expect("❌ Failed to run Monte Carlo simulation."),
     } 
}
