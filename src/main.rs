/// https://docs.rs/csv/latest/csv/tutorial/
/// For optimization, refer to documentation above, but for now it runs pretty fast.
use clap::Parser;
use tracing::*;

use tflow::*;

fn main() {
	tracing_subscriber::fmt::init();

	let cli = Cli::parse();

	match &cli.command {
		Commands::Historical(opts) => historical::parser::run(opts)
			.expect(" ❌ Failed to process csv."),
		Commands::MonteCarlo(opts) => montecarlo::simulation::run(opts)
			.expect("❌ Failed to run Monte Carlo simulation."),
		Commands::Live(opts) => live::server::run(opts)
			.expect("❌ Failed to run Monte Carlo simulation."),
	}
}
