#![allow(
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::fallible_impl_from,
    clippy::option_if_let_else
)]

use itertools::Itertools;
use num::Integer;

type WorryLevel = u64;

struct Monkey {
    items: Vec<WorryLevel>,
    operation: Box<dyn Fn(WorryLevel) -> WorryLevel>,
    divisible_by: WorryLevel,
    success: usize,
    failure: usize,
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let mut lines = s.lines().skip(1);

        // Starting items
        let starting_items = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();

        // Operation
        let operation = lines.next().unwrap().rsplit(' ').take(2).collect_vec();
        let operation: Box<dyn Fn(WorryLevel) -> WorryLevel> = match operation.as_slice() {
            [n, "+"] => match n.parse::<WorryLevel>() {
                Ok(n) => Box::new(move |old| old + n),
                Err(_) => Box::new(move |old| old + old),
            },
            [n, "*"] => match n.parse::<WorryLevel>() {
                Ok(n) => Box::new(move |old| old * n),
                Err(_) => Box::new(move |old| old * old),
            },
            _ => panic!("Unknown operation"),
        };

        // Divisible by, success, failure
        let divisible_by = lines.next().unwrap().rsplit_once(' ').unwrap().1.parse().unwrap();
        let success = lines.next().unwrap().rsplit_once(' ').unwrap().1.parse().unwrap();
        let failure = lines.next().unwrap().rsplit_once(' ').unwrap().1.parse().unwrap();

        Self {
            items: starting_items,
            operation,
            divisible_by,
            success,
            failure,
        }
    }
}

fn play_rounds(mut monkeys: Vec<Monkey>, rounds: usize, gets_bored: impl Fn(WorryLevel) -> WorryLevel) -> usize {
    let mut monkey_business = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for imonkey in 0..monkeys.len() {
            // Capture current monkey
            let monkey_items = monkeys[imonkey].items.clone();
            monkeys[imonkey].items.truncate(0);

            for worry_level in monkey_items {
                // Inspect item
                let mut new_worry_level = (monkeys[imonkey].operation)(worry_level);
                monkey_business[imonkey] += 1;

                // Gets bored
                new_worry_level = gets_bored(new_worry_level);

                // Test
                if new_worry_level % monkeys[imonkey].divisible_by == 0 {
                    let success_monkey = monkeys[imonkey].success;
                    monkeys[success_monkey].items.push(new_worry_level);
                }
                else {
                    let failure_monkey = monkeys[imonkey].failure;
                    monkeys[failure_monkey].items.push(new_worry_level);
                }
            }
        }
    }

    // Get top 2
    monkey_business
        .into_iter()
        .sorted()
        .rev()
        .take(2)
        .reduce(|a, b| a * b)
        .unwrap()
}

pub fn keep_away(input: &str) -> usize {
    let monkeys = input.split("\n\n").map(Monkey::from).collect::<Vec<_>>();
    play_rounds(monkeys, 20, |worry_level| worry_level / 3)
}

pub fn keep_away_lcm(input: &str) -> usize {
    let monkeys = input.split("\n\n").map(Monkey::from).collect::<Vec<_>>();
    let monkey_lcm = monkeys.iter().map(|m| m.divisible_by).fold(1, |a, b| a.lcm(&b));
    play_rounds(monkeys, 10_000, |worry_level| worry_level % monkey_lcm)
}

pub fn main() {
    let input = std::fs::read_to_string("input/day11.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", keep_away(&input));
    println!("PART 2 = {}", keep_away_lcm(&input));
    println!("Execution time: {:?}", now.elapsed());
}
