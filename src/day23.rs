#![allow(
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]

use std::{ops::Add, option::Option};

use enum_iterator::{all, cardinality, Sequence};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

type ElfPosition = (i32, i32);

#[derive(Sequence, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Add<Direction> for ElfPosition {
    type Output = Self;

    fn add(self, direction: Direction) -> Self::Output {
        match direction {
            Direction::North => (self.0 - 1, self.1),
            Direction::South => (self.0 + 1, self.1),
            Direction::East => (self.0, self.1 + 1),
            Direction::West => (self.0, self.1 - 1),
            Direction::NorthEast => (self.0 - 1, self.1 + 1),
            Direction::NorthWest => (self.0 - 1, self.1 - 1),
            Direction::SouthEast => (self.0 + 1, self.1 + 1),
            Direction::SouthWest => (self.0 + 1, self.1 - 1),
        }
    }
}

fn neighbors(map: &HashSet<ElfPosition>, elf: ElfPosition) -> [bool; cardinality::<Direction>()] {
    all::<Direction>()
        .map(|d| map.contains(&(elf + d)))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn propose(neighbors: [bool; cardinality::<Direction>()], round: usize) -> Option<Direction> {
    if neighbors.iter().all(|n| !n) {
        return None;
    }

    [Direction::North, Direction::South, Direction::West, Direction::East]
        .into_iter()
        .cycle()
        .skip(round % 4)
        .take(4)
        .find(|d| match d {
            Direction::North => !neighbors[0] && !neighbors[4] && !neighbors[5],
            Direction::South => !neighbors[1] && !neighbors[6] && !neighbors[7],
            Direction::East => !neighbors[2] && !neighbors[4] && !neighbors[6],
            Direction::West => !neighbors[3] && !neighbors[5] && !neighbors[7],
            _ => unreachable!(),
        })
}

fn simulate(input: &str, max_rounds: Option<usize>) -> usize {
    // Build map
    let mut elves = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().filter_map(move |(j, c)| match c {
                '#' => Some((i as i32, j as i32)),
                _ => None,
            })
        })
        .collect::<HashSet<ElfPosition>>();

    for round in 0.. {
        // Generate proposals
        let mut all_proposals: HashMap<ElfPosition, Vec<ElfPosition>> = HashMap::new();
        for elf in elves.iter() {
            let neighbors = neighbors(&elves, *elf);
            if let Some(proposal) = propose(neighbors, round).map(|d| *elf + d) {
                all_proposals.entry(proposal).or_default().push(*elf);
            }
        }

        // Move elves
        let mut someone_moved = false;
        all_proposals
            .iter()
            // Filter collisions
            .filter(|(_, candidates)| candidates.len() == 1)
            // Move elf
            .for_each(|(proposal, candidates)| {
                elves.remove(&candidates[0]);
                elves.insert(*proposal);
                someone_moved = true;
            });

        // Break if max rounds have been simulated (Part 1)
        if let Some(max_rounds) = max_rounds {
            if round >= max_rounds {
                break;
            }
        }
        // Break if noone moved (Part 2)
        else if !someone_moved {
            return round + 1;
        }
    }

    // Calculate bounds
    let (min_x, max_x) = elves.iter().map(|(x, _)| x).minmax().into_option().unwrap();
    let (min_y, max_y) = elves.iter().map(|(_, y)| y).minmax().into_option().unwrap();
    (max_x - min_x + 1) as usize * (max_y - min_y + 1) as usize - elves.len()
}

pub fn part1(input: &str) -> usize {
    simulate(input, Some(10))
}

pub fn part2(input: &str) -> usize {
    simulate(input, None)
}

pub fn main() {
    let input = std::fs::read_to_string("input/day23.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", part1(&input));
    println!("PART 2 = {}", part2(&input));
    println!("Execution time: {:?}", now.elapsed());
}
