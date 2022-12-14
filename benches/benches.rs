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

    // let input14 = read_to_string("input/day14.txt").expect("Input file not found");
    // c.bench_function("Day 14 | Part 1", |b| b.iter(|| day14::reach_abbyss(&input14)));
    // c.bench_function("Day 14 | Part 2", |b| b.iter(|| day14::reach_pyramid(&input14)));

    // let input15 = read_to_string("input/day15.txt").expect("Input file not found");
    // c.bench_function("Day 15 | Part 1", |b| b.iter(|| day15::no_beacon_at_2m(&input15)));
    // c.bench_function("Day 15 | Part 2", |b| b.iter(|| day15::find_distress_beacon(&input15)));

    // let input16 = read_to_string("input/day16.txt").expect("Input file not found");
    // c.bench_function("Day 16 | Part 1", |b| b.iter(|| day16::explore_tunnels(&input16)));
    // c.bench_function("Day 16 | Part 2", |b| b.iter(|| day16::explore_with_elephant(&input16)));

    // let input17 = read_to_string("input/day17.txt").expect("Input file not found");
    // c.bench_function("Day 17 | Part 1", |b| b.iter(|| day17::simulate_rocks_2022(&input17)));
    // c.bench_function("Day 17 | Part 2", |b| b.iter(|| day17::simulate_rocks_1e12(&input17)));

    // let input18 = read_to_string("input/day18.txt").expect("Input file not found");
    // c.bench_function("Day 18 | Part 1", |b| b.iter(|| day18::surface_area(&input18)));
    // c.bench_function("Day 18 | Part 2", |b| b.iter(|| day18::exterior_surface_area(&input18)));

    // let input19 = read_to_string("input/day19.txt").expect("Input file not found");
    // c.bench_function("Day 19 | Part 1", |b| b.iter(|| day19::quality_levels(&input19)));
    // c.bench_function("Day 19 | Part 2", |b| b.iter(|| day19::open_geodes(&input19)));

    // let input20 = read_to_string("input/day20.txt").expect("Input file not found");
    // c.bench_function("Day 20 | Part 1", |b| b.iter(|| day20::mix(&input20)));
    // c.bench_function("Day 20 | Part 2", |b| b.iter(|| day20::mix_with_key(&input20)));

    // let input21 = read_to_string("input/day21.txt").expect("Input file not found");
    // c.bench_function("Day 21 | Part 1", |b| b.iter(|| day21::compute_monkeys(&input21)));
    // c.bench_function("Day 21 | Part 2", |b| b.iter(|| day21::build_equation(&input21)));

    // let input23 = read_to_string("input/day23.txt").expect("Input file not found");
    // c.bench_function("Day 23 | Part 1", |b| b.iter(|| day23::move_elves_10(&input23)));
    // c.bench_function("Day 23 | Part 2", |b| b.iter(|| day23::converge_elves(&input23)));

    let input24 = read_to_string("input/day24.txt").expect("Input file not found");
    c.bench_function("Day 24 | Part 1", |b| b.iter(|| day24::reach_goal(&input24)));
    c.bench_function("Day 24 | Part 2", |b| b.iter(|| day24::reach_gsg(&input24)));
}

criterion_group!(benches, bench1);
criterion_main!(benches);
