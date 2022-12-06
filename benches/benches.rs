use std::fs::read_to_string;

#[allow(clippy::wildcard_imports)]
use advent_of_code_2022::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench1(c: &mut Criterion) {
    let input01 = read_to_string("input/day01.txt").expect("Input file not found");
    c.bench_function("Day 1 | Part 1", |b| b.iter(|| day01::max_calories(&input01)));
    c.bench_function("Day 1 | Part 2", |b| b.iter(|| day01::max_3_calories(&input01)));

    let input02 = read_to_string("input/day02.txt").expect("Input file not found");
    c.bench_function("Day 2 | Part 1", |b| b.iter(|| day02::calculate_wins(&input02)));
    c.bench_function("Day 2 | Part 2", |b| b.iter(|| day02::calculate_wins_guess(&input02)));

    let input03 = read_to_string("input/day03.txt").expect("Input file not found");
    c.bench_function("Day 3 | Part 1", |b| b.iter(|| day03::get_item_type(&input03)));
    c.bench_function("Day 3 | Part 2", |b| b.iter(|| day03::get_badge(&input03)));

    let input04 = read_to_string("input/day04.txt").expect("Input file not found");
    c.bench_function("Day 4 | Part 1", |b| b.iter(|| day04::get_fully_contained(&input04)));
    c.bench_function("Day 4 | Part 2", |b| b.iter(|| day04::get_overlapping(&input04)));

    let input05 = read_to_string("input/day05.txt").expect("Input file not found");
    c.bench_function("Day 5 | Part 1", |b| b.iter(|| day05::crate_on_top_9000(&input05)));
    c.bench_function("Day 5 | Part 2", |b| b.iter(|| day05::crate_on_top_9001(&input05)));

    let input06 = read_to_string("input/day06.txt").expect("Input file not found");
    c.bench_function("Day 6 | Part 1", |b| b.iter(|| day06::find_marker(&input06)));
    c.bench_function("Day 6 | Part 2", |b| b.iter(|| day06::find_message(&input06)));
}

criterion_group!(benches, bench1);
criterion_main!(benches);
