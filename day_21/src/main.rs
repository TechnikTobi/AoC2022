use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

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

// struct
// Expression
// {
// 	expression_string: String
// }

// impl
// Expression
// {
// 	fn 
// 	evaluate
// 	(
// 		&self
// 	)
// 	-> f64
// 	{
// 		if self.expression_string
// 		return 0.0;
// 	}
// }

fn main() 
{
	let lines = read_string_data(
		std::path::Path::new("./data/input.txt")
	).unwrap();

	let mut monkey_expressions = HashMap::new();
	for line in &lines
	{
		let monkey = line[0..4].to_string();
		let expression = line[6..].trim().to_string();

		monkey_expressions.insert(monkey, expression);
	}

	let part_1_result = evaluate_part_1(&monkey_expressions, &String::from("root"));
	println!("Part 1: {}", part_1_result);

	let part_2_equation = evaluate_part_2(&monkey_expressions, &String::from("root"));
	println!("Part 2: {}", part_2_equation);

}

fn 
evaluate_part_1
(
	monkey_expressions: &HashMap<String, String>,
	monkey_to_evaluate: &String
) 
-> f64
{
	if !monkey_expressions.contains_key(monkey_to_evaluate)
	{
		panic!("Illegal key");
	}

	let expression = monkey_expressions[monkey_to_evaluate].clone();

	if expression.contains(' ')
	{
		let left_monkey = expression[0..4].to_string();
		let operator = expression.chars().nth(5).unwrap();
		let right_monkey = expression[7..11].to_string();

		let left_operand = evaluate_part_1(monkey_expressions, &left_monkey);
		let right_operand = evaluate_part_1(monkey_expressions, &right_monkey);

		match operator
		{
			'+' => return left_operand + right_operand,
			'-' => return left_operand - right_operand,
			'*' => return left_operand * right_operand,
			'/' => return left_operand / right_operand,
			_ => panic!("Illegal operator"),
		}

	}
	else
	{
		return expression.parse::<f64>().unwrap();
	}

}


fn
evaluate_part_2
(
	monkey_expressions: &HashMap<String, String>,
	monkey_to_evaluate: &String
)
-> String
{
	if !monkey_expressions.contains_key(monkey_to_evaluate)
	{
		panic!("Illegal key");
	}

	if monkey_to_evaluate == &String::from("humn")
	{
		return String::from("x");
	}

	let expression = monkey_expressions[monkey_to_evaluate].clone();

	if expression.contains(' ')
	{
		let left_monkey = expression[0..4].to_string();
		let operator = expression.chars().nth(5).unwrap();
		let right_monkey = expression[7..11].to_string();

		let mut left_operand = evaluate_part_2(monkey_expressions, &left_monkey);
		let mut right_operand = evaluate_part_2(monkey_expressions, &right_monkey);

		if !left_operand.contains('x')
		{
			left_operand = format!("{}", evaluate_part_1(monkey_expressions, &left_monkey));
		}
		else
		{
			// println!("Left: {}", left_operand);
			// println!("Right: {}", right_operand);
		}

		if !right_operand.contains('x')
		{
			right_operand = format!("{}", evaluate_part_1(monkey_expressions, &right_monkey));
		}
		else
		{
			// println!("Left: {}", left_operand);
			// println!("Right: {}", right_operand);
		}

		if monkey_to_evaluate == &String::from("root")
		{
			return format!("{}={}", left_operand, right_operand);
		}


		match operator
		{
			'+' => return format!("({}+{})", left_operand, right_operand),
			'-' => return format!("({}-{})", left_operand, right_operand),
			'*' => return format!("({}*{})", left_operand, right_operand),
			'/' => return format!("({}/{})", left_operand, right_operand),
			_ => panic!("Illegal operator"),
		}

	}
	else
	{
		return expression;
	}
}
