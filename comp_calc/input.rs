use std::io;
/* TODO:
 * Implement nested parenthesis => Panics when trying to operate "a)b", and does not follow order of operations
 * Implement non-integers as input
 * Remove unnecessary "println!"s
 * Implement negative numbers (for divisions, mostly)
 */


pub fn get_raw() -> String {
	let mut raw = String::new();

	match io::stdin().read_line(&mut raw){
		Ok(_n) => {
		return raw;
		}
		Err(_error) => panic!("Input could not be read"),
	};
}

pub fn string_eval(func: String) -> f64 {
	let mut values = Vec::<f64>::new();
	let mut operat = Vec::<char>::new();

	let mut i: usize = 0;

	let c: Vec<char> = func.chars().collect();

	while i < func.len()-1{
		if c[i] == ' ' {
			continue;
		}
		else if c[i] == '(' {

			let mut exp = String::new();

			while c[i] != ')' {
				i+=1;

				exp.push(c[i]);

				if i > func.len()-1 {
					panic!("Closing parenthesis not found");
				}
			}
			values.push(string_eval(exp)); // Recursive works better
		}
		else if (c[i]).is_digit(10){
			let mut val = 0.0;

			while (i < func.len()-1) && (c[i].is_digit(10)){
				val= (val*10.0) + (c[i].to_digit(10).unwrap() as f64);
				i += 1;
			}
			i -= 1;
			values.push(val);
		}
/*
		else if c[i] == ')'{ // If I can get this to work it would solve the nested issue 
			println!("{} found in {i}", c[i]);
			while !operat.is_empty() && *(operat.last().unwrap()) ==  '('{
				println!("Loop for '(' entered");

				let num2 = values.pop().unwrap_or(0.0);

				let num1 = values.pop().unwrap_or(0.0);

				let op   = operat.pop().unwrap_or(' ');

				if op != '('{
					values.push(operate(num1, num2, op));
					println!("{num1}{op}{num2}={}", values.last().unwrap());
				}
			}

			if !operat.is_empty(){
				operat.pop();
			}
		}
*/
		else {
			while !(operat.is_empty()) && (precedence( *(operat.last().unwrap_or(&' ') ))
					>= precedence(c[i])){
				let num2 = values.pop().unwrap();

				let num1 = values.pop().unwrap();

				let op   = operat.pop().unwrap();

				values.push(operate(num1, num2, op));
			}

			operat.push(c[i]);

		}
		i+=1;
	}

	while !operat.is_empty() {
		let num2 = values.pop().unwrap_or(0.0);

		let num1 = values.pop().unwrap_or(0.0);

		let op   = operat.pop().unwrap_or(' ');

		values.push(operate(num1, num2, op));
	}

	return *(values.last().unwrap_or(&0.0));
}

pub fn precedence(op: char) -> u8 {
	if op=='+'||op=='-'{
		return 1;
	}
	if op=='*'||op=='/'{
		return 2;
	}
	return 0;
}

pub fn operate(a: f64, b: f64, op: char) -> f64{
	match op {
		'+' => return a+b,
		'-' => return a-b,
		'*' => return a*b,
		'/' => return a/b,
		 _  => panic!("Operation |{op}| not supported"),
	}
}

pub fn read_fun() {
	let inp= get_raw();

	let res= string_eval(inp);

	println!("Reuslt of the expression entered: {}", res);
}
