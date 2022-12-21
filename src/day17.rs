#![allow(
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

use hashbrown::hash_map::Entry;
use hashbrown::{HashMap, HashSet};
use num::Integer;

enum Shift {
    Left,
    Right,
}

struct Chamber {
    chamber: HashSet<(i64, i64)>,
}

impl Chamber {
    fn new() -> Self {
        let mut chamber = HashSet::new();
        for x in 0..7 {
            chamber.insert((x, 0));
        }
        Self { chamber }
    }

    fn is_valid(&self, position: (i64, i64), rock: &[(i64, i64)]) -> bool {
        for (x, y) in rock {
            let p = (position.0 + x, position.1 + y);
            if self.chamber.contains(&p) || p.0 < 0 || p.0 >= 7 {
                return false;
            }
        }
        true
    }

    fn put(&mut self, position: (i64, i64), rock: &[(i64, i64)]) -> i64 {
        let mut height = position.1;
        for (x, y) in rock {
            let p = (position.0 + x, position.1 + y);
            self.chamber.insert(p);
            height = height.max(p.1);
        }
        height
    }
}

fn simulate_rocks(input: &str, n_rocks: usize) -> i64 {
    // Create rocks
    const ROCKS: [&[(i64, i64)]; 5] = [
        // -
        &[(0, 0), (1, 0), (2, 0), (3, 0)],
        // +
        &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        // ┘
        &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        // |
        &[(0, 0), (0, 1), (0, 2), (0, 3)],
        // ■
        &[(0, 0), (1, 0), (0, 1), (1, 1)],
    ];

    // Cycle instructions
    let mut instructions = input
        .chars()
        .map(|c| match c {
            '<' => Shift::Left,
            '>' => Shift::Right,
            _ => unreachable!("Invalid instruction"),
        })
        .enumerate()
        .cycle();

    // Create chamber
    let mut chamber = Chamber::new();

    let mut highest = 0;
    let mut i_ins = 0;
    let mut cache = HashMap::new();

    for i_rock in 0..n_rocks {
        // Check if we can skip the cycles
        match cache.entry((i_rock % ROCKS.len(), i_ins)) {
            Entry::Vacant(e) => {
                e.insert((i_rock, highest));
            },
            Entry::Occupied(e) => {
                let (old_i_rock, old_highest) = e.get();
                let (n_cycles, left_in_cycle) = (n_rocks - i_rock).div_mod_floor(&(i_rock - old_i_rock));
                if left_in_cycle == 0 {
                    // Skip cycles
                    return highest + n_cycles as i64 * (highest - old_highest);
                }
            },
        }

        // Get rock
        let rock = ROCKS[i_rock % ROCKS.len()];
        let mut rock_position = (2, highest + 4);

        // Fall
        loop {
            // Update instruction number
            let ins = instructions.next().unwrap();
            i_ins = ins.0;

            // Move Left / Right
            match ins.1 {
                Shift::Left => {
                    if chamber.is_valid((rock_position.0 - 1, rock_position.1), rock) {
                        rock_position.0 -= 1;
                    }
                },
                Shift::Right => {
                    if chamber.is_valid((rock_position.0 + 1, rock_position.1), rock) {
                        rock_position.0 += 1;
                    }
                },
            }

            // Move down
            if chamber.is_valid((rock_position.0, rock_position.1 - 1), rock) {
                rock_position.1 -= 1;
            }
            else {
                break;
            }
        }

        // Get max height
        let height = chamber.put(rock_position, rock);
        highest = highest.max(height);
    }

    highest
}

pub fn simulate_rocks_2022(input: &str) -> i64 {
    simulate_rocks(input, 2022)
}

pub fn simulate_rocks_1e12(input: &str) -> i64 {
    simulate_rocks(input, 1e12 as usize)
}

pub fn main() {
    let input = std::fs::read_to_string("input/day17.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", simulate_rocks_2022(&input));
    println!("PART 2 = {}", simulate_rocks_1e12(&input));
    println!("Execution time: {:?}", now.elapsed());
}
