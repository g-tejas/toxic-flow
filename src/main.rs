/// https://docs.rs/csv/latest/csv/tutorial/
/// For optimization, refer to documentation above, but for now it runs pretty fast.
use std::error::Error;
use std::time::Instant;

use clap::Parser;

use args::Cli;
use defines::{InputRow, OutputRow};

mod args;
mod defines;

fn main() {
    let args = Cli::parse();

    let now = Instant::now();

    process_csv(args.input_file, args.output_file, args.volume_bucket_size)
        .expect("Failed to process csv");

    println!("Processing csv took: {} seconds", now.elapsed().as_secs());
}

fn process_csv(
    input_file: String,
    output_file: String,
    bucket_size: f64, // All volume / quantity related stuff has to be in f64
) -> Result<(), Box<dyn Error>> {
    // we want to panic regardless
    let mut rdr = csv::Reader::from_path(input_file)?;
    let mut wtr = csv::Writer::from_path(output_file)?;
    let mut rows: Vec<OutputRow> = Vec::new();

    let mut curr_bucket: i64 = 1;
    let mut curr_vol: f64 = 0.;
    let mut curr_buy_vol: f64 = 0.;
    let mut curr_sell_vol: f64 = 0.;
    let mut start_time: i64 = -1;
    let mut last_time: i64 = 0;

    for row in rdr.deserialize() {
        let mut row: InputRow = row?;
        last_time = row.time;

        if start_time == -1 {
            start_time = row.time;
        }
        while row.qty + curr_vol > bucket_size {
            // how much u need to fill up basically
            let delta: f64 = bucket_size - curr_vol;

            if row.is_buyer_maker {
                curr_buy_vol += delta;
            } else {
                curr_sell_vol += delta;
            }

            curr_bucket += 1;
            let new_output: OutputRow = OutputRow {
                bucket_no: curr_bucket,
                agg_buy: curr_buy_vol,
                agg_sell: curr_sell_vol,
                start_time,
                end_time: row.time,
                order_imbalance: (curr_buy_vol - curr_sell_vol).abs(),
            };
            rows.push(new_output);
            start_time = row.time;
            row.qty -= delta;
            curr_vol = 0.;
            curr_buy_vol = 0.;
            curr_sell_vol = 0.;
        }
        curr_vol += row.qty;

        if row.is_buyer_maker {
            curr_buy_vol += row.qty;
        } else {
            curr_sell_vol += row.qty;
        }
    }
    curr_bucket += 1;
    let left_over: OutputRow = OutputRow {
        bucket_no: curr_bucket,
        agg_buy: curr_buy_vol,
        agg_sell: curr_sell_vol,
        start_time,
        end_time: last_time,
        order_imbalance: (curr_buy_vol - curr_sell_vol).abs(),
    };
    rows.push(left_over);

    for row in rows {
        //println!("{:?}", row);
        wtr.serialize(row)?;
    }
    wtr.flush()?;
    Ok(())
}
