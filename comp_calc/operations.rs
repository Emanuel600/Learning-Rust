/* TODO:
 * Add user input => Will be done on another source file (input.rs)
 * Remove all enums (easier on main with just the "Complex" struct)
 */




#![allow(dead_code)]

use std::fmt;
use std::ops;

#[derive(Clone)]
pub struct Complex {
	real: f64,
	imag: f64,
}

impl fmt::Display for Complex {
	fn fmt(&self, f: &mut fmt:: Formatter) -> fmt::Result {
		if self.imag >= 0.0{
			write!(f, "{} + {}i", self.real, self.imag)
		}
		else{
			write!(f, "{} - {}i", self.real, -self.imag)
		}
	}
}

impl Complex {
	pub fn new(r: f64, i: f64) -> Complex{ // Creates a new complex number
		Complex {real: r, imag: i}
	}

	pub fn get_mag(&self) -> f64{ // Returns magnitude of a complex number
		((self.real).powi(2)+(self.imag).powi(2)).sqrt()
	}
	pub fn get_ang(&self) -> f64{ // Returns the angle (Rad) of a complex number
		let mut angle=0.0;
				let pi = std::f64::consts::PI;

				if self.real != 0.0{
					angle= ((self.imag)/(self.real)).atan();
					if self.real < 0.0{
						angle *= -1.0;
						if self.imag > 0.0{
							angle += pi/2.0;
						}
						else if self.imag < 0.0{
							angle -= pi/2.0;
						}
						else{
							angle = pi;
						}
					}
				}

				else if self.imag == 0.0{
						return 0.0;
					}

				else{
					if self.imag < 0.0{
						return -pi/2.0;
					}
					else{
						return pi/2.0;
					}
				}

				angle
	}
}


/* Implementing regular math operators for complex numbers */
// Addition
impl ops::Add<Complex> for Complex{
	type Output = Complex;

	fn add(self, num1: Complex) -> Complex{
		Complex {real: self.real + num1.real, imag: self.imag + num1.imag}
	}
}
// Subtraction
impl ops::Sub<Complex> for Complex{
	type Output = Complex;

	fn sub(self, num1: Complex) -> Complex{ // Returns (num1-num2)
		Complex {real: self.real - self.real, imag: self.imag - num1.imag}
	}
}
// Multiplication
impl ops::Mul<Complex> for Complex{
	type Output = Complex;

	fn mul(self, num1: Complex) -> Complex{
		let ang = self.get_ang() + num1.get_ang();

		let mag = self.get_mag() * num1.get_mag();

		let r = mag*(ang.cos());
		let i = mag*(ang.sin());

		Complex {real: r, imag: i}
	}
}
// Division
impl ops::Div<Complex> for Complex{
	type Output = Complex;

	fn div(self, num1: Complex) -> Complex{
		let ang = self.get_ang() - num1.get_ang();

		let mag = self.get_mag() / num1.get_mag();

		let r = mag*(ang.cos());
		let i = mag*(ang.sin());

		Complex {real: r, imag: i}
	}
}
