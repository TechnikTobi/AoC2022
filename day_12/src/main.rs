use std::collections::VecDeque;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

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
}

fn
is_neighbour_part_1
(
	a: &Position,
	b: &Position
)
-> bool
{
	return (a.x - b.x).abs() + (a.y - b.y).abs() <= 1;
}

fn
is_neighbour_part_2
(
	a: &Position,
	b: &Position
)
-> bool
{
	if a.x == -1
	{
		return b.z == 1;
	}
	if b.x == -1
	{
		return a.z == 1;
	}
	return (a.x - b.x).abs() + (a.y - b.y).abs() <= 1;
}

fn
dijkstra
(
	start: &Position,
	end: &Position,
	positions_original: &VecDeque<Position>,
	neighbour_function: fn(&Position, &Position) -> bool,
)
-> i64
{
	let mut positions = positions_original.clone();
	let mut predecessor = HashMap::new();
	let mut distance = HashMap::new();

	for position in &positions
	{
		distance.insert(
			position.clone(),
			if position.equals(&start){ 0i64 } else { 100000000000i64 }
		);		
	}

	while !positions.is_empty()
	{
		let mut positions_vec = Vec::from_iter(positions);
		positions_vec.sort_by(|a,b| distance[a].cmp(&distance[b]));
		positions = VecDeque::from_iter(positions_vec);
		let u = positions.pop_front().unwrap();

		for position in &mut positions
		{
			if neighbour_function(&position, &u) && position.z <= (u.z + 1)
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
	while !predecessor[&u].equals(start)
	{
		u = predecessor[&u].clone();
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

	for (y, line) in lines.iter().enumerate()
	{
		for (x, elevation) in line.chars().enumerate()
		{
			let new_position_elevation = match elevation
			{
				'S' => 1,
				'E' => 26,
				elevation => elevation as i64 - 96
			};
			let new_position = Position 
			{
				x: x as i64,
				y: y as i64,
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
		}
	}



	// Part 1
	let mut positions_part_1 = positions.clone();

	let part_1_result = dijkstra(
		&start.clone(),
		&end.clone(),
		&mut positions_part_1,
		is_neighbour_part_1
	);

	println!("{}", part_1_result);



	// Part 2
	let mut positions_part_2 = positions.clone();
	start = Position { x: -1, y: -1, z: 100, visited: false, is_end: false};
	positions_part_2.push_front(start.clone());

	let part_2_result = dijkstra(
		&start.clone(),
		&end.clone(),
		&mut positions_part_2,
		is_neighbour_part_2
	) - 1; // Minus 1 due to the artifical start node

	println!("{}", part_2_result);

}
