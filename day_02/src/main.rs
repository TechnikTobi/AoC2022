use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

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

	// General data read in
	let games = read_string_data(
		std::path::Path::new("./data/input.txt"),
	).unwrap();


	// Part 1
	let mut points = 0;

	for game in &games
	{
		points += match game.as_str()
		{
			"A X" => 1 + 3,
			"A Y" => 2 + 6,
			"A Z" => 3,
			"B X" => 1,
			"B Y" => 2 + 3,
			"B Z" => 3 + 6,
			"C X" => 1 + 6,
			"C Y" => 2,
			"C Z" => 3 + 3,
			_ => panic!("Illegal combination"),
		};
	}

	println!("My total points for Part 1: {}", points);
	

	// Part 2
	points = 0;

	for game in &games
	{
		points += match game.as_str()
		{
			"A X" => 0 + 3,
			"A Y" => 3 + 1,
			"A Z" => 6 + 2,
			"B X" => 0 + 1,
			"B Y" => 3 + 2,
			"B Z" => 6 + 3,
			"C X" => 0 + 2,
			"C Y" => 3 + 3,
			"C Z" => 6 + 1,
			_ => panic!("Illegal combination"),
		};
	}

	println!("My total points for Part 2: {}", points);
	
}
