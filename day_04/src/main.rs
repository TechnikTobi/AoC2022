use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

pub fn read_data
<T: FromStr + std::default::Default + std::fmt::Display>
(
	path: &std::path::Path,
)
-> Result<Vec<Vec<T>>, Box<dyn std::error::Error>>
{	
	let file = File::open(path)?;
	let lines = BufReader::new(file).lines();
	let mut data = Vec::<Vec::<T>>::new();

	for result_line in lines
	{
		let line = result_line?;
		let mut elements = line
			.split([',', '-'])
			.map(T::from_str)
			.map(Result::unwrap_or_default)
			.collect::<Vec<T>>();

		data.push(elements);
	}	

	return Ok(data);
}


fn main() {
	
	let raw_assignments = read_data::<u32>(
		std::path::Path::new("./data/input.txt"),
	).unwrap();

	// Part 1
	let mut total_overlaps = 0;

	for assignment in &raw_assignments
	{
		let a1_start = assignment[0];
		let a1_end = assignment[1];
		let a2_start = assignment[2];
		let a2_end = assignment[3];

		let a1_len = a1_end - a1_start;
		let a2_len = a2_end - a2_start;

		if a1_len >= a2_len
		{
			if a1_start <= a2_start && a2_end <= a1_end
			{
				total_overlaps += 1;
			}
		}
		else
		{
			if a2_start <= a1_start && a1_end <= a2_end
			{
				total_overlaps += 1;
			}
		}
	}

	println!("Total overlaps: {}", total_overlaps);


	// Part 2
	let mut partial_overlaps = 0;

	for assignment in &raw_assignments
	{
		let a1_start = assignment[0];
		let a1_end = assignment[1];
		let a2_start = assignment[2];
		let a2_end = assignment[3];

		let a1_len = a1_end - a1_start;
		let a2_len = a2_end - a2_start;

		if a1_start <= a2_start && a1_end >= a2_start
		{
			partial_overlaps += 1;
		}
		else if a2_start <= a1_start && a2_end >= a1_start
		{
			partial_overlaps += 1;
		}
	}

	println!("Partial overlaps: {}", partial_overlaps);
}
