use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
use std::collections::HashMap;

trait filesys_object
{
	fn get_name(&self) -> String;
	fn get_size(&self) -> u64;
}

struct directory
{
	name: String,
	children: Vec<Box<dyn filesys_object>>
}

struct file
{
	name: String,
	size: u64
}

impl filesys_object
for directory
{
	fn get_name(&self) -> String
	{
		self.name.clone()
	}

	fn get_size(&self) -> u64
	{
		self.children.iter().map(|x| x.get_size()).sum()
	}
}

impl filesys_object
for file
{
	fn get_name(&self) -> String
	{
		self.name.clone()
	}

	fn get_size(&self) -> u64
	{
		self.size
	}
}

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
// -> Box<dyn filesys_object>
{

	let mut directories = HashMap::new();
	directories.insert(
		"/".to_string(),
		Vec::<file>::new()
	);

	// let mut root_directory = directory {name: "/".to_string(), children: Vec::new()};

	let mut current_path = "".to_string();

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
					Some('l') => { // List...
						
					},
					_ => panic!("AH")
				}
			},
			Some('d') => { // Dir
				let directory_name = format!("{}{}/", current_path, &line[4..]);
				directories.insert(
					directory_name,
					Vec::<file>::new()
				);
			},
			Some(digit) => { // File
				let mut line_parts = line.split(' ').map(|x| x.to_string()).collect::<Vec<String>>();
				let file_size = line_parts[0].parse::<u64>().unwrap();
				line_parts.remove(0);
				let file_name = line_parts.join(" ");
				println!("Current path: {}", current_path);
				println!("{}", file_name);
				println!("{}", file_size);
				directories.get_mut(&current_path).unwrap().push(file {name: file_name, size: file_size});
			}
			_ => panic!("AH")
		}
	}

	let mut dir_sizes = HashMap::new();
	for entry in &directories
	{
		let mut subdir_size = 0u64;
		for e in &directories
		{
			if e.0.contains(entry.0) && e.0 != entry.0
			{
				for f in e.1
				{
					subdir_size += f.size;
				}
			}
		}

		let mut dir_size = subdir_size;
		for f in entry.1
		{
			dir_size += f.size;
		}
		dir_sizes.insert(entry.0.clone(), dir_size);
		println!("{}: {}", entry.0, dir_size);
	}

	println!("Final sum for part 1: {}", dir_sizes.iter().map(|x| x.1).filter(|&x| *x < 100000u64).sum::<u64>());

	let disk_size = 70000000u64;
	let space_required = 30000000u64;

	let mut used_space = 0u64;
	for e in &directories
	{
		for f in e.1
		{
			used_space += f.size;
		}
	}

	let needs_to_be_freed = space_required - (disk_size-used_space);

	let mut current_smallest_viable_dir = "/".to_string();
	let mut current_smallest_viable_size = disk_size;
	for dir in dir_sizes
	{
		if dir.1 >= needs_to_be_freed && dir.1 < current_smallest_viable_size
		{
			current_smallest_viable_size = dir.1;
			current_smallest_viable_dir = dir.0.clone();
		}
	}

	println!("{}", current_smallest_viable_size);

}

fn main() 
{

	let problem_data = read_string_data(
		std::path::Path::new("./data/input.txt"),
	).unwrap();

	parse_string_data(&problem_data);
}
