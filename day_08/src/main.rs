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

	// Part 1
	let part_1_start = Instant::now();
	
	let visible_count = 2 * heights.len() as u64 
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


	// for y in 1..heights.len()-1
	// {
	// 	visible_count += heights[y].par_iter().enumerate().map(|(x, tree_height)|
	// 		{
	// 			let mut visible_from_left = true;
	// 			let mut visible_from_right = true;
	// 			let mut visible_from_top = true;
	// 			let mut visible_from_bottom = true;

	// 			for other_x in (0..x).rev()
	// 			{
	// 				if heights[y][other_x] >= *tree_height
	// 				{
	// 					visible_from_left = false;
	// 					break;
	// 				}
	// 			}
	// 			if visible_from_left { return 1; }


	// 			for other_x in (x+1)..heights[0].len()
	// 			{
	// 				if heights[y][other_x] >= *tree_height
	// 				{
	// 					visible_from_right = false;
	// 					break;
	// 				}
	// 			}
	// 			if visible_from_right { return 1; }


	// 			for other_y in (0..y).rev()
	// 			{
	// 				if heights[other_y][x] >= *tree_height
	// 				{
	// 					visible_from_top = false;
	// 					break;
	// 				}
	// 			}
	// 			if visible_from_top { return 1; }


	// 			for other_y in (y+1)..heights.len()
	// 			{
	// 				if heights[other_y][x] >= *tree_height
	// 				{
	// 					visible_from_bottom = false;
	// 					break;
	// 				}
	// 			}
	// 			if visible_from_bottom { return 1; }

	// 			return 0;
	// 		}
	// 	).sum::<u64>();


	// 	// for x in 1..heights[0].len()-1
	// 	// {
	// 	// 	let tree_height = heights[y][x];

	// 	// 	let mut visible_from_left = true;
	// 	// 	let mut visible_from_right = true;
	// 	// 	let mut visible_from_top = true;
	// 	// 	let mut visible_from_bottom = true;

	// 	// 	for other_x in (0..x).rev()
	// 	// 	{
	// 	// 		if heights[y][other_x] >= tree_height
	// 	// 		{
	// 	// 			visible_from_left = false;
	// 	// 			break;
	// 	// 		}
	// 	// 	}
	// 	// 	if visible_from_left { visible_count += 1; continue; }


	// 	// 	for other_x in (x+1)..heights[0].len()
	// 	// 	{
	// 	// 		if heights[y][other_x] >= tree_height
	// 	// 		{
	// 	// 			visible_from_right = false;
	// 	// 			break;
	// 	// 		}
	// 	// 	}
	// 	// 	if visible_from_right { visible_count += 1; continue; }


	// 	// 	for other_y in (0..y).rev()
	// 	// 	{
	// 	// 		if heights[other_y][x] >= tree_height
	// 	// 		{
	// 	// 			visible_from_top = false;
	// 	// 			break;
	// 	// 		}
	// 	// 	}
	// 	// 	if visible_from_top { visible_count += 1; continue; }


	// 	// 	for other_y in (y+1)..heights.len()
	// 	// 	{
	// 	// 		if heights[other_y][x] >= tree_height
	// 	// 		{
	// 	// 			visible_from_bottom = false;
	// 	// 			break;
	// 	// 		}
	// 	// 	}
	// 	// 	if visible_from_bottom { visible_count += 1; continue; }

	// 	// }
	// }
	

	// let mut visible_count = 4;

	// for y in 1..heights.len()-1
	// {
	// 	let mut left_depth = 0;
	// 	while heights[y][left_depth] < heights[y][left_depth+1]
	// 	{
	// 		left_depth += 1;
	// 	}

	// 	let mut right_depth = heights[y].len()-1;
	// 	while heights[y][right_depth-1] > heights[y][right_depth]
	// 	{
	// 		right_depth -= 1;
	// 	}

	// 	visible_count += (heights[y].len() - right_depth + left_depth + 1) as u64;
	// }

	// for x in 1..heights[0].len()-1
	// {
	// 	println!(" ");
	// 	let mut top_depth = 0;
	// 	let mut top_visible = 0;
	// 	let mut top_max_height = -1;
	// 	/*
	// 	while heights[top_depth][x] < heights[top_depth+1][x]
	// 	{
	// 		top_depth += 1;
	// 	}
	// 	*/
	// 	while top_max_height < 9 && top_depth < heights.len() - 1 {
	// 		print!("\n{}", heights[top_depth][x]);
	// 		if heights[top_depth][x] > top_max_height
	// 		{
	// 			print!(" is visible!");
	// 			top_max_height = heights[top_depth][x];
	// 			top_visible += 1;
	// 		}
	// 		top_depth += 1;
	// 	}

	// 	let mut bottom_depth = heights.len() as i64 - 1;
	// 	let mut bottom_visible = 0;
	// 	let mut bottom_max_height = -1;
	// 	while bottom_max_height < top_max_height - 1 && bottom_depth >= 0 {
	// 		if heights[bottom_depth as usize][x]  > bottom_max_height
	// 		{
	// 			bottom_max_height = heights[top_depth][x];
	// 			bottom_visible += 1;
	// 		}
	// 		bottom_depth -= 1;
	// 	}

	// 	// let mut bottom_depth = heights.len() -1;
	// 	// while heights[bottom_depth-1][x] > heights[bottom_depth][x]
	// 	// {
	// 	// 	bottom_depth -= 1;
	// 	// }

	// 	// visible_count += ((heights.len() - 1) - bottom_depth + 1 + top_depth + 1) as u64;
	// 	println!("\nVisible: {}", top_visible);
	// 	visible_count += top_visible + bottom_visible;
	// }

	let part_1_end = Instant::now();

	println!("Visible trees in total for part 1: {}", visible_count);
	println!("Time for part 1: {}µs", part_1_end.duration_since(part_1_start).as_micros());

	// Part 2

	let part_2_start = Instant::now();

	let max_scenic_score = heights
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

					for other_x in (0..x).rev()
					{
						// scenic_left += 1;
						if row_heights[other_x] >= tree_height
						{
							scenic_left = max((x - other_x) as u64, 1);
							break;
						}
					}
					
					for other_x in (x+1)..row_heights.len()
					{
						scenic_right += 1;
						if row_heights[other_x] >= tree_height
						{
							// scenic_right = max((other_x - x) as u64, 1);
							break;
						}
					}

					for other_y in (0..y).rev()
					{
						scenic_top += 1;
						if heights[other_y][x] >= tree_height
						{
							// scenic_top = max((y - other_y) as u64, 1);
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
					row_max_scenic_score = max(scenic_score, row_max_scenic_score);

				}

				return row_max_scenic_score;
			}
		).max().unwrap();

	// let mut max_scenic_score = 0u64;

	// for x in 1..heights[0].len()-1
	// {
	// 	for y in 1..heights.len()-1
	// 	{
	// 		let tree_height = heights[y][x];

	// 		if tree_height == 0 { continue; }

	// 		let mut scenic_left = 0;
	// 		let mut scenic_right = 0;
	// 		let mut scenic_top = 0;
	// 		let mut scenic_bottom = 0;

	// 		for other_x in (0..x).rev()
	// 		{
	// 			if heights[y][other_x] >= tree_height
	// 			{
	// 				scenic_left = max((x - other_x) as u64, 1);
	// 				break;
	// 			}
	// 		}
	// 		if scenic_left == 0 {continue;}
			
	// 		for other_x in (x+1)..heights[0].len()
	// 		{
	// 			if heights[y][other_x] >= tree_height
	// 			{
	// 				scenic_right = max((other_x - x) as u64, 1);
	// 				break;
	// 			}
	// 		}
	// 		if scenic_right == 0 {continue;}

	// 		for other_y in (0..y).rev()
	// 		{
	// 			if heights[other_y][x] >= tree_height
	// 			{
	// 				scenic_top = (y - other_y) as u64;
	// 				break;
	// 			}
	// 		}
	// 		if scenic_top == 0 {continue;}

	// 		for other_y in (y+1)..heights.len()
	// 		{
	// 			if heights[other_y][x] >= tree_height
	// 			{
	// 				scenic_bottom = (other_y - y) as u64;
	// 				break;
	// 			}
	// 		}
	// 		if scenic_bottom == 0 {continue;}

	// 		let scenic_score = scenic_left * scenic_right * scenic_top * scenic_bottom;
	// 		max_scenic_score = max(scenic_score, max_scenic_score);
	// 	}
	// }

	let part_2_end = Instant::now();

	println!("Max scenic score for part 2: {}", max_scenic_score);
	println!("Time for part 2: {}µs", part_2_end.duration_since(part_2_start).as_micros());

}
