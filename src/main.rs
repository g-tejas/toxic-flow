/// https://docs.rs/csv/latest/csv/tutorial/
/// For optimization, refer to documentation above, but for now it runs pretty fast.
use clap::Parser;
use tflow::*;
use tflow::historical::parser::get_buckets;
use std::time::Instant;


fn main() {
    let cli = Cli::parse();

    let now = Instant::now(); // For timing

     match &cli.command {
        Commands::Historical(opts) => get_buckets(opts).expect("Failed to process csv."),
        Commands::MonteCarlo => println!("Hello world!"),
     } 

    println!("Processing csv took: {} seconds", now.elapsed().as_secs());
}
