# ☣️ Toxic Flow

Calculate everything and anything related to VPIN. Publishes a merged order book as a gRPC stream.

## Description

A pure rust CLI websocket client for creating
the [VPIN](https://www.stern.nyu.edu/sites/default/files/assets/documents/con_035928.pdf) measure for cryptocurrency
markets. VPIN (Volume-synchronised
probability of informed trading) is a way of measuring flow toxicity within a market, commonly used by liquidity
providers as leading indicator of liquidity-induced volatility.

* Live mode: Prints order_imbalance and VPIN bucket to terminal
* Historical: Parses binance tick (trade) data using the parameters defined.
* Monte-carlo: Runs bivariate monte-carlo simulation on probability of informed event & arrival rate of informed traders
  for optimal bucketing.

Original Paper: https://www.stern.nyu.edu/sites/default/files/assets/documents/con_035928.pdf

### Dependencies

* Rust and Cargo

### Executing program

* Download some data from [data.binance.vision](https://data.binance.vision/?prefix=data/futures/um/)
* Clone project and run `cargo install --path .`
* Run `bucketinator --help` to get flags

```
Usage: bucketinator [OPTIONS] --input-dir <INPUT_DIR>

Options:
  -n, --n <N>
          Sample of volume buckets used in the estimation [default: 50]
  -v, --volume-bucket-size <VOLUME_BUCKET_SIZE>
          Volume bucket size [default: 6894]
  -s, --start-date <START_DATE>
          Enter the start date in the format YYYY-MM-DD. For example, 2020-12-31 [default: 2020-01-01]
  -e, --end-date <END_DATE>
          Enter the end date in the format YYYY-MM-DD. For example, 2020-12-31 [default: 2022-12-20]
  -i, --input-dir <INPUT_DIR>
          Path to input directory with all the CSV files
  -o, --output-file <OUTPUT_FILE>
          Path to output file [default: output.csv]
  -h, --help
          Print help information
  -V, --version
          Print version information
```

## Help

Ask me

## Authors

- [Aloysius](https://github.com/pooty3)
- Me
- Wei Ming
