use std::time::Instant;
use indicatif::{ProgressState, ProgressBar, ProgressStyle};
use csv::StringRecord;
use std::{error::Error, fmt::Write};
use super::Options;

pub fn run(opt: &Options) -> Result<(), Box<dyn Error>> {
    let now = Instant::now();
    
    let date_range = opt.get_date_range();
    let mut rows: Vec<super::OutputRow> = Vec::new();

    // These parameter's must persist across all csv files so
    // put them outside all for loops
    let mut curr_bucket: i64 = 1;
    let mut curr_vol: f64 = 0.;
    let mut curr_buy_vol: f64 = 0.;
    let mut curr_sell_vol: f64 = 0.;
    let mut start_time: i64 = -1;
    let mut last_time: i64 = 0;

    let pb = ProgressBar::new((date_range.1 - date_range.0).num_days() as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );

    for dt in date_range {
        let mut rdr = csv::Reader::from_path(format!("{}/BTCUSDT-trades-{}.csv", opt.input_dir, dt))?;

        // Some of the binance csv files have headers, some do not, we need to take care of those that don't
        // Since header rows do not appear in the iterator, we have to assert beforehand
        let headers = rdr.headers().unwrap();
        let real_headers: StringRecord = StringRecord::from(vec![
            "id",
            "price",
            "qty",
            "quote_qty",
            "time",
            "is_buyer_maker",
        ]);
        if headers == &real_headers {
        } else {
            rdr.set_headers(real_headers);
        }

        for row in rdr.deserialize() {
            let mut row: super::InputRow = row?;

            last_time = row.time;

            if start_time == -1 {
                start_time = row.time;
            }
            while row.qty + curr_vol > opt.volume_bucket_size {
                // how much u need to fill up basically
                let delta: f64 = opt.volume_bucket_size - curr_vol;

                if row.is_buyer_maker {
                    curr_buy_vol += delta;
                } else {
                    curr_sell_vol += delta;
                }

                curr_bucket += 1;
                let new_output: super::OutputRow = super::OutputRow {
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

        // Writing out
        let mut wtr = csv::Writer::from_path(opt.output_file.clone())?;
        for row in &rows {
            wtr.serialize(row)?;
        }
        wtr.flush()?;
        pb.inc(1);
    }

    // Write out one last time, for the remainder volume
    let mut wtr = csv::Writer::from_path(opt.output_file.clone())?;

    curr_bucket += 1;
    let left_over: super::OutputRow = super::OutputRow {
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
    pb.finish_with_message(format!("Processing csv took: {} seconds", now.elapsed().as_secs()));
    Ok(())
}
