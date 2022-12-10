#![allow(clippy::must_use_candidate, clippy::missing_panics_doc, clippy::fallible_impl_from)]

use itertools::Itertools;

enum Op {
    Noop,
    Add(i64),
}

impl From<&str> for Op {
    fn from(s: &str) -> Self {
        let mut ins = s.split_whitespace();
        match ins.next().unwrap() {
            "noop" => Self::Noop,
            "addx" => Self::Add(ins.next().unwrap().parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

pub fn get_signal_strength(input: &str) -> i64 {
    let mut x = 1;
    let mut cycles = 1;
    let mut signal_strength = 0;
    for op in input.lines().map(Op::from) {
        match op {
            Op::Noop => {
                if (cycles + 20) % 40 == 0 {
                    signal_strength += x * cycles;
                }
                cycles += 1;
            },
            Op::Add(n) => {
                for _ in 0..2 {
                    if (cycles + 20) % 40 == 0 {
                        signal_strength += x * cycles;
                    }
                    cycles += 1;
                }
                x += n;
            },
        }
    }
    signal_strength
}

const fn draw(col: usize, x: i64) -> char {
    if (x - 1) <= col as i64 && col as i64 <= (x + 1) {
        '█'
    } else {
        ' '
    }
}

pub fn draw_pixels(input: &str) -> String {
    let mut result = [[' '; 40]; 6];
    let mut iterator = result.iter_mut().flat_map(|line| line.iter_mut().enumerate());
    let mut x = 1;
    for op in input.lines().map(Op::from) {
        match op {
            Op::Noop => {
                let (column, c) = iterator.next().unwrap();
                *c = draw(column, x);
            },
            Op::Add(n) => {
                for _ in 0..2 {
                    let (column, c) = iterator.next().unwrap();
                    *c = draw(column, x);
                }
                x += n;
            },
        }
    }
    result.map(|line| line.iter().join("")).join("\n")
}

pub fn main() {
    let input = std::fs::read_to_string("input/day10.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", get_signal_strength(&input));
    println!("PART 2 = \n{}", draw_pixels(&input));
    println!("Execution time: {:?}", now.elapsed());
}
