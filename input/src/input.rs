use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

pub fn read_data
<T: FromStr + std::default::Default + std::fmt::Display>
(
	path: &std::path::Path,
	delimiter: Option<char>
)
-> Result<Vec<T>, Box<dyn std::error::Error>>
{	
	let file = File::open(path)?;
	let lines = BufReader::new(file).lines();
	let mut data = Vec::<T>::new();

	for result_line in lines
	{
		let line = result_line?;
		let mut elements = line.split(delimiter.unwrap_or(' '))
			.map(T::from_str)
			.map(Result::unwrap_or_default)
			.collect::<Vec<T>>();

		data.append(&mut elements);
	}	

	return Ok(data);
}

pub fn read_vecs
<T: FromStr + std::default::Default + std::fmt::Display + std::clone::Clone>
(
	path: &std::path::Path,
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

	return Ok(data);
}

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


#[cfg(test)]
mod tests {

	#[test]
	fn
	test1() 
	{
		let data = crate::read_data::<i32>(
			std::path::Path::new("tests/input1.txt"),
			None
		).unwrap();

		assert_eq!(data.len(), 2000);
		assert_eq!(data[0], 104);
	}
}
