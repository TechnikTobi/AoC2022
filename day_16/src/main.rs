use std::collections::HashMap;
use std::collections::VecDeque;
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

fn 
parse_string
(
	data: &String
)
-> (String, i64, Vec::<String>)
{
	let valve_name = &data[6..8].to_string();
	let valve_flow = &data[23..].to_string().split(';').next().unwrap().parse::<i64>().unwrap();
	let tunnels_raw = data.split("to valve").last().unwrap();
	let tunnels = if tunnels_raw.chars().nth(0) == Some('s') { &tunnels_raw[2..] } else { &tunnels_raw[1..] };
	let tunnels_vec = tunnels.split(", ").map(|x| x.to_string()).collect::<Vec<String>>();
	return (valve_name.clone(), valve_flow.clone(), tunnels_vec);
}


fn
dijkstra
(
	start: &String,
	on_off_original: &HashMap<String, bool>,
	flows_original: &HashMap<String, i64>,
	leads_to_original: &HashMap<String, Vec<String>>,
	remaining_time: i64
)
-> (String, i64, i64)
{
	let mut on_off = on_off_original.clone();
	let mut flows = flows_original.clone();
	let mut leads_to = leads_to_original.clone();

	let mut positions = on_off.keys().map(|x| x.clone()).collect::<VecDeque<String>>();
	let destinations = positions.clone();
	let mut predecessor = HashMap::new();
	let mut distance = HashMap::new();

	for position in &positions
	{
		distance.insert(
			position.clone(),
			if position == start { 0i64 } else { i64::MAX }
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
			if leads_to[&u.clone()].contains(position)
			{
				let alternative = distance[&u] + 1;
				if alternative < distance[&position.clone()]
				{
					distance.insert(position.clone(), alternative);
					predecessor.insert(position.clone(), u.clone());
				}
			}
		}
	}

	let mut next_destination = start.clone();
	let mut next_destination_value = 0;
	let mut next_destination_cost = 0;

	println!("Analysis start:");
	for destination in &destinations
	{
		if on_off[&destination.clone()]
		{
			continue;
		}

		let flow = flows[&destination.clone()];
		if flow == 0
		{
			continue;
		}

		let mut path = VecDeque::new();
		path.push_front(destination.clone());
		let mut u = destination.clone();

		if !predecessor.contains_key(&u)
		{
			continue;
		}
		while !(&predecessor[&u] == start)
		{
			u = predecessor[&u].clone();
			path.push_front(u.clone());
		}

		let destination_cost = path.len() as i64;
		let destination_value = (remaining_time - destination_cost) * flow;

		// println!("Destination: {}", destination);
		// println!("Factor: {}", remaining_time - destination_cost);
		// println!("Flow: {}", flow);
		// println!("Value: {}", destination_value);
		// println!("\n");


		if destination_value > next_destination_value
		{
			next_destination = destination.clone();
			next_destination_value = destination_value;
			next_destination_cost = destination_cost+1;
		}
	}

	return (next_destination, next_destination_value, next_destination_cost);	
}

fn
recursive_visit
(
	start: &String,
	leads_to: &HashMap<String, Vec<String>>,
	flows: &HashMap<String, i64>,
	on_off_orig: &HashMap<String, bool>,
	remaining_time: i64,
	so_far: &String,
	remaining: usize
)
-> i64
{

	let mut best_result = 0;

	if remaining_time > 0 && remaining > 0
	{
		// Only perform this branch if this valve is closed
		// Also only makes sense if opening this valve actually does something
		let this_flow = flows[&start.clone()];
		if on_off_orig[&start.clone()] == false && this_flow > 0
		{
			let mut on_off_this_on = on_off_orig.clone();
			on_off_this_on.insert(start.clone(), true);
			for neighbour in &leads_to[&start.clone()]
			{
				// println!("?{}{}", so_far, neighbour);
				let result = recursive_visit(neighbour, leads_to, flows, &on_off_this_on, remaining_time-2, &format!("{}{}", so_far, neighbour), remaining-1);
				best_result = std::cmp::max(best_result, result + this_flow*remaining_time);
			}

		}


		let on_off_this_off = on_off_orig;
		for neighbour in &leads_to[&start.clone()]
		{
			if so_far[..so_far.len()-2].ends_with(neighbour)
			{
				continue;
			}
			// println!("!{}{}", so_far, neighbour);
			let result = recursive_visit(neighbour, leads_to, flows, &on_off_this_off, remaining_time-1, &format!("{}{}", so_far, neighbour), remaining);
			best_result = std::cmp::max(best_result, result);
		}
	}
	else
	{
		// panic!("AH");
	}

	// println!("{}", best_result);
		
	return best_result;
}

