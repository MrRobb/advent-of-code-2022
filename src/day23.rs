#![allow(
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]

use std::{ops::Add, option::Option};

use enum_iterator::{all, cardinality, Sequence};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use rayon::prelude::*;

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

fn simulate(input: &str, max_rounds: Option<usize>) -> i32 {
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

    let mut round = 0;
    loop {
        // Generate proposals
        let proposals = elves
            .par_iter()
            .map(|elf| {
                let neighbors = neighbors(&elves, *elf);
                (*elf, propose(&neighbors, round).map(|d| *elf + d))
            })
            .collect::<HashMap<ElfPosition, Option<ElfPosition>>>();

        // Remove collisions
        let mut someone_moved = false;
        proposals
            .par_iter()
            .filter(|(_, p)| proposals.values().filter(|p2| p == p2).count() == 1)
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(elf, proposal)| {
                if let Some(proposal) = proposal {
                    elves.remove(elf);
                    elves.insert(*proposal);
                    someone_moved = true;
                }
            });

        // Update round
        round += 1;

        // Break if max rounds have been simulated (Part 1)
        if let Some(max_rounds) = max_rounds {
            if round >= max_rounds {
                break;
            }
        }
        // Break if noone moved (Part 2)
        else if !someone_moved {
            return round as i32;
        }
    }

    // Calculate bounds
    let (min_x, max_x) = elves.iter().map(|(x, _)| x).minmax().into_option().unwrap();
    let (min_y, max_y) = elves.iter().map(|(_, y)| y).minmax().into_option().unwrap();
    (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i32
}

pub fn part1(input: &str) -> i32 {
    simulate(input, Some(10))
}

pub fn part2(input: &str) -> i32 {
    simulate(input, None)
}

fn neighbors(map: &HashSet<ElfPosition>, elf: ElfPosition) -> [Option<ElfPosition>; cardinality::<Direction>()] {
    let mut neighbors = [None; cardinality::<Direction>()];
    for (i, d) in all::<Direction>().enumerate() {
        let neighbor = elf + d;
        if map.contains(&neighbor) {
            neighbors[i] = Some(neighbor);
        }
    }
    neighbors
}

fn propose(neighbors: &[Option<ElfPosition>; cardinality::<Direction>()], round: usize) -> Option<Direction> {
    [Direction::North, Direction::South, Direction::West, Direction::East]
        .into_iter()
        .cycle()
        .skip(round % 4)
        .take(4)
        .find(|d| {
            if neighbors.iter().all(Option::is_none) {
                false
            } else {
                match d {
                    Direction::North => neighbors[0].is_none() && neighbors[4].is_none() && neighbors[5].is_none(),
                    Direction::South => neighbors[1].is_none() && neighbors[6].is_none() && neighbors[7].is_none(),
                    Direction::East => neighbors[2].is_none() && neighbors[4].is_none() && neighbors[6].is_none(),
                    Direction::West => neighbors[3].is_none() && neighbors[5].is_none() && neighbors[7].is_none(),
                    _ => unreachable!(),
                }
            }
        })
}

pub fn main() {
    let input = std::fs::read_to_string("input/day23.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", part1(&input));
    println!("PART 2 = {}", part2(&input));
    println!("Execution time: {:?}", now.elapsed());
}
