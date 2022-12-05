use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct
ProblemData
{
	pub crates: Vec<VecDeque<char>>,
	pub instructions: Vec<Vec<u32>>
}

impl
ProblemData
{
	pub fn
	new()
	-> ProblemData
	{
		ProblemData
		{
			crates: Vec::<VecDeque<char>>::new(),
			instructions: Vec::<Vec::<u32>>::new()
		}
	}
}

pub fn read_problem_data
(
	path: &std::path::Path,
)
-> Result<ProblemData, Box<dyn std::error::Error>>
{	
	// File handling...
	let file = File::open(path)?;
	let lines = BufReader::new(file).lines();

	// Buffer-related variables for the crate status information at start of the file
	let mut current_crate_status_lines = Vec::<String>::new();
	let mut found_empty_line = false;

	// The parsed data of the problem 
	let mut problem_data = ProblemData::new();

	for result_line in lines
	{
		let line = result_line?;
		
		if line == "" && !found_empty_line
		{
			// Empty line between current crate status and instructions
			found_empty_line = true;
		}
		else if !found_empty_line
		{
			// Push back the lines with current crate status information
			current_crate_status_lines.push(line.clone());
		}
		else
		{
			// Decode the relevant information of the move instruction
			let instruction = line
				.split(' ')
				.filter(|&x| x.trim().parse::<u32>().is_ok())
				.map(u32::from_str)
				.map(Result::unwrap_or_default)
				.collect::<Vec<u32>>();

			problem_data.instructions.push(instruction);
		}
	}	

	// Count how many crates there are
	let crate_ids_line = current_crate_status_lines.pop().unwrap();
	let crates_count = crate_ids_line
		.split(' ')
		.map(u32::from_str)
		.map(Result::unwrap_or_default)
		.filter(|&x| x > 0)
		.collect::<Vec<u32>>()
		.len();

	// Create crates
	for _ in 0..crates_count
	{
		problem_data.crates.push(VecDeque::<char>::new());
	}

	// Fill crates according to their current status
	for id in 0..crates_count
	{
		for line in &current_crate_status_lines
		{
			let crate_char = line.as_bytes()[(1 + id*4) as usize] as char;
			if crate_char != ' '
			{
				problem_data.crates[id as usize].push_back(crate_char);
			}
		}
	}

	return Ok(problem_data);
}

fn main() {

	let problem_data = read_problem_data(
		std::path::Path::new("./data/input.txt"),
	).unwrap();



	// Part 1
	let mut part1_data = problem_data.clone();

	for instruction in &part1_data.instructions
	{
		let count = instruction[0] as u32;
		let source = instruction[1] as usize - 1;
		let destination = instruction[2] as usize - 1;

		for _ in 0..count
		{
			let crane_value = part1_data.crates[source].pop_front().unwrap();
			part1_data.crates[destination].push_front(crane_value);
		}
	}

	print!("Answer part 1: ");
	for c in &part1_data.crates
	{
		print!("{}", c.front().unwrap());
	}
	print!("\n");
	


	// Part 2
	let mut part2_data = problem_data.clone();

	for instruction in &part2_data.instructions
	{
		let count = instruction[0] as u32;
		let source = instruction[1] as usize - 1;
		let destination = instruction[2] as usize - 1;
		let mut crane = VecDeque::<char>::new();

		for _ in 0..count
		{
			crane.push_back(part2_data.crates[source].pop_front().unwrap());
		}

		for _ in 0..count
		{
			part2_data.crates[destination].push_front(crane.pop_back().unwrap());
		}
	}

	print!("Answer part 2: ");
	for c in &part2_data.crates
	{
		print!("{}", c.front().unwrap());
	}
	print!("\n");

}
