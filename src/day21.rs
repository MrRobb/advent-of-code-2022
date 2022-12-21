#![allow(
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

use hashbrown::HashMap;
use parse_display::{Display, FromStr};
use xxcalc::calculator::Calculator;
use xxcalc::linear_solver::LinearSolver;

#[derive(Display, FromStr, Debug)]
enum Operation {
    #[display("{0}")]
    Number(usize),
    #[display("{0} + {1}")]
    Add(String, String),
    #[display("{0} - {1}")]
    Subtract(String, String),
    #[display("{0} * {1}")]
    Multiply(String, String),
    #[display("{0} / {1}")]
    Divide(String, String),
}

#[derive(Display, FromStr, Debug)]
#[display("{name}: {operation}")]
struct Monkey {
    name: String,
    operation: Operation,
}

fn compute_monkey(monkeys: &HashMap<String, Operation>, monkey: &str) -> usize {
    match &monkeys[monkey] {
        Operation::Number(n) => *n,
        Operation::Add(a, b) => compute_monkey(monkeys, a) + compute_monkey(monkeys, b),
        Operation::Subtract(a, b) => compute_monkey(monkeys, a) - compute_monkey(monkeys, b),
        Operation::Multiply(a, b) => compute_monkey(monkeys, a) * compute_monkey(monkeys, b),
        Operation::Divide(a, b) => compute_monkey(monkeys, a) / compute_monkey(monkeys, b),
    }
}

pub fn compute_monkeys(input: &str) -> usize {
    let monkeys = input
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .map(|m: Monkey| (m.name, m.operation))
        .collect::<HashMap<_, _>>();
    compute_monkey(&monkeys, "root")
}

fn build_expression(monkeys: &HashMap<String, Operation>, monkey_name: &str) -> String {
    if monkey_name == "humn" {
        return "x".to_string();
    }
    let monkey = &monkeys[monkey_name];
    match monkey {
        Operation::Number(n) => n.to_string(),
        Operation::Add(a, b) => format!("({} + {})", build_expression(monkeys, a), build_expression(monkeys, b)),
        Operation::Subtract(a, b) => format!("({} - {})", build_expression(monkeys, a), build_expression(monkeys, b)),
        Operation::Multiply(a, b) => format!("({} * {})", build_expression(monkeys, a), build_expression(monkeys, b)),
        Operation::Divide(a, b) => format!("({} / {})", build_expression(monkeys, a), build_expression(monkeys, b)),
    }
}

pub fn build_equation(input: &str) -> usize {
    let monkeys = input
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .map(|m: Monkey| (m.name, m.operation))
        .collect::<HashMap<_, _>>();
    let equation = match &monkeys["root"] {
        Operation::Add(a, b) | Operation::Subtract(a, b) | Operation::Multiply(a, b) | Operation::Divide(a, b) => {
            format!("{} = {}", build_expression(&monkeys, a), compute_monkey(&monkeys, b))
        },
        Operation::Number(_) => unreachable!(),
    };
    LinearSolver.process(&equation).unwrap().as_f64().unwrap().round() as usize
}

pub fn main() {
    let input = std::fs::read_to_string("input/day21.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", compute_monkeys(&input));
    println!("PART 2 = {}", build_equation(&input));
    println!("Execution time: {:?}", now.elapsed());
}
