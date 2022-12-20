use std::collections::HashMap;
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
	interesting_valves_orig: &Vec<String>,
	flows: &HashMap<String, i64>,
	distances: &HashMap<(String, String), i64>,
	remaining_time: i64,
)
-> i64
{
	let mut best_result = 0;

	for valve in interesting_valves_orig
	{
		// We know about the valve at this point: It has positive flow & is closed
		let distance = distances[&(start.clone(), valve.clone())];

		// Check if visiting that valve is even possible
		if distance > remaining_time+1
		{
			continue;
		}

		let flow = flows[&valve.clone()];
		let interesting_valves = interesting_valves_orig.iter().filter(|x| *x != valve).map(|x| x.clone()).collect::<Vec<String>>();

		let result = recursive_visit_v2(valve, &interesting_valves, flows, distances, remaining_time-distance-1);
		best_result = std::cmp::max(best_result, result + (remaining_time-distance-1)*flow);
	}

	return best_result;
}

fn
part_2
(
	start: &String,
	interesting_valves_orig: &Vec<String>,
	flows: &HashMap<String, i64>,
	distances: &HashMap<(String, String), i64>,
	remaining_time: i64,
	mother_call: bool,
)
-> i64
{
	let mut best_result = 0;

	for valve in interesting_valves_orig
	{

		if mother_call
		{
			println!("{}", valve);
		}

		// We know about the valve at this point: It has positive flow & is closed
		let distance = distances[&(start.clone(), valve.clone())];

		let flow = flows[&valve.clone()];
		let interesting_valves = interesting_valves_orig.iter().filter(|x| *x != valve).map(|x| x.clone()).collect::<Vec<String>>();

		// Check if visiting that valve is even possible
		if distance > remaining_time+1
		{
			continue;
		}

		let result = part_2(valve, &interesting_valves, flows, distances, remaining_time-distance-1, false);
		best_result = std::cmp::max(best_result, result + (remaining_time-distance-1)*flow);
	}

	let other_result = recursive_visit_v2(&String::from("AA"), interesting_valves_orig, flows, distances, 26);
	// let flows_new = flows.iter().map(|(key, value)| (key.clone(), value.clone()) ).collect::<Vec<(String, i64)>>();
	// let distances_new = distances.iter().map(|((key1, key2), value)| (key1.clone(), key2.clone(), value.clone()) ).collect::<Vec<(String, String, i64)>>();
	// let other_result = part_1(String::from("AA"), interesting_valves_orig.clone(), flows_new, distances_new, 26);
	best_result = std::cmp::max(best_result, other_result);

	return best_result;
}



fn main() 
{
	let lines = read_string_data(
		std::path::Path::new("./data/example.txt"),
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

	let distances = floyd_warshall(&leads_to);	

	// Part 1
	// Example: 20*28 + 13*25 + 21*21 + 22*13 + 3*9 + 2*6

	let interesting_valves = flows.iter().filter(|(_, &value)| value > 0).map(|(key, _)| key.to_string()).collect::<Vec<String>>();
	// let part_1_result = recursive_visit_v2(&start, &interesting_valves, &flows, &distances, 30);

	// println!("Part 1: {}", part_1_result);


	// Part 2

	let start = String::from("AA");
	// let part_2_result = recursive_visit_part_2_old(&start, &start, &leads_to, &flows, &distances, &on_off, 26, 26);
	let part_2_result = part_2(&start, &interesting_valves, &flows, &distances, 26, true);

	println!("Part 2: {}", part_2_result);


}
