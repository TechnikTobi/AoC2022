mod input;

use crate::input::*;

fn main() {

	let calories_per_elf = read_vecs::<u64>(
		std::path::Path::new("./data/input.txt"),
	).unwrap();

	let mut max_calories = 0;
	let mut calories_sums = Vec::<u64>::new();

	for calories_vec in calories_per_elf
	{
		let sum = calories_vec.iter().sum();
		max_calories = if sum > max_calories { sum } else { max_calories };
		calories_sums.push(sum);
	}

	calories_sums.sort_by(|a, b| b.cmp(a));

	let number_top_elves = 3;
	let mut total_sum = 0;

	for i in 0..number_top_elves
	{
		total_sum += calories_sums[i];
	}

	println!("Max Calories: {}", max_calories);
	println!("Total Calories of top {} elves: {}", number_top_elves, total_sum);
}