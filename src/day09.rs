#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use std::collections::HashSet;

fn follow(tail: &mut (i32, i32), head: (i32, i32)) {
    let (dx, dy) = (head.0 - tail.0, head.1 - tail.1);
    if dx == 0 && dy.abs() == 2 {
        tail.1 += dy / 2;
    } else if dx.abs() == 2 && dy == 0 {
        tail.0 += dx / 2;
    } else if dx.abs() > 1 || dy.abs() > 1 {
        tail.0 += if dx > 0 { 1 } else { -1 };
        tail.1 += if dy > 0 { 1 } else { -1 };
    }
}

fn direction(dir: &str) -> (i32, i32) {
    match dir {
        "R" => (1, 0),
        "U" => (0, 1),
        "L" => (-1, 0),
        "D" => (0, -1),
        _ => panic!(),
    }
}

fn visited_knot_position(input: &str, knot_length: usize) -> usize {
    let mut knots = vec![(0, 0); knot_length];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(knots[knot_length - 1]);

    for line in input.lines() {
        let (dir, steps) = line
            .split_once(' ')
            .map(|(dir, steps)| (direction(dir), steps.parse::<i32>().unwrap()))
            .unwrap();
        for _ in 0..steps {
            knots[0] = (knots[0].0 + dir.0, knots[0].1 + dir.1);
            for i in 1..knots.len() {
                let prev = knots[i - 1];
                follow(&mut knots[i], prev);
            }
            visited.insert(knots[knot_length - 1]);
        }
    }

    visited.len()
}

pub fn part1(input: &str) -> usize {
    visited_knot_position(input, 2)
}

pub fn part2(input: &str) -> usize {
    visited_knot_position(input, 10)
}

pub fn main() {
    let input = std::fs::read_to_string("input/day09.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", part1(&input));
    println!("PART 2 = {}", part2(&input));
    println!("Execution time: {:?}", now.elapsed());
}
