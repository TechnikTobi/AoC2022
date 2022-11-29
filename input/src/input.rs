use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

pub fn get_data
<T: FromStr + std::default::Default + std::fmt::Display>
(
	path: &std::path::Path,
	delimiter: Option<char>
)
-> Result<Vec<T>, Box<dyn std::error::Error>>
{	
	println!("Test");

	let file = File::open(path)?;
	let lines = BufReader::new(file).lines();
	let mut data = Vec::<T>::new();

	for result_line in lines
	{
		if let Ok(line) = result_line
		{
			println!("{}", line);
			/*
			let mut elements = line.split(delimiter.unwrap_or(' '));
			data.append(&mut elements.collect::<Vec<T>>());
			*/
			
			/*
			let elements = line.split(delimiter.unwrap_or(' '));
			for element in elements
			{
				println!("{}", T::from_str(element).unwrap_or_default());
			}
			*/

			/*
			let elements = line.split(delimiter.unwrap_or(' '))
				.map(|&element| T::from_str(&element).unwrap_or_default())
				.collect::<Vec<T>>();
			*/

			let mut elements = line.split(delimiter.unwrap_or(' '))
				.map(T::from_str)
				.map(Result::unwrap_or_default)
				.collect::<Vec<T>>();

			data.append(&mut elements);
		}
	}	

	return Ok(data);
}



pub fn get_string_data
(
	path: &std::path::Path
)
-> Vec<String>
{
	return Vec::new();	
}
