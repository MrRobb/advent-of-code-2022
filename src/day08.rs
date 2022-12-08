#![allow(
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::struct_excessive_bools
)]

use std::cmp::Ordering;

use itertools::{FoldWhile, Itertools};

#[derive(Debug, Clone, Copy)]
struct Tree {
    height: i64,
    visible_top: bool,
    visible_bottom: bool,
    visible_left: bool,
    visible_right: bool,
}

pub fn find_visible_trees(input: &str) -> usize {
    let mut forest: Vec<Vec<Tree>> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| Tree {
                    height: i64::from(c.to_digit(10).unwrap()),
                    visible_top: false,
                    visible_bottom: false,
                    visible_left: false,
                    visible_right: false,
                })
                .collect()
        })
        .collect();

    // Visible top
    for col in 0..forest[0].len() {
        let mut max_height = -1;
        for row in 0..forest.len() {
            if forest[row][col].height > max_height {
                forest[row][col].visible_top = true;
                max_height = forest[row][col].height;
                println!("TOP: {} ({}, {})", forest[row][col].height, row, col);
            }
        }
    }

    // Visible bottom
    for col in 0..forest[0].len() {
        let mut max_height = -1;
        for row in (0..forest.len()).rev() {
            if forest[row][col].height > max_height {
                forest[row][col].visible_bottom = true;
                max_height = forest[row][col].height;
                println!("BOTTOM: {} ({}, {})", forest[row][col].height, row, col);
            }
        }
    }

    // Visible left
    for row in 0..forest.len() {
        let mut max_height = -1;
        for col in 0..forest[row].len() {
            if forest[row][col].height > max_height {
                forest[row][col].visible_left = true;
                max_height = forest[row][col].height;
                println!("LEFT: {} ({}, {})", forest[row][col].height, row, col);
            }
        }
    }

    // Visible right
    for row in 0..forest.len() {
        let mut max_height = -1;
        for col in (0..forest[row].len()).rev() {
            if forest[row][col].height > max_height {
                forest[row][col].visible_right = true;
                max_height = forest[row][col].height;
                println!("RIGHT: {} ({}, {})", forest[row][col].height, row, col);
            }
        }
    }

    forest
        .iter()
        .flatten()
        .filter(|t| t.visible_top | t.visible_bottom | t.visible_left | t.visible_right)
        .count()
}

fn scenic_score(forest: &Vec<Vec<Tree>>, i: usize, j: usize, height: i64) -> usize {
    // Top
    let score_top = (0..i)
        .rev()
        .fold_while(0, |score, row| match forest[row][j].height.cmp(&height) {
            Ordering::Less => FoldWhile::Continue(score + 1),
            Ordering::Equal | Ordering::Greater => FoldWhile::Done(score + 1),
        })
        .into_inner();
    let score_bottom = ((i + 1)..forest.len())
        .fold_while(0, |score, row| match forest[row][j].height.cmp(&height) {
            Ordering::Less => FoldWhile::Continue(score + 1),
            Ordering::Equal | Ordering::Greater => FoldWhile::Done(score + 1),
        })
        .into_inner();
    let score_left = (0..j)
        .rev()
        .fold_while(0, |score, col| match forest[i][col].height.cmp(&height) {
            Ordering::Less => FoldWhile::Continue(score + 1),
            Ordering::Equal | Ordering::Greater => FoldWhile::Done(score + 1),
        })
        .into_inner();
    let score_right = ((j + 1)..forest[i].len())
        .fold_while(0, |score, col| match forest[i][col].height.cmp(&height) {
            Ordering::Less => FoldWhile::Continue(score + 1),
            Ordering::Equal | Ordering::Greater => FoldWhile::Done(score + 1),
        })
        .into_inner();
    score_top * score_left * score_bottom * score_right
}

pub fn highest_scenic_score(input: &str) -> usize {
    let forest: Vec<Vec<Tree>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Tree {
                    height: i64::from(c.to_digit(10).unwrap()),
                    visible_top: false,
                    visible_bottom: false,
                    visible_left: false,
                    visible_right: false,
                })
                .collect()
        })
        .collect();

    // Highest score
    forest
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, tree)| (i, j, tree)))
        .map(|(i, j, tree)| scenic_score(&forest, i, j, tree.height))
        .max()
        .unwrap()
}

pub fn main() {
    let input = std::fs::read_to_string("input/day08.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", find_visible_trees(&input));
    println!("PART 2 = {}", highest_scenic_score(&input));
    println!("Execution time: {:?}", now.elapsed());
}
