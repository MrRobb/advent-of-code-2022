use itertools::Itertools;

pub fn calculate_max_calories(input: &str) -> u64 {
	input
		.split("\n\n")
		.map(|elf| elf.lines().flat_map(str::parse::<u64>).sum::<u64>())
		.max()
		.unwrap()
}

pub fn calculate_max_3_calories(input: &str) -> u64 {
	input
		.split("\n\n")
		.map(|elf| elf.lines().flat_map(str::parse::<u64>).sum::<u64>())
		.sorted()
		.rev()
		.take(3)
		.sum()
}

pub fn main() {
	let input = std::fs::read_to_string("input/day01.txt").expect("Input file not found");
	let now = std::time::Instant::now();
	println!("PART 1 = {}", calculate_max_calories(&input));
	println!("PART 2 = {}", calculate_max_3_calories(&input));
	println!("Execution time: {:?}", now.elapsed());
}
