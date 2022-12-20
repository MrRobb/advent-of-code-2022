#![allow(clippy::must_use_candidate, clippy::missing_panics_doc, clippy::nonminimal_bool)]

use std::collections::HashSet;

use itertools::Itertools;

const fn is_next(point: &(i64, i64, i64), next: &(i64, i64, i64)) -> bool {
    let (x, y, z) = *point;
    let (nx, ny, nz) = *next;
    x == nx && y == ny && z == nz + 1
        || x == nx && y == ny && z == nz - 1
        || x == nx && y == ny + 1 && z == nz
        || x == nx && y == ny - 1 && z == nz
        || x == nx + 1 && y == ny && z == nz
        || x == nx - 1 && y == ny && z == nz
}

pub fn surface_area(input: &str) -> usize {
    let points = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    points
        .iter()
        .enumerate()
        .map(|(i, point)| {
            6 - points
                .iter()
                .enumerate()
                .filter(|(j, possible_next)| i != *j && is_next(point, possible_next))
                .count()
        })
        .sum()
}

const fn sides(point: &(i64, i64, i64)) -> [(i64, i64, i64); 6] {
    let (x, y, z) = *point;
    [
        (x, y, z + 1),
        (x, y, z - 1),
        (x, y + 1, z),
        (x, y - 1, z),
        (x + 1, y, z),
        (x - 1, y, z),
    ]
}

pub fn exterior_surface_area(input: &str) -> usize {
    let points = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let (min_x, max_x) = points.iter().map(|(x, _, _)| x).minmax().into_option().unwrap();
    let (min_y, max_y) = points.iter().map(|(_, y, _)| y).minmax().into_option().unwrap();
    let (min_z, max_z) = points.iter().map(|(_, _, z)| z).minmax().into_option().unwrap();

    let mut queue = vec![(min_x - 1, min_y - 1, min_z - 1)];
    let mut outside = HashSet::new();
    outside.insert((min_x - 1, min_y - 1, min_z - 1));

    while let Some(point) = queue.pop() {
        for side in sides(&point) {
            if min_x - 1 <= side.0
                && side.0 <= max_x + 1
                && min_y - 1 <= side.1
                && side.1 <= max_y + 1
                && min_z - 1 <= side.2
                && side.2 <= max_z + 1
                && !points.contains(&side)
                && !outside.contains(&side)
            {
                outside.insert(side);
                queue.push(side);
            }
        }
    }

    points
        .iter()
        .flat_map(sides)
        .filter(|side| outside.contains(side))
        .count()
}

pub fn main() {
    let input = std::fs::read_to_string("input/day18.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", surface_area(&input));
    println!("PART 2 = {}", exterior_surface_area(&input));
    println!("Execution time: {:?}", now.elapsed());
}
