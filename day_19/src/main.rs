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

fn
parse_blueprint
(
	raw_blueprint: &String
)
-> Vec<Vec<i64>>
{
	let mut ore_robot_costs: Vec::<i64> = Vec::new();
	let mut clay_robot_costs: Vec::<i64> = Vec::new();
	let mut obsidian_robot_costs: Vec::<i64> = Vec::new();
	let mut geode_robot_costs: Vec::<i64> = Vec::new();

	for (index, costs) in raw_blueprint.split("costs").enumerate()
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

	return vec![ore_robot_costs, clay_robot_costs, obsidian_robot_costs, geode_robot_costs];	
}

fn main() 
{
	let lines = read_string_data(
		std::path::Path::new("./data/input.txt")
	).unwrap();

	let max_minutes_part_1 = 24;
	let max_minutes_part_2 = 32;
	let mut quality_level_sum = 0;

	for line in &lines
	{
		let blueprint_id = line[10..].split(":").next().unwrap().parse::<i64>().unwrap();

		let costs = parse_blueprint(&line);
		let mut robots = vec![0i64, 0, 0, 0];
		let mut resources = vec![costs[0][0].clone(), 0, 0, 0];

		// Get result
		let blueprint_state = ProblemState { costs, robots, next_robot_index: 0, resources, minute: 0, max_minutes: max_minutes_part_1 }; 
		let blueprint_result = part_1(blueprint_state);
		println!("Result for blueprint {}: {}", blueprint_id, blueprint_result);
		
		quality_level_sum += blueprint_result * blueprint_id;
	}

	let part_1_result = quality_level_sum;
	println!("Part 1: {}\n", part_1_result);


	let mut part_2_result = 1;
	for (line_index, line) in lines.iter().enumerate()
	{
		if line_index > 2
		{
			break;
		}

		let blueprint_id = line[10..].split(":").next().unwrap().parse::<i64>().unwrap();

		let costs = parse_blueprint(&line);
		let mut robots = vec![0i64, 0, 0, 0];
		let mut resources = vec![costs[0][0].clone(), 0, 0, 0];

		// Get result
		let blueprint_state = ProblemState { costs, robots, next_robot_index: 0, resources, minute: 0, max_minutes: max_minutes_part_2 }; 
		let blueprint_result = part_1(blueprint_state);
		println!("Result for blueprint {}: {}", blueprint_id, blueprint_result);
		part_2_result *= blueprint_result;
	}

	println!("Part 2: {}", part_2_result);
	
}


#[derive(Clone)]
struct
ProblemState
{
	costs: Vec<Vec<i64>>,
	robots: Vec<i64>,
	next_robot_index: usize,
	resources: Vec<i64>,
	minute: i64,
	max_minutes: i64 
}

fn
part_1
(
	mut state: ProblemState
)
-> i64
{

	// println!("{} {:?}", state.minute, state.resources);

	let mut new_robot_constructed = false;
	while !new_robot_constructed && state.minute <= state.max_minutes
	{
		// Check for invalid robot indices
		if ![0, 1, 2, 3].contains(&state.next_robot_index)
		{
			panic!("Unknown robot index")
		}

		// Check if we can build the next robot
		if state.resources.iter().enumerate().all(|(resource_index, resource)| resource >= &state.costs[state.next_robot_index][resource_index])
		{
			for (cost_index, cost) in state.costs[state.next_robot_index].iter().enumerate()
			{
				state.resources[cost_index] -= cost;
			}
			new_robot_constructed = true;	
		}

		// Mine resources
		for (robot_id, robot_count) in state.robots.iter().enumerate()
		{
			state.resources[robot_id] += robot_count;
		}

		// Advance time
		state.minute += 1;

		// Construct the new robot
		if new_robot_constructed
		{
			state.robots[state.next_robot_index] += 1;
		}
	}

	let mut geodes = state.resources[3];
	if state.minute <= state.max_minutes
	{
		for new_next_robot_index in [0, 1, 2, 3]
		{
			// Don't build obsidian robots when we don't even have yet any clay robots
			if new_next_robot_index == 2 && state.robots[1] == 0
			{
				continue;
			}

			// Don't build geode robots when we don't even have yet any obsidian robots
			if new_next_robot_index == 3 && state.robots[2] == 0
			{
				continue;
			}

			if 
				(new_next_robot_index == 0 && state.robots[0] == state.costs.iter().map(|cost| cost[0]).max().unwrap()) ||
				(new_next_robot_index == 1 && state.robots[1] == state.costs[2][1]) ||
				(new_next_robot_index == 2 && state.robots[2] == state.costs[3][2])
			{
				continue;
			}

			let mut new_state = state.clone();
			new_state.next_robot_index = new_next_robot_index;
			let new_state_result = part_1(new_state);
			geodes = std::cmp::max(geodes, new_state_result);
		}
	}

	return geodes;
	
}
