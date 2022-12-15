#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use itertools::Itertools;

#[derive(Clone, PartialEq, Eq)]
enum Cell {
    Sand,
    Rock,
    Air,
}

type Map = Vec<Vec<Cell>>;

fn build_map(lines: Vec<Vec<(usize, usize)>>) -> ((usize, usize), Map) {
    const SAND_POURING: (usize, usize) = (500, 0);

    // Calculate boundaries
    let (&min_y, max_y) = lines
        .iter()
        .flatten()
        .chain(std::iter::once(&SAND_POURING))
        .map(|(_, y)| y)
        .minmax()
        .into_option()
        .map(|(min, max)| (min, max + 2))
        .unwrap();
    let height = max_y - min_y;
    let (min_x, max_x) = (SAND_POURING.0 - (height - 1), SAND_POURING.0 + (height - 1));

    // Build map
    let mut map = vec![vec![Cell::Air; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];

    // Add ground
    for x in min_x..=max_x {
        map[(max_y - min_y) as usize][(x - min_x) as usize] = Cell::Rock;
    }

    // Add rocks
    for component in lines {
        for ((prev_x, prev_y), (end_x, end_y)) in component.into_iter().tuple_windows() {
            if prev_x == end_x {
                for y in prev_y.min(end_y)..=end_y.max(prev_y) {
                    map[(y - min_y) as usize][(prev_x - min_x) as usize] = Cell::Rock;
                }
            }
            else {
                for x in prev_x.min(end_x)..=end_x.max(prev_x) {
                    map[(prev_y - min_y) as usize][(x - min_x) as usize] = Cell::Rock;
                }
            }
        }
    }

    ((SAND_POURING.1 - min_y, SAND_POURING.0 - min_x), map)
}

#[allow(dead_code)]
fn print_map(map: &Map) {
    for line in map.iter() {
        for cell in line {
            match cell {
                Cell::Sand => print!("o"),
                Cell::Rock => print!("#"),
                Cell::Air => print!("."),
            }
        }
        println!();
    }
}

fn update_map(map: &mut Map, sand_pouring: (usize, usize)) {
    let mut grain = (sand_pouring.0, sand_pouring.1);
    loop {
        let i = grain.0;
        let j = grain.1;

        // try to go down
        if i + 1 < map.len() && map[i + 1][j] == Cell::Air {
            grain = (i + 1, j);
        }
        // if there's no down, try to go left
        else if j > 0 && i + 1 < map.len() && map[i + 1][j - 1] == Cell::Air {
            grain = (i + 1, j - 1);
        }
        // if there's no down and no left, try to go right
        else if j + 1 < map[i].len() && i + 1 < map.len() && map[i + 1][j + 1] == Cell::Air {
            grain = (i + 1, j + 1);
        }
        else {
            break;
        }
    }
    map[grain.0][grain.1] = Cell::Sand;
}

fn produce_sand(input: &str, end_condition: impl Fn(&Map) -> bool) -> usize {
    let lines = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|s| {
                    let (x, y) = s.split_once(',').unwrap();
                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect()
        })
        .collect();

    let (sand_pouring, mut map) = build_map(lines);
    let mut iterations = 0;
    while !end_condition(&map) {
        update_map(&mut map, sand_pouring);
        iterations += 1;
    }

    iterations
}

fn has_abyss_flow(map: &Map) -> bool {
    map.iter().rev().nth(1).unwrap().iter().any(|cell| cell == &Cell::Sand)
}

fn has_pyramid(map: &Map) -> bool {
    map.first().unwrap().iter().any(|cell| cell == &Cell::Sand)
}

pub fn reach_abbyss(input: &str) -> usize {
    produce_sand(input, has_abyss_flow) - 1
}

pub fn reach_pyramid(input: &str) -> usize {
    produce_sand(input, has_pyramid)
}

pub fn main() {
    let input = std::fs::read_to_string("input/day14.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", reach_abbyss(&input));
    println!("PART 2 = {}", reach_pyramid(&input));
    println!("Execution time: {:?}", now.elapsed());
}
