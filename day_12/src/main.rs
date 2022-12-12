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

	fn
	distance_to
	(
		&self,
		other: &Position
	)
	-> i64
	{
		return (self.x - other.x).abs() + (self.y - other.y).abs();	
	}

	fn
	distance_to_nearest_heigher
	(
		&self,
		positions: &Vec<Position>
	)
	-> i64
	{
		let mut distance = positions.len() as i64;
		for other in positions.iter().filter(|&x| x.z == self.z + 1)
		{
			distance = std::cmp::min(distance, self.distance_to(other));
		}
		return distance;
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
	let mut end = Position { x: -1, y: -1, z: -1, visited: false, is_end: false};

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
				visited: elevation == 'S',
				is_end: elevation == 'E',
			};
			if elevation == 'S'
			{
				start = new_position.clone();
			}
			if elevation == 'E'
			{
				end = new_position.clone();
			}
			positions.push(new_position);
			x += 1;
		}
		y += 1;
	}

	let width = lines[0].len() as i64;
	let height = lines.len() as i64;

	positions.sort_by(|a, b| b.z.cmp(&a.z));
	let mut current = start.clone();
	let mut last_good_current = VecDeque::<Position>::new();
	last_good_current.push_back(current.clone());

	let mut visited = 0;

	let mut counter = 0;

	while !current.is_end
	{

		counter += 1;
		if counter % 100 == 0
		{
			for i in 0..height
			{
				for j in 0..width
				{
					// if positions.iter().filter(|&x| x.x == j && x.y == i).next().unwrap().visited
					// {
					// 	print!("#");
					// }
					// else
					// {
					// 	print!(" ");
					// }
					if let Some(pos) = last_good_current.iter().filter(|&x| x.x == j && x.y == i).next()
			{
				if pos.visited
				{
					print!("#");
				}
			}
			else
			{
				print!(" ");
			}
				}
				print!("\n");
			}
			print!("\n");
		}


		visited += 1;
		println!("Visiting {:?}", current);
		let mut options = positions
			.iter()
			.filter(|&x| x.is_neighbour(&current))
			.filter(|&x| !x.visited)
			.filter(|&x| x.z - current.z <= 1)
			.map(|x| x.clone())
			.collect::<Vec<Position>>();
		
		if options.is_empty()
		{
			current = last_good_current.pop_back().unwrap();
			visited -= 1;
		}
		else
		{
			// options.sort_by(|a, b| a.distance_to(&end).cmp(&b.distance_to(&end)));
			// options.sort_by(|a,)
			// options.sort_by(|a, b| a.distance_to(&end).cmp(&b.distance_to(&end)));
			// options.sort_by(|a, b| b.x.cmp(&a.x));
			// options.sort_by(|a, b| b.z.cmp(&a.z));
			options.sort_by(|a, b| a.distance_to_nearest_heigher(&positions).cmp(&b.distance_to_nearest_heigher(&positions)));
			if current.x < 2 
			{
				options.sort_by(|a, b| b.y.cmp(&a.y));	
			}
			// if (current.x > 7 && current.x < 11)
			// {
			// 	options.sort_by(|a, b| b.y.cmp(&a.y));	
			// }
			options.sort_by(|a, b| b.z.cmp(&a.z));
			current = options[0].clone();
			last_good_current.push_back(current.clone());

			for mut position in &mut positions
			{
				if position.equals(&current)
				{
					position.visited = true;
				}
			}
		}
	}

	for i in 0..height
	{
		for j in 0..width
		{
			if let Some(pos) = last_good_current.iter().filter(|&x| x.x == j && x.y == i).next()
			{
				print!("#");	
			}
			else
			{
				print!(" ");
			}
		}
		print!("\n");
	}
	print!("\n");


	// let a = recursive_solve(
	// 	&start,
	// 	&mut positions
	// );

	println!("{}", last_good_current.len());
	println!("{}", visited);
}
