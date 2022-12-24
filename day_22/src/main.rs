use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

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

#[derive(Debug, Clone)]
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
				EDirection::North => EDirection::East,
				EDirection::East => EDirection::South,
				EDirection::South => EDirection::West,
				EDirection::West => EDirection::North,
			}
		}
		else if rotation == 'L'
		{
			match self
			{
				EDirection::North => EDirection::West,
				EDirection::West => EDirection::South,
				EDirection::South => EDirection::East,
				EDirection::East => EDirection::North
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

	println!("{} {}", max_x, max_y);

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

	// Part 1
	// {
	// 	let mut current_position_part_1 = Position {x: start_x as i64, y: start_y as i64};
	// 	let mut current_direction_part_1 = EDirection::East;

	// 	// Storing digits while parsing the instructions	
	// 	let mut temp = String::new();
	// 	for character in instructions.chars()
	// 	{
	// 		if character.is_ascii_digit()
	// 		{
	// 			temp.push(character);
	// 		}
	// 		else
	// 		{
	// 			let distance = temp.parse::<i64>().unwrap();

	// 			current_position_part_1 = board_move_part_1(&board_map, &current_position_part_1, &current_direction_part_1, distance, max_x as i64, max_y as i64);
	// 			current_direction_part_1 = current_direction_part_1.rotate(character);

	// 			temp.clear();
	// 		}
	// 	}

	// 	// For any remaining data in temp
	// 	let distance = temp.parse::<i64>().unwrap();
	// 	current_position_part_1 = board_move_part_1(&board_map, &current_position_part_1, &current_direction_part_1, distance, max_x as i64, max_y as i64);

	// 	let part_1_result = 1000 * (current_position_part_1.y+1) + 4 * (current_position_part_1.x+1) + match current_direction_part_1 {
	// 		EDirection::East => 0,
	// 		EDirection::South => 1,
	// 		EDirection::West => 2,
	// 		EDirection::North => 3
	// 	};

	// 	println!("Part 1: {}", part_1_result);
	// }




	{
		let mut current_position_part_2 = Position {x: start_x as i64, y: start_y as i64};
		let mut current_direction_part_2 = EDirection::East;

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

				(current_position_part_2, current_direction_part_2) = board_move_part_2(&board_map, &current_position_part_2, &current_direction_part_2, distance, max_x as i64, max_y as i64, 50);
				current_direction_part_2 = current_direction_part_2.rotate(character);

				temp.clear();
			}
		}

		// For any remaining data in temp
		let distance = temp.parse::<i64>().unwrap();
		(current_position_part_2, current_direction_part_2) = board_move_part_2(&board_map, &current_position_part_2, &current_direction_part_2, distance, max_x as i64, max_y as i64, 50);

		let part_2_result = 1000 * (current_position_part_2.y+1) + 4 * (current_position_part_2.x+1) + match current_direction_part_2 {
			EDirection::East => 0,
			EDirection::South => 1,
			EDirection::West => 2,
			EDirection::North => 3
		};
		println!("Part 2: {}", part_2_result);
	}


}

fn
board_move_part_1
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
		EDirection::West => (-1, 0),
		EDirection::South => (0, 1),
		EDirection::East => (1, 0),
	};

	let mut new_position = current_position.clone();
	
	for _ in 0..distance
	{
		let mut new_position_candidate = new_position.clone();
		
		while board_map[&new_position_candidate] == EField::Teleport || new_position_candidate == new_position
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
	}

	return new_position;
}


