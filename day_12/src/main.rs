use std::collections::VecDeque;
use std::collections::HashMap;
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


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
dijkstra
(
	start: &Position,
	end: &Position,
	positions_original: &VecDeque<Position>
)
-> i64
{
	let mut positions = positions_original.clone();
	let mut predecessor = HashMap::new();
	let mut distance = HashMap::new();

	for position in &positions
	{
		if position.equals(start)
		{
			distance.insert(
				start.clone(),
				0
			);
		}
		else
		{
			distance.insert(
				position.clone(),
				100000000000i64
			);
		}
	}

	while !positions.is_empty()
	{
		let mut positions_vec = Vec::from_iter(positions);
		positions_vec.sort_by(|a,b| distance[a].cmp(&distance[b]));
		positions = VecDeque::from_iter(positions_vec);
		let u = positions.pop_front().unwrap();

		for mut position in &mut positions
		{
			if position.is_neighbour(&u) && position.z <= (u.z + 1)
			{
				let alternative = distance[&u] + 1;
				if alternative < distance[&position]
				{
					distance.insert(position.clone(), alternative);
					predecessor.insert(position.clone(), u.clone());
				}
			}
		}
	}

	let mut path = VecDeque::new();
	path.push_front(end.clone());
	let mut u = end.clone();
	if !predecessor.contains_key(&u)
	{
		return 1000000000i64;
	}
	while predecessor[&u] != *start
	{
		u = predecessor[&u].clone();
		if !predecessor.contains_key(&u)
		{
			return 1000000000i64;
		}
		path.push_front(u.clone());
	}

	return path.len() as i64;
}

fn main() 
{
	// Preprocessing
	let lines = read_string_data(
		std::path::Path::new("./data/input.txt"),
	).unwrap();

	let mut positions = VecDeque::new();
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
			positions.push_back(new_position);
			x += 1;
		}
		y += 1;
	}

	
	let part_1_result = dijkstra(
		&start.clone(),
		&end.clone(),
		&mut positions.clone()
	);





	println!("{}", part_1_result);



	let mut min_distance = 10000000000i64;
	let mut counter = 0;
	for position in &positions
	{
		if position.z == 1
		{
			counter += 1;
			println!("{}", counter);
			let new_distance = dijkstra(
				&position.clone(),
				&end.clone(),
				&mut positions.clone()
			);
			min_distance = std::cmp::min(min_distance, new_distance);
		}
	}

	println!("{}", min_distance);








}
