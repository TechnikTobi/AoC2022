use std::cmp::max;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::time::Instant;

pub fn read_data
(
	path: &std::path::Path,
)
-> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>>
{	
	let file = File::open(path)?;
	let lines = BufReader::new(file).lines();
	let mut data = Vec::<Vec<u8>>::new();

	for result_line in lines
	{
		let line = result_line?;
		let elements = line.chars()
			.map(|character|
				match character {
					'0' => 0,
					'1' => 1,
					'2' => 2,
					'3' => 3,
					'4' => 4,
					'5' => 5,
					'6' => 6,
					'7' => 7,
					'8' => 8,
					'9' => 9,
					_ => panic!("AH"),
				}
			)
			.collect::<Vec<u8>>();

		data.push(elements);
	}	

	return Ok(data);
}

fn main() 
{
	let heights = read_data(
		std::path::Path::new("./data/input.txt"),
	).unwrap();

	// Part 1
	let part_1_start = Instant::now();

	let mut visible_count = 0u64;

	for y in 0..heights.len()
	{
		for x in 0..heights[0].len()
		{
			let tree_height = heights[y][x];

			let mut visible_from_left = true;
			let mut visible_from_right = true;
			let mut visible_from_top = true;
			let mut visible_from_bottom = true;

			for other_x in 0..x
			{
				if heights[y][other_x] >= tree_height
				{
					visible_from_left = false;
					break;
				}
			}

			for other_x in (x+1)..heights[0].len()
			{
				if heights[y][other_x] >= tree_height
				{
					visible_from_right = false;
					break;
				}
			}

			for other_y in 0..y
			{
				if heights[other_y][x] >= tree_height
				{
					visible_from_top = false;
					break;
				}
			}

			for other_y in (y+1)..heights.len()
			{
				if heights[other_y][x] >= tree_height
				{
					visible_from_bottom = false;
					break;
				}
			}

			visible_count += ((visible_from_left) || (visible_from_right) || (visible_from_top) || (visible_from_bottom)) as u64;
			
		}
	}

	let part_1_end = Instant::now();

	println!("Visible trees in total for part 1: {}", visible_count);
	println!("Time for part 1: {}µs", part_1_end.duration_since(part_1_start).as_micros());

	// Part 2

	let part_2_start = Instant::now();

	let mut max_scenic_score = 0u64;

	// Left
	for x in 0..heights[0].len()
	{
		for y in 0..heights.len()
		{
			let tree_height = heights[y][x];

			let mut scenic_left = 0;
			let mut scenic_right = 0;
			let mut scenic_top = 0;
			let mut scenic_bottom = 0;

			for other_x in (0..x).rev()
			{
				scenic_left += 1;
				if heights[y][other_x] >= tree_height
				{
					break;
				}
			}
			
			for other_x in (x+1)..heights[0].len()
			{
				scenic_right += 1;
				if heights[y][other_x] >= tree_height
				{
					break;
				}
			}

			for other_y in (0..y).rev()
			{
				scenic_top += 1;
				if heights[other_y][x] >= tree_height
				{
					break;
				}
			}

			for other_y in (y+1)..heights.len()
			{
				scenic_bottom += 1;
				if heights[other_y][x] >= tree_height
				{
					break;
				}
			}

			let scenic_score = scenic_left * scenic_right * scenic_top * scenic_bottom;
			max_scenic_score = max(scenic_score, max_scenic_score);
		}
	}

	let part_2_end = Instant::now();

	println!("Max scenic score for part 2: {}", max_scenic_score);
	println!("Time for part 2: {}µs", part_2_end.duration_since(part_2_start).as_micros());

}
