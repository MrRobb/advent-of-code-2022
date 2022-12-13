#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use std::cmp::Ordering;

use itertools::Itertools;

#[derive(PartialEq, Eq, Clone)]
enum Value {
    Int(u32),
    List(Vec<Self>),
}

impl Value {
    fn parse(s: &str) -> Self {
        if s.starts_with('[') {
            let s = &s[1..s.len() - 1];
            let mut children: Vec<Self> = Vec::new();
            let (mut depth, mut offset) = (0, 0);

            if s.is_empty() {
                return Self::List(children);
            }

            for (i, char) in s.chars().enumerate() {
                match char {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    ',' if depth == 0 => {
                        children.push(Self::parse(&s[offset..i]));
                        offset = i + 1;
                    },
                    _ => {},
                }
            }

            children.push(Self::parse(&s[offset..]));
            Self::List(children)
        }
        else {
            Self::Int(
                s.chars()
                    .take_while(char::is_ascii_digit)
                    .collect::<String>()
                    .parse()
                    .unwrap(),
            )
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => a.cmp(&b),
            (Self::List(a), Self::List(b)) => a.cmp(&b),
            (Self::List(children), Self::Int(b)) => children.cmp(&vec![Self::Int(*b)]),
            (Self::Int(a), Self::List(children)) => vec![Self::Int(*a)].cmp(&children),
        }
    }
}

pub fn how_many_sorted(input: &str) -> usize {
    let mut total = 0;
    for (i, group) in input.split("\n\n").enumerate() {
        for (a, b) in group.lines().tuples() {
            if Value::parse(a) < Value::parse(b) {
                total += i + 1;
            }
        }
    }
    total
}

pub fn sort_all(input: &str) -> usize {
    let dividers = ["[[2]]", "[[6]]"].map(Value::parse);

    let sorted_packets = input
        .split("\n\n")
        .flat_map(str::lines)
        .map(Value::parse)
        .chain(dividers.clone().into_iter())
        .sorted()
        .collect::<Vec<_>>();

    dividers
        .into_iter()
        .map(|divider| sorted_packets.iter().position(|packet| *packet == divider).unwrap() + 1)
        .product()
}

pub fn main() {
    let input = std::fs::read_to_string("input/day13.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", how_many_sorted(&input));
    println!("PART 2 = {}", sort_all(&input));
    println!("Execution time: {:?}", now.elapsed());
}
