#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

fn parse_range(range: &str) -> ((u64, u64), (u64, u64)) {
    let (p1, p2) = range.split_once(',').unwrap();
    let p1 = p1.split_once('-').unwrap();
    let p2 = p2.split_once('-').unwrap();
    (
        (p1.0.parse().unwrap(), p1.1.parse().unwrap()),
        (p2.0.parse().unwrap(), p2.1.parse().unwrap()),
    )
}

pub fn get_fully_contained(input: &str) -> usize {
    input
        .lines()
        .map(parse_range)
        .filter(|((a, b), (c, d))| (a <= c && b >= d) || (a >= c && b <= d))
        .count()
}

pub fn get_overlapping(input: &str) -> usize {
    input
        .lines()
        .map(parse_range)
        .filter(|((a, b), (c, d))| a.max(c) <= b.min(d))
        .count()
}

pub fn main() {
    let input = std::fs::read_to_string("input/day04.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", get_fully_contained(&input));
    println!("PART 2 = {}", get_overlapping(&input));
    println!("Execution time: {:?}", now.elapsed());
}
