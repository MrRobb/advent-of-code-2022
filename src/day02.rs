#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

#[derive(PartialEq, Clone, Copy, Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for RPS {
    fn from(s: &str) -> Self {
        match s {
            "X" | "A" => Self::Rock,
            "Y" | "B" => Self::Paper,
            "Z" | "C" => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

impl RPS {
    fn beats(self, other: Self) -> u64 {
        match (self, other) {
            (Self::Rock, Self::Scissors) | (Self::Paper, Self::Rock) | (Self::Scissors, Self::Paper) => 6,
            (a, b) if a == b => 3,
            _ => 0,
        }
    }

    const fn value(self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    const fn from_game_result(result: GameResult, other: Self) -> Self {
        match result {
            GameResult::Draw => other,
            GameResult::Win => match other {
                Self::Rock => Self::Paper,
                Self::Paper => Self::Scissors,
                Self::Scissors => Self::Rock,
            },
            GameResult::Loss => match other {
                Self::Rock => Self::Scissors,
                Self::Paper => Self::Rock,
                Self::Scissors => Self::Paper,
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum GameResult {
    Win,
    Draw,
    Loss,
}

impl From<&str> for GameResult {
    fn from(s: &str) -> Self {
        match s {
            "Z" => Self::Win,
            "Y" => Self::Draw,
            "X" => Self::Loss,
            _ => unreachable!(),
        }
    }
}

pub fn calculate_wins(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(a, b)| (RPS::from(a), RPS::from(b)))
        .map(|(a, b)| b.beats(a) + b.value())
        .sum()
}

pub fn calculate_wins_guess(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(a, b)| (RPS::from(a), RPS::from_game_result(GameResult::from(b), RPS::from(a))))
        .map(|(a, b)| b.beats(a) + b.value())
        .sum()
}

pub fn main() {
    let input = std::fs::read_to_string("input/day02.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", calculate_wins(&input));
    println!("PART 2 = {}", calculate_wins_guess(&input));
    println!("Execution time: {:?}", now.elapsed());
}
