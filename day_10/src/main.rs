use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::time::Instant;

struct
Instruction
{
	operator: String,
	operand: i64
}

fn read_data
(
	path: &std::path::Path,
)
-> Result<Vec<Instruction>, Box<dyn std::error::Error>>
{	
	let file = File::open(path)?;
	let lines = BufReader::new(file).lines();
	let mut data = Vec::<Instruction>::new();

	for result_line in lines
	{
		let line = result_line?;
		let direction = line.split(' ').nth(0).unwrap().to_string();
		let count: i64;
		if let Some(value) = line.split(' ').nth(1)
		{
			count = value.parse::<i64>().unwrap();
		}
		else
		{
			count = 0;
		}
		data.push(Instruction {operator: direction, operand: count} );
	}	

	return Ok(data);
}

fn
calc_signal_strength
(
	register_x: i64,
	cycle_count: i64,
)
-> i64
{
	if (cycle_count - 20) % 40 == 0
	{
		
		return cycle_count * register_x;
	}
	return 0;
}

fn
increment_cycle
(
	register_x: i64,
	cycle_count: &mut i64
)
-> String
{
	*cycle_count += 1;
	
	let crt_position = (*cycle_count) % 40;
	let suffix = if crt_position == 0 { "\n" } else { "" };

	if 
	(register_x +0 == crt_position) ||
	(register_x +1 == crt_position) ||
	(register_x +2 == crt_position)
	{
		return format!("{}{}", "█", suffix);
	}
	else
	{
		return format!("{}{}", " ", suffix);
	}
}

fn main() 
{
	let instructions = read_data(
		std::path::Path::new("./data/input.txt"),
	).unwrap();

	let iterations = 5000;

	// Part 1
	let mut part_1_result = 0;

	let mut part_1_times = Vec::new();
	for _iteration in 0..iterations
	{
		let part_1_start = Instant::now();

		let mut register_x = 1i64;
		let mut cycle_count = 0i64;
		let mut signal = 0i64;

		for i in &instructions
		{
			match i.operator.as_str()
			{
				"addx" => {
					cycle_count += 1;
					signal += calc_signal_strength(register_x, cycle_count);
					cycle_count += 1;
					signal += calc_signal_strength(register_x, cycle_count);
					register_x += i.operand;
				},
				"noop" => {
					cycle_count += 1;
					signal += calc_signal_strength(register_x, cycle_count);
				},
				_ => panic!("UNKNOWN INSTRUCTION")
			}
		}

		part_1_result = signal;

		let part_1_end = Instant::now();
		part_1_times.push(part_1_end.duration_since(part_1_start).as_nanos());
	}

	part_1_times.sort();
	println!("---------- DAY: 10 - PART 1 ----------");
	println!("Result:   \n{}\n",  part_1_result);
	println!("Iterations: {}", iterations);
	println!("Mean:       {}ns",   part_1_times.iter().sum::<u128>()/(part_1_times.len() as u128));
	println!("Median:     {}ns",   part_1_times[part_1_times.len() / 2]);
	println!("Min:        {}ns",   part_1_times[0]);
	println!("Max:        {}ns",   part_1_times[part_1_times.len() -1]);	
	println!("Total:      {}ns\n", part_1_times.iter().sum::<u128>());

	println!("{}", part_1_result);



	// Part 2
	let mut part_2_result = String::new();

	let mut part_2_times = Vec::new();
	for _iteration in 0..iterations
	{
		let part_2_start = Instant::now();

		let mut register_x = 1i64;
		let mut cycle_count = 0i64;
		let mut output = String::new();

		for i in &instructions
		{
			match i.operator.as_str()
			{
				"addx" => {
					output.push_str(&increment_cycle(register_x, &mut cycle_count));
					output.push_str(&increment_cycle(register_x, &mut cycle_count));
					register_x += i.operand;
				},
				"noop" => {
					output.push_str(&increment_cycle(register_x, &mut cycle_count));
				},
				_ => panic!("UNKNOWN INSTRUCTION")
			}
		}

		part_2_result = output;

		let part_2_end = Instant::now();
		part_2_times.push(part_2_end.duration_since(part_2_start).as_micros());
	}

	part_2_times.sort();
	println!("---------- DAY: 10 - PART 2 ----------");
	println!("Result:   \n{}\n",   part_2_result);
	println!("Iterations: {}", iterations);
	println!("Mean:       {}µs",   part_2_times.iter().sum::<u128>()/(iterations as u128));
	println!("Median:     {}µs",   part_2_times[iterations / 2]);
	println!("Min:        {}µs",   part_2_times[0]);
	println!("Max:        {}µs",   part_2_times[iterations -1]);	
	println!("Total:      {}µs\n", part_2_times.iter().sum::<u128>());

}
