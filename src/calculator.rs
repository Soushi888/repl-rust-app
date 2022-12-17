pub struct Calculator {
	mem: Option<f64>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
	UnexpectedToken,
	UnexpectedEndOfInput,
}

impl Calculator {
	pub fn new() -> Calculator { Self { mem: None } }

	pub fn evaluate(&mut self, input: &str) -> Result<i32, Error> {
		let result;

		let mem_str: &str = &self.mem.unwrap_or(0.0).to_string();

		let expression = if input.trim().contains("_") {
			input.trim().replace("_", mem_str)
		} else {
			input.trim().to_string()
		};

		let mut operation_elements = expression.split_whitespace();

		if operation_elements.clone().count() != 3usize {
			return Err(Error::UnexpectedEndOfInput);
		}
		let first_number = operation_elements.next().unwrap();
		let operation = operation_elements.next().unwrap();
		let second_number = operation_elements.next().unwrap();

		let first_number = first_number.parse::<i32>().unwrap();
		let second_number = second_number.parse::<i32>().unwrap();

		if operation == "+" {
			result = first_number + second_number;
		} else if operation == "-" {
			result = first_number - second_number;
		} else if operation == "*" {
			result = first_number * second_number;
		} else if operation == "/" {
			result = first_number / second_number;
		} else {
			return Err(Error::UnexpectedToken);
		}

		self.mem = Some(result as f64);
		Ok(result)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn addition() {
		let mut calculator = Calculator::new();
		assert_eq!(calculator.evaluate("1 + 1").unwrap(), 2);
		assert_eq!(calculator.evaluate("2 + 2").unwrap(), 4);
		assert_eq!(calculator.evaluate("3 + 3").unwrap(), 6);
	}

	#[test]
	fn subtraction() {
		let mut calculator = Calculator::new();
		assert_eq!(calculator.evaluate("1 - 1").unwrap(), 0);
		assert_eq!(calculator.evaluate("2 - 2").unwrap(), 0);
		assert_eq!(calculator.evaluate("3 - 3").unwrap(), 0);
	}

	#[test]
	fn multiplication() {
		let mut calculator = Calculator::new();
		assert_eq!(calculator.evaluate("1 * 1").unwrap(), 1);
		assert_eq!(calculator.evaluate("2 * 2").unwrap(), 4);
		assert_eq!(calculator.evaluate("3 * 3").unwrap(), 9);
	}

	#[test]
	fn division() {
		let mut calculator = Calculator::new();
		assert_eq!(calculator.evaluate("1 / 1").unwrap(), 1);
		assert_eq!(calculator.evaluate("2 / 2").unwrap(), 1);
		assert_eq!(calculator.evaluate("3 / 3").unwrap(), 1);
	}

	#[test]
	fn memory() {
		let mut calculator = Calculator::new();
		assert_eq!(calculator.evaluate("1 + 1").unwrap(), 2);
		assert_eq!(calculator.evaluate("2 + 2").unwrap(), 4);
		assert_eq!(calculator.evaluate("3 + 3").unwrap(), 6);
		assert_eq!(calculator.evaluate("_ + 1").unwrap(), 7);
		assert_eq!(calculator.evaluate("_ - 3").unwrap(), 4);
		assert_eq!(calculator.evaluate("_ * 2").unwrap(), 8);
	}
}