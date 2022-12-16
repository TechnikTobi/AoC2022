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
	let start = String::from("AA");
	let distances = floyd_warshall(&leads_to);	
	let part_1_result = recursive_visit_v2(&start, &leads_to, &flows, &distances, &on_off, 30, &String::new());

	println!("Part 1: {}", part_1_result);


}
