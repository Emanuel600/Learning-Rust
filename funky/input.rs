use std::io;

pub fn operate(num1: f64, op: char, num2: f64) -> f64 {
	match op {
		'+' => num1+num2,
		'-' => num1-num2,
		'*' => num1*num2,
		'/' => num1/num2,
		'^' => num1.powf(num2),
		 _  => panic!("Operator '{op}' is not implemented"),
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

	if op=='^' { return 3; }

	return 0;
}
