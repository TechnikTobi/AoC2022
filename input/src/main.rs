mod input;

use crate::input::*;

fn main() {

	let test = read_data::<i32>(
		std::path::Path::new("./test"),
		Some('a')
	);

	for item_vec in test
	{
		for item in item_vec
		{
			print!("{} ", item);
		}
		print!("\n");
	}
}
