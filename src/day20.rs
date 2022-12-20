#![allow(
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::nonminimal_bool,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation
)]

fn groove_coordinates(nums: &[i64], positions: &[usize]) -> i64 {
    let index_zero = positions.iter().position(|&p| nums[p] == 0).unwrap();
    let index_1000th = (index_zero + 1000) % positions.len();
    let index_2000th = (index_zero + 2000) % positions.len();
    let index_3000th = (index_zero + 3000) % positions.len();
    nums[positions[index_1000th]] + nums[positions[index_2000th]] + nums[positions[index_3000th]]
}

pub fn mix(input: &str) -> i64 {
    // [input index] = num
    let nums = input.lines().map(|line| line.parse().unwrap()).collect::<Vec<i64>>();
    // [final index] = input index
    let mut positions = (0..nums.len()).collect::<Vec<usize>>();

    for (input_index, num) in nums.iter().enumerate() {
        let final_index = positions.iter().position(|&p| p == input_index).unwrap();
        positions.remove(final_index);
        let new_final_index = (final_index as i64 + num).rem_euclid(positions.len() as i64);
        positions.insert(new_final_index as usize, input_index);
    }

    groove_coordinates(&nums, &positions)
}

pub fn mix_with_key(input: &str) -> i64 {
    // Constants
    const DECRYPTION_KEY: i64 = 811_589_153;
    const ITERATIONS: usize = 10;

    // [input index] = num
    let nums = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap() * DECRYPTION_KEY)
        .collect::<Vec<i64>>();
    // [final index] = input index
    let mut positions = (0..nums.len()).collect::<Vec<usize>>();

    for _ in 0..ITERATIONS {
        for (input_index, num) in nums.iter().enumerate() {
            let final_index = positions.iter().position(|&p| p == input_index).unwrap();
            positions.remove(final_index);
            let new_final_index = (final_index as i64 + num).rem_euclid(positions.len() as i64);
            positions.insert(new_final_index as usize, input_index);
        }
    }

    groove_coordinates(&nums, &positions)
}

pub fn main() {
    let input = std::fs::read_to_string("input/day20.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", mix(&input));
    println!("PART 2 = {}", mix_with_key(&input));
    println!("Execution time: {:?}", now.elapsed());
}
