mod input;

use crate::input::get_data;

fn main() {
	println!("Hello, world!");
	get_data::<i32>(
		std::path::Path::new("./test"),
		Some('a')
	);
}
