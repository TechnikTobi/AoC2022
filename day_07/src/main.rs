use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

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
-> Box<dyn filesys_object>
{

	let mut root_directory = directory {name: "/".to_string(), children: Vec::new()};
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
								while last_char != '/'
								{
									last_char = current_path.pop().unwrap_or('/');
								}
							},
							Some('/') => { // Go to root
								current_path = "/".to_string();
							},
							_ => { // Go to subdir
								let subdir = &line[5..];
								current_path.push('/');
								current_path.push_str(subdir);
							},
						}
					},
					Some('l') => { // List...
						
					},
					_ => panic!("AH")
				}
			},
			Some('d') => { // Dir
				let mut path = "".to_string();
				let mut current_dir = root_directory;
				while path != current_path 
				{
					current_dir.children
				}
			},
			Some(digit) => { // File

			}
			_ => panic!("AH")
		}
	}

	Box::new(root_directory)
}

fn main() 
{

	println!("3");
}
