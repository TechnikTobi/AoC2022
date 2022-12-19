use std::collections::HashMap;
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

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
struct Cube
{
	x: i64,
	y: i64,
	z: i64
}

#[derive(PartialEq, Eq)]
enum
EMaterial
{
	Water,
	Air,
	Lava
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

	// Part 1
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

	println!("Part 1: {}", total_surface_area);



	// Part 2
	let mut min_x = i64::MAX;
	let mut min_y = i64::MAX;
	let mut min_z = i64::MAX;
	let mut max_x = i64::MIN;
	let mut max_y = i64::MIN;
	let mut max_z = i64::MIN;

	// Prepare flooding container
	let mut floodfill_container = HashMap::new();
	for cube in &cubes
	{	
		min_x = std::cmp::min(min_x, cube.x);
		min_y = std::cmp::min(min_y, cube.y);
		min_z = std::cmp::min(min_z, cube.z);

		max_x = std::cmp::max(max_x, cube.x);
		max_y = std::cmp::max(max_y, cube.y);
		max_z = std::cmp::max(max_z, cube.z);
	}
	
	for x in min_x-1..=max_x+1
	{
		for y in min_y-1..=max_y+1
		{
			for z in min_z-1..=max_z+1
			{
				floodfill_container.insert(Cube { x: x, y: y, z: z }, EMaterial::Air);
			}
		}
	}

	// Fill in the given cubes
	for cube in &cubes
	{
		floodfill_container.insert(cube.clone(), EMaterial::Lava);
	}

	// Flooding
	recursive_floodfill(&Cube {x: min_x-1, y: min_y-1, z: min_z -1}, &mut floodfill_container);

	let mut total_outside_surface_area = 0;
	for cube in &cubes
	{
		let mut sides = 0;
		
		// Top & Bottom
		sides += floodfill_container.iter().filter(|(other, material)| other.x == cube.x && other.y == cube.y && (other.z - cube.z).abs() == 1 && **material == EMaterial::Water).count();

		// Left & Right
		sides += floodfill_container.iter().filter(|(other, material)| other.z == cube.z && other.y == cube.y && (other.x - cube.x).abs() == 1 && **material == EMaterial::Water).count();

		// Front & Back
		sides += floodfill_container.iter().filter(|(other, material)| other.z == cube.z && other.x == cube.x && (other.y - cube.y).abs() == 1 && **material == EMaterial::Water).count();

		total_outside_surface_area += sides;
	}

	println!("Part 2: {}", total_outside_surface_area);

}

fn
recursive_floodfill
(
	seed: &Cube,
	floodfill_container: &mut HashMap<Cube, EMaterial>
)
{
	if !floodfill_container.contains_key(seed)
	{
		return;
	}

	if floodfill_container[&seed] != EMaterial::Air
	{
		return;
	}

	floodfill_container.insert(seed.clone(), EMaterial::Water);

	// Top & Bottom
	recursive_floodfill(&Cube {x: seed.x, y: seed.y, z: seed.z + 1}, floodfill_container);
	recursive_floodfill(&Cube {x: seed.x, y: seed.y, z: seed.z - 1}, floodfill_container);

	// Left & Right
	recursive_floodfill(&Cube {x: seed.x - 1, y: seed.y, z: seed.z}, floodfill_container);
	recursive_floodfill(&Cube {x: seed.x + 1, y: seed.y, z: seed.z}, floodfill_container);

	// Front & Back
	recursive_floodfill(&Cube {x: seed.x, y: seed.y - 1, z: seed.z}, floodfill_container);
	recursive_floodfill(&Cube {x: seed.x, y: seed.y + 1, z: seed.z}, floodfill_container);
	
}