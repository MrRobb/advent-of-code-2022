use std::fs::read_to_string;

#[allow(clippy::wildcard_imports)]
use advent_of_code_2022::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench1(c: &mut Criterion) {
    let input01 = read_to_string("input/day01.txt").expect("Input file not found");
    let input02 = read_to_string("input/day02.txt").expect("Input file not found");

    c.bench_function("Day 1 | Part 1", |b| b.iter(|| day01::max_calories(&input01)));
    c.bench_function("Day 1 | Part 2", |b| b.iter(|| day01::max_3_calories(&input01)));

    c.bench_function("Day 2 | Part 1", |b| b.iter(|| day02::calculate_wins(&input02)));
    c.bench_function("Day 2 | Part 2", |b| b.iter(|| day02::calculate_wins_guess(&input02)));
}

criterion_group!(benches, bench1);
criterion_main!(benches);
