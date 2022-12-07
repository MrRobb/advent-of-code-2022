# 🎄 Advent of Code 2022

[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/MrRobb/advent-of-code-2022/blob/master/LICENSE)

|       | Problem                                          | Solution                                                                         | Execution time        | Lines of code | Finished |
|-------|--------------------------------------------------|----------------------------------------------------------------------------------|-----------------------|---------------|----------|
| Day 1 | [Problem 1](https://adventofcode.com/2022/day/1) | [day01.rs](https://github.com/MrRobb/advent-of-code-2022/blob/main/src/day01.rs) | 27.138 μs + 30.321 μs | 25            | ✓        |
| Day 2 | [Problem 2](https://adventofcode.com/2022/day/2) | [day02.rs](https://github.com/MrRobb/advent-of-code-2022/blob/main/src/day02.rs) | 34.969 μs + 48.740 μs | 87            | ✓        |
| Day 3 | [Problem 3](https://adventofcode.com/2022/day/3) | [day03.rs](https://github.com/MrRobb/advent-of-code-2022/blob/main/src/day03.rs) | 19.755 μs + 20.559 μs | 38            | ✓        |
| Day 4 | [Problem 4](https://adventofcode.com/2022/day/4) | [day04.rs](https://github.com/MrRobb/advent-of-code-2022/blob/main/src/day04.rs) | 41.928 μs + 44.308 μs | 31            | ✓        |
| Day 5 | [Problem 5](https://adventofcode.com/2022/day/5) | [day05.rs](https://github.com/MrRobb/advent-of-code-2022/blob/main/src/day05.rs) | 80.709 μs + 76.145 μs | 86            | ✓        |
| Day 6 | [Problem 6](https://adventofcode.com/2022/day/6) | [day06.rs](https://github.com/MrRobb/advent-of-code-2022/blob/main/src/day06.rs) | 1.9594 μs + 7.1957 μs | 19            | ✓        |
| Day 7 | [Problem 7](https://adventofcode.com/2022/day/7) | [day07.rs](https://github.com/MrRobb/advent-of-code-2022/blob/main/src/day07.rs) | 243.91 μs + 243.73 μs | 64            | ✓        |

> The benchmarks are measured (non-scientifically) with [cargo-criterion](https://github.com/bheisler/cargo-criterion) on a AMD Ryzen 5 3600 Desktop. More in the [benchmarks](#benchmarks) section.
> The lines of code are measured using [ghloc](https://github.com/MrRobb/ghloc-rs), excluding comments and empty lines.

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

## Benchmarks

### Install Criterion

To run the benchmarks you need to install [cargo-criterion](https://github.com/bheisler/cargo-criterion) first:

```sh
cargo install cargo-criterion
```

### Run benchmarks

Once you have Criterion installed, you can run the benchmarks with:

```sh
cargo criterion
```