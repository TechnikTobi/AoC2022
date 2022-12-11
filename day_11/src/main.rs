use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

/// Reads in groups of lines (groups are separated by empty lines) and puts all
/// elements of a group into a Vec<T> vector. 
#[allow(dead_code)]
pub fn read_vecs
<T: FromStr + std::default::Default + std::fmt::Display + std::clone::Clone>
(
	path: &std::path::Path
)
-> Result<Vec<Vec<T>>, Box<dyn std::error::Error>>
{	
	let file = File::open(path)?;
	let lines = BufReader::new(file).lines();
	let mut data = Vec::<Vec::<T>>::new();

	let mut temp = Vec::<T>::new();

	for result_line in lines
	{
		let line = result_line?;

		if line == ""
		{
			data.push(temp.clone());
			temp.clear();
		}
		else
		{
			temp.push(T::from_str(&line).unwrap_or_default());
		}

	}	

	data.push(temp.clone());

	return Ok(data);
}

#[derive(Debug, Clone)]
struct
Monkey
{
	starting_items: VecDeque<i64>,
	operation: char,
	operand_str: String,
	test: i64,
	true_monkey: usize,
	false_monkey: usize,
	inspected_items: u64
}

fn main() 
{
	// Preprocessing
	let raw_monkeys = read_vecs::<String>(
		std::path::Path::new("./data/input.txt"),
	).unwrap();

	let mut monkeys = Vec::new();

	for raw_monkey in &raw_monkeys
	{		
		let starting_items = raw_monkey[1][18..].split(", ")
			.map(i64::from_str)
			.map(Result::unwrap_or_default)
			.collect::<VecDeque<i64>>();
		let operation = raw_monkey[2].chars().nth(23).unwrap();
		let operand_str = raw_monkey[2][25..].to_string(); //.parse::<i64>().unwrap();
		let test = raw_monkey[3][21..].parse::<i64>().unwrap();
		let true_monkey = raw_monkey[4][29..].parse::<usize>().unwrap();
		let false_monkey = raw_monkey[5][30..].parse::<usize>().unwrap();

		monkeys.push(Monkey
			{
				starting_items: starting_items,
				operation: operation,
				operand_str: operand_str,
				test: test,
				true_monkey: true_monkey,
				false_monkey: false_monkey,
				inspected_items: 0
			}
		);
	}

	let mut monkeys_part_1 = monkeys.clone();
	let mut monkeys_part_2 = monkeys.clone();

	// Part 1
	let rounds = 20;

	for _ in 0..rounds
	{
		
		for i in 0..monkeys_part_1.len()
		{
			while !monkeys_part_1[i].starting_items.is_empty()
			{
				monkeys_part_1[i].inspected_items += 1;
				let mut item = monkeys_part_1[i].starting_items.pop_front().unwrap();
				let operand = if monkeys_part_1[i].operand_str == "old"
				{
					item
				}
				else 
				{
					monkeys_part_1[i].operand_str.parse::<i64>().unwrap()
				};

				match monkeys_part_1[i].operation
				{
					'*' => {
						item *= operand;
					},
					'+' => {
						item += operand;
					},
					_ => panic!("AH"),
				};

				item /= 3;
				
				
				let new_monkey_id = if item % monkeys_part_1[i].test == 0
				{
					monkeys_part_1[i].true_monkey
				}
				else 
				{
					monkeys_part_1[i].false_monkey
				};

				monkeys_part_1[new_monkey_id].starting_items.push_back(item);
			}
		}
	}

	monkeys_part_1.sort_by(|a, b| a.inspected_items.cmp(&b.inspected_items));
	let monkey_business_part_1 = monkeys_part_1[monkeys_part_1.len()-1].inspected_items * monkeys_part_1[monkeys_part_1.len()-2].inspected_items;
	println!("Part 1: {}", monkey_business_part_1);



	// Part 2
	let rounds = 10000;
	let mut divisor = 1;
	for monkey in &monkeys
	{
		divisor *= monkey.test;
	}

	for _ in 0..rounds
	{
		
		for i in 0..monkeys_part_2.len()
		{
			while !monkeys_part_2[i].starting_items.is_empty()
			{
				monkeys_part_2[i].inspected_items += 1;
				let mut item = monkeys_part_2[i].starting_items.pop_front().unwrap() % divisor;
				let operand = if monkeys_part_2[i].operand_str == "old"
				{
					item
				}
				else 
				{
					monkeys_part_2[i].operand_str.parse::<i64>().unwrap()
				};

				match monkeys_part_2[i].operation
				{
					'*' => {
						item *= operand;
					},
					'+' => {
						item += operand;
					},
					_ => panic!("AH"),
				};

				let new_monkey_id = if item % monkeys_part_2[i].test == 0
				{
					monkeys_part_2[i].true_monkey
				}
				else 
				{
					monkeys_part_2[i].false_monkey
				};

				monkeys_part_2[new_monkey_id].starting_items.push_back(item);
			}
		}
	}

	monkeys_part_2.sort_by(|a, b| a.inspected_items.cmp(&b.inspected_items));
	let monkey_business_part_2 = monkeys_part_2[monkeys_part_2.len()-1].inspected_items * monkeys_part_2[monkeys_part_2.len()-2].inspected_items;
	println!("Part 2: {}", monkey_business_part_2);

}
