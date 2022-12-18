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
-> Result<Vec<Vec<T>>, Box<dyn std::error::Error>>
{	
	let file = File::open(path)?;
	let lines = BufReader::new(file).lines();
	let mut data = Vec::<Vec<T>>::new();

	for result_line in lines
	{
		let line = result_line?;
		let elements = line.split(delimiter.unwrap_or(' '))
			.map(T::from_str)
			.map(Result::unwrap_or_default)
			.collect::<Vec<T>>();

		data.push(elements);
	}	

	return Ok(data);
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Cube
{
	x: i64,
	y: i64,
	z: i64
}

fn main() 
{
	let cube_coordinates = read_data::<i64>(
		std::path::Path::new("./data/input.txt"),
		Some(',')
	).unwrap();

	let mut cubes = Vec::new();

	for data in cube_coordinates
	{
		cubes.push(Cube {x: data[0], y: data[1], z: data[2] });
	}

	let mut total_surface_area = 0;
	for cube in &cubes
	{
		let mut sides = 0;
		
		// Top & Bottom
		sides += 2 - cubes.iter().filter(|other| other.x == cube.x && other.y == cube.y && (other.z - cube.z).abs() == 1).count();

		// Left & Right
		sides += 2 - cubes.iter().filter(|other| other.z == cube.z && other.y == cube.y && (other.x - cube.x).abs() == 1).count();

		// Front & Back
		sides += 2 - cubes.iter().filter(|other| other.z == cube.z && other.x == cube.x && (other.y - cube.y).abs() == 1).count();

		total_surface_area += sides;
	}

	println!("{}", total_surface_area);
}
