# Advent of Code 2021

[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/MrRobb/advent-of-code-2022/blob/master/LICENSE)

|        | Problem                                            | Solution                                                                         | Execution time        | Finished |
|--------|----------------------------------------------------|----------------------------------------------------------------------------------|-----------------------|----------|
| Day 1  | [Problem 1](https://adventofcode.com/2021/day/1)   | [day1.rs](https://github.com/MrRobb/advent-of-code-2021/blob/main/src/day1.rs)   | 26.920 μs + 26.927 μs | ✓        |


> The benchmarks are measured (non-scientifically) with [cargo-criterion](https://github.com/bheisler/cargo-criterion) on a Macbook Pro 13' M1.

### Benchmark all

```sh
Benchmark 1: ./target/release/advent-of-code-2021
  Time (mean ± σ):      12.6 ms ±   0.4 ms    [User: 45.9 ms, System: 3.0 ms]
  Range (min … max):    12.1 ms …  14.5 ms    200 runs
```

## Install Rust

If you don't have Rust installed ([how dare you](https://media.giphy.com/media/U1aN4HTfJ2SmgB2BBK/giphy.gif)) just run this:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

> If you are not using a Unix-like OS, check the instructions [here](https://www.rust-lang.org/tools/install)
## Usage

### Clone

```sh
git clone https://github.com/MrRobb/advent-of-code-2021.git
cd advent-of-code-2021
```

### Build

```sh
cargo build --release
```

### Run

#### Run all

```sh
cargo run --release
```

#### Run a specific day

```sh
cargo run --release --bin day1
```
