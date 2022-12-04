#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

fn parse_range(range: &str) -> ((u64, u64), (u64, u64)) {
    let (p1, p2) = range.split_once(',').unwrap();
    let (p1, p2) = (p1.split_once('-').unwrap(), p2.split_once('-').unwrap());
    let (p1, p2) = (
        (p1.0.parse::<u64>().unwrap(), p1.1.parse::<u64>().unwrap()),
        (p2.0.parse::<u64>().unwrap(), p2.1.parse::<u64>().unwrap()),
    );
    (p1, p2)
}

pub fn get_item_type(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (p1, p2) = parse_range(line);
            u64::from((p1.0 <= p2.0 && p1.1 >= p2.1) || (p1.0 >= p2.0 && p1.1 <= p2.1))
        })
        .sum()
}

pub fn get_badge(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (p1, p2) = parse_range(line);
            u64::from(p1.0.max(p2.0) <= p1.1.min(p2.1))
        })
        .sum()
}

pub fn main() {
    let input = std::fs::read_to_string("input/day04.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", get_item_type(&input));
    println!("PART 2 = {}", get_badge(&input));
    println!("Execution time: {:?}", now.elapsed());
}
