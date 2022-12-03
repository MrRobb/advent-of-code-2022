#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use itertools::Itertools;

/// Calculates the priority given a character.
/// - Lowercase item types a through z have priorities 1 through 26.
/// - Uppercase item types A through Z have priorities 27 through 52.
pub const fn char_to_priority(c: char) -> u64 {
    c as u64 - 'A' as u64 + 27 - 60 * c.is_ascii_lowercase() as u64
}

pub fn get_item_type(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            let common = a
                .chars()
                .find(|x| b.contains(*x))
                .expect("No common character found in line");
            char_to_priority(common)
        })
        .sum()
}

pub fn get_badge(input: &str) -> u64 {
    input
        .lines()
        .tuples()
        .map(|(a, b, c)| {
            let common = a
                .chars()
                .find(|x| b.contains(*x) && c.contains(*x))
                .expect("No common character");
            char_to_priority(common)
        })
        .sum()
}

pub fn main() {
    let input = std::fs::read_to_string("input/day03.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", get_item_type(&input));
    println!("PART 1 = {}", get_badge(&input));
    println!("Execution time: {:?}", now.elapsed());
}
