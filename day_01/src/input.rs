use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

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