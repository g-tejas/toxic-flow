use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    /// Sample of volume buckets used in the estimation
    #[arg(short)]
    pub n: u32,
    /// Volume bucket size
    #[arg(short)]
    pub volume_bucket_size: f64,
    /// Path to input file
    #[arg(short)]
    pub input_file: String,
    /// Path to output file
    #[arg(short)]
    pub output_file: String,
}
