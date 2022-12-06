use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

/// Opens a file, reads in each line as data of type T, puts them all into a vector.
#[allow(dead_code)]
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

	return Ok(data);
}



/// Special case combination of read_vecs and read_string_data, where each
/// group of lines (groups are separated by empty lines) gets joined to a single
/// string using the given separator or a space by default. 
/// Relies on the function read_vecs
#[allow(dead_code)]
pub fn read_string_data_empty_line
(
	path: &std::path::Path,
	sep: Option<&String>
)
-> Result<Vec<String>, Box<dyn std::error::Error>>
{
	let vectors = read_vecs::<String>(path)?;
	let mut data = Vec::<String>::new();

	for vector in vectors
	{
		data.push(vector.join(sep.unwrap_or(&" ".to_string())));
	}

	return Ok(data);
}


/// Reads in each line of the input file as a string and puts all of them into a vector.
#[allow(dead_code)]
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


/// Reads in the first line of the specified input file
#[allow(dead_code)]
pub fn read_line
(
	path: &std::path::Path
)
-> Result<String, Box<dyn std::error::Error>>
{
	let file = File::open(path)?;
	let lines = BufReader::new(file).lines();

	for result_line in lines
	{
		return Ok(result_line?);
	}

    return Ok("".to_string());
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
