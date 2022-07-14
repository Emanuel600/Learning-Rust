/* TODO:
 * Fix implementation of COps::Mul
 * Separate code into multiple sources
 * Implement Div
 * Add user input
 */




#![allow(dead_code)]

use std::fmt;

#[derive(Clone)]
struct Complex {
	real: f64,
	imag: f64,
}

impl fmt::Display for Complex {
	fn fmt(&self, f: &mut fmt:: Formatter) -> fmt::Result {
		write!(f, "{} + ({}i)", self.real, self.imag)
	}
}
// Returns complex numbers
enum COps {
	Add,	// Add two complex numbers
	Sub,	// Subtract two complex numbers
	Mul,	// Return value is extremely odd
//	Div,
}

// Return real values
enum ROps {
	Mag,	// Returns the magnitude of a complex number
	Ang,	// Returns the angle of a complex number
}

impl ROps { // All code here has ben tested and verified
	fn run(&self, num: Complex) -> f64{
		match self {
			Self::Mag => ((num.real).powi(2)+(num.imag).powi(2)).sqrt(),
			Self::Ang => {
				if num.real != 0.0{
					((num.imag)/(num.real)).atan()
				}
				else{
					std::f64::consts::PI/2.0
				}
			},
		}
	}
}

impl COps {
	fn run(&self, num1: Complex, num2: Complex) -> Complex {
		match self {
			Self::Add => Complex {real: num1.real + num2.real, imag: num1.imag + num2.imag},
			Self::Sub => Complex {real: num1.real - num2.real, imag: num1.imag - num2.imag},
			Self::Mul => {
				let op = ROps::Ang;

				let ang = op.run(num1.clone()) + op.run(num2.clone());

				let op = ROps::Mag;

				let mag = op.run(num1.clone()) * op.run(num2.clone());

				// Reuse mag and ang for real and imaginary
				let ang = mag*(ang.sin());
				let mag = mag*(ang.cos());

				Complex {real: mag, imag: ang}
			},
		}
	}
}

fn main() {
	let comp1 = Complex { real: 0.0, imag: 1.0 };
	let comp2 = Complex { real: 51.0, imag: 19.0 };

	let op = ROps::Mag;

	let comp3 = op.run(comp2);

	println!("Complex number: {}", comp3);
}
