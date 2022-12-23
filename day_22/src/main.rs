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

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct
Position
{
	x: i64,
	y: i64,
}

#[derive(PartialEq, Eq)]
enum 
EField 
{
	Teleport,
	Empty,
	Wall
}

#[derive(Debug)]
enum 
EDirection 
{
	North,
	East,
	South,
	West
}

impl
EDirection
{
	fn
	rotate
	(
		&self,		
		rotation: char
	)
	-> EDirection
	{
		return if rotation == 'R'
		{
			match self
			{
				EDirection::North => EDirection::West,
				EDirection::West => EDirection::South,
				EDirection::South => EDirection::East,
				EDirection::East => EDirection::North
			}
			
		}
		else if rotation == 'L'
		{
			match self
			{
				EDirection::North => EDirection::East,
				EDirection::East => EDirection::South,
				EDirection::South => EDirection::West,
				EDirection::West => EDirection::North,
			}
		}
		else
		{
			panic!("AH")
		};
	}
}

fn main() 
{
	let lines = read_string_data(
		std::path::Path::new("./data/input.txt")
	).unwrap();

	// Ignore empty line and line with move instructions
	let max_y = lines.len()-2;
	let max_x = lines.iter().enumerate().filter(|(line_index, _)| line_index <= &max_y).map(|(_, line)| line.len()).max().unwrap();

	// Move instructions
	let instructions = lines.last().unwrap();

	let mut board_map = HashMap::new();
	for y in 0..max_y
	{
		let line = lines[y].clone();
		for x in 0..max_x
		{
			let value: EField;
			if x >= line.len()
			{
				value = EField::Teleport;
			}
			else
			{
				value = match line.chars().nth(x).unwrap()
				{
					' ' => EField::Teleport,
					'.' => EField::Empty,
					'#' => EField::Wall,
					_ => panic!("AH2"),
				};
			}
			board_map.insert(Position {x: x as i64, y: y as i64}, value);
		}
	}

	let start_x = lines[0]
		.chars()
		.enumerate()
		.filter(|(_, ch)| ch == &'.')
		.map(|(index, _)| index)
		.min()
		.unwrap();
	let start_y = 0;

	let mut current_position = Position {x: start_x as i64, y: start_y as i64};
	let mut current_direction = EDirection::West;

	// Storing digits while parsing the instructions	
	let mut temp = String::new();
	for character in instructions.chars()
	{
		if character.is_ascii_digit()
		{
			temp.push(character);
		}
		else
		{
			let distance = temp.parse::<i64>().unwrap();
			current_position = board_move(&board_map, &current_position, &current_direction, distance, max_x as i64, max_y as i64);
			current_direction = current_direction.rotate(character);
			temp.clear();
		}
	}

	// For any remaining data in temp
	let distance = temp.parse::<i64>().unwrap();
	current_position = board_move(&board_map, &current_position, &current_direction, distance, max_x as i64, max_y as i64);

	let part_1_result = 1000 * (current_position.y+1) + 4 * (current_position.x+1) + match current_direction {
		EDirection::West => 0,
		EDirection::South => 1,
		EDirection::East => 2,
		EDirection::North => 3
	};

	println!("Part 1: {}", part_1_result);

}

fn
board_move
(
	board_map: &HashMap<Position, EField>,
	current_position: &Position,
	current_direction: &EDirection,
	distance: i64,
	max_x: i64, 
	max_y: i64,
)
-> Position
{
	let (delta_x, delta_y) = match current_direction
	{
		EDirection::North => (0i64, -1i64),
		EDirection::West => (1, 0),
		EDirection::South => (0, 1),
		EDirection::East => (-1, 0),
	};

	let mut new_position = current_position.clone();
	
	for _ in 0..distance
	{
		let mut new_position_candidate = Position
		{
			x: (new_position.x + delta_x + max_x) % max_x,
			y: (new_position.y + delta_y + max_y) % max_y
		};

		// Teleport to the next field on the board
		while board_map[&new_position_candidate] == EField::Teleport
		{
			new_position_candidate =  Position
			{
				x: (new_position_candidate.x + delta_x + max_x) % max_x,
				y: (new_position_candidate.y + delta_y + max_y) % max_y
			};
		}

		if board_map[&new_position_candidate] == EField::Empty
		{
			new_position = new_position_candidate;
		}

		// for y in 0..max_y
		// {
		// 	for x in 0..max_x
		// 	{
		// 		let position = Position {x: x as i64, y: y as i64};
		// 		if position == new_position
		// 		{
		// 			print!("█");
		// 		}
		// 		else
		// 		{
		// 			print!("{}", match board_map[&position] { EField::Teleport => "_", EField::Empty => ".", EField::Wall => "#"});
		// 		}
		// 	}
		// 	print!("\n");
		// }
		// print!("\n");
	}

	// println!("FINISHED MOVE: {:?}", new_position);

	return new_position;
}
