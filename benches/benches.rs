use std::fs::read_to_string;

#[allow(clippy::wildcard_imports)]
use advent_of_code_2022::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench1(c: &mut Criterion) {
    // let input01 = read_to_string("input/day01.txt").expect("Input file not found");
    // c.bench_function("Day 1 | Part 1", |b| b.iter(|| day01::max_calories(&input01)));
    // c.bench_function("Day 1 | Part 2", |b| b.iter(|| day01::max_3_calories(&input01)));

    // let input02 = read_to_string("input/day02.txt").expect("Input file not found");
    // c.bench_function("Day 2 | Part 1", |b| b.iter(|| day02::calculate_wins(&input02)));
    // c.bench_function("Day 2 | Part 2", |b| b.iter(|| day02::calculate_wins_guess(&input02)));

    // let input03 = read_to_string("input/day03.txt").expect("Input file not found");
    // c.bench_function("Day 3 | Part 1", |b| b.iter(|| day03::get_item_type(&input03)));
    // c.bench_function("Day 3 | Part 2", |b| b.iter(|| day03::get_badge(&input03)));

    // let input04 = read_to_string("input/day04.txt").expect("Input file not found");
    // c.bench_function("Day 4 | Part 1", |b| b.iter(|| day04::get_fully_contained(&input04)));
    // c.bench_function("Day 4 | Part 2", |b| b.iter(|| day04::get_overlapping(&input04)));

    // let input05 = read_to_string("input/day05.txt").expect("Input file not found");
    // c.bench_function("Day 5 | Part 1", |b| b.iter(|| day05::crate_on_top_9000(&input05)));
    // c.bench_function("Day 5 | Part 2", |b| b.iter(|| day05::crate_on_top_9001(&input05)));

    // let input06 = read_to_string("input/day06.txt").expect("Input file not found");
    // c.bench_function("Day 6 | Part 1", |b| b.iter(|| day06::find_marker(&input06)));
    // c.bench_function("Day 6 | Part 2", |b| b.iter(|| day06::find_message(&input06)));

    // let input07 = read_to_string("input/day07.txt").expect("Input file not found");
    // c.bench_function("Day 7 | Part 1", |b| b.iter(|| day07::sum_of_directories(&input07)));
    // c.bench_function("Day 7 | Part 2", |b| b.iter(|| day07::smallest_directory(&input07)));

    // let input08 = read_to_string("input/day08.txt").expect("Input file not found");
    // c.bench_function("Day 8 | Part 1", |b| b.iter(|| day08::find_visible_trees(&input08)));
    // c.bench_function("Day 8 | Part 2", |b| b.iter(|| day08::highest_scenic_score(&input08)));

    // let input09 = read_to_string("input/day09.txt").expect("Input file not found");
    // c.bench_function("Day 9 | Part 1", |b| b.iter(|| day09::part1(&input09)));
    // c.bench_function("Day 9 | Part 2", |b| b.iter(|| day09::part2(&input09)));

    // let input10 = read_to_string("input/day10.txt").expect("Input file not found");
    // c.bench_function("Day 10 | Part 1", |b| b.iter(|| day10::get_signal_strength(&input10)));
    // c.bench_function("Day 10 | Part 2", |b| b.iter(|| day10::draw_pixels(&input10)));

    // let input11 = read_to_string("input/day11.txt").expect("Input file not found");
    // c.bench_function("Day 11 | Part 1", |b| b.iter(|| day11::keep_away(&input11)));
    // c.bench_function("Day 11 | Part 2", |b| b.iter(|| day11::keep_away_lcm(&input11)));

    // let input12 = read_to_string("input/day12.txt").expect("Input file not found");
    // c.bench_function("Day 12 | Part 1", |b| b.iter(|| day12::min_path_to_s(&input12)));
    // c.bench_function("Day 12 | Part 2", |b| b.iter(|| day12::min_path_to_a(&input12)));

    // let input13 = read_to_string("input/day13.txt").expect("Input file not found");
    // c.bench_function("Day 13 | Part 1", |b| b.iter(|| day13::how_many_sorted(&input13)));
    // c.bench_function("Day 13 | Part 2", |b| b.iter(|| day13::sort_all(&input13)));

    let input14 = read_to_string("input/day14.txt").expect("Input file not found");
    c.bench_function("Day 14 | Part 1", |b| b.iter(|| day14::reach_abbyss(&input14)));
    c.bench_function("Day 14 | Part 2", |b| b.iter(|| day14::reach_pyramid(&input14)));
}

criterion_group!(benches, bench1);
criterion_main!(benches);
