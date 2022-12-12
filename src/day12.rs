#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use pathfinding::prelude::dijkstra;

const fn height(c: char) -> i64 {
    match c {
        'S' => 'a' as i64,
        'E' => 'z' as i64,
        c => c as i64,
    }
}

fn distance(heightmap: &[Vec<char>], idx1: &(usize, usize), idx2: &(usize, usize)) -> i64 {
    let (i1, j1) = idx1;
    let (i2, j2) = idx2;
    let h1 = heightmap[*i1][*j1];
    let h2 = heightmap[*i2][*j2];
    height(h1) - height(h2)
}

fn neighbors(idx: &(usize, usize), heightmap: &[Vec<char>]) -> Vec<((usize, usize), usize)> {
    let mut neighbors = Vec::new();
    if idx.0 > 0 && distance(heightmap, idx, &(idx.0 - 1, idx.1)) <= 1 {
        neighbors.push(((idx.0 - 1, idx.1), 1));
    }
    if idx.0 < heightmap.len() - 1 && distance(heightmap, idx, &(idx.0 + 1, idx.1)) <= 1 {
        neighbors.push(((idx.0 + 1, idx.1), 1));
    }
    if idx.1 > 0 && distance(heightmap, idx, &(idx.0, idx.1 - 1)) <= 1 {
        neighbors.push(((idx.0, idx.1 - 1), 1));
    }
    if idx.1 < heightmap[0].len() - 1 && distance(heightmap, idx, &(idx.0, idx.1 + 1)) <= 1 {
        neighbors.push(((idx.0, idx.1 + 1), 1));
    }
    neighbors
}

fn min_path(input: &str, goal: impl Fn(&[Vec<char>], &(usize, usize)) -> bool) -> usize {
    // Build map
    let mut start = (0, 0);
    let heightmap: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == 'E' {
                        start = (i, j);
                    }
                    c
                })
                .collect()
        })
        .collect();

    // Apply Dijkstra E -> .. -> S
    let result = dijkstra(
        &start,
        |node| neighbors(node, &heightmap),
        |node| goal(&heightmap, node),
    );

    // Get cost result
    result.unwrap().0.len() - 1
}

pub fn min_path_to_s(input: &str) -> usize {
    min_path(input, |heightmap, &(i, j)| heightmap[i][j] == 'S')
}

pub fn min_path_to_a(input: &str) -> usize {
    min_path(input, |heightmap, &(i, j)| height(heightmap[i][j]) == 'a' as i64)
}

pub fn main() {
    let input = std::fs::read_to_string("input/day12.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", min_path_to_s(&input));
    println!("PART 2 = {}", min_path_to_a(&input));
    println!("Execution time: {:?}", now.elapsed());
}