fn
board_move_part_2
(
	board_map: &HashMap<Position, EField>,
	current_position: &Position,
	current_direction: &EDirection,
	distance: i64,
	max_x: i64,
	max_y: i64,
	cube_side_length: i64,
)
-> (Position, EDirection)
{

	// println!("STARTING MOVE {} {:?}", distance, current_direction);
	println!("Starting move from: {:?}, now heading: {:?}", current_position, current_direction);

	let mut direction = current_direction.clone();

	let mut new_position = current_position.clone();
	
	for _ in 0..distance
	{
		let (delta_x, delta_y) = match direction
		{
			EDirection::North => (0i64, -1i64),
			EDirection::West => (-1, 0),
			EDirection::South => (0, 1),
			EDirection::East => (1, 0),
		};

		// All boards have same shape
		// See: https://www.reddit.com/r/adventofcode/comments/zsgbe7/2022_day_22_question_about_your_input/?utm_source=share&utm_medium=web2x&context=3
		// Shape:
		//    _ _
		//   |1|2|
		//  _|3|
		// |4|5|
		// |6| 

		// Sides 6 and 3 are parallel
		// View for the different sides:

		//    4
		//  1 6 5
		//    2

		let mut direction_change = false;

		let mut new_x = new_position.x + delta_x;
		let mut new_y = new_position.y + delta_y;

		let mut new_direction_candidate = direction.clone();

		if new_position.y >= 3 * cube_side_length                               // Side 6
		{
			if new_x < 0                                          // Going to 1N
			{
				new_x = new_position.y - 2*cube_side_length;
				new_y = 0;
				new_direction_candidate = EDirection::South;
				direction_change = true;
			}
			else if new_x >= cube_side_length                     // Going to 5S
			{
				new_x = new_position.y - 2*cube_side_length;
				new_y = 3*cube_side_length - 1;
				new_direction_candidate = EDirection::North;
				direction_change = true;
			}
			else if new_y >= 4 * cube_side_length                 // Going to 2N
			{
				new_x = new_position.x + 2*cube_side_length;
				new_y = 0;
				new_direction_candidate = EDirection::South;
				direction_change = true;
			}
			else {}
		}
		else if new_position.x < cube_side_length                               // Side 4
		{
			if new_x < 0                                          // Going to 1W (upside down!)
			{
				new_x = cube_side_length;
				new_y = cube_side_length - (new_position.y - 2*cube_side_length) - 1;
				new_direction_candidate = EDirection::East;
				direction_change = true;
			}
			else if new_y < 2*cube_side_length                    // Going to 3W
			{
				new_x = cube_side_length;
				new_y = new_position.x + cube_side_length;
				new_direction_candidate = EDirection::East;
				direction_change = true;
			}
			else {}
		}
		else if new_position.y >= 2*cube_side_length                            // Side 5
		{
			if new_x >= 2*cube_side_length                        // Going to 2E (upside down!)
			{
				new_x = 3 * cube_side_length -1;
				new_y = cube_side_length - (new_position.y - 2*cube_side_length) - 1;
				new_direction_candidate = EDirection::West;
				direction_change = true;
			}
			else if new_y >= 3 * cube_side_length                 // Going to 6E
			{
				new_x = cube_side_length -1;
				new_y = new_position.x + 2*cube_side_length;
				new_direction_candidate = EDirection::West;
				direction_change = true;
			}
			else {}
		}
		else if new_position.y >= cube_side_length                              // Side 3
		{
			if new_x < cube_side_length                           // Going to 4N
			{
				new_x = new_position.y - cube_side_length;
				new_y = 2*cube_side_length;
				new_direction_candidate = EDirection::South;
				direction_change = true;
			}
			else if new_x >= 2*cube_side_length                   // Going to 2S
			{
				new_x = new_position.y + cube_side_length;
				new_y = cube_side_length -1;
				new_direction_candidate = EDirection::North;
				direction_change = true;
			}
			else {}
		}
		else if new_position.x >= 2*cube_side_length                            // Side 2
		{
			if new_x >= 3 * cube_side_length                      // Going to 5E (upside down!)
			{
				new_x = 2*cube_side_length -1;
				new_y = 2*cube_side_length + (cube_side_length - new_position.y -1);
				new_direction_candidate = EDirection::West;
				direction_change = true;
			}
			else if new_y < 0                                     // Going to 6S
			{
				new_x = new_position.x - 2*cube_side_length;
				new_y = 4*cube_side_length - 1;
				new_direction_candidate = EDirection::North;
				direction_change = true;
			}
			else if new_y >= cube_side_length                     // Going to 3E
			{
				new_x = 2*cube_side_length -1;
				new_y = new_position.x - cube_side_length;
				new_direction_candidate = EDirection::West;
				direction_change = true;
			}
			else {}
		}
		else                                                                    // Side 1
		{
			if new_x < cube_side_length                           // Going to 4W (upside down!)
			{
				new_x = 0; 
				new_y = 2*cube_side_length + (cube_side_length - new_position.y -1);
				new_direction_candidate = EDirection::East;
				direction_change = true;
			}
			else if new_y < 0                                     // Going to 6W
			{
				new_x = 0;
				new_y = new_position.x + 2*cube_side_length;
				new_direction_candidate = EDirection::East;
				direction_change = true;
			}
			else {}
		}

		let new_position_candidate =  Position
		{
			x: new_x,
			y: new_y,
		};

		if direction_change == true || board_map[&new_position_candidate] == EField::Teleport
		{
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

			// for y in 0..max_y
			// {
			// 	for x in 0..max_x
			// 	{
			// 		let position = Position {x: x as i64, y: y as i64};
			// 		if position == new_position_candidate
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

			// println!("new_position_candidate: {:?}", new_position_candidate);
			if board_map[&new_position_candidate] == EField::Teleport{
				panic!("You were not supposed to do that!");
			}
			
		}

		if board_map[&new_position_candidate] == EField::Wall
		{
			break;
		}

		new_position = new_position_candidate;
		direction = new_direction_candidate;
		// println!("new position: {:?}", new_position);

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

	return (new_position, direction);
}
