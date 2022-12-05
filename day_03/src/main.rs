use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/// Reads in each line of the input file as a string and puts all of them into a vector.
#[allow(dead_code)]
pub fn read_string_data
(
	path: &std::path::Path
)
-> Result<Vec<String>, Box<dyn std::error::Error>>
{
	let file = File::open(path)?;
	let lines = BufReader::new(file).lines();
	let mut data = Vec::<String>::new();

	for result_line in lines
	{
		let line = result_line?;
		data.push(line);
	}

	return Ok(data);	
}

fn main() {

	let rucksacks = read_string_data(
		std::path::Path::new("./data/input.txt"),
	).unwrap();

	// Part 1
	let mut priorities_sum = 0;

	for rucksack in &rucksacks
	{
		let rucksack_len = rucksack.len();
		let rucksack_mid = rucksack_len/2;
		let mut found = false;

		for i in 0..rucksack_mid
		{
			for j in 0..rucksack_mid
			{
				if rucksack.chars().nth(i).unwrap() == rucksack.chars().nth(rucksack_mid+j).unwrap()
				{
					let mut priority = rucksack.chars().nth(i).unwrap() as u32 - 64;
					priority = if priority > 26 { priority - 32 } else { priority + 26 };
					priorities_sum += priority;
					found = true;
					break;
				}
			}

			if found
			{
				break;
			}
		}
	}

	println!("Priorities sum: {}", priorities_sum);



	// Part 2
	let group_len = 3;
	let mut badge_sum = 0;
	for group_i in 0..rucksacks.len()/group_len
	{
		let mut group = Vec::new();

		for j in 0..group_len
		{
			group.push(rucksacks[group_i*group_len + j].clone());
		}

		for item_elve_1 in group[0].chars()
		{
			let mut it_is_this_one = true;
			for group_member in &group
			{
				if !group_member.contains(item_elve_1)
				{
					it_is_this_one = false;
					break;
				}
			}

			if it_is_this_one
			{
				let mut badge_value = item_elve_1 as u32 - 64;
				badge_value = if badge_value > 26 { badge_value - 32 } else { badge_value + 26 };
				badge_sum += badge_value;
				break;
			}
		}
	}

	println!("Badge sum: {}", badge_sum);
}