fn
floyd_warshall
(
	leads_to: &HashMap<String, Vec<String>>,
)
-> HashMap<(String, String), i64>
{
	let mut distances = HashMap::new();
	let valves = leads_to.keys().map(|x| x.to_string()).collect::<Vec<String>>();

	for v1 in &valves
	{
		for v2 in &valves
		{
			if v1 == v2
			{
				distances.insert((v1.clone(), v2.clone()), 0);
			}
			else if leads_to[&v1.clone()].contains(v2)
			{
				distances.insert((v1.clone(), v2.clone()), 1);
			}
			else
			{
				distances.insert((v1.clone(), v2.clone()), i64::MAX/3);
			}
			
		}
	}

	for v1 in &valves
	{
		for v2 in &valves
		{
			for v3 in &valves
			{
				distances.insert(
					(v2.clone(), v3.clone()),
					std::cmp::min(distances[&(v2.clone(), v3.clone())], distances[&(v2.clone(), v1.clone())] + distances[&(v1.clone(), v3.clone())])
				);
			}
		}
	}

	return distances;
}

fn 
recursive_visit_v2
(
	start: &String,
	leads_to: &HashMap<String, Vec<String>>,
	flows: &HashMap<String, i64>,
	distances: &HashMap<(String, String), i64>,
	on_off_orig: &HashMap<String, bool>,
	remaining_time: i64,
	so_far: &String,
)
-> i64
{
	// println!("{}", so_far);

	let valves = leads_to.keys().map(|x| x.to_string()).collect::<Vec<String>>();

	let mut best_result = 0;
	// let mut best_result_string = so_far.clone();

	for valve in &valves
	{

		let flow = flows[&valve.clone()];
		if flow <= 0
		{
			continue;
		}

		if on_off_orig[&valve.clone()]
		{
			continue;
		}

		// We know about the valve at this point: It has positive flow & is closed
		let distance = distances[&(start.clone(), valve.clone())];

		// Check if visiting that valve is even possible
		if distance > remaining_time+1
		{
			continue;
		}

		let mut on_off = on_off_orig.clone();
		on_off.insert(valve.clone(), true);

		let result = recursive_visit_v2(valve, leads_to, flows, distances, &on_off, remaining_time-distance-1, &format!("{}{}", so_far, valve));
		best_result = std::cmp::max(best_result, result + (remaining_time-distance-1)*flow);
	}

	return best_result;
}

fn main() 
{
	let lines = read_string_data(
		std::path::Path::new("./data/input.txt"),
	).unwrap();

	// Preprocessing
	let mut on_off = HashMap::new();
	let mut flows = HashMap::new();
	let mut leads_to = HashMap::new();
	for line in &lines
	{
		let (name, flow, tunnels) = parse_string(line);
		on_off.insert(name.clone(), false);
		flows.insert(name.clone(), flow);
		leads_to.insert(name, tunnels);
	}

	let on_off_original = on_off.clone();
	let flows_original = flows.clone();
	let leads_to_original = leads_to.clone();



	// Part 1
	// Example: 20*28 + 13*25 + 21*21 + 22*13 + 3*9 + 2*6
	// let max_time = 30;
	// let mut current = "AA".to_string();
	// // let mut destination = String::new();
	// let mut minute = 1;
	// let mut part_1_result = 0;
	// let mut pressure_sum = 0;

	// while minute < max_time
	// {
	// 	let (destination, value, cost) = dijkstra(
	// 		&current, 
	// 		&on_off, 
	// 		&flows, 
	// 		&leads_to, 
	// 		(max_time-minute)
	// 	);

	// 	println!("Current: {}", current);
	// 	println!("Minutes so far: {}", minute);
	// 	println!("Pressure sum: {}", pressure_sum);
	// 	println!("Destination: {}", destination);
	// 	println!("\n");

	// 	if current == destination
	// 	{
	// 		break;
	// 	}

	// 	pressure_sum += flows[&destination.clone()];
	// 	current = destination;
	// 	part_1_result += value;
	// 	minute += cost;
	// 	on_off.insert(current.clone(), true);

	// }

	let start = String::from("AA");
	let remaining = flows.iter().filter(|(_, &value)| value > 0).count();
	let distances = floyd_warshall(&leads_to);
	// let part_1_result = recursive_visit(&start, &leads_to, &flows, &on_off, 29, &start, remaining);
	
	let part_1_result = recursive_visit_v2(&start, &leads_to, &flows, &distances, &on_off, 30, &String::new());


	println!("Part 1: {}", part_1_result);




}
