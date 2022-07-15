/* TODO:
 * Fix implementation of ROps::Ang
 * Implement Div
 * Add user input
 */




#![allow(dead_code)]

use std::fmt;

#[derive(Clone)]
pub struct Complex {
	real: f64,
	imag: f64,
}

impl fmt::Display for Complex {
	fn fmt(&self, f: &mut fmt:: Formatter) -> fmt::Result {
		write!(f, "{} + ({}i)", self.real, self.imag)
	}
}

impl Complex {
	pub fn new(r: f64, i: f64) -> Complex{
		Complex {real: r, imag: i}
	}
}

// Returns complex numbers
pub enum COps {
	Add,	// Add two complex numbers
	Sub,	// Subtract two complex numbers
	Mul,	// Multiplies two complex numbers
//	Div,
}

// Return real values
pub enum ROps {
	Mag,	// Returns the magnitude of a complex number
	Ang,	// Returns the angle of a complex number (only works for first quadrant)
}

impl ROps { // All code here has ben tested and verified
	pub fn run(&self, num: Complex) -> f64{
		match self {
			Self::Mag => ((num.real).powi(2)+(num.imag).powi(2)).sqrt(),
			Self::Ang => {
				if num.real != 0.0{
					((num.imag)/(num.real)).atan()
				}
				else{
					if num.imag < 0.0 {
						-(std::f64::consts::PI/2.0)
					}
					else {
						std::f64::consts::PI/2.0
					}
				}
			},
		}
	}
}

impl COps {
	pub fn run(&self, num1: Complex, num2: Complex) -> Complex {
		match self {
			Self::Add => Complex {real: num1.real + num2.real, imag: num1.imag + num2.imag},
			Self::Sub => Complex {real: num1.real - num2.real, imag: num1.imag - num2.imag},
			Self::Mul => {
				let op = ROps::Ang;

				let ang = op.run(num1.clone()) + op.run(num2.clone());

				let op = ROps::Mag;

				let mag = op.run(num1.clone()) * op.run(num2.clone());

				let r = mag*(ang.cos());
				let i = mag*(ang.sin());

				Complex {real: r, imag: i}
			},
		}
	}
}

