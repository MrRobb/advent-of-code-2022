# Advent of Code 2022

[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/MrRobb/advent-of-code-2022/blob/master/LICENSE)

|        | Problem                                            | Solution                                                                           | Execution time        | Finished |
|--------|----------------------------------------------------|------------------------------------------------------------------------------------|-----------------------|----------|
| Day 1  | [Problem 1](https://adventofcode.com/2022/day/1)   | [day01.rs](https://github.com/MrRobb/advent-of-code-2022/blob/main/src/day01.rs)   | 27.138 μs + 30.321 μs | ✓        |
| Day 2  | [Problem 2](https://adventofcode.com/2022/day/2)   | [day02.rs](https://github.com/MrRobb/advent-of-code-2022/blob/main/src/day02.rs)   | 34.969 μs + 48.740 μs | ✓        |


> The benchmarks are measured (non-scientifically) with [cargo-criterion](https://github.com/bheisler/cargo-criterion) on a AMD Ryzen 5 3600 Desktop.

## Install Rust

If you don't have Rust installed ([how dare you](https://media.giphy.com/media/U1aN4HTfJ2SmgB2BBK/giphy.gif)) just run this:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

> If you are not using a Unix-like OS, check the instructions [here](https://www.rust-lang.org/tools/install)
## Usage

### Clone

```sh
git clone https://github.com/MrRobb/advent-of-code-2022.git
cd advent-of-code-2022
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
cargo run --release --bin day01
```
