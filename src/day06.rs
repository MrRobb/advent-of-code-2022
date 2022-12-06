#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

fn all_unique(w: &[u8]) -> bool {
    !w.iter().enumerate().any(|(i, wc)| w[..i].contains(wc))
}

pub fn find_marker(input: &str) -> usize {
    const MARKER_SIZE: usize = 4;
    input.as_bytes().windows(MARKER_SIZE).position(all_unique).unwrap() + MARKER_SIZE
}

pub fn find_message(input: &str) -> usize {
    const MESSAGE_SIZE: usize = 14;
    input.as_bytes().windows(MESSAGE_SIZE).position(all_unique).unwrap() + MESSAGE_SIZE
}

pub fn main() {
    let input = std::fs::read_to_string("input/day06.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", find_marker(&input));
    println!("PART 2 = {}", find_message(&input));
    println!("Execution time: {:?}", now.elapsed());
}
