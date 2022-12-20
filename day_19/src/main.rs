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

fn main() 
{
	let lines = read_string_data(
		std::path::Path::new("./data/example.txt")
	).unwrap();

	let max_minutes = 24;
	let mut quality_level_sum = 0;

	for line in &lines
	{
		let blueprint_id = line[10..].split(":").next().unwrap().parse::<i64>().unwrap();

		let mut ore_robot_costs: Vec::<i64> = Vec::new();
		let mut clay_robot_costs: Vec::<i64> = Vec::new();
		let mut obsidian_robot_costs: Vec::<i64> = Vec::new();
		let mut geode_robot_costs: Vec::<i64> = Vec::new();

		for (index, costs) in line.split("costs").enumerate()
		{
			println!("{}", costs.split(".").next().unwrap());
			match index
			{
				1 => {
					ore_robot_costs = vec![
						costs.split(".").next().unwrap().trim().split(" ").next().unwrap().parse::<i64>().unwrap(), 
						0, 
						0,
						0
					];
				},
				2 => {
					clay_robot_costs = vec![
						costs.split(".").next().unwrap().trim().split(" ").next().unwrap().parse::<i64>().unwrap(), 
						0, 
						0,
						0
					];
				},
				3 => {
					obsidian_robot_costs = vec![
						costs.split(".").next().unwrap().trim().split(" ").next().unwrap().parse::<i64>().unwrap(),
						costs.split(".").next().unwrap().trim().split("and ").last().unwrap().split("clay").next().unwrap().trim().parse::<i64>().unwrap(),
						0,
						0
					];
				},
				4 => {
					geode_robot_costs = vec![
						costs.split(".").next().unwrap().trim().split(" ").next().unwrap().parse::<i64>().unwrap(),
						0,
						costs.split(".").next().unwrap().trim().split("and ").last().unwrap().split("obsidian").next().unwrap().trim().parse::<i64>().unwrap(),
						0
					];
				},
				_ => ()
			}	
		}

		let costs = vec![ore_robot_costs, clay_robot_costs, obsidian_robot_costs, geode_robot_costs];
		println!("{:?}", costs);
		let mut robots = vec![1i64, 0, 0, 0];
		let mut resources = vec![0i64, 0, 0, 0];

		// Get result and insert it
		let blueprint_result = part_1_recusion(&costs, &robots, &robots,&resources, 1, max_minutes);
		println!("Result for blueprint {}: {}", blueprint_id, blueprint_result);
		
		quality_level_sum += blueprint_result * blueprint_id;

		break;
	}

	let part_1_result = quality_level_sum;
	println!("Part 1: {}", part_1_result);
	
}

fn
part_1_recusion
(
	costs: &Vec<Vec<i64>>,
	old_robots: &Vec<i64>,
	robots: &Vec<i64>,
	resources: &Vec<i64>,
	minute: i64,
	max_minutes: i64
)
-> i64
{

	// println!("{} {:?}", minute, resources);

	if minute > max_minutes
	{
		return resources[3];
	}

	let mut new_resources = resources.clone();

	for (robot_id, robot_count) in old_robots.iter().enumerate()
	{
		new_resources[robot_id] += robot_count;
	}

	println!("After minute {} you have {:?}", minute, new_resources);
	println!("{:?}", new_resources);
	println!("{:?}", costs);

	let mut results = Vec::new();
	for (robot_id, robot_costs) in costs.iter().enumerate()
	{
		// if robot_id != 3 && costs.iter().all(|c| c[robot_id] <= new_resources[robot_id] + robots[robot_id])
		// {
		// 	println!("Skipping robot {}", robot_id);
		// 	continue;
		// }
		if robot_id == 1
		{
			println!("{}", robots[robot_id]);
			if new_resources[0] + robots[0] >= costs[2][0] && new_resources[1] + robots[1] >= costs[2][1]
			{
				continue;
			}
		}

		let mut can_afford_robot = true;
		let mut new_resources_clone = new_resources.clone();
		for (cost_id, cost) in robot_costs.iter().enumerate()
		{
			new_resources_clone[cost_id] -= cost;
			if new_resources_clone[cost_id] < 0
			{
				can_afford_robot = false;
			}
		}

		if !can_afford_robot
		{
			continue;
		}

		let mut new_robots = robots.clone();
		new_robots[robot_id] += 1;

		results.push(part_1_recusion(costs, &robots, &new_robots, &new_resources_clone, minute+1, max_minutes));
	}
	if results.is_empty()
	{
		results.push(part_1_recusion(costs, &robots, &robots, &new_resources, minute+1, max_minutes));
	}

	return *results.iter().max().unwrap();
}