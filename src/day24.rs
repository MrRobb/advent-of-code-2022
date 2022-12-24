#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use hashbrown::HashSet;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Map {
    walls: Vec<(usize, usize)>,
    blizzards: Vec<(Direction, (usize, usize))>,
    start: (usize, usize),
    goal: (usize, usize),
    height: usize,
    width: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut walls = Vec::new();
        let mut blizzards = Vec::new();
        let mut start = (0, 0);
        let mut goal = (0, 0);
        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                match c {
                    '#' => walls.push((i, j)),
                    '<' => blizzards.push((Direction::Left, (i, j))),
                    '>' => blizzards.push((Direction::Right, (i, j))),
                    '^' => blizzards.push((Direction::Up, (i, j))),
                    'v' => blizzards.push((Direction::Down, (i, j))),
                    '.' if i == 0 => start = (i, j),
                    '.' if i == input.lines().count() - 1 => goal = (i, j),
                    _ => {},
                }
            }
        }
        Self {
            walls,
            blizzards,
            start,
            goal,
            height: input.lines().count(),
            width: input.lines().next().unwrap().chars().count(),
        }
    }

    fn update(&mut self) {
        for (direction, position) in &mut self.blizzards {
            match direction {
                Direction::Left => {
                    if position.1 == 1 {
                        *position = (position.0, self.width - 2);
                    }
                    else {
                        *position = (position.0, position.1 - 1);
                    }
                },
                Direction::Right => {
                    if position.1 == self.width - 2 {
                        *position = (position.0, 1);
                    }
                    else {
                        *position = (position.0, position.1 + 1);
                    }
                },
                Direction::Up => {
                    if position.0 == 1 {
                        *position = (self.height - 2, position.1);
                    }
                    else {
                        *position = (position.0 - 1, position.1);
                    }
                },
                Direction::Down => {
                    if position.0 == self.height - 2 {
                        *position = (1, position.1);
                    }
                    else {
                        *position = (position.0 + 1, position.1);
                    }
                },
            }
        }
    }

    fn neighbors(&self, position: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let blizzard_positions = self.blizzards.iter().map(|(_, position)| position).collect::<Vec<_>>();

        // Wait
        if !blizzard_positions.contains(&position) {
            neighbors.push(*position);
        }

        // Left
        if position.1 > 0
            && !self.walls.contains(&(position.0, position.1 - 1))
            && !blizzard_positions.contains(&&(position.0, position.1 - 1))
        {
            neighbors.push((position.0, position.1 - 1));
        }

        // Right
        if position.1 < self.width - 1
            && !self.walls.contains(&(position.0, position.1 + 1))
            && !blizzard_positions.contains(&&(position.0, position.1 + 1))
        {
            neighbors.push((position.0, position.1 + 1));
        }

        // Up
        if position.0 > 0
            && !self.walls.contains(&(position.0 - 1, position.1))
            && !blizzard_positions.contains(&&(position.0 - 1, position.1))
        {
            neighbors.push((position.0 - 1, position.1));
        }

        // Down
        if position.0 < self.height - 1
            && !self.walls.contains(&(position.0 + 1, position.1))
            && !blizzard_positions.contains(&&(position.0 + 1, position.1))
        {
            neighbors.push((position.0 + 1, position.1));
        }

        neighbors
    }

    fn _print(&self) {
        for i in 0..self.height {
            for j in 0..self.width {
                if self.walls.contains(&(i, j)) {
                    print!("#");
                }
                else if self.blizzards.iter().any(|(_, position)| position == &(i, j)) {
                    let direction = self
                        .blizzards
                        .iter()
                        .find(|(_, position)| position == &(i, j))
                        .unwrap()
                        .0;
                    match direction {
                        Direction::Left => print!("<"),
                        Direction::Right => print!(">"),
                        Direction::Up => print!("^"),
                        Direction::Down => print!("v"),
                    }
                }
                else if self.start == (i, j) {
                    print!("S");
                }
                else if self.goal == (i, j) {
                    print!("G");
                }
                else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }

    fn minutes_to(&mut self, from: (usize, usize), to: (usize, usize)) -> usize {
        let mut possible_positions = HashSet::new();
        possible_positions.insert(from);
        let mut minutes = 0;

        while !possible_positions.contains(&to) {
            // Move blizzards
            self.update();

            // Expand possible positions
            possible_positions = possible_positions
                .par_iter()
                .flat_map(|position| self.neighbors(position))
                .collect();
            minutes += 1;
        }

        minutes
    }
}

pub fn reach_goal(input: &str) -> usize {
    let mut map = Map::new(input);
    map.minutes_to(map.start, map.goal)
}

pub fn reach_gsg(input: &str) -> usize {
    let mut map = Map::new(input);
    map.minutes_to(map.start, map.goal) + map.minutes_to(map.goal, map.start) + map.minutes_to(map.start, map.goal)
}

pub fn main() {
    let input = std::fs::read_to_string("input/day24.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", reach_goal(&input));
    println!("PART 2 = {}", reach_gsg(&input));
    println!("Execution time: {:?}", now.elapsed());
}
