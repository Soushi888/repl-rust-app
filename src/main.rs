mod calculator;

use calculator::*;
use std::io::stdin;

fn main() {
	let mut calculator = Calculator::new();
	loop {
		let mut input = String::new();
		stdin().read_line(&mut input).unwrap();
		match calculator.evaluate(&input) {
			Ok(result) => println!("{}", result),
			Err(error) => println!("{:?}", error),
		}
	}
}

