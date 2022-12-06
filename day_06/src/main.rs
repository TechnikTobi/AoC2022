use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::VecDeque;

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

fn main() {
    
    let mut line = read_line(
        std::path::Path::new("./data/input.txt")
    ).unwrap().chars().collect::<VecDeque<_>>();

    // Part 1 & 2
    let marker_len = 14; // 4; for part 1
    let mut marker = VecDeque::<char>::new();


    for marker_index in 1..(line.len()+1)
    {
        marker.push_back(line.pop_front().unwrap());

        while marker.len() > marker_len
        {
            marker.pop_front();
        }

        let mut all_are_different = true;
        for i in 0..marker.len()
        {
            for j in 0..marker.len()
            {
                if i != j && marker[i] == marker[j]
                {
                    all_are_different = false;
                }
            }
        }

        if marker.len() == marker_len && all_are_different
        {
            println!("Maker position: {}", marker_index);
            break;
        }
    }

}
