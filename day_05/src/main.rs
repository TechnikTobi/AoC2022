use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
use std::collections::VecDeque;

pub fn read_data
(
	path: &std::path::Path,
)
-> Result<Vec<Vec<u32>>, Box<dyn std::error::Error>>
{	
	let file = File::open(path)?;
	let lines = BufReader::new(file).lines();
	let mut data = Vec::<Vec::<u32>>::new();

	let mut temp_lines = Vec::<String>::new();
	let mut found_empty_line = false;

	for result_line in lines
	{
		let line = result_line?;
		
		if line == "" && !found_empty_line
		{
			found_empty_line = true;
		}
		else if !found_empty_line
		{
			temp_lines.push(line.clone());
		}
		else
		{
			let elements = line
				.split(' ')
				.filter(|&x| x.trim().parse::<u32>().is_ok())
				.map(u32::from_str)
				.map(Result::unwrap_or_default)
				.collect::<Vec<u32>>();

			data.push(elements);
		}
	}	

	let last_temp_line = temp_lines[temp_lines.len()-1].clone();
	temp_lines.pop();
	let crate_ids = last_temp_line
		.split(' ')
		.map(u32::from_str)
		.map(Result::unwrap_or_default)
		.filter(|&x| x > 0)
		.collect::<Vec<u32>>();


	
	let mut crates = Vec::<VecDeque<char>>::new();

	for id in &crate_ids
	{
		crates.push(VecDeque::<char>::new());
	}

	for id in &crate_ids
	{
		for line in &temp_lines
		{
			let crate_char = line.as_bytes()[(1 + (id - 1)*4) as usize] as char;
			if crate_char != ' '
			{
				crates[(id-1) as usize].push_back(crate_char);
			}
		}
	}


	/*
	for instruction in &data
	{
		let count = instruction[0] as u32;
		let source = instruction[1] as usize - 1;
		let destination = instruction[2] as usize - 1;

		for i in 0..count
		{
			let crane_value = crates[source].pop_front().unwrap();
			crates[destination].push_front(crane_value);
		}
	}

	print!("Answer part 1: ");
	for c in &crates
	{
		print!("{}", c.front().unwrap());
	}
	print!("\n");
	*/

	for instruction in &data
	{
		let count = instruction[0] as u32;
		let source = instruction[1] as usize - 1;
		let destination = instruction[2] as usize - 1;
		let mut crane = VecDeque::<char>::new();

		for i in 0..count
		{
			crane.push_back(crates[source].pop_front().unwrap());
		}

		for i in 0..count
		{
			crates[destination].push_front(crane.pop_back().unwrap());
		}
	}

	print!("Answer part 2: ");
	for c in &crates
	{
		print!("{}", c.front().unwrap());
	}
	print!("\n");




	return Ok(data);
}

fn main() {

	let raw_assignments = read_data(
		std::path::Path::new("./data/input.txt"),
	).unwrap();

}
