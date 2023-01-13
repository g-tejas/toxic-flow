/// https://docs.rs/csv/latest/csv/tutorial/
/// For optimization, refer to documentation above, but for now it runs pretty fast.
use clap::Parser;
use tflow::*;
use tflow::historical::parser::run;

fn main() {
    let cli = Cli::parse();

     match &cli.command {
        Commands::Historical(opts) => run(opts).expect("Failed to process csv."),
        Commands::MonteCarlo => println!("Hello world!"),
     } 
}
