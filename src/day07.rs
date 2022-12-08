#![allow(clippy::must_use_candidate, clippy::missing_panics_doc, clippy::redundant_else)]

use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

use itertools::Itertools;

fn build_filesystem(input: &str) -> HashMap<PathBuf, u64> {
    let mut filesystem = HashMap::new();
    let mut current_dir = PathBuf::from("/");
    filesystem.insert(current_dir.clone(), 0);
    let commands = input.split("$ ").skip(1);
    for command in commands {
        let mut lines = command.lines();
        let mut command = lines.next().unwrap().split_ascii_whitespace();
        match command.next().unwrap() {
            "cd" => match command.next().unwrap() {
                "/" => {
                    current_dir = PathBuf::from("/");
                },
                ".." => {
                    current_dir.pop();
                },
                dir => {
                    current_dir.push(dir);
                },
            },
            "ls" => {
                for line in lines {
                    let (size_or_dir, name) = line.split(' ').collect_tuple().unwrap();
                    match size_or_dir.parse::<u64>() {
                        Ok(size) => {
                            // File
                            for ancestor in current_dir.ancestors() {
                                *filesystem.get_mut(&ancestor.to_path_buf()).unwrap() += size;
                            }
                        },
                        Err(_) => {
                            // Directory
                            filesystem.insert(current_dir.join(name), 0);
                        },
                    }
                }
            },
            _ => unreachable!(),
        }
    }
    filesystem
}

pub fn sum_of_directories(input: &str) -> u64 {
    let filesystem = build_filesystem(input);
    filesystem.into_values().filter(|size| *size <= 100_000).sum()
}

pub fn smallest_directory(input: &str) -> u64 {
    let filesystem = build_filesystem(input);
    let filesystem_size = *filesystem.get(&PathBuf::from_str("/").unwrap()).unwrap();
    filesystem
        .into_values()
        .filter(|size| *size >= (filesystem_size - 40_000_000))
        .min()
        .unwrap()
}

pub fn main() {
    let input = std::fs::read_to_string("input/day07.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", sum_of_directories(&input));
    println!("PART 2 = {}", smallest_directory(&input));
    println!("Execution time: {:?}", now.elapsed());
}
