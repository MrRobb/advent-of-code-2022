#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use advent_of_code_2021::{
	day01
};

fn main() {
	let mains = [
		day01::main,
	];

	let now = std::time::Instant::now();

	for (day, main) in mains.iter().enumerate() {
		println!(
			"------------------------------------ DAY {} ------------------------------------",
			day + 1
		);
		main();
		println!();
	}

	println!("------------------------------------  ALL   ------------------------------------");
	println!("Execution time: {:?}\n", now.elapsed());
}
