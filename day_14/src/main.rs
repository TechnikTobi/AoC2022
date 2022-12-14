use std::collections::HashMap;
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

#[derive(PartialEq, Eq, Hash, Clone)]
struct 
Position
{
	x: i64,
	y: i64
}

#[derive(PartialEq, Clone, Copy)]
enum
Material
{
	Air,
	Rock,
	Sand,
	SandSource
}

fn parse_string
(
	data: &String
)
-> Vec<Position>
{
	let mut positions: Vec<Position> = Vec::new();
	for raw_position in data.split(" -> ")
	{
		let dimensions = raw_position.split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
		assert!(dimensions.len() == 2);

		let next_position = Position {x: dimensions[0], y: dimensions[1]};

		if !positions.is_empty()
		{
			let last_position = positions[positions.len()-1].clone();
		
			for y in 0..(last_position.y - next_position.y).abs() +1
			{
				for x in 0..(last_position.x - next_position.x).abs() +1
				{
					positions.push(Position { 
						x: std::cmp::min(last_position.x, next_position.x) + x,
						y: std::cmp::min(last_position.y, next_position.y) + y 
					});
				}
			}	
			
		}

		positions.push(next_position);
	}
	return positions;
}

fn main() 
{
	let lines = read_string_data(
		std::path::Path::new("./data/input.txt"),
	).unwrap();

	// Preprocessing
	let mut map = HashMap::new();
	for line in &lines
	{
		for position in parse_string(line)
		{
			map.insert(position, Material::Rock);
		}
	}
	map.insert(Position { x: 500, y: 0 }, Material::SandSource);

	let mut min_x = i64::MAX;
	let mut min_y = i64::MAX;
	let mut max_x = i64::MIN;
	let mut max_y = i64::MIN;

	for position in map.keys()
	{
		min_x = std::cmp::min(min_x, position.x);
		min_y = std::cmp::min(min_y, position.y);
		max_x = std::cmp::max(max_x, position.x);
		max_y = std::cmp::max(max_y, position.y);
	}

	let mut min_position = Position {x: min_x-1, y: min_y+0};
	let mut max_position = Position {x: max_x+1, y: max_y+1};

	// Fill rest with air
	for y in min_position.y..max_position.y+1
	{
		for x in min_position.x..max_position.x+1
		{
			let position = Position {x: x, y: y};
			if !map.contains_key(&position)
			{
				map.insert(position, Material::Air);
			}
		}
	}
	let mut map_part_2 = map.clone();

	

	// Part 1
	let mut current_moving_sand_position = Position {x: 500, y: 1};

	loop
	{
		let below = Position {x: current_moving_sand_position.x, y: current_moving_sand_position.y+1 };
		let left = Position {x: below.x-1, y: below.y };
		let right = Position {x: left.x+2, y: left.y };
		
		if (map[&below] == Material::Air)
		{
			current_moving_sand_position.y += 1;
		}
		else if (map[&left] == Material::Air)
		{
			current_moving_sand_position.x -= 1;
			current_moving_sand_position.y += 1;
		}
		else if (map[&right] == Material::Air)
		{
			current_moving_sand_position.x += 1;
			current_moving_sand_position.y += 1;
		}
		else
		{
			map.insert(current_moving_sand_position.clone(), Material::Sand);
			current_moving_sand_position = Position {x: 500, y: 1};
		}

		if current_moving_sand_position.y >= max_y
		{
			break;
		}
		
	}

	for y in min_position.y..max_position.y+1
	{
		for x in min_position.x..max_position.x+1
		{
			let position = Position {x: x, y: y};
			if position == current_moving_sand_position
			{
				print!("o");
			}
			else
			{
				match map[&position]
				{
					Material::Air => print!("."),
					Material::Rock => print!("#"),
					Material::Sand => print!("o"),
					Material::SandSource => print!("+")
				}
			}
		}
		print!("\n");
	}
	print!("\n");

	let part_1_result = map.iter().filter(|(_, &material)| material == Material::Sand).count();
	println!("{}", part_1_result);
	



	// Part 2
	// Fill in the floor
	for x in (min_x-max_y)..(max_x+max_y+1)
	{
		let floor_position = Position { x: x, y: max_y + 2 };
		map_part_2.insert(floor_position, Material::Rock);
	}

	// New max position
	min_position = Position {x: min_x-max_y-1, y: min_y+0};
	max_position = Position {x: max_x+max_y+1, y: max_y+3};

	// Refill with air
	for y in min_position.y..max_position.y+1
	{
		for x in min_position.x..max_position.x+1
		{
			let position = Position {x: x, y: y};
			if !map_part_2.contains_key(&position)
			{
				map_part_2.insert(position, Material::Air);
			}
		}
	}

	let mut current_moving_sand_position = Position {x: 500, y: 0};

	loop
	{
		let below = Position {x: current_moving_sand_position.x, y: current_moving_sand_position.y+1 };
		let left = Position {x: below.x-1, y: below.y };
		let right = Position {x: left.x+2, y: left.y };
		
		if (map_part_2[&below] == Material::Air)
		{
			current_moving_sand_position.y += 1;
		}
		else if (map_part_2[&left] == Material::Air)
		{
			current_moving_sand_position.x -= 1;
			current_moving_sand_position.y += 1;
		}
		else if (map_part_2[&right] == Material::Air)
		{
			current_moving_sand_position.x += 1;
			current_moving_sand_position.y += 1;
		}
		else
		{
			if map_part_2[&current_moving_sand_position] == Material::Sand
			{
				break;
			}
			map_part_2.insert(current_moving_sand_position.clone(), Material::Sand);
			current_moving_sand_position = Position {x: 500, y: 0};
		}

		if current_moving_sand_position.y >= max_y + 2
		{
			break;
		}

		// for y in min_position.y..max_position.y+1
		// {
		// 	for x in min_position.x..max_position.x+1
		// 	{
		// 		let position = Position {x: x, y: y};
		// 		if position == current_moving_sand_position
		// 		{
		// 			print!("o");
		// 		}
		// 		else
		// 		{
		// 			match map_part_2[&position]
		// 			{
		// 				Material::Air => print!("."),
		// 				Material::Rock => print!("#"),
		// 				Material::Sand => print!("o"),
		// 				Material::SandSource => print!("+")
		// 			}
		// 		}
		// 	}
		// 	print!("\n");
		// }
		// print!("\n");
		
	}

	for y in min_position.y..max_position.y+1
	{
		for x in min_position.x..max_position.x+1
		{
			let position = Position {x: x, y: y};
			if position == current_moving_sand_position
			{
				print!("o");
			}
			else
			{
				match map_part_2[&position]
				{
					Material::Air => print!("."),
					Material::Rock => print!("#"),
					Material::Sand => print!("o"),
					Material::SandSource => print!("+")
				}
			}
		}
		print!("\n");
	}
	print!("\n");

	let part_2_result = map_part_2.iter().filter(|(_, &material)| material == Material::Sand).count();
	println!("{}", part_2_result);

}
