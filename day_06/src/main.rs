use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::VecDeque;
use std::collections::HashSet;

pub fn read_line
(
	path: &std::path::Path
)
-> Result<String, Box<dyn std::error::Error>>
{
	let file = File::open(path)?;
	let lines = BufReader::new(file).lines();

	for result_line in lines
	{
		return Ok(result_line?);
	}

	return Ok("".to_string());
}

fn main() {
	
	let mut line = read_line(
		std::path::Path::new("./data/input.txt")
	).unwrap().chars().collect::<VecDeque<_>>();

	// Part 1 & 2
	let marker_len = 14; // 4; for part 1
	let mut marker = VecDeque::<char>::new();

	for marker_index in 1..(line.len()+1)
	{
		marker.push_back(line.pop_front().unwrap());

		while marker.len() > marker_len
		{
			marker.pop_front();
		}

		if marker.iter().map(|&x| x).collect::<HashSet<char>>().len() == marker_len
		{
			println!("Maker position: {}", marker_index);
			return;
		}
	}

}
