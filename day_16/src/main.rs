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

	let mut best_result = 0;
	// let mut best_result_string = so_far.clone();

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
		let interesting_valves = interesting_valves_orig.iter().filter(|x| *x!= valve).map(|x| x.clone()).collect::<Vec<String>>();
		let mut on_off = on_off_orig.clone();
		on_off.insert(valve.clone(), true);

		let result = recursive_visit_v2(valve, &interesting_valves, leads_to, flows, distances, &on_off, remaining_time-distance-1, &format!("{}{}", so_far, valve));
		best_result = std::cmp::max(best_result, result + (remaining_time-distance-1)*flow);
	}

	return best_result;
}

fn 
recursive_visit_
(
	start: &String,
	interesting_valves_orig: &Vec<String>,
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

	let mut best_result = 0;
	// let mut best_result_string = so_far.clone();

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
		let interesting_valves = interesting_valves_orig.iter().filter(|x| *x!= valve).map(|x| x.clone()).collect::<Vec<String>>();
		let mut on_off = on_off_orig.clone();
		on_off.insert(valve.clone(), true);

		let result = recursive_visit_v2(valve, &interesting_valves, leads_to, flows, distances, &on_off, remaining_time-distance-1, &format!("{}{}", so_far, valve));
		best_result = std::cmp::max(best_result, result + (remaining_time-distance-1)*flow);
	}

	return best_result;
}

fn 
recursive_visit_part_2_old
(
	start_me: &String,
	start_elephant: &String,
	leads_to: &HashMap<String, Vec<String>>,
	flows: &HashMap<String, i64>,
	distances: &HashMap<(String, String), i64>,
	on_off_orig: &HashMap<String, bool>,
	remaining_time_me: i64,
	remaining_time_elephant: i64,
)
-> i64
{
	let mut best_result = 0;

	if remaining_time_me <= 0 || remaining_time_elephant <= 0
	{
		return best_result;
	}

	let valves = leads_to.keys().map(|x| x.to_string()).collect::<Vec<String>>();

	let mut viable_valves = Vec::new();

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
		let distance_me = distances[&(start_me.clone(), valve.clone())];
		let distance_elephant = distances[&(start_elephant.clone(), valve.clone())];

		// Check if visiting that valve is even possible for one of the two
		if distance_me > remaining_time_me+1 && distance_elephant > remaining_time_elephant+1
		{
			continue;
		}

		viable_valves.push(valve);
	}

	viable_valves.sort();

	

	for (i_1, v1) in viable_valves.iter().enumerate()
	{
		for (i_2, v2) in viable_valves.iter().enumerate()
		{
			if i_2 <= i_1
			{
				continue;
			}

			let flow_1 = flows[v1.clone()];
			let flow_2 = flows[v2.clone()];

			let distance_v1_me = distances[&(start_me.clone(), (*v1).clone())];
			let distance_v1_elephant = distances[&(start_elephant.clone(), (*v1).clone())];
			let distance_v2_me = distances[&(start_me.clone(), (*v2).clone())];
			let distance_v2_elephant = distances[&(start_elephant.clone(), (*v2).clone())];

			if distance_v1_elephant <= remaining_time_elephant + 1
			{
				let mut on_off = on_off_orig.clone();
				on_off.insert((*v1).clone(), true);
				if distance_v2_me <= remaining_time_me + 1
				{
					// elephant -> v1
					// me -> v2
					on_off.insert((*v2).clone(), true);
					let result = recursive_visit_part_2_old(v2, v1, leads_to, flows, distances, &on_off, remaining_time_me-distance_v2_me-1, remaining_time_elephant-distance_v1_elephant-1);
					best_result = std::cmp::max(best_result, result + (remaining_time_me-distance_v2_me-1)*flow_2 + (remaining_time_elephant-distance_v1_elephant-1)*flow_1);
				}
				else 
				{
					// elephant -> v1
					// me -> staying where I am	
					let result = recursive_visit_part_2_old(start_me, v1, leads_to, flows, distances, &on_off, remaining_time_me, remaining_time_elephant-distance_v1_elephant-1);
					best_result = std::cmp::max(best_result, result + (remaining_time_elephant-distance_v1_elephant-1)*flow_1);
				}
			}

			if distance_v2_elephant <= remaining_time_elephant + 1
			{
				let mut on_off = on_off_orig.clone();
				on_off.insert((*v2).clone(), true);
				if distance_v1_me <= remaining_time_me + 1
				{
					// elephant -> v2
					// me -> v1
					on_off.insert((*v1).clone(), true);
					let result = recursive_visit_part_2_old(v1, v2, leads_to, flows, distances, &on_off, remaining_time_me-distance_v1_me-1, remaining_time_elephant-distance_v2_elephant-1);
					best_result = std::cmp::max(best_result, result + (remaining_time_me-distance_v1_me-1)*flow_1 + (remaining_time_elephant-distance_v2_elephant-1)*flow_2);
				}
				else
				{
					// elephant -> v2
					// me -> staying where I am	
					let result = recursive_visit_part_2_old(start_me, v2, leads_to, flows, distances, &on_off, remaining_time_me, remaining_time_elephant-distance_v2_elephant-1);
					best_result = std::cmp::max(best_result, result + (remaining_time_elephant-distance_v2_elephant-1)*flow_2);
				}
			}

			if distance_v1_elephant > remaining_time_elephant + 1 && distance_v2_elephant > remaining_time_elephant + 1
			{
				if distance_v1_me <= remaining_time_me + 1
				{
					// elephant -> staying
					// me -> v1
					let mut on_off = on_off_orig.clone();
					on_off.insert((*v1).clone(), true);
					let result = recursive_visit_part_2_old(v1, start_elephant, leads_to, flows, distances, &on_off, remaining_time_me-distance_v1_me-1, remaining_time_elephant);
					best_result = std::cmp::max(best_result, result + (remaining_time_me-distance_v1_me-1)*flow_1);
				}
				else if distance_v2_me <= remaining_time_me + 1
				{	
					// elephant -> staying
					// me -> v2
					let mut on_off = on_off_orig.clone();
					on_off.insert((*v2).clone(), true);
					let result = recursive_visit_part_2_old(v2, start_elephant, leads_to, flows, distances, &on_off, remaining_time_me-distance_v2_me-1, remaining_time_elephant);
					best_result = std::cmp::max(best_result, result + (remaining_time_me-distance_v2_me-1)*flow_2);
				}
			}
		}
	}

	return best_result;
}






// fn
// part_2_queue
// (
// 	flows: &HashMap<String, i64>,
// 	distances: &HashMap<(String, String), i64>,
// )
// {
// 	let interesting = flows.iter().filter(|(key, &value)| value > 0 || **key == "AA".to_string()).map(|(key, _)| key.to_string()).collect::<Vec<String>>();
	
// }

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

	// let start = String::from("AA");
	// let interesting_valves = flows.iter().filter(|(_, &value)| value > 0).map(|(key, _)| key.to_string()).collect::<Vec<String>>();
	// let part_1_result = recursive_visit_v2(&start, &interesting_valves, &leads_to, &flows, &distances, &on_off, 30, &String::new());

	// println!("Part 1: {}", part_1_result);


	// Part 2
	let start = String::from("AA");
	let part_2_result = recursive_visit_part_2_old(&start, &start, &leads_to, &flows, &distances, &on_off, 26, 26);

	println!("Part 2: {}", part_2_result);


}
