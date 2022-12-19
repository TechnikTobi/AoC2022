use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::BufRead;
use std::io::BufReader;

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

#[derive(PartialEq, Eq, Clone)]
enum
ECaveField
{
	Air,
	Rock,
	MovingRock
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct
Position
{
	x: u64,
	y: u64,
}

fn
has_moving
(
	cave: &HashMap<Position, ECaveField>
)
-> bool
{
	return cave.iter().filter(|(_, value)| **value == ECaveField::MovingRock).count() > 0;
}

fn 
trim_cave
(
	cave: &mut HashMap<Position, ECaveField>
) 
{
	if cave.is_empty()
	{
		return;
	}
	loop 
	{
		let max_y = cave.iter().map(|(pos, _)| pos.y).max().unwrap();
		if cave.iter().filter(|(pos, value)| pos.y == max_y && **value == ECaveField::Air).count() == 7
		{
			cave.retain(|pos, _| pos.y < max_y);
		}
		else {
			break;
		}
	}
}

fn
fill_cave
(
	jets: &String,
	rocks: &Vec<Vec<String>>,
	max_rock_count: u64,
)
-> i64
{

	let mut rock_counter = 0u64;
	let mut jet_counter = 0;

	let mut cave: HashMap<Position, ECaveField> = HashMap::new();

	let mut jet_index: usize = 0;
	let mut rock_index: usize = 0;

	let mut all_heights: HashMap<(u64, u64, u64, u64, u64, u64, u64, u64, u64), (u64, u64)> = HashMap::new();
	let mut height_archive: HashMap<u64, u64> = HashMap::new();
	height_archive.insert(0, 0);

	let mut cycle_found = false;

	let mut height_after_cycle_detection: u64 = 0;
	let mut height_before_start_of_cycle: u64 = 0;

	let mut rock_count_after_cycle_detection: u64 = 0;
	let mut rock_count_before_start_of_cycle: u64 = 0;

	loop
	{

		if has_moving(&cave)
		{
			let jet = jets.chars().nth(jet_index).unwrap();
			jet_index = (jet_index+1) % jets.len();
			jet_counter += 1;

			// Push
			let mut x_offset: i64 = if jet == '<' { -1 } else { 1 };
			for field in cave.iter().filter(|(_, value)| **value == ECaveField::MovingRock)
			{
				let x = field.0.x;
				if x == (if jet == '<' {0} else {6})
				{
					x_offset = 0;
					break;
				}
				let position_left = Position{x: (x as i64 + x_offset) as u64, y:field.0.y};
				if cave[&position_left] == ECaveField::Rock
				{
					x_offset = 0;
					break;
				}
			}

			// Falling
			let mut y_offset = -1i64;
			for field in cave.iter().filter(|(_, value)| **value == ECaveField::MovingRock)
			{
				let y = field.0.y;
				if y == 0
				{
					y_offset = 0;
					break;
				}
				let position_below = Position{x: (field.0.x as i64 + x_offset) as u64, y: (y as i64 + y_offset) as u64};
				if cave[&position_below] == ECaveField::Rock
				{
					y_offset = 0;
					break;
				}
			}

			// Either move the rock or fixate its position
			let old_positions = cave.iter().filter(|(_, value)| **value == ECaveField::MovingRock).map(|(position, _)| position.clone()).collect::<Vec<Position>>();
			for old_position in &old_positions
			{
				cave.insert(old_position.clone(), ECaveField::Air);
			}
			for old_position in &old_positions
			{
				let new_position = Position {x: (old_position.x as i64 + x_offset) as u64, y: (old_position.y as i64 + y_offset) as u64};
				cave.insert(new_position.clone(), if y_offset == -1 {ECaveField::MovingRock} else {ECaveField::Rock} );
			}

			// Trim any empty space at the top of the cave
			trim_cave(&mut cave);

		}
		else
		{
			let heights = (0..7).map(|x| cave.iter().filter(|(pos, value)| pos.x == x && **value == ECaveField::Rock).map(|(pos, _)| pos.y).max().unwrap_or(0)).collect::<Vec<u64>>();
			let max_height = heights.iter().max().unwrap();
			let min_height = heights.iter().min().unwrap();
			let heights_tuple = (
				rock_index as u64,
				jet_index as u64,
				max_height - heights[0],
				max_height - heights[1],
				max_height - heights[2],
				max_height - heights[3],
				max_height - heights[4],
				max_height - heights[5],
				max_height - heights[6]
			);
			if all_heights.contains_key(&heights_tuple) && max_height != min_height && jet_counter > jets.len()
			{
				cycle_found = true;

				let (previous_height, previous_rock_count) = all_heights[&heights_tuple];
				height_after_cycle_detection = *max_height;
				height_before_start_of_cycle = previous_height;
				rock_count_after_cycle_detection = rock_counter;
				rock_count_before_start_of_cycle = previous_rock_count;

				break;
			}
			else if jet_counter > jets.len()
			{
				all_heights.insert(heights_tuple, (*max_height, rock_counter));
			}

			height_archive.insert(rock_counter, *max_height);

			// Increment counter
			rock_counter += 1;
			if rock_counter >= max_rock_count { break; }

			// Insert emtpy space below rock
			let max_y = cave.iter().map(|(pos, _)| pos.y as i64).max().unwrap_or(-1i64);
			for y in max_y+1..=max_y+3
			{
				for x in 0..7
				{
					cave.insert(Position {x:x, y:y as u64}, ECaveField::Air);
				}
			}

			// Insert new moving rock
			for (y, line) in rocks[rock_index].iter().rev().enumerate()
			{
				for (x, character) in line.chars().enumerate()
				{
					let position = Position {x:x as u64, y:(max_y+4+y as i64) as u64};
					let value = match character
					{
						'.' => ECaveField::Air,
						'#' => ECaveField::MovingRock,
						_ => panic!("AH"),
					};

					cave.insert(position, value);
				}
			}

			rock_index = (rock_index+1) % rocks.len();
		}
	}

	if !cycle_found
	{
		height_after_cycle_detection = cave.iter().map(|(pos, _)| pos.y).max().unwrap() + 1;
		height_before_start_of_cycle = 0;
		rock_count_after_cycle_detection = max_rock_count-1;
		rock_count_before_start_of_cycle = 0;
	}
	

	let cycle_height = height_after_cycle_detection - height_before_start_of_cycle;
	let rocks_used_in_cycle = rock_count_after_cycle_detection - rock_count_before_start_of_cycle;

	// Note: This rounds down the result as we use integers for the division
	let times_cycle_fits = (max_rock_count - rock_count_before_start_of_cycle) / rocks_used_in_cycle;
	
	let rocks_used_to_fill_with_cycle = times_cycle_fits * (rock_count_after_cycle_detection - rock_count_before_start_of_cycle) + rock_count_before_start_of_cycle;
	let rocks_remaining_above_last_cycle = max_rock_count - rocks_used_to_fill_with_cycle;

	let archive_index = rocks_remaining_above_last_cycle + rock_count_before_start_of_cycle;
	let height_above_cycles = height_archive[&archive_index] - height_archive[&rock_count_before_start_of_cycle];

	let final_height = times_cycle_fits * cycle_height + height_before_start_of_cycle + height_above_cycles;

	return final_height as i64;

}

fn main() 
{
	let jets = read_line(
		std::path::Path::new("./data/input.txt"),
	).unwrap();
	
	let rock1 = vec![
		"..####.".to_string(),
	];

	let rock2 = vec![
		"...#...".to_string(),
		"..###..".to_string(),
		"...#...".to_string(),
	];

	let rock3 = vec![
		"....#..".to_string(),
		"....#..".to_string(),
		"..###..".to_string(),
	];

	let rock4 = vec![
		"..#....".to_string(),
		"..#....".to_string(),
		"..#....".to_string(),
		"..#....".to_string(),
	];

	let rock5 = vec![
		"..##...".to_string(),
		"..##...".to_string(),
	];

	let rocks = vec![rock1, rock2, rock3, rock4, rock5];

	// Part 1 - For some reason does not work for example input
	let part_1_result = fill_cave(&jets, &rocks, 2023);
	println!("Part 1: {}", part_1_result);

	// Part 2
	let part_2_result = fill_cave(&jets, &rocks, 1000000000000+1);
	println!("Part 2: {}", part_2_result);

}