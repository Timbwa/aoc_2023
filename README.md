# AOC 2023 🎄
Solutions to [Advent of Code 2023](https://adventofcode.com/) in Rust

## Pre-requisites ⛏️
* In order to run make sure you've installed Rust. Checkout [rustup](https://www.rust-lang.org/tools/install).
* You should create your own input folder within the repository. (Ignored by default) and name the files for respective files like so: ``day_{day_number}.txt`` & ``day_{day_number}_test.txt`` where ``day_number`` ranges from ``01-25`` and `test` is for test input.

## Run 🚀

```
Rust Program for running AOC solutions

Usage: aoc_2023.exe [OPTIONS]

Options:
  -r, --run-latest  Run the latest day solutions or run all solutions (Default = All Solutions)
  -t, --test        Run using the test input
  -h, --help        Print help
  -V, --version     Print version
 ```

### Example to run latest solution on test input
```shell
cargo run -- -r -t
```
