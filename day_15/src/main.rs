use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
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
	y: i64
}

#[derive(PartialEq, Clone, Copy)]
enum
Field
{
	NoBeacon,
	Unknown
}

fn 
parse_string
(
	data: &String
)
-> (Position, Position) // Sensor, closest beacon
{
	let mut temp = String::new();
	let mut values = Vec::<i64>::new();

	for (index, character) in data.chars().enumerate()
	{
		if character.is_digit(10) || character == '-'
		{
			temp.push(character);
		}
		
		if character == ',' || character == ':' || index == data.len()-1
		{
			values.push(temp.parse::<i64>().unwrap());
			temp.clear();
		}
	}

	let sensor = Position {x: values[0], y: values[1]};
	let beacon = Position {x: values[2], y: values[3]};

	println!("{:?}", sensor);
	println!("{:?}", beacon);

	return (sensor, beacon);
}

fn
manhatten_distance
(
	a: &Position,
	b: &Position
)
-> i64
{
	return (a.x - b.x).abs() + (a.y - b.y).abs();
}

fn main() 
{

	let lines = read_string_data(
		std::path::Path::new("./data/input.txt"),
	).unwrap();

	let part_1_y = 2000000;
	let part_2_min = 0;
	let part_2_max = 4000000;
	
	// Preprocessing
	let mut closest_beacon = HashMap::new();
	for line in &lines
	{
		let (sensor, beacon) = parse_string(line);
		closest_beacon.insert(sensor.clone(), beacon.clone());
	}

	// Compute the min/max values of the grid
	let mut min_x = i64::MAX;
	let mut min_y = i64::MAX;
	let mut max_x = i64::MIN;
	let mut max_y = i64::MIN;

	for (sensor, beacon) in &closest_beacon
	{
		let distance = manhatten_distance(&sensor, &beacon);
		min_x = std::cmp::min(min_x, sensor.x-distance);
		min_y = std::cmp::min(min_y, sensor.y-distance);
		max_x = std::cmp::max(max_x, sensor.x+distance);
		max_y = std::cmp::max(max_y, sensor.y+distance);
	}



	// Part 1
	
	// We only need to look at one row
	let mut y_row = (min_x..max_x+1).map(|_| Field::Unknown).collect::<Vec<Field>>();

	// For every sensor, compute the part of the row that overlaps with the 
	// sensor's manhatten circle
	for (sensor, beacon) in &closest_beacon
	{
		let distance = manhatten_distance(&sensor, &beacon);
		let distance_to_row = (sensor.y - part_1_y).abs();

		let one_sided_range = std::cmp::max(0,distance - distance_to_row);
		let offset = min_x.abs();
		let row_start = sensor.x - one_sided_range + offset;
		let row_end = sensor.x + one_sided_range + offset;

		// For all these fields in the row, we know that no beacon can be there
		// due to the overlap with the sensor's manhatten circle
		(row_start..row_end).for_each(|x| y_row[x as usize] = Field::NoBeacon);
	}

	println!("Part 1: {}", y_row.iter().filter(|&field| *field == Field::NoBeacon).count());



	// Part 2

	// The set of all candidate positions where the beacon in question might be located
	let mut candidates = HashSet::new();

	// For every sensor, add the border of its radius as candidates
	for (sensor, beacon) in &closest_beacon
	{
		let distance = manhatten_distance(&sensor, &beacon);

		// While doing that, remove previously added candidates that are within 
		// the radius of the current sensor
		candidates.retain(|candidate| manhatten_distance(sensor, candidate) > distance);

		// Add manhatten circle to the candidates
		for i in 0..distance+2
		{
			let mut new_position_1 = sensor.clone();
			let mut new_position_2 = sensor.clone();
			let mut new_position_3 = sensor.clone();
			let mut new_position_4 = sensor.clone();
			
			new_position_1.x += i;
			new_position_2.x += i;
			new_position_3.x -= i;
			new_position_4.x -= i;

			new_position_1.y += distance+1 - i;
			new_position_2.y -= distance+1 - i;
			new_position_3.y += distance+1 - i;
			new_position_4.y -= distance+1 - i;

			// Note that the x coordinates of some of the new positions are identical
			if new_position_1.x >= part_2_min && new_position_1.x <= part_2_max
			{
				if new_position_1.y >= part_2_min && new_position_1.y <= part_2_max
				{
					candidates.insert(new_position_1);
				}
				if new_position_2.y >= part_2_min && new_position_2.y <= part_2_max
				{
					candidates.insert(new_position_2);
				}
			}

			if new_position_3.x >= part_2_min && new_position_3.x <= part_2_max
			{
				if new_position_3.y >= part_2_min && new_position_3.y <= part_2_max
				{
					candidates.insert(new_position_3);
				}
				if new_position_4.y >= part_2_min && new_position_4.y <= part_2_max
				{
					candidates.insert(new_position_4);
				}
			}
		}
	}

	// Cleanup all candidates one final time
	for (sensor, beacon) in &closest_beacon
	{
		let distance = manhatten_distance(sensor, beacon);
		candidates.retain(|candidate| manhatten_distance(sensor, candidate) > distance);
	}

	// Compute tuning frequency of beacon that is in distress
	assert_eq!(candidates.len(), 1);
	for candidate in &candidates
	{
		println!("Part 2: {}", candidate.x * 4000000 + candidate.y);
	}

}
