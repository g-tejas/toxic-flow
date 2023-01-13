# ☣️ Toxic Flow

Calculate everything and anything related to VPIN.

## Description

An in-depth paragraph about your project and overview of use.

## Getting Started

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

## Version History

* 0.2
    * Various bug fixes and optimizations
    * See [commit change]() or See [release history]()
* 0.1
    * Initial Release

## License

This project is licensed under the [NAME HERE] License - see the LICENSE.md file for details

## Acknowledgments

Inspiration, code snippets, etc.

* [awesome-readme](https://github.com/matiassingers/awesome-readme)
* [PurpleBooth](https://gist.github.com/PurpleBooth/109311bb0361f32d87a2)
* [dbader](https://github.com/dbader/readme-template)
* [zenorocha](https://gist.github.com/zenorocha/4526327)
* [fvcproductions](https://gist.github.com/fvcproductions/1bfc2d4aecb01a834b46)
