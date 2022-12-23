use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

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
	println!("Part 2 Equation: {}", part_2_equation);

	let part_2_result = resolve_part_2(&part_2_equation);
	println!("Part 2: {}", part_2_result);

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
		return String::from("(x)");
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

fn 
resolve_part_2
(
	equation: &String
) 
-> f64
{

	let mut left_side = equation.split('=').nth(0).unwrap().trim();
	let mut right_side = equation.split('=').nth(1).unwrap().trim();

	// Make sure that x is on the left side
	// Just a convention of mine
	if right_side.contains('x')
	{
		let temp = left_side;
		left_side = right_side;
		right_side = temp;
	}

	// Parse the value on the right
	let right_side_value = right_side.trim().parse::<f64>().unwrap();

	// Remove parentheses
	let mut left_expression = &left_side[1..left_side.len()-1];
	let mut raw_operand = String::new();
	let operator: char;

	if left_expression.chars().next().unwrap() == '('
	{
		// Numeric operand is on the right
		while left_expression.chars().last().unwrap().is_ascii_digit()
		{
			raw_operand.push(left_expression.chars().last().unwrap());
			left_expression = &left_expression[..left_expression.len()-1];
		}

		operator = left_expression.chars().last().unwrap();
		left_expression = &left_expression[..left_expression.len()-1];

	}
	else if left_expression.chars().last().unwrap() == ')'
	{
		// Numeric operand is on the left
		while left_expression.chars().next().unwrap().is_ascii_digit()
		{
			raw_operand.push(left_expression.chars().next().unwrap());
			left_expression = &left_expression[1..];
		}
		operator = left_expression.chars().next().unwrap();
		left_expression = &left_expression[1..];
	}
	else
	{
		// Expression of form "(x)"
		return right_side_value;
	}

	let left_side_operand = raw_operand.parse::<f64>().unwrap();



	let new_right_side_value = match operator
	{
		'+' => right_side_value - left_side_operand,
		'-' => right_side_value + left_side_operand,
		'*' => right_side_value / left_side_operand,
		'/' => right_side_value * left_side_operand,
		_ => panic!("AH"),
	};

	let new_equation = format!("{} = {}", left_expression, new_right_side_value);

	// println!("Input: {}", equation);
	// println!("left: {}", left_expression);
	// println!("right: {}", right_side);
	// println!("New: {}", new_equation);
	// println!("\n");

	return resolve_part_2(&new_equation);
}