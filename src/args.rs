use chrono::NaiveDate;
use clap::Parser;

use crate::defines::DateRange;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    /// Sample of volume buckets used in the estimation
    #[arg(short, long, default_value_t = 50)]
    pub n: u32,
    /// Volume bucket size
    #[arg(short, long, default_value_t = 6894.)]
    pub volume_bucket_size: f64,
    /// Enter the start date in the format YYYY-MM-DD. For example, 2020-12-31.
    #[arg(short, long, default_value_t = String::from("2020-01-01"))]
    pub start_date: String,
    /// Enter the end date in the format YYYY-MM-DD. For example, 2020-12-31.
    #[arg(short, long, default_value_t = String::from("2022-12-20"))]
    pub end_date: String,
    /// Path to input directory with all the CSV files.
    #[arg(short, long)]
    pub input_dir: String,
    /// Path to output file
    #[arg(short, long, default_value_t = String::from("output.csv"))]
    pub output_file: String,
}

impl Cli {
    /// Returns the a struct, DateRange, with a custom iterator
    pub fn get_date_range(&self) -> DateRange {
        let start_date: NaiveDate =
            NaiveDate::parse_from_str(self.start_date.as_str(), "%Y-%m-%d").unwrap();
        let end_date: NaiveDate =
            NaiveDate::parse_from_str(self.end_date.as_str(), "%Y-%m-%d").unwrap();
        DateRange(start_date, end_date)
    }
}
