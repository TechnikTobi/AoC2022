mod input;

use crate::input::*;

fn main() {
	println!("Hello, world!");
	read_data::<i32>(
		std::path::Path::new("./test"),
		Some('a')
	);
}
