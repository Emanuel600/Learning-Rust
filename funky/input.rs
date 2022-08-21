use std::io;

enum Variable {}

trait Operate{fn operate(&self, op: char, num: f64) -> f64;}

impl Operate for f64 {
	fn operate(&self, op: char, num: f64) -> f64 {
		println!("Called for {self}{op}{num}");

		match op {
			'+' => self+num,
			'-' => self-num,
			'*' => self*num,
			'/' => self/num,
			'^' => self.powf(num),
			 _  => panic!("Operator '{op}' is not implemented"),
		}
	}
}

pub fn get_raw() -> String{
	let mut raw = String::new();

	match io::stdin().read_line(&mut raw){
		Ok(_n) => raw,
		Err(_error) => panic!("Failure when reading line"),
	}
}

pub fn precedence(op: char) -> u8{
	if (op=='+') | (op=='-') { return 1; }

	if (op=='*') | (op=='/') { return 2; }

	if op=='^' { return 2; }

	return 0;
}

pub fn str_eval(func: String) -> f64{ // Maybe add a conpressor function that slowly reduces the string
	let chars: Vec<char> = func.chars().collect();

	let mut values = Vec::<f64>::new();
	let mut operat = Vec::<char>::new();

	let mut i: usize = 0;

	let size = chars.len();

	while i < size-1 {
		if chars[i] == ' ' { i+=1 ; continue; } // Skip whitespaces

		if chars[i] == '(' {
			let mut block = String::new();
			
			while chars[i] != ')' {
				i += 1;

				block.push(chars[i]);
			}
			values.push(str_eval(block));
		}

		else if chars[i] == ')' { i+=1 ; continue; }

		else if chars[i].is_digit(10) {
			let mut val = 0.0;

			while (i < size-1) & (chars[i].is_digit(10)) { // Get integer part of a number
				val= (val*10.0) + (chars[i].to_digit(10).unwrap() as f64);
				i += 1;
			}

			if (chars[i] == '.') | (chars[i] == ',') {
				let mut cases = -1;	// Number of decimal cases
				let mut decimal = 0.0;	// Value after decimal point

				i += 1;

				while (i < size-1) & (chars[i].is_digit(10)) { // Get decimal part of a number
					decimal= decimal + (chars[i].to_digit(10).unwrap() as f64)*((10_f64).powi(cases));
					cases -= 1;
					i += 1;
				}

				val = val + decimal;
			}
			i -= 1;

			values.push(val);
		}

		else {
			while !(operat.is_empty()) && (precedence( *(operat.last().unwrap_or(&' ') ))
						>= precedence(chars[i])){
				let num2 = values.pop().unwrap();

				let num1 = values.pop().unwrap();

				let op   = operat.pop().unwrap();

				values.push(num1.operate(op, num2));
			}

			operat.push(chars[i]);

		}

		i+=1;
	}

	while !operat.is_empty() {
		let num2 = values.pop().unwrap_or(0.0);

		let num1 = values.pop().unwrap_or(0.0);

		let op   = operat.pop().unwrap_or(' ');

		values.push(num1.operate(op, num2));
	}

	return *(values.last().unwrap_or(&0.0));
}

pub fn call_solver() -> f64 {
	println!("Enter an equation to be solved");

	return str_eval(get_raw());
}






