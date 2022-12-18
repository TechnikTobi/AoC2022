use std::collections::HashMap;
use std::collections::VecDeque;
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

fn
move_left
(
	buffer: &mut Vec<String>
)
{

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
fill_cave
(
	jets: &String,
	rocks: &Vec<Vec<String>>,
	max_rock_count: u64,
)
-> (u64, u64, u64, u64)
{

	let mut rock_counter = 0u64;
	let mut jet_counter = 0;

	let mut cave: HashMap<Position, ECaveField> = HashMap::new();

	let mut jet_index: usize = 0;
	let mut rock_index: usize = 0;

	let mut all_heights: HashMap<(u64, u64, u64, u64, u64, u64, u64), (u64, u64)> = HashMap::new();

	loop
	{
		// println!("{}", rock_counter);

		if has_moving(&cave)
		{
			let jet = jets.chars().nth(jet_index).unwrap();
			jet_index += 1;
			jet_index = jet_index % jets.len();
			jet_counter += 1;

			// Push
			if jet != '<' && jet != '>' { panic!("Illegal jet"); }

			let mut can_move = true;
			let mut x_offset: i64 = if jet == '<' { -1 } else { 1 };
			for field in cave.iter().filter(|(_, value)| **value == ECaveField::MovingRock)
			{
				let x = field.0.x;
				if x == (if jet == '<' {0} else {6})
				{
					can_move = false;
					break;
				}
				let position_left = Position{x: (x as i64 + x_offset) as u64, y:field.0.y};
				if cave[&position_left] == ECaveField::Rock
				{
					can_move = false;
					break;
				}
			}
			if !can_move { x_offset = 0; }

			let old_positions = cave.iter().filter(|(_, value)| **value == ECaveField::MovingRock).map(|(position, _)| position.clone()).collect::<Vec<Position>>();
			for old_position in &old_positions
			{
				cave.insert(old_position.clone(), ECaveField::Air);
			}
			for old_position in &old_positions
			{
				let new_position = Position {x: (old_position.x as i64 + x_offset) as u64, y: old_position.y};
				cave.insert(new_position.clone(), ECaveField::MovingRock);
			}

			// Falling

			// Check if everything below is free
			let mut can_fall = true;
			for field in cave.iter().filter(|(_, value)| **value == ECaveField::MovingRock)
			{
				let y = field.0.y;
				if y == 0
				{
					can_fall = false;
					break;
				}
				let position_below = Position{x: field.0.x, y:y-1};
				if cave[&position_below] == ECaveField::Rock
				{
					can_fall = false;
					break;
				}
			}

			let old_positions = cave.iter().filter(|(_, value)| **value == ECaveField::MovingRock).map(|(position, _)| position.clone()).collect::<Vec<Position>>();
			if can_fall
			{
				for old_position in &old_positions
				{
					cave.insert(old_position.clone(), ECaveField::Air);
				}
				for old_position in &old_positions
				{
					let new_position = Position {x: old_position.x, y: old_position.y -1};
					cave.insert(new_position.clone(), ECaveField::MovingRock);
				}
				trim_cave(&mut cave);
			}
			else 
			{
				for old_position in old_positions
				{
					cave.insert(old_position, ECaveField::Rock);
				}
			}

		}
		else
		{
			rock_counter += 1;
			if rock_counter >= max_rock_count { break; }

			let heights = (0..7).map(|x| cave.iter().filter(|(pos, value)| pos.x == x && **value == ECaveField::Rock).map(|(pos, _)| pos.y).max().unwrap_or(0)).collect::<Vec<u64>>();
			let max_height = heights.iter().max().unwrap();
			let min_height = heights.iter().min().unwrap();
			let heights_tuple = (
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
				// Print
				let max_y = cave.iter().map(|(pos, _)| pos.y).max().unwrap();
				for y in (0..=max_y).rev()
				{
					for x in 0..7
					{
						match cave[&Position{x:x, y:y}]
						{
							ECaveField::Air => print!("."),
							ECaveField::Rock => print!("#"),
							ECaveField::MovingRock => print!("@"),
						}
					}
					print!("\n");
				}
				print!("\n");
				let (previous_height, previous_rock_count) = all_heights[&heights_tuple];
				return (max_height-previous_height, previous_height, rock_counter, previous_rock_count);
			}
			else if jet_counter > jets.len()
			{
				all_heights.insert(heights_tuple, (*max_height, rock_counter));
			}

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

			rock_index += 1;
			rock_index = rock_index % rocks.len();
		}
	}

	// Print
	let max_y = cave.iter().map(|(pos, _)| pos.y).max().unwrap();
	for y in (0..=max_y).rev()
	{
		for x in 0..7
		{
			match cave[&Position{x:x, y:y}]
			{
				ECaveField::Air => print!("."),
				ECaveField::Rock => print!("#"),
				ECaveField::MovingRock => print!("@"),
			}
		}
		print!("\n");
	}
	print!("\n");

	let height = cave.iter().map(|(pos, _)| pos.y).max().unwrap() + 1;
	return (height, 0, max_rock_count-1, 0);
}

fn main() 
{
	let jets = read_line(
		std::path::Path::new("./data/example.txt"),
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

	// Part 1
	let mut max_rock_count = 67;
	let mut part_1_result = 0;
	while max_rock_count > 1
	{
		println!("Old max rock count: {}", max_rock_count);
		
		let (height, offset, rock_count, previous_rock_count) = fill_cave(&jets, &rocks, max_rock_count);
		println!("height: {}     offset: {}     rock_count: {}     previous_rock_count: {}", height, offset, rock_count, previous_rock_count);

		part_1_result += std::cmp::max((max_rock_count-previous_rock_count) / (rock_count-previous_rock_count) - 1, 1) * (height) + offset + 1;
		max_rock_count = max_rock_count % rock_count;

		println!("New max rock count: {}", max_rock_count);
	}
	println!("Part 1: {}", part_1_result - 1);


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


/*
use std::collections::HashMap;
use std::collections::VecDeque;
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

fn
move_left
(
	buffer: &mut Vec<String>
)
{

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

	// Part 1
	let max_rock_count = 136;
	let mut rock_counter = 0u64;

	let mut cave: HashMap<Position, ECaveField> = HashMap::new();

	let mut jet_index: usize = 0;
	let mut rock_index: usize = 0;

	loop
	{
		println!("{}", rock_counter);

		if has_moving(&cave)
		{
			let jet = jets.chars().nth(jet_index).unwrap();
			jet_index += 1;
			jet_index = jet_index % jets.len();

			let mut x_offset = 0i64;

			// Push
			if jet == '<' // Left
			{
				let mut can_move_left = true;
				for field in cave.iter().filter(|(_, value)| **value == ECaveField::MovingRock)
				{
					let x = field.0.x;
					if x == 0
					{
						can_move_left = false;
						break;
					}
					let position_left = Position{x: x-1, y:field.0.y};
					if cave[&position_left] == ECaveField::Rock
					{
						can_move_left = false;
						break;
					}
				}

				if can_move_left { x_offset = -1; }
			}
			else if jet == '>' // Right
			{
				let mut can_move_right = true;
				for field in cave.iter().filter(|(_, value)| **value == ECaveField::MovingRock)
				{
					let x = field.0.x;
					if x == 6
					{
						can_move_right = false;
						break;
					}
					let position_right = Position{x: x+1, y:field.0.y};
					if cave[&position_right] == ECaveField::Rock
					{
						can_move_right = false;
						break;
					}
				}

				if can_move_right { x_offset = 1; }
			}
			else 
			{
				panic!("Illegal jet");	
			}
			let old_positions = cave.iter().filter(|(_, value)| **value == ECaveField::MovingRock).map(|(position, _)| position.clone()).collect::<Vec<Position>>();
			for old_position in &old_positions
			{
				cave.insert(old_position.clone(), ECaveField::Air);
			}
			for old_position in &old_positions
			{
				let new_position = Position {x: (old_position.x as i64 + x_offset) as u64, y: old_position.y};
				cave.insert(new_position.clone(), ECaveField::MovingRock);
			}

			// Falling

			// Check if everything below is free
			let mut can_fall = true;
			for field in cave.iter().filter(|(_, value)| **value == ECaveField::MovingRock)
			{
				let y = field.0.y;
				if y == 0
				{
					can_fall = false;
					break;
				}
				let position_below = Position{x: field.0.x, y:y-1};
				if cave[&position_below] == ECaveField::Rock
				{
					can_fall = false;
					break;
				}
			}

			let old_positions = cave.iter().filter(|(_, value)| **value == ECaveField::MovingRock).map(|(position, _)| position.clone()).collect::<Vec<Position>>();
			if can_fall
			{
				for old_position in &old_positions
				{
					cave.insert(old_position.clone(), ECaveField::Air);
				}
				for old_position in &old_positions
				{
					let new_position = Position {x: old_position.x, y: old_position.y -1};
					cave.insert(new_position.clone(), ECaveField::MovingRock);
				}
				trim_cave(&mut cave);
			}
			else 
			{
				for old_position in old_positions
				{
					cave.insert(old_position, ECaveField::Rock);
				}
			}

		}
		else
		{
			rock_counter += 1;
			if rock_counter >= max_rock_count
			{
				break;
			}

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

			rock_index += 1;
			rock_index = rock_index % rocks.len();
		}
	}

	// Print
	let max_y = cave.iter().map(|(pos, _)| pos.y).max().unwrap();
	for y in (0..=max_y).rev()
	{
		for x in 0..7
		{
			match cave[&Position{x:x, y:y}]
			{
				ECaveField::Air => print!("."),
				ECaveField::Rock => print!("#"),
				ECaveField::MovingRock => print!("@"),
			}
		}
		print!("\n");
	}
	print!("\n");

	let part_1_result = cave.iter().map(|(pos, _)| pos.y).max().unwrap() + 1;
	println!("Part 1: {}", part_1_result);


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
*/