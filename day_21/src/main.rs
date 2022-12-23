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

	let part_2_expression = resolve_part_2(&part_2_equation);
	println!("Part 2 Expression: {}", part_2_expression);

	println!("Part 2: {}", evaluate_expression(&part_2_expression));

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

		if !right_operand.contains('x')
		{
			right_operand = format!("{}", evaluate_part_1(monkey_expressions, &right_monkey));
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
-> String
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

	// Remove parentheses
	let mut expression = &left_side[1..left_side.len()-1];

	if expression == String::from("x")
	{
		return right_side.to_string();
	}

	let mut operator = '!';
	let mut depth_counter = 0;
	let mut expression_left = String::new();
	for char in expression.chars()
	{
		if char == '('
		{
			depth_counter += 1;
		}
		if char == ')'
		{
			depth_counter -= 1;
		}

		if depth_counter == 0
		{
			if char != '(' && char != ')' && !char.is_ascii_digit()
			{
				operator = char;
				break;
			}
		}

		expression_left.push(char);
	}

	let expression_right = &expression[expression_left.len()+1..];


	let new_right_side_value: String;
	let new_equation: String;
	if expression_left.contains('x')
	{
		new_right_side_value = match operator
		{
			'+' => format!("({}-{})", right_side, expression_right),
			'-' => format!("({}+{})", right_side, expression_right),
			'*' => format!("({}/{})", right_side, expression_right),
			'/' => format!("({}*{})", right_side, expression_right),
			_ => panic!("AH1"),
		};
		new_equation = format!("{}={}", expression_left, new_right_side_value);
	}
	else
	{
		new_right_side_value = match operator
		{
			'+' => format!("({}-{})", right_side, expression_left),
			'-' => format!("({}-{})", expression_left, right_side),
			'*' => format!("({}/{})", right_side, expression_left),
			'/' => format!("({}/{})", expression_left, right_side),
			_ => panic!("AH2"),
		};
		new_equation = format!("{}={}", expression_right, new_right_side_value);
	}

	return resolve_part_2(&new_equation);
}


fn
evaluate_expression
(
	expression: &String
)
-> f64
{
	if expression.chars().all(|char| char.is_ascii_digit())
	{
		return expression.parse::<f64>().unwrap();
	}

	let new_expression = &expression.as_str()[1..expression.len()-1];
	let mut operator = '!';
	let mut depth_counter = 0;
	let mut expression_left = String::new();
	for char in new_expression.chars()
	{
		if char == '('
		{
			depth_counter += 1;
		}
		if char == ')'
		{
			depth_counter -= 1;
		}

		if depth_counter == 0
		{
			if char != '(' && char != ')' && !char.is_ascii_digit()
			{
				operator = char;
				break;
			}
		}

		expression_left.push(char);
	}
	let expression_right = &new_expression[expression_left.len()+1..];

	let expression_left_value = evaluate_expression(&expression_left);
	let expression_right_value = evaluate_expression(&expression_right.to_string());

	return match operator
	{
		'+' => expression_left_value + expression_right_value,
		'-' => expression_left_value - expression_right_value,
		'*' => expression_left_value * expression_right_value,
		'/' => expression_left_value / expression_right_value,
		_ => panic!("AH3"),
	};
	
}