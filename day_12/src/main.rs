use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

#[allow(dead_code)]
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


#[derive(Debug, Clone)]
struct
Position
{
	x: i64,
	y: i64,
	z: i64, // elevation
	visited: bool,
	is_end: bool
}

impl
Position
{
	fn
	equals
	(
		&self,
		other: &Position
	)
	-> bool
	{
		return
		(self.x == other.x) &&
		(self.y == other.y) &&
		(self.z == other.z);
	}

	fn
	is_neighbour
	(
		&self,
		other: &Position
	)
	-> bool
	{
		return (self.x - other.x).abs() + (self.y - other.y).abs() <= 1;
	}
}

fn
recursive_solve
(
	current: &Position,
	positions: &mut Vec<Position>
)
-> (Vec<Position>, usize)
{
	let mut positions_clone = positions.clone();
	println!("{}", positions_clone.len());

	for position in positions
	{
		if !position.is_neighbour(current)
		{
			continue;
		}

		if position.equals(current)
		{
			continue;
		}

		if (position.z - current.z).abs() > 1
		{
			continue;
		}

		if position.visited
		{
			continue;
		}

		if position.is_end
		{
			return (positions_clone.to_vec(), positions_clone.iter().filter(|&x| x.visited).count());
		}

		for clone in &mut positions_clone
		{
			if clone.equals(&position)
			{
				clone.visited = true;
			}
		}

		// println!("Visiting {:?}", position);

		let (result_vec, result) = recursive_solve(&position, &mut positions_clone);
		if result > 0
		{
			return (result_vec.to_vec(), result);
		}

		for clone in &mut positions_clone
		{
			for other in &result_vec
			{
				if clone.equals(&other) && other.visited
				{
					clone.visited = true;
				}
			}
		}
	}
	return (positions_clone.to_vec(), 0);
}

fn main() 
{
	// Preprocessing
	let lines = read_string_data(
		std::path::Path::new("./data/input.txt"),
	).unwrap();

	let mut positions = Vec::new();
	let mut start = Position { x: -1, y: -1, z: -1, visited: false, is_end: false};

	let mut y = 0;
	for line in &lines
	{
		let mut x = 0;
		for elevation in line.chars()
		{
			let new_position_elevation = match elevation
			{
				'S' => 1,
				'E' => 26,
				elevation => elevation as i64 - 96
			};
			let new_position = Position 
			{
				x: x,
				y: y,
				z: new_position_elevation,
				visited: false,
				is_end: elevation == 'E',
			};
			if elevation == 'S'
			{
				start = new_position.clone();
			}
			positions.push(new_position);
			x += 1;
		}
		y += 1;
	}

	positions.sort_by(|a, b| b.z.cmp(&a.z));

	let a = recursive_solve(
		&start,
		&mut positions
	);

	println!("{}", a.1-1);
}
