#![allow(clippy::must_use_candidate, clippy::missing_panics_doc, clippy::redundant_else)]

use std::collections::{HashMap, VecDeque};
use std::mem::drop;

use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
enum FilesystemNode {
    #[display("dir {name}")]
    Directory {
        name: String,
        #[from_str(default)]
        children: HashMap<String, FilesystemNode>,
        #[from_str(default)]
        size: u64,
    },
    #[display("{size} {name}")]
    File { name: String, size: u64 },
}

impl FilesystemNode {
    const fn is_directory(&self) -> bool {
        matches!(self, Self::Directory { .. })
    }

    const fn size(&self) -> u64 {
        match self {
            Self::File { size, .. } | Self::Directory { size, .. } => *size,
        }
    }

    fn iter(&self) -> FilesystemIterator {
        FilesystemIterator {
            queue: vec![self].into(),
        }
    }
}

struct FilesystemIterator<'a> {
    queue: VecDeque<&'a FilesystemNode>,
}

impl<'a> Iterator for FilesystemIterator<'a> {
    type Item = &'a FilesystemNode;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.queue.pop_front()?;
        match node {
            FilesystemNode::Directory { ref children, .. } => {
                self.queue.extend(children.values());
            },
            FilesystemNode::File { .. } => {},
        }
        Some(node)
    }
}

#[derive(Display, FromStr, PartialEq, Debug)]
enum Commands {
    #[display("cd {0}")]
    Cd(String),
    #[display("ls")]
    Ls,
}

fn build_filesystem_i<'a>(
    mut commands: impl Iterator<Item = &'a str>,
    current_dir: &mut FilesystemNode,
) -> impl Iterator<Item = &'a str> {
    while let Some(next_command) = commands.next() {
        let mut lines = next_command.lines();
        let command = lines.next().unwrap().parse().unwrap();
        match command {
            Commands::Cd(dir) => match dir.as_str() {
                ".." => {
                    return commands;
                },
                _ => match current_dir {
                    FilesystemNode::Directory { ref mut children, .. } => {
                        commands = build_filesystem_i(commands, children.get_mut(&dir).unwrap());
                    },
                    FilesystemNode::File { .. } => panic!("Cannot cd into files"),
                },
            },
            Commands::Ls => match current_dir {
                FilesystemNode::Directory { ref mut children, .. } => {
                    *children = lines
                        .flat_map(str::parse)
                        .map(|item| match item {
                            FilesystemNode::File { ref name, .. } | FilesystemNode::Directory { ref name, .. } => {
                                (name.clone(), item)
                            },
                        })
                        .collect();
                },
                FilesystemNode::File { .. } => panic!("Cannot list files"),
            },
        }
    }
    commands
}

fn build_filesystem(input: &str) -> FilesystemNode {
    let mut commands = input.split("$ ").skip(1);
    let mut lines = commands.next().unwrap().lines();
    let command = lines.next().unwrap().parse().unwrap();
    if let Commands::Cd(root) = command {
        let mut root = FilesystemNode::Directory {
            name: root,
            children: HashMap::new(),
            size: 0,
        };
        drop(build_filesystem_i(&mut commands, &mut root));
        root
    }
    else {
        panic!("Invalid input: {:?}", commands.collect_vec())
    }
}

fn calculate_sizes(filesystem: &mut FilesystemNode) -> u64 {
    match filesystem {
        FilesystemNode::Directory {
            ref mut children,
            ref mut size,
            ..
        } => {
            *size = children.values_mut().map(calculate_sizes).sum();
            *size
        },
        FilesystemNode::File { ref size, .. } => *size,
    }
}

pub fn sum_of_directories(input: &str) -> u64 {
    let mut filesystem = build_filesystem(input);
    let _filesystem_size = calculate_sizes(&mut filesystem);
    filesystem
        .iter()
        .filter(|node| node.is_directory())
        .map(FilesystemNode::size)
        .filter(|size| *size <= 100_000)
        .sum()
}

pub fn smallest_directory(input: &str) -> u64 {
    let mut filesystem = build_filesystem(input);
    let filesystem_size = calculate_sizes(&mut filesystem);
    let mut min_size = u64::MAX;
    for node in filesystem.iter() {
        if let FilesystemNode::Directory { ref size, .. } = node {
            if *size >= (filesystem_size - 40_000_000) && *size <= min_size {
                min_size = *size;
            }
        }
    }
    min_size
}

pub fn main() {
    let input = std::fs::read_to_string("input/day07.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", sum_of_directories(&input));
    println!("PART 2 = {}", smallest_directory(&input));
    println!("Execution time: {:?}", now.elapsed());
}
