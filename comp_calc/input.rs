use std::io;
/* TODO:
 * Fix parenthesis (it tries to use '(' as an operator, causing the program to panic and crash)
 * Test if the order of operations is always respected
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
			operat.push(c[i]);
		}
		else if (c[i]).is_digit(10){
			let mut val = 0.0;

			while (i < func.len()-1) && (c[i].is_digit(10)){
				val= (val*10.0) + (c[i].to_digit(10).unwrap() as f64);
				i += 1;
			}
			i -= 1;
			values.push(val);
			println!("Value pushed: {}", values.last().unwrap());
		}

		else if c[i] == ')'{
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

		else {
			println!("else called on {}, {i}", c[i]);
			println!("Current last element on operat: {}", operat.last().unwrap_or(&'e'));
			while !(operat.is_empty()) && (precedence( *(operat.last().unwrap_or(&' ') ))
					>= precedence(c[i])){
				println!("While loop entered on {i} iter");
				let num2 = values.pop().unwrap();

				let num1 = values.pop().unwrap();

				let op   = operat.pop().unwrap();

				if op != '('{
					values.push(operate(num1, num2, op));
					println!("Result of operation {num1}{op}{num2}: {}", values.last().unwrap());
				}
			}

			operat.push(c[i]);

			println!("Current last element on operat: {}", operat.last().unwrap_or(&'e'));
		}
		i+=1;
	}

	while !operat.is_empty() {
		let num2 = values.pop().unwrap_or(0.0);

		let num1 = values.pop().unwrap_or(0.0);

		let op   = operat.pop().unwrap_or(' ');

		if op != '('{
			values.push(operate(num1, num2, op));
		}
	}

	return *(values.last().unwrap_or(&0.0));
}

pub fn precedence(op: char) -> u8 {
	println!("precedence called for {op}");
	if op=='+'||op=='-'{
		return 1;
	}
	if op=='*'||op=='/'{
		return 2;
	}
	return 0;
}

pub fn operate(a: f64, b: f64, op: char) -> f64{
	println!("fn operate called for {a}{op}{b}");
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
