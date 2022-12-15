use std::collections::HashMap;
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
	Sensor,
	Beacon,
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
		// std::path::Path::new("./data/example.txt"),
        std::path::Path::new("./data/input.txt"),
	).unwrap();

    let part_1_y = 2000000;
    let part_2_min = 0;
    let part_2_max = 4000000;
    // let part_1_y = 10;
    
    // Preprocessing
    let mut closest_beacon = HashMap::new();
    let mut map = HashMap::new();
    for line in &lines
    {
        let (sensor, beacon) = parse_string(line);
        closest_beacon.insert(sensor.clone(), beacon.clone());
        map.insert(sensor, Field::Sensor);
        map.insert(beacon, Field::Beacon);
    }

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

    for (sensor, beacon) in &closest_beacon
    {
        let distance = manhatten_distance(&sensor, &beacon);
        min_x = std::cmp::min(min_x, sensor.x-distance);
        min_y = std::cmp::min(min_y, sensor.y-distance);
        max_x = std::cmp::max(max_x, sensor.x+distance);
        max_y = std::cmp::max(max_y, sensor.y+distance);
    }

    let min_position = Position {x: min_x, y: min_y};
	let max_position = Position {x: max_x, y: max_y};

    let mut part_1_map = map.clone();

    let part_1_closest_beacon = closest_beacon.clone();
    let part_2_closest_beacon = closest_beacon.clone();

    // println!("Checking 1...");
    // for (sensor, beacon) in part_1_closest_beacon
    // {
    //     let distance = manhatten_distance(&sensor, &beacon);
    //     println!("{:?}, {}", sensor, distance);

    //     for x in (sensor.x-distance-2)..(sensor.x+distance+2)
    //     {
    //         let position = Position {x: x, y: part_1_y};
    //         if manhatten_distance(&sensor, &position) <= distance
    //         {
    //             if !map.contains_key(&position)
    //             {
    //                 part_1_map.insert(position, Field::NoBeacon);
    //             }
    //         }
    //     }
    // }

    // let mut part_1_sum = 0;

    // for x in min_x..max_x+1
    // {
    //     let position = Position{x: x, y: part_1_y};
    //     if !part_1_map.contains_key(&position)
    //     {
    //         continue;
    //     }
    //     let value = part_1_map[&position];
    //     if value != Field::Unknown && value != Field::Beacon && value != Field::Sensor
    //     {
    //         part_1_sum += 1;
    //     }
    // }

    // println!("{}", part_1_sum);










    println!("Checking 2...");

    let mut candidates = HashMap::new();

    for (sensor, beacon) in &part_2_closest_beacon
    {
        let distance = manhatten_distance(&sensor, &beacon);
        println!("{:?}, {}", sensor, distance);

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

            new_position_1.y += (distance+1 - i);
            new_position_2.y -= (distance+1 - i);
            new_position_3.y += (distance+1 - i);
            new_position_4.y -= (distance+1 - i);


            if new_position_1.x >= part_2_min && new_position_1.x <= part_2_max
            {
                if new_position_1.y >= part_2_min && new_position_1.y <= part_2_max
                {
                    candidates.insert(new_position_1, Field::Unknown);
                }
                if new_position_2.y >= part_2_min && new_position_2.y <= part_2_max
                {
                    candidates.insert(new_position_2, Field::Unknown);
                }
            }

            if new_position_3.x >= part_2_min && new_position_3.x <= part_2_max
            {
                if new_position_3.y >= part_2_min && new_position_3.y <= part_2_max
                {
                    candidates.insert(new_position_3, Field::Unknown);
                }
                if new_position_4.y >= part_2_min && new_position_4.y <= part_2_max
                {
                    candidates.insert(new_position_4, Field::Unknown);
                }
            }
        }
    }

    let candidates_keys = candidates.keys().map(|x| x.clone()).collect::<Vec<Position>>();
    for (sensor, beacon) in &part_2_closest_beacon
    {
        for candidate in &candidates_keys
        {
            if candidates[&candidate] == Field::Unknown
            {
                let distance = manhatten_distance(sensor, beacon);
                let candidate_distance = manhatten_distance(sensor, candidate);
                if candidate_distance <= distance
                {
                    candidates.remove(candidate);
                }
            }
        }
    }

    for (candidate, field) in &candidates
    {
        if *field == Field::Unknown
        {
            println!("{:?}", candidate);
            println!("{}", candidate.x * 4000000 + candidate.y);
        }
    }

}
