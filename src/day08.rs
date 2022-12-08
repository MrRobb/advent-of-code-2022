#![allow(
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::struct_excessive_bools
)]

use std::cmp::Ordering;

use itertools::{FoldWhile, Itertools};

type Tree = i64;

pub fn find_visible_trees(input: &str) -> usize {
    let forest: Vec<Vec<Tree>> = input
        .lines()
        .map(|line| line.chars().map(|c| Tree::from(c.to_digit(10).unwrap())).collect())
        .collect();
    let mut visibles: Vec<Vec<bool>> = vec![vec![false; forest[0].len()]; forest.len()];

    // Visible top
    for col in 0..forest[0].len() {
        let mut max_height = -1;
        for row in 0..forest.len() {
            if forest[row][col] > max_height {
                visibles[row][col] = true;
                max_height = forest[row][col];
            }
        }
    }

    // Visible bottom
    for col in 0..forest[0].len() {
        let mut max_height = -1;
        for row in (0..forest.len()).rev() {
            if forest[row][col] > max_height {
                visibles[row][col] = true;
                max_height = forest[row][col];
            }
        }
    }

    // Visible left
    for row in 0..forest.len() {
        let mut max_height = -1;
        for col in 0..forest[row].len() {
            if forest[row][col] > max_height {
                visibles[row][col] = true;
                max_height = forest[row][col];
            }
        }
    }

    // Visible right
    for row in 0..forest.len() {
        let mut max_height = -1;
        for col in (0..forest[row].len()).rev() {
            if forest[row][col] > max_height {
                visibles[row][col] = true;
                max_height = forest[row][col];
            }
        }
    }

    visibles.into_iter().flatten().filter(|&visible| visible).count()
}

fn scenic_score(forest: &Vec<Vec<Tree>>, i: usize, j: usize, height: Tree) -> usize {
    // Top
    let score_top = (0..i)
        .rev()
        .fold_while(0, |score, row| match forest[row][j].cmp(&height) {
            Ordering::Less => FoldWhile::Continue(score + 1),
            Ordering::Equal | Ordering::Greater => FoldWhile::Done(score + 1),
        })
        .into_inner();
    // Bottom
    let score_bottom = ((i + 1)..forest.len())
        .fold_while(0, |score, row| match forest[row][j].cmp(&height) {
            Ordering::Less => FoldWhile::Continue(score + 1),
            Ordering::Equal | Ordering::Greater => FoldWhile::Done(score + 1),
        })
        .into_inner();
    // Left
    let score_left = (0..j)
        .rev()
        .fold_while(0, |score, col| match forest[i][col].cmp(&height) {
            Ordering::Less => FoldWhile::Continue(score + 1),
            Ordering::Equal | Ordering::Greater => FoldWhile::Done(score + 1),
        })
        .into_inner();
    // Right
    let score_right = ((j + 1)..forest[i].len())
        .fold_while(0, |score, col| match forest[i][col].cmp(&height) {
            Ordering::Less => FoldWhile::Continue(score + 1),
            Ordering::Equal | Ordering::Greater => FoldWhile::Done(score + 1),
        })
        .into_inner();
    // Total
    score_top * score_left * score_bottom * score_right
}

pub fn highest_scenic_score(input: &str) -> usize {
    let forest: Vec<Vec<Tree>> = input
        .lines()
        .map(|line| line.chars().map(|c| Tree::from(c.to_digit(10).unwrap())).collect())
        .collect();

    // Highest score
    forest
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, tree)| (i, j, tree)))
        .map(|(i, j, tree)| scenic_score(&forest, i, j, *tree))
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
