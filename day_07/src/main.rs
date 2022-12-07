use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

fn read_string_data
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

fn parse_string_data
(
	data: &Vec<String>
)
{

	let mut directories = HashMap::from([("/".to_string(), Vec::<u64>::new())]);
	let mut current_path = "/".to_string();

	for line in data
	{
		match line.chars().nth(0)
		{
			Some('$') => { // Do...
				match line.chars().nth(2)
				{
					Some('c') => { // Change Dir...
						match line.chars().nth(5)
						{
							Some('.') => { // Go to parent
								let mut last_char = ' ';
								current_path.pop();
								while last_char != '/'
								{
									last_char = current_path.pop().unwrap_or('/');
								}
								current_path.push('/');
							},
							Some('/') => { // Go to root
								current_path = "/".to_string();
							},
							_ => { // Go to subdir
								let subdir = &line[5..];
								current_path.push_str(subdir);
								current_path.push('/');
							},
						}
					},
					Some('l') => (),
					_ => panic!("AH")
				}
			},
			Some('d') => { // Dir
				let directory_name = format!("{}{}/", current_path, &line[4..]);
				directories.insert(
					directory_name,
					Vec::<u64>::new()
				);
			},
			Some(_) => { // File
				let file_size = line
					.split(' ')
					.map(|x| x.to_string())
					.collect::<Vec<String>>()[0]
					.parse::<u64>()
					.unwrap_or_default();
				directories.get_mut(&current_path).unwrap().push(file_size);
			}
			_ => panic!("AH")
		}
	}


	
	// Part 1
	let mut directory_sizes = Vec::new();
	for entry in &directories
	{
		let mut directory_size = 0u64;
		for other_entry in &directories
		{
			if 
			(other_entry.0.contains(entry.0)) &&                                // other_entry is an entry for a subdirectory
			(other_entry.0 != entry.0)                                          // ensuring that they are not the same directory
			{
				directory_size += other_entry.1
					.iter()
					.sum::<u64>();
			}
		}

		directory_size += entry.1
					.iter()
					.sum::<u64>();

		directory_sizes.push(directory_size);
		println!("{}: {}", entry.0, directory_size);
	}

	println!("Final sum for part 1: {}", directory_sizes.iter().filter(|&x| *x < 100000u64).sum::<u64>());



	// Part 2
	let disk_size = 70000000u64;
	let space_required = 30000000u64;

	let used_space = directories
		.iter()
		.map(
			|directory| directory.1
				.iter()
				.sum::<u64>()
		)
		.sum::<u64>();

	let needs_to_be_freed = space_required - (disk_size-used_space);
	let smallest_viable_directory_size = directory_sizes
		.iter()
		.filter(|size| *size >= &needs_to_be_freed)
		.min()
		.unwrap_or(&used_space);
	
	println!("Final size for part 2: {}", smallest_viable_directory_size);

}

fn main() 
{

	let problem_data = read_string_data(
		std::path::Path::new("./data/input.txt"),
	).unwrap();

	parse_string_data(&problem_data);
}
