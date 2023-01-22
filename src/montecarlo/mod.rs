use clap::Args;
pub mod simulation;

#[derive(Args, Debug)]
pub struct Options {
    /// Nnumber of monte carlo simulations
    #[arg(short, default_value_t = 10000)]
    pub epochs: u64,
    /// arrival rate (not actually volume bucket size)
    #[arg(short, default_value_t = 500.0)]
    pub vbs: f64,
    /// Nnumber of buckets per vpin but not sure why it's needed here
    #[arg(short, default_value_t = 50)]
    pub n: i64,
    /// probability of information event occuring
    max_alpha: i64,
    /// arrival rate of informed traders
    max_mu: i64,
    /// [not important] information event is good/bad
    #[arg(short, long, default_value_t = 0.5)]
    delta: f64
}
