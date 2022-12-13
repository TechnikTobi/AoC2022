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
		if line.len() > 0
		{
			data.push(line);
		}
	}

	return Ok(data);	
}

trait
Value
{
	fn to_list(&self) -> List;
	fn is_list(&self) -> bool;
	fn get_value(&self) -> u64;
	fn get_values(&self) -> &Vec<Box<dyn Value>>;
	fn compare(&self, other: &dyn Value) -> std::cmp::Ordering;
	fn print(&self);
}

#[derive(Debug, Clone)]
struct Integer
{
	value: u64
}

struct List
{
	values: Vec<Box<dyn Value>>
}

impl Value for Integer
{
	fn to_list(&self) -> List { List { values: vec![Box::new(self.clone())] } }

	fn is_list(&self) -> bool { false }

	fn compare(&self, right: &dyn Value) -> std::cmp::Ordering // self is Left
	{
		if right.is_list()
		{
			return self.to_list().compare(right);
		}
		else
		{
			return self.value.cmp(&right.get_value()).reverse();
		}
	}

	fn get_value(&self) -> u64 { self.value }

	fn get_values(&self) -> &Vec<Box<dyn Value>> { panic!("AH2"); }

	fn print(&self) {print!("{}", self.value); }
}

impl Value for List
{
	fn to_list(&self) -> List { panic!("AH3"); }

	fn is_list(&self) -> bool { true }

	fn compare(&self, right: &dyn Value) -> std::cmp::Ordering
	{
		if right.is_list()
		{
			let right_list = right.get_values();
			let length = std::cmp::min(self.values.len(), right_list.len());
			for i in 0..length
			{
				let comparison_result = self.values[i].compare(&*right_list[i]);
				if comparison_result != std::cmp::Ordering::Equal
				{
					return comparison_result;
				}
			}
			return self.values.len().cmp(&right_list.len()).reverse();
		}
		return self.compare(&right.to_list());
	}

	fn get_value(&self) -> u64 { panic!("AH"); }

	fn get_values(&self) -> &Vec<Box<dyn Value>> { &self.values }

	fn print(&self) 
	{
		print!("[");
		for value in &self.values
		{
			value.print();
			print!(", ");
		}
		print!("]");
	}
}

fn parse_string
(
	data: String
)
-> Box<dyn Value>
{
	if data.is_empty()
	{
		return Box::new(List {values: Vec::new() });
	}

	if 
		data.chars().next().unwrap() == '[' && 
		data.chars().last().unwrap() == ']'
	{
		let mut substrings = Vec::new();
		let mut temp = String::new();
		let mut bracket_count = 0;

		for character in data[1..data.len()-1].chars()
		{
			match character
			{
				' ' => continue,
				'[' => bracket_count += 1,
				_ => ()
			};
			
			if bracket_count > 0 { temp.push(character); }
			if character == ']'  { bracket_count -= 1; }

			if bracket_count == 0
			{
				if character == ','
				{
					substrings.push(temp);
					temp = String::new();
				}
				else
				{
					temp.push(character);
				}
			}
		}
		substrings.push(temp);

		let values = substrings
			.iter()
			.map(|substring| parse_string(substring.clone()))
			.collect::<Vec<Box<dyn Value>>>();
		return Box::new(List {values: values} );
	}

	return Box::new(Integer { value: data.parse::<u64>().unwrap() });
}

fn main() 
{
	let lines = read_string_data(
		std::path::Path::new("./data/input.txt"),
	).unwrap();



	// Part 1
	let mut part_1_result = 0;
	for i in (0..lines.len()).step_by(2)
	{
		let left = parse_string(lines[i].clone());
		let right = parse_string(lines[i+1].clone());

		if left.compare(&*right) == std::cmp::Ordering::Greater
		{
			part_1_result += i/2+1;
		}
	}
	println!("Part 1: {}", part_1_result);



	// Part 2
	let mut values = lines
		.iter()
		.map(|line| parse_string(line.clone()))
		.collect::<Vec<Box<dyn Value>>>();
	
	let two_box = Box::new(Integer {value: 2});
	let six_box = Box::new(Integer {value: 6});

	values.push(Box::new(List { values: vec![two_box]}));
	values.push(Box::new(List { values: vec![six_box]}));

	values.sort_by(|a,b| a.compare(&**b).reverse());

	let two_list = List { values: vec![ Box::new(Integer {value: 2}) ] };
	let six_list = List { values: vec![ Box::new(Integer {value: 6}) ] };

	let mut marker_1_index = 0;
	let mut marker_2_index = 0;

	for (index, value) in values.iter().enumerate()
	{
		if value.compare(&two_list).is_eq()
		{
			marker_1_index = index + 1;
		}
		if value.compare(&six_list).is_eq()
		{
			marker_2_index = index + 1;
		}
	}

	println!("Part 2: {}", marker_1_index * marker_2_index);

}
