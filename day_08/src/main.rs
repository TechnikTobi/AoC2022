use std::cmp::max;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::time::Instant;
use rayon::prelude::*;

pub fn read_data
(
	path: &std::path::Path,
)
-> Result<Vec<Vec<i8>>, Box<dyn std::error::Error>>
{	
	let file = File::open(path)?;
	let lines = BufReader::new(file).lines();
	let mut data = Vec::<Vec<i8>>::new();

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
			.collect::<Vec<i8>>();

		data.push(elements);
	}	

	return Ok(data);
}

fn main() 
{
	let heights = read_data(
		std::path::Path::new("./data/input.txt"),
	).unwrap();

	let iterations = 5000;



	// Part 1
	let mut visible_count = 0;

	let mut part_1_times = Vec::new();
	for _iteration in 0..iterations
	{

		let part_1_start = Instant::now();
		
		visible_count = 2 * heights.len() as u64 
		+ heights
			.par_iter()
			.enumerate()
			.map(|(y, row_heights)|
				{
					let mut row_visible_count = 0;
					for x in 1..heights[0].len()-1
					{
						let tree_height = row_heights[x];
						let mut visible_from_left = true;
						let mut visible_from_right = true;
						let mut visible_from_top = true;
						let mut visible_from_bottom = true;

						for other_x in (0..x).rev()
						{
							if row_heights[other_x] >= tree_height
							{
								visible_from_left = false;
								break;
							}
						}
						if visible_from_left { row_visible_count += 1; continue; }


						for other_x in (x+1)..heights[0].len()
						{
							if row_heights[other_x] >= tree_height
							{
								visible_from_right = false;
								break;
							}
						}
						if visible_from_right { row_visible_count += 1; continue; }


						for other_y in (0..y).rev()
						{
							if heights[other_y][x] >= tree_height
							{
								visible_from_top = false;
								break;
							}
						}
						if visible_from_top { row_visible_count += 1; continue; }


						for other_y in (y+1)..heights.len()
						{
							if heights[other_y][x] >= tree_height
							{
								visible_from_bottom = false;
								break;
							}
						}
						if visible_from_bottom { row_visible_count += 1; continue; }

					}

					return row_visible_count;
				}
			).sum::<u64>();

		let part_1_end = Instant::now();
		part_1_times.push(part_1_end.duration_since(part_1_start).as_micros());
	}

	part_1_times.sort();
	println!("Visible trees in total for part 1: {}", visible_count);
	println!("Time for part 1 - Mean:   {}µs", part_1_times.iter().sum::<u128>()/(part_1_times.len() as u128));
	println!("Time for part 1 - Median: {}µs", part_1_times[part_1_times.len() / 2]);



	// Part 2
	let mut max_scenic_score = 0;

	let mut part_2_times = Vec::new();
	for _iteration in 0..iterations
	{
		let part_2_start = Instant::now();

		max_scenic_score = heights
			.par_iter()
			.enumerate()
			.map(|(y, row_heights)|
				{
					let mut row_max_scenic_score = 0u64;
					for x in 2..heights[0].len()-2
					{
						let tree_height = heights[y][x];

						if y == 0 || y == heights.len()-1 {continue;}

						let mut scenic_left = 0;
						let mut scenic_right = 0;
						let mut scenic_top = 0;
						let mut scenic_bottom = 0;
						let mut no_break: bool;

						no_break = true;
						for other_x in (0..x).rev()
						{
							if row_heights[other_x] >= tree_height
							{
								scenic_left = max((x - other_x) as u64, 1);
								no_break = false;
								break;
							}
						}
						if no_break { scenic_left = max(x as u64, 1); }
						
						no_break = true;
						for other_x in (x+1)..row_heights.len()
						{
							if row_heights[other_x] >= tree_height
							{
								scenic_right = max((other_x - x) as u64, 1);
								no_break = false;
								break;
							}
						}
						if no_break { scenic_right = max((row_heights.len() - x - 1) as u64, 1); }

						no_break = true;
						for other_y in (0..y).rev()
						{
							if heights[other_y][x] >= tree_height
							{
								scenic_top = max((y - other_y) as u64, 1);
								no_break = false;
								break;
							}
						}
						if no_break { scenic_top = max(y as u64, 1); }

						no_break = true;
						for other_y in (y+1)..heights.len()
						{
							if heights[other_y][x] >= tree_height
							{
								scenic_bottom = max((other_y - y) as u64, 1);
								no_break = false;
								break;
							}
						}
						if no_break { scenic_bottom = max((heights.len() - y - 1) as u64, 1); }

						let scenic_score = scenic_left * scenic_right * scenic_top * scenic_bottom;
						row_max_scenic_score = max(scenic_score, row_max_scenic_score);

					}

					return row_max_scenic_score;
				}
			).max().unwrap();

		let part_2_end = Instant::now();

		part_2_times.push(part_2_end.duration_since(part_2_start).as_micros());
	}

	part_2_times.sort();
	println!("Max scenic score for part 2: {}", max_scenic_score);
	println!("Time for part 2 - Mean:   {}µs", part_2_times.iter().sum::<u128>()/(part_2_times.len() as u128));
	println!("Time for part 2 - Median: {}µs", part_2_times[part_2_times.len() / 2]);

}
