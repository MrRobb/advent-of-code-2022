#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use std::collections::VecDeque;

use itertools::Itertools;

type Crate = char;
type Tower = VecDeque<Crate>;

struct Cargo {
    towers: Vec<Tower>,
}

impl Cargo {
    fn new(input: &str) -> Self {
        let mut towers = Vec::new();
        for line in input.lines() {
            for (i, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
                let dl = chunk.next().unwrap();
                let c = chunk.next().unwrap();
                let dr = chunk.next().unwrap();
                match [dl, dr] {
                    ['[', ']'] => {
                        if i >= towers.len() {
                            towers.resize(i + 1, Tower::new());
                        }
                        towers[i].push_front(c);
                    },
                    [' ', ' '] => continue,
                    _ => unreachable!(),
                }
            }
        }
        Self { towers }
    }

    fn move_crate_9000(&mut self, movement: &Movement) {
        let from = &mut self.towers[movement.from - 1];
        let crates = from.split_off(from.len() - movement.count);
        self.towers[movement.to - 1].extend(crates.iter().rev());
    }

    fn move_crate_9001(&mut self, movement: &Movement) {
        let from = &mut self.towers[movement.from - 1];
        let crates = from.split_off(from.len() - movement.count);
        self.towers[movement.to - 1].extend(crates);
    }

    fn top_crates(&self) -> String {
        self.towers.iter().map(|t| t.iter().last().unwrap()).collect()
    }
}

#[derive(Debug)]
struct Movement {
    from: usize,
    to: usize,
    count: usize,
}

impl Movement {
    fn new(input: &str) -> Self {
        let (count, rest) = input.split_once("move ").unwrap().1.split_once(" from ").unwrap();
        let (from, to) = rest.split_once(" to ").unwrap();
        Self {
            from: from.parse().unwrap(),
            to: to.parse().unwrap(),
            count: count.parse().unwrap(),
        }
    }
}

pub fn crate_on_top_9000(input: &str) -> String {
    let (cargo_input, rearrangement) = input.split_once("\n\n").unwrap();
    let mut cargo = Cargo::new(cargo_input);
    for line in rearrangement.lines() {
        let movement = Movement::new(line);
        cargo.move_crate_9000(&movement);
    }
    cargo.top_crates()
}

pub fn crate_on_top_9001(input: &str) -> String {
    let (cargo_input, rearrangement) = input.split_once("\n\n").unwrap();
    let mut cargo = Cargo::new(cargo_input);
    for line in rearrangement.lines() {
        let movement = Movement::new(line);
        cargo.move_crate_9001(&movement);
    }
    cargo.top_crates()
}

pub fn main() {
    let input = std::fs::read_to_string("input/day05.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", crate_on_top_9000(&input));
    println!("PART 2 = {}", crate_on_top_9001(&input));
    println!("Execution time: {:?}", now.elapsed());
}
