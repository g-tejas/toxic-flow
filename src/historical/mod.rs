use serde::Deserialize;
use chrono::{Duration, NaiveDate};
use clap::Args;
use std::mem;
pub mod parser;

#[derive(Debug, Deserialize)]
pub struct InputRow {
    pub id: String,
    pub price: f64,
    pub qty: f64,
    pub quote_qty: f64,
    pub time: i64,
    pub is_buyer_maker: bool,
}

pub struct DateRange(pub NaiveDate, pub NaiveDate);

impl Iterator for DateRange {
    type Item = NaiveDate;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 {
            let next = self.0 + Duration::days(1);
            Some(mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
}

#[derive(Args, Debug)]
pub struct Options {
    /// Sample of volume buckets used in the estimation
    #[arg(short, default_value_t = 50)]
    pub n: u32,
    /// Volume bucket size
    #[arg(short, default_value_t = 6894.)]
    pub volume_bucket_size: f64,
    /// Enter the start date in the format YYYY-MM-DD. For example, 2020-12-31.
    #[arg(short, default_value_t = String::from("2020-01-01"))]
    pub start_date: String,
    /// Enter the end date in the format YYYY-MM-DD. For example, 2020-12-31.
    #[arg(short, default_value_t = String::from("2022-12-20"))]
    pub end_date: String,
    /// Path to input directory with all the CSV files.
    #[arg(short)]
    pub input_dir: String,
    /// Path to output file
    #[arg(short, default_value_t = String::from("output.csv"))]
    pub output_file: String,
}

impl Options {
    /// Returns the a struct, DateRange, with a custom iterator
    pub fn get_date_range(&self) -> DateRange {
        let start_date: NaiveDate =
            NaiveDate::parse_from_str(self.start_date.as_str(), "%Y-%m-%d").unwrap();
        let end_date: NaiveDate =
            NaiveDate::parse_from_str(self.end_date.as_str(), "%Y-%m-%d").unwrap();
        DateRange(start_date, end_date)
    }
}
