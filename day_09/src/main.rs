use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct
Position
{
	x: i64,
	y: i64
}

impl 
Position
{
	fn
	add
	(
		&mut self,
		other: &Position
	)	
	{
		self.x += other.x;
		self.y += other.y;
	}

	fn
	distance
	(
		&self,
		other: &Position
	)
	-> i64
	{
		return (self.x - other.x).abs() + (self.y - other.y).abs();
	}
}

#[derive(Debug)]
struct
Move
{
	direction: char,
	count: i64
}


fn read_data
(
	path: &std::path::Path,
)
-> Result<Vec<Move>, Box<dyn std::error::Error>>
{	
	let file = File::open(path)?;
	let lines = BufReader::new(file).lines();
	let mut data = Vec::<Move>::new();

	for result_line in lines
	{
		let line = result_line?;
		let direction = line.chars().nth(0).unwrap();
		let count = line[2..].parse::<i64>().unwrap();
		data.push(Move {direction: direction, count: count} );
	}	

	return Ok(data);
}

fn 
get_tail_direction
(
	head: &Position,
	tail: &Position
)
-> Position
{
	if (head.x != tail.x) && (head.y != tail.y)
	{
		if head.distance(tail) <= 2
		{
			return Position {x: 0, y: 0};
		}
		if (head.x > tail.x) && (head.y > tail.y)
		{
			return Position {x: 1, y: 1};
		}
		if (head.x > tail.x) && (head.y < tail.y)
		{
			return Position {x: 1, y: -1};
		}
		if (head.x < tail.x) && (head.y > tail.y)
		{
			return Position {x: -1, y: 1};
		}
		if (head.x < tail.x) && (head.y < tail.y)
		{
			return Position {x: -1, y: -1};
		}
		panic!("AH");
	}
	else
	{
		
		if head.x == tail.x
		{
			return match (head.y - tail.y).signum()
			{
				1 => Position {x: 0, y: 1},
				-1 => Position {x: 0, y: -1},
				0 => Position {x: 0, y: 0},
				_ => panic!("AH")
			};
		}
		if head.x > tail.x
		{
			return Position {x: 1, y: 0};
		}
		return Position {x: -1, y: 0};
	}
	
	
	
}

fn main() 
{
	let moves = read_data(
		std::path::Path::new("./data/input.txt"),
	).unwrap();

	let iterations = 5000;

	// Part 1
	let mut visited_part_1_len = 0;

	let mut part_1_times = Vec::new();
	for _iteration in 0..iterations
	{

		let part_1_start = Instant::now();
		let start = Position {x: 0, y:0};
		let mut head = start.clone();
		let mut tail = start.clone();
		let mut visited = HashSet::<Position>::new();

		for m in &moves
		{
			let head_direction: Position;
			match m.direction 
			{
				'L' => { head_direction = Position {x: -1, y:  0} },
				'R' => { head_direction = Position {x:  1, y:  0} },
				'U' => { head_direction = Position {x:  0, y: -1} },
				'D' => { head_direction = Position {x:  0, y:  1} },
				other_char => panic!("UNKNOWN DIRECTION: {}", other_char),
			};

			for _ in 0..m.count
			{
				head.add(&head_direction);
				if head.distance(&tail) > 1
				{
					let tail_direction = get_tail_direction(&head, &tail);
					tail.add(&tail_direction);
				}
				visited.insert(tail.clone());
			}
		}

		visited_part_1_len = visited.len();

		let part_1_end = Instant::now();
		part_1_times.push(part_1_end.duration_since(part_1_start).as_micros());
	}

	part_1_times.sort();
	println!("---------- DAY: 09 - PART 1 ----------");
	println!("Result:     {}\n", visited_part_1_len);
	println!("Iterations: {}", iterations);
	println!("Mean:       {}µs", part_1_times.iter().sum::<u128>()/(part_1_times.len() as u128));
	println!("Median:     {}µs", part_1_times[part_1_times.len() / 2]);
	println!("Min:        {}µs", part_1_times[0]);
	println!("Max:        {}µs\n", part_1_times[part_1_times.len() -1]);	



	// Part 2
	let mut visited_part_2_len = 0;

	let mut part_2_times = Vec::new();
	for _iteration in 0..iterations
	{
		let part_2_start = Instant::now();
		let start = Position {x: 0, y:0};
		let knots_count = 10;
		let mut knots = Vec::<Position>::new();
		for _ in 0..knots_count
		{
			knots.push(start.clone());
		}
		let mut visited_part_2 = HashSet::<Position>::new();

		for m in &moves
		{
			let head_direction: Position;
			match m.direction 
			{
				'L' => { head_direction = Position {x: -1, y:  0} },
				'R' => { head_direction = Position {x:  1, y:  0} },
				'U' => { head_direction = Position {x:  0, y: -1} },
				'D' => { head_direction = Position {x:  0, y:  1} },
				_ => panic!("AH"),
			};

			for _ in 0..m.count
			{	
				knots[0].add(&head_direction);

				for i in 1..knots_count
				{
					if knots[i-1].distance(&knots[i]) > 1
					{
						let tail_direction = get_tail_direction(&knots[i-1], &knots[i]);
						knots[i].add(&tail_direction);
					}
					else 
					{
						break;
					}
				}

				visited_part_2.insert(knots[knots_count-1].clone());
			}
		}

		visited_part_2_len = visited_part_2.len();

		let part_2_end = Instant::now();

		part_2_times.push(part_2_end.duration_since(part_2_start).as_micros());
	}

	part_2_times.sort();
	println!("---------- DAY: 09 - PART 2 ----------");
	println!("Result:     {}\n", visited_part_2_len);
	println!("Iterations: {}", iterations);
	println!("Mean:       {}µs", part_2_times.iter().sum::<u128>()/(part_2_times.len() as u128));
	println!("Median:     {}µs", part_2_times[part_2_times.len() / 2]);
	println!("Min:        {}µs", part_2_times[0]);
	println!("Max:        {}µs", part_2_times[part_2_times.len() -1]);
}
