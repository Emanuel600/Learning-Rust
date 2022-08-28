use std::time::Instant;

use crate::input;

pub fn measure_time(){
	let input = input::get_raw();

	let iterations = 5_000_000;

	let start = Instant::now();

	for _i in 0..iterations {
		shunting_yard_parser(&input);
	}

	let finish = Instant::now();

	println!("Time: {:?}", finish.duration_since(start)/iterations);
}

pub fn shunting_yard_parser(input: &String) -> f64 {
	if !input.is_ascii() { panic!("Invalid string format detected"); }

	let chars: Vec<char> = input.chars().collect();

	let mut values = Vec::<f64>::new();
	let mut operat = Vec::<char>::new();

	let mut i: usize = 0;

	let size = input.len();

	while i < size {
		match chars[i] {
			' ' | '\n' =>  { i += 1; continue; } // ignore whitespace

			'(' => { operat.push(chars[i]); i += 1; continue; }

			 _  => {}
		}
		// Gets the numerical values
		if chars[i].is_digit(10){
			let mut val = 0.0;

			while (i < size) & (chars[i].is_digit(10)) {
				val= (val*10.0) + (chars[i].to_digit(10).unwrap() as f64);

				i += 1;
			}
			if (chars[i] == '.') | (chars[i] == ',') {
				let mut decimal = 0.0;
				let mut cases = -1;

				i += 1;

				while (i < size) & (chars[i].is_digit(10)) {
					decimal= decimal + (chars[i].to_digit(10).unwrap() as f64)*((10_f64).powi(cases));
					cases -= 1;

					i += 1;
				}
				val += decimal;
			}
			i -= 1;
			values.push(val);
		} // closes "char.is_digit(10)" {finished reading the number}

		else if chars[i] == ')' {
			while !operat.is_empty() & (*operat.last().unwrap() != '(') {
				let num2 = values.pop().unwrap();
				let num1 = values.pop().unwrap();

				let op   = operat.pop().unwrap();

				values.push(input::operate(num1, op, num2));
			}
			if !operat.is_empty() { operat.pop(); }
		}

		else { // If it's not a digit, it should be an operator or function
			while !( operat.is_empty() | (operat.last() == Some(&'(')) ) & (input::precedence( *(operat.last().unwrap_or(&' ')) )
					>= input::precedence(chars[i]) )
			{ // opens while
				println!("Last operator: {}", operat.last().unwrap_or(&'e'));

				let num2 = values.pop().unwrap();
				let num1 = values.pop().unwrap();

				let op   = operat.pop().unwrap();

				values.push(input::operate(num1, op, num2));
			} // closes while
			operat.push(chars[i]);
		} // closes else

		i += 1; // Moves to next character

	} // closes big while loop

	// Whole thing parsed, if anything remains it should be 2 values and an operator
	while !operat.is_empty() {
		let num2 = values.pop().unwrap();
		let num1 = values.pop().unwrap();

		let op   = operat.pop().unwrap();

		values.push(input::operate(num1, op, num2));
	}

	return *values.last().unwrap();
}

pub fn call_shunt_parser() -> f64 {
	shunting_yard_parser(&input::get_raw())
}
